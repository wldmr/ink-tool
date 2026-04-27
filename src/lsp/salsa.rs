#![allow(non_camel_case_types)]

mod composition;
mod subqueries;

pub use crate::lsp::{
    idset::{Id, IdSet},
    ink_visitors::{
        doc_symbols::document_symbols as get_document_symbols,
        ws_symbols::from_doc as get_workspace_symbols,
    },
    location::TextRange,
    salsa::subqueries::{
        diagnostics::{DuplicateDefinitions, DuplicateImports, FileDiagnostics},
        ink_inventory::{InkInventory, Name, NameMap},
        local_resolutions::LocalResolutions,
        story_structure::StoryRoots,
    },
};
use bimap::BiHashMap;
use composition::composite_query;
use ink_document::{
    ids::{DefId, NodeId, UsageId},
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
pub type NodeLocations = BiHashMap<NodeId, TextRange, BuildHasherDefault<IdentityHasher>>;
pub type NodeText = HashMap<NodeId, Name, BuildHasherDefault<IdentityHasher>>;

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
        pub fn ink_inventory(docid: DocId) -> InkInventory;
        pub fn local_resolutions(docid: DocId) -> LocalResolutions;
        pub fn definition(docid: DocId, usg: UsageId) -> Vec<(DocId, DefId)>;
        pub fn usages(docid: DocId, def: DefId) -> Vec<(DocId, UsageId)>;

        pub fn file_globals(docid: DocId) -> NameMap<Vec1<DefId>>;
        /// Locations where global names are defined
        pub fn globals(story: StoryRoot) -> NameMap<Vec1<Def>>;
        /// Inverse of `globals`
        pub fn global_names(story: StoryRoot) -> HashMap<Def, Vec1<Name>>;

        pub fn node_locations(docid: DocId) -> NodeLocations;
        pub fn node_text(docid: DocId) -> NodeText;

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
        .depth_first::<ink_syntax::Usages>()
        .map(|ident| (NodeId::new(ident), doc.lsp_range(ident.range()).into()))
        .collect::<NodeLocations>()
});

subquery!(Ops, node_text, NodeText, |self, db| {
    let mut result = NodeText::default();
    let doc = db.document(self.docid);
    let mut qname: Option<ink_syntax::QualifiedName> = None;
    for node in doc.root().depth_first::<ink_syntax::AllNamed>() {
        use ink_syntax::AllNamed::{Expr, Identifier, QualifiedName};
        use ink_syntax::Expr as Ex;
        match node {
            Identifier(ident) | Expr(Ex::Identifier(ident)) => {
                let range = if let Some(q) = qname {
                    q.start_byte()..ident.end_byte()
                } else {
                    ident.byte_range()
                };
                let name = Name::from(doc.text(range));
                result.insert(NodeId::new(ident), name);
            }
            QualifiedName(q) | Expr(Ex::QualifiedName(q)) => {
                qname = Some(q);
                result.insert(NodeId::new(q), Name::from(doc.node_text(q)));
            }
            _ => qname = None,
        }
    }
    log::debug!("Names in file {:?}: {result:#?}", self.docid);
    result
});

impl mini_milc::Subquery<Ops, NameMap<Vec1<DefId>>> for file_globals {
    fn value(
        &self,
        db: &impl mini_milc::Db<Ops>,
        old: mini_milc::Old<NameMap<Vec1<DefId>>>,
    ) -> mini_milc::Updated<NameMap<Vec1<DefId>>> {
        let mut result = NameMap::default();
        let inv = db.ink_inventory(self.docid);

        for list in &inv.lists {
            result.register(list.name, list.id);
            // List items are globally visible without the preceding list name
            for (item, defs) in &list.items {
                for def in defs {
                    result.register(*item, *def);
                    result.register(format!("{list}.{item}"), *def);
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
                result.register(*name, *def);
            }
        }

        for toplevel in &inv.sections {
            result.register(toplevel.name, toplevel.name_id);
            for (label, defs) in &toplevel.body.labels {
                // Subsection names take precedence over labels
                if !toplevel.sub_names.contains(label) {
                    for def in defs {
                        result.register(format!("{toplevel}.{label}"), *def);
                    }
                }
            }

            for subsection in &toplevel.subsections {
                result.register(format!("{toplevel}.{subsection}"), subsection.name_id);
                for (label, defs) in &subsection.body.labels {
                    for def in defs {
                        result.register(format!("{toplevel}.{subsection}.{label}"), *def);
                        // The "shortcut" name exists if the knot itself doesn't define that label already.
                        if !toplevel.body.labels.contains_key(label) {
                            result.register(format!("{toplevel}.{label}"), *def);
                        }
                    }
                }
            }
        }
        old.update(result)
    }
}

subquery!(Ops, globals, NameMap<Vec1<Def>>, |self, db| {
    let mut result = NameMap::default();
    let stories = db.stories();
    for docid in stories[&self.story].resolved.keys() {
        let globals = db.file_globals(*docid);
        for (name, defs) in globals.iter() {
            result.register_extend(*name, defs.into_iter().map(|def| (*docid, *def)));
        }
    }
    result
});

type DefMap = HashMap<Def, Vec1<Name>>; // Seems like rust-analyze can't format the following without this alias.
subquery!(Ops, global_names, DefMap, |self, db| {
    let mut result = HashMap::new();
    let globals = db.globals(self.story);
    for (name, defs) in globals.iter() {
        for def in defs {
            result.register(*def, *name);
        }
    }
    result
});

subquery!(Ops, definition, Vec<(DocId, DefId)>, |self, db| {
    let mut result = Vec::new();
    let local = db.local_resolutions(self.docid);

    // Either we find something locally, *or* we look globally.
    // In other words: Local definitions shadow global ones.

    if let Some(local_defs) = local.definitions.get(&self.usg) {
        result.extend(local_defs.iter().map(|range| (self.docid, *range)));
    } else {
        let text = db.node_text(self.docid);
        if let Some(name) = text.get(self.usg.as_ref()) {
            let roots = db.stories_of(self.docid);

            for root in roots.iter() {
                let globals = db.globals(*root);
                if let Some(global_defs) = globals.get(name) {
                    result.extend(global_defs);
                }
            }
        }
    }

    result
});

subquery!(Ops, usages, Vec<Usg>, |self, db| {
    let mut result = Vec::new();
    let local = db.local_resolutions(self.docid);
    // Try locals first, …
    if let Some(locals) = local.usages.get(&self.def) {
        result.extend(locals.iter().map(|it| (self.docid, *it)));
    }

    // A definition might also be visible globally under several names:
    let my_stories = db.stories_of(self.docid);
    let stories = db.stories();
    for story in my_stories.iter() {
        let global_names = db.global_names(*story);
        let Some(my_names) = global_names.get(&(self.docid, self.def)) else {
            continue;
        };
        let story_files = stories
            .get(story)
            .expect("Story must have at least one file");
        for (file, _) in &story_files.resolved {
            let res = db.local_resolutions(*file);
            for name in my_names {
                if let Some(refs) = res.unresolved.get(name) {
                    result.extend(refs.into_iter().map(|it| (*file, *it)));
                }
            }
        }
    }
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
