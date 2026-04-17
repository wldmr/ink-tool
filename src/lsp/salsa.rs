#![allow(non_camel_case_types)]

mod composition;
mod subqueries;

use crate::lsp::{
    idset::{Id, IdSet},
    ink_visitors::{
        doc_symbols::document_symbols as get_document_symbols,
        ws_symbols::from_doc as get_workspace_symbols,
    },
    location::TextRange,
    salsa::subqueries::{
        diagnostics::{DuplicateDefinitions, DuplicateImports, FileDiagnostics},
        ink_inventory::{InkInventory, Name, NameMap, NameSet, Section, SectionKind},
        story_structure::StoryRoots,
    },
};
use composition::composite_query;
use ink_document::{
    ids::{DefId, NodeId, ScopeId, UsageId},
    InkDocument,
};
use itertools::Itertools;
use lsp_types::{DocumentSymbol, Uri, WorkspaceSymbol};
use mini_milc::{subquery, Db, HasChanged};
use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasherDefault,
};
pub(crate) use subqueries::node_info::match_flags;
pub use subqueries::node_info::{DefRange, IdentRange, NodeFlag, NodeInfos};
pub use subqueries::story_structure::StoryRoot;
use tree_traversal::TreeTraversal;
use type_sitter::Node as _;
use ustr::IdentityHasher;
use util::nonempty::{MapOfNonEmpty, Vec1};

pub type DocId = Id<Uri>;
pub type DocIds = IdSet<Uri>;
pub type Def = (DocId, DefId);
pub type Usg = (DocId, UsageId);
pub type NodeLocations = HashMap<NodeId, TextRange, BuildHasherDefault<IdentityHasher>>;

composite_query!({
    pub impl Ops<OpsV, InkGetters> {
        // === Inputs ===
        fn document(id: DocId) -> InkDocument;

        fn doc_ids() -> DocIds;
        fn opened_docs() -> HashSet<DocId>;

        // === Leaf Queries ===
        fn document_symbols(id: DocId) -> Vec<DocumentSymbol>;
        fn workspace_symbols(id: DocId) -> Vec<WorkspaceSymbol>;

        // === Intermediate Queries ===
        fn ink_inventory(docid: DocId) -> InkInventory;
        fn node_locations(docid: DocId) -> NodeLocations;

        fn local_defs(docid: DocId, scope: ScopeId, name: Name) -> Vec<DefId>;

        /// The globally visible simple names, without the namespaced names they contain.
        fn global_defs(story: StoryRoot) -> NameMap<Vec1<Def>>;

        /// The globally visible names defined by `defid`.
        ///
        /// This includes knots, toplevel stitches, lists …
        fn global_namespaced_defs(docid: DocId, defid: DefId) -> NameMap<Vec1<DefId>>;

        fn usages(docid: DocId, node: DefId) -> Vec<Usg>;

        fn node_infos(docid: DocId) -> NodeInfos;
        fn definition_of(docid: DocId, range: IdentRange) -> Vec<(DocId, DefRange)>;
        fn usages_of(docid: DocId, range: DefRange) -> Vec<(DocId, IdentRange)>;

        /// The longest prefix that all Uris share
        fn common_path_prefix() -> String;
        /// The path without the common prefix
        fn short_path(id: DocId) -> String;

        // === Story Structure ===

        /// All the story roots in the project
        fn stories() -> StoryRoots;

        /// All the stories that this file is contained in.
        fn stories_of(docid: DocId) -> Vec1<StoryRoot>;

        // === Errors / Diagnostics ===
        pub fn duplicate_definitions(story: StoryRoot) -> DuplicateDefinitions;
        pub fn duplicate_imports(story: StoryRoot) -> DuplicateImports;
        pub fn file_diagnostics(docid: DocId) -> FileDiagnostics;
    }
});

// Inputs
subquery!(Ops, document, InkDocument);
subquery!(Ops, doc_ids, DocIds);
subquery!(Ops, opened_docs, HashSet<DocId>);

subquery!(Ops, common_path_prefix, String, |self, db| {
    db.doc_ids()
        .values()
        .map(|it| it.path().to_string())
        .reduce(|a, b| {
            a.chars()
                .zip(b.chars())
                .take_while(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect()
        })
        .unwrap_or_default()
});

subquery!(Ops, short_path, String, |self, db| {
    let prefix = db.common_path_prefix().len();
    let ids = db.doc_ids();
    let path = ids[self.id].path().as_str();
    path[prefix..].to_string()
});

subquery!(Ops, node_locations, NodeLocations, |self, db| {
    let doc = db.document(self.docid);
    doc.root()
        .depth_first::<ink_syntax::AllNamed>()
        .filter(|it| {
            it.as_identifier().is_some()
                || it.as_knot_block().is_some()
                || it.as_stitch_block().is_some()
        })
        .map(|ident| (NodeId::new(ident), doc.lsp_range(ident.range()).into()))
        .collect::<NodeLocations>()
});

impl mini_milc::Subquery<Ops, NameMap<Vec1<Def>>> for global_defs {
    fn value(
        &self,
        db: &impl mini_milc::Db<Ops>,
        old: mini_milc::Old<NameMap<Vec1<(DocId, DefId)>>>,
    ) -> mini_milc::Updated<NameMap<Vec1<(DocId, DefId)>>> {
        let mut result = NameMap::default();
        let stories = db.stories();
        for docid in stories[&self.story].resolved.keys().copied() {
            let inv = db.ink_inventory(docid);

            for list in &inv.lists {
                result.register(list.name, (docid, list.id));
                // List items are globally visible without the preceding list name
                for (item, defs) in &list.items {
                    for def in defs {
                        result.register(*item, (docid, *def));
                    }
                }
            }

            let globals = std::iter::empty() // just so the chain looks more uniform ;)
                .chain(&inv.vars)
                .chain(&inv.consts)
                .chain(&inv.externals)
                .chain(&inv.body.labels);

            for (name, defs) in globals {
                for def in defs {
                    result.register(*name, (docid, *def));
                }
            }

            for toplevel in &inv.sections {
                result.register(toplevel.name, (docid, toplevel.name_id));
            }
        }
        old.update(result)
    }
}

impl mini_milc::Subquery<Ops, NameMap<Vec1<DefId>>> for global_namespaced_defs {
    fn value(
        &self,
        db: &impl mini_milc::Db<Ops>,
        old: mini_milc::Old<NameMap<Vec1<DefId>>>,
    ) -> mini_milc::Updated<NameMap<Vec1<DefId>>> {
        let mut result = NameMap::default();
        let inv = db.ink_inventory(self.docid);
        for list in &inv.lists {
            if list.id == self.defid {
                for (name, defs) in &list.items {
                    for def in defs {
                        result.register(*name, *def);
                    }
                }
            }
        }
        let section = inv.sections.iter().find(|it| it.name_id == self.defid);
        if let Some(section) = section {
            let sec = section.name;

            for (label, defs) in &section.body.labels {
                // Subsection names take precedence over labels
                if !section.sub_names.contains(label) {
                    for def in defs {
                        result.register(format!("{sec}.{label}"), *def);
                    }
                }
            }

            for subsection in &section.subsections {
                let sub = subsection.name;
                result.register(format!("{sec}.{sub}"), subsection.name_id);
                for (label, defs) in &subsection.body.labels {
                    for def in defs {
                        result.register(format!("{sec}.{sub}.{label}"), *def);
                        // The "shortcut" name exists if the knot itself doesn't define that label already.
                        if !section.body.labels.contains_key(label) {
                            result.register(format!("{sec}.{label}"), *def);
                        }
                    }
                }
            }
        }

        old.update(result)
    }
}

impl mini_milc::Subquery<Ops, Vec<DefId>> for local_defs {
    fn value(
        &self,
        db: &impl mini_milc::Db<Ops>,
        old: mini_milc::Old<Vec<DefId>>,
    ) -> mini_milc::Updated<Vec<DefId>> {
        let mut result = Vec::new();
        let inv = db.ink_inventory(self.docid);

        if self.scope == inv.scope_id {
            // The file scope can only contain temps locally.
            if let Some(defs) = inv.body.temps.get(&self.name) {
                result.extend(defs);
            }
        } else {
            let section = inv
                .sections
                .iter()
                .find(|it| it.scope_id == self.scope || it.subscopes.contains_key(&self.scope));
            if let Some(section) = section {
                if section.scope_id == self.scope {
                    // We're in the Knot body.
                    if let Some(temps) = section.body.temps.get(&self.name) {
                        result.extend(temps);
                    }
                    if let Some(params) = section.params.get(&self.name) {
                        result.extend(params);
                    }
                } else {
                    // We're in a specfic stitch body.
                    let idx = section.subscopes[&self.scope];
                    let sub = &section.subsections[idx];
                    if let Some(temps) = sub.body.temps.get(&self.name) {
                        result.extend(temps);
                    }
                    if let Some(params) = sub.params.get(&self.name) {
                        result.extend(params);
                    }
                }
                // Subsection and label meanings are the same for the body and subscopes
                // (Except they're not, but close enough. FIX later.)
                for subsection in &section.subsections {
                    if subsection.name == self.name {
                        result.push(subsection.name_id);
                    }
                    for (label, defs) in &subsection.body.labels {
                        if *label == self.name {
                            result.extend(defs);
                        }
                    }
                }
            }
        }
        old.update(result)
    }
}

subquery!(Ops, usages, Vec<Usg>, |self, db| {
    let mut result = Vec::new();
    let infos = db.node_infos(self.docid);

    // Try locals first, …

    // A definition might also be visible globally under several names:
    result
});

subquery!(Ops, definition_of, Vec<(DocId, DefRange)>, |self, db| {
    let mut result = Vec::new();
    let infos = db.node_infos(self.docid);

    // Either we find something locally, *or* we look globally.
    // In other words: Local definitions shadow global ones.
    if let Some(local_defs) = infos.local_definitions(self.range) {
        result.extend(local_defs.iter().map(|range| (self.docid, *range)));
    } else if let Some(global_names) = infos.unresolved_names(self.range) {
        let stories = db.stories();
        let roots = db.stories_of(self.docid);

        let targets = roots
            .iter()
            .flat_map(|root| stories[&root].resolved.keys())
            .unique() // might have picked up duplicates if we are in multple overlapping stories;
            .copied();

        for target in targets {
            let def_info = db.node_infos(target);
            for global_name in global_names {
                if let Some(global_defs) = def_info.global_ranges(global_name) {
                    result.extend(global_defs.iter().map(|range| (target, *range)));
                }
            }
        }
    }
    result
});

subquery!(Ops, usages_of, Vec<(DocId, IdentRange)>, |self, db| {
    let mut result = Vec::new();
    let infos = db.node_infos(self.docid);

    // Try locals first, …
    if let Some(usages) = infos.local_usages(self.range) {
        result.extend(usages.iter().map(|range| (self.docid, *range)));
    }

    // A definition might also be visible globally under several names:
    if let Some(global_names) = infos.global_names(self.range) {
        let stories = db.stories();
        let roots = db.stories_of(self.docid);

        let targets = roots
            .iter()
            .flat_map(|root| stories[&root].resolved.keys())
            .unique() // might have picked up duplicates if we are in multple overlapping stories;
            .copied();

        for target in targets {
            let ref_info = db.node_infos(target);
            for global_name in global_names {
                if let Some(resolved) = ref_info.unresolved_ranges(global_name) {
                    result.extend(resolved.iter().map(|range| (target, *range)))
                }
            }
        }
    }

    result
});

subquery!(Ops, workspace_symbols, Vec<WorkspaceSymbol>, |self, db| {
    let docs = db.doc_ids();
    let doc = db.document(self.id);
    let uri = docs.get(self.id).unwrap();
    get_workspace_symbols(uri, &doc)
});

subquery!(Ops, document_symbols, Vec<DocumentSymbol>, |self, db| {
    get_document_symbols(&db.document(self.id))
});

pub trait InkSetters: Db<Ops> {
    fn modify_opened<C: HasChanged>(&mut self, f: impl FnOnce(&mut HashSet<DocId>) -> C) -> bool {
        self.modify(opened_docs {}, f)
    }

    fn modify_docs<C: HasChanged>(&mut self, f: impl FnOnce(&mut DocIds) -> C) -> bool {
        self.modify(doc_ids {}, f)
    }

    fn modify_document<C: HasChanged>(
        &mut self,
        id: DocId,
        default: impl FnOnce() -> InkDocument,
        update: impl FnOnce(&mut InkDocument) -> C,
    ) -> bool {
        self.modify_with_default(document { id }, default, update)
    }
}
impl<D: InkGetters> InkSetters for D {}
