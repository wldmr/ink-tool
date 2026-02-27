#![allow(non_camel_case_types)]

mod composition;
mod subqueries;

use crate::lsp::{
    idset::{Id, IdSet},
    ink_visitors::{
        definitions::Defs,
        doc_symbols::document_symbols as get_document_symbols,
        names::{self, Meta},
        ws_symbols::from_doc as get_workspace_symbols,
    },
    location::TextRange,
};
use composition::composite_query;
use ink_document::InkDocument;
use lsp_types::{DocumentSymbol, Position, Range, Uri, WorkspaceSymbol};
use mini_milc::{subquery, Db, HasChanged};
use std::collections::{HashMap, HashSet};
use subqueries::node_info::NodeInfos;

pub(crate) type DocId = Id<Uri>;
pub(crate) type DocIds = IdSet<Uri>;

type Names = Vec<(String, Meta)>;
type WorkspaceNames = HashMap<String, Vec<(DocId, Meta)>>;

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
        fn document_names(id: DocId) -> Names; // TODO: Remove this and workspace_names in favor of `definitions`
        fn workspace_names() -> WorkspaceNames;

        /// All the definitions in this document
        fn definitions(docid: DocId) -> Defs;
        /// For each definition in this file, list all the references to that definition
        fn references(docid: DocId) -> References;

        fn node_infos(docid: DocId) -> NodeInfos;
        fn definition_of(docid: DocId, range: TextRange) -> SalsaLocations;
        fn usages_of(docid: DocId, range: TextRange) -> SalsaLocations;

        /// Resolve the start(!) of an identifier to the place(s) where it is defined
        ///
        /// Empty if unresolved
        // TODO: It feels weird for this to require the the start of the identifier. Too fragile.
        // I'd prefer the NodeId, but that requires infrastructure to resolve those to actual nodes/ranges
        fn resolve_definition(docid: DocId, pos: lsp_types::Position) -> Vec<(DocId, Range)>;
    }
});

// Inputs
subquery!(Ops, document, InkDocument);
subquery!(Ops, doc_ids, DocIds);
subquery!(Ops, opened_docs, HashSet<DocId>);

subquery!(Ops, document_names, Names, |self, db| {
    names::document_names(&db.document(self.id))
});

subquery!(Ops, definitions, Defs, |self, db| {
    let ids = db.doc_ids();
    let uri = ids.get(self.docid).unwrap();
    let doc = db.document(self.docid);
    crate::lsp::ink_visitors::definitions::document_definitions(&uri, &doc)
});

/// For each identifier start position, list all the locations that link to it
type References = HashMap<Position, HashSet<(DocId, Range)>>;
subquery!(Ops, references, References, |self, db| {
    let mut result = References::new();

    // TODO: Think about this, this feels very inefficient. We walk all documents everytime?

    let docs = db.doc_ids();
    for docid in docs.ids() {
        let doc = db.document(docid);
        for usage in doc.usages() {
            for (defdoc, defrange) in db.resolve_definition(docid, usage.range.start).iter() {
                if *defdoc == self.docid {
                    result
                        .entry(defrange.start)
                        .or_default()
                        .insert((docid, usage.range));
                }
            }
        }
    }

    result
});

type SalsaLocations = Vec<(DocId, TextRange)>;

subquery!(Ops, definition_of, SalsaLocations, |self, db| {
    let mut result = Vec::new();
    let infos = db.node_infos(self.docid);

    // Either we find something locally, *or* we look globally.
    // In other words: Local definitions shadow global ones.
    if let Some(local_defs) = infos.local_definitions(self.range) {
        result.extend(local_defs.iter().map(|range| (self.docid, *range)));
    } else if let Some(global_names) = infos.unresolved_names(self.range) {
        let docs = db.doc_ids();
        for docid in docs.ids() {
            let def_info = db.node_infos(docid);
            for global_name in global_names {
                if let Some(global_defs) = def_info.global_ranges(global_name) {
                    result.extend(global_defs.iter().map(|range| (docid, *range)));
                }
            }
        }
    }
    result
});

subquery!(Ops, usages_of, SalsaLocations, |self, db| {
    let mut result = Vec::new();
    let infos = db.node_infos(self.docid);

    // Try locals first, …
    if let Some(usages) = infos.local_usages(self.range) {
        result.extend(usages.iter().map(|range| (self.docid, *range)));
    }

    // A definition might also be visible globally under several names:
    if let Some(global_names) = infos.global_names(self.range) {
        let docs = db.doc_ids();
        for docid in docs.ids() {
            let ref_info = db.node_infos(docid);
            for global_name in global_names {
                if let Some(resolved) = ref_info.unresolved_ranges(global_name) {
                    result.extend(resolved.iter().map(|range| (docid, *range)))
                }
            }
        }
    }

    result
});

subquery!(Ops, workspace_names, WorkspaceNames, |self, db| {
    let mut names = WorkspaceNames::new();
    // liberally clone the things, because we don’t expect many global name clashes
    // (that is, we will mostly encounter each string once and must clone anyway).
    for id in db.doc_ids().ids() {
        for (name, meta) in db.document_names(id).iter().cloned() {
            names.entry(name).or_default().push((id, meta));
        }
    }
    names
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

pub struct LspDiagnostic {
    doc: DocId,
    diagnostic: lsp_types::Diagnostic,
}
