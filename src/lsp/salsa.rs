#![allow(non_camel_case_types)]

mod composition;
mod subqueries;

use crate::lsp::salsa::subqueries::ink_inventory::IMap;
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
use derive_more::derive::Deref;
use ink_document::{
    ids::{DefId, NodeId, UsageId},
    InkDocument,
};
use itertools::Itertools as _;
use lsp_types::{DocumentSymbol, Uri, WorkspaceSymbol};
use mini_milc::{subquery, Db, HasChanged};
use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasherDefault,
    ops::Index,
};
pub(crate) use subqueries::node_flags::match_flags;
pub use subqueries::node_flags::{NodeFlag, NodeFlags};
pub use subqueries::story_structure::StoryRoot;
use tree_traversal::TreeTraversal;
use type_sitter::Node as _;
use ustr::IdentityHasher;
use util::nonempty::Vec1;

pub type DocId = Id<Uri>;
pub type DocIds = IdSet<Uri>;
pub type Def = (DocId, DefId);
pub type Usg = (DocId, UsageId);
pub type NodeText = IMap<NodeId, Name>;

#[derive(Debug, Default, PartialEq, Eq, Deref)]
pub struct NodeLocations(BiHashMap<NodeId, TextRange, BuildHasherDefault<IdentityHasher>>);

impl<I: Into<NodeId>> Index<I> for NodeLocations {
    type Output = TextRange;
    fn index(&self, index: I) -> &Self::Output {
        self.0.get_by_left(&index.into()).unwrap()
    }
}

impl<'a> Index<&'a TextRange> for NodeLocations {
    type Output = NodeId;
    fn index(&self, index: &'a TextRange) -> &Self::Output {
        self.0.get_by_right(index).unwrap()
    }
}

impl FromIterator<(NodeId, TextRange)> for NodeLocations {
    fn from_iter<T: IntoIterator<Item = (NodeId, TextRange)>>(iter: T) -> Self {
        NodeLocations(iter.into_iter().collect())
    }
}

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

        // = "Logical" information about nodes =
        // (based on tree-sitter node ids)
        pub fn ink_inventory(docid: DocId) -> InkInventory;
        pub fn local_resolutions(docid: DocId) -> LocalResolutions;
        pub fn definition(docid: DocId, usg: UsageId) -> Vec<(DocId, DefId)>;
        pub fn usages(docid: DocId, def: DefId) -> Vec<(DocId, UsageId)>;

        /// Locations where global names are defined in this file.
        pub fn file_globals(docid: DocId) -> NameMap<Vec1<DefId>>;
        /// Locations where global names are defined in this story.
        pub fn globals(story: StoryRoot) -> NameMap<Vec1<Def>>;
        /// Inverse of `globals`
        pub fn global_names(story: StoryRoot) -> HashMap<Def, Vec1<Name>>;

        // = "Physical" information about nodes =
        // (relates node-ids to user-visible things like text and locations)
        fn node_locations(docid: DocId) -> NodeLocations;
        fn node_text(docid: DocId) -> NodeText;
        fn node_flags(docid: DocId) -> NodeFlags;
        /// Which files contain mention of specific names (global or local)
        fn names_mentioned(story: StoryRoot) -> IMap<Name, Vec<DocId>>;

        /// The longest prefix that all Uris share
        fn common_path_prefix() -> String;
        /// The path without the common prefix
        fn short_path(id: DocId) -> String;

        // === Story Structure ===
        /// The files imported by this file. The text range is the location in the file.
        fn imported_files(docid: DocId) -> Vec<(Name, TextRange)>;

        /// All the story roots in the project
        fn stories() -> StoryRoots;

        /// All the stories that this file is contained in.
        fn stories_of(docid: DocId) -> Vec1<StoryRoot>;

        // === Errors / Diagnostics ===
        /// Global definitons can't be named the same
        pub fn duplicate_globals(story: StoryRoot) -> DuplicateDefinitions;
        /// VARs are even more annoying: They clash with locals as well!
        pub fn var_clash(story: StoryRoot) -> DuplicateDefinitions;
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
    use ink_syntax::AllNamed::*;
    let doc = db.document(self.docid);
    doc.root()
        // XXX: ink_syntax::Usages doesn't work here, because it doesn't capture Expr(Identifier(_)) and Expr(QualifiedName(_)).
        // I consider this a bug, but I'm not sure if the culprit is tree_sitter_ink or type_sitter.
        .depth_first::<ink_syntax::AllNamed>()
        .filter(|node| {
            matches!(
                node,
                Ink(_)
                    | KnotBlock(_)
                    | StitchBlock(_)
                    | Identifier(_)
                    | QualifiedName(_)
                    | Expr(ink_syntax::Expr::Identifier(_))
                    | Expr(ink_syntax::Expr::QualifiedName(_))
            )
        })
        .map(|node| (node.into(), doc.lsp_range(node.range()).into()))
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
                result.insert(ident.into(), doc.text(range).into());
            }
            QualifiedName(q) | Expr(Ex::QualifiedName(q)) => {
                qname = Some(q);
                result.insert(q.into(), doc.node_text(q).into());
            }
            _ => qname = None,
        }
    }
    result
});

subquery!(Ops, names_mentioned, IMap<Name, Vec<DocId>>, |self, db|{
    let mut result = IMap::<Name, Vec<DocId>>::default();
    for file in db.stories()[&self.story].resolved.keys().copied() {
        for name in db.node_text(file).values().copied(){
            result.entry(name).or_default().push(file);
        }
    }
    result
});

subquery!(Ops, imported_files, Vec<(Name, TextRange)>, |self, db| {
    let doc = db.document(self.docid);
    doc.root()
        .depth_first::<ink_syntax::Include>()
        .map(|incl| {
            let node = incl.path();
            let path = doc.text(node.byte_range());
            let range = doc.lsp_range(node.range());
            (Name::from(path), TextRange::from(range))
        })
        .collect_vec()
});

subquery!(Ops, definition, Vec<Def>, |self, db| {
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
