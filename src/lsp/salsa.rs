#![allow(non_camel_case_types)]

mod composition;
mod subqueries;

use crate::lsp::{
    idset::{Id, IdSet},
    ink_visitors::{
        doc_symbols::document_symbols as get_document_symbols,
        ws_symbols::from_doc as get_workspace_symbols,
    },
};
use composition::composite_query;
use ink_document::InkDocument;
use lsp_types::{DocumentSymbol, Uri, WorkspaceSymbol};
use mini_milc::{subquery, Db, HasChanged};
use std::collections::HashSet;
use subqueries::node_info::{DefRange, IdentRange, NodeInfos};

pub(crate) type DocId = Id<Uri>;
pub(crate) type DocIds = IdSet<Uri>;

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
        fn node_infos(docid: DocId) -> NodeInfos;
        fn definition_of(docid: DocId, range: IdentRange) -> Vec<(DocId, DefRange)>;
        fn usages_of(docid: DocId, range: DefRange) -> Vec<(DocId, IdentRange)>;
    }
});

// Inputs
subquery!(Ops, document, InkDocument);
subquery!(Ops, doc_ids, DocIds);
subquery!(Ops, opened_docs, HashSet<DocId>);

subquery!(Ops, definition_of, Vec<(DocId, DefRange)>, |self, db| {
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

subquery!(Ops, usages_of, Vec<(DocId, IdentRange)>, |self, db| {
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
