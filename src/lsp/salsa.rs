#![allow(non_camel_case_types)]

mod composition;

use crate::lsp::{
    idset::{Id, IdSet},
    ink_visitors::{
        doc_symbols::document_symbols as get_document_symbols,
        globals::Globals,
        names::{self, Meta},
        ws_symbols::from_doc as get_workspace_symbols,
    },
};
use composition::composite_query;
use ink_document::InkDocument;
use lsp_types::{DocumentSymbol, Uri, WorkspaceSymbol};
use mini_milc::{subquery, Cached, Db, HasChanged};
use std::collections::{HashMap, HashSet};

pub(crate) type DocId = Id<Uri>;
pub(crate) type DocIds = IdSet<Uri>;

type Names = Vec<(String, Meta)>;
type WorkspaceNames = HashMap<String, Vec<(DocId, Meta)>>;

composite_query!({
    pub impl Ops<OpsV, InkGetters> {
        fn document(id: DocId) -> InkDocument;
        fn doc_ids() -> DocIds;
        fn opened_docs() -> HashSet<DocId>;
        // fn definition_of(id: LocId) -> LspResult<LocId>;
        fn document_names(id: DocId) -> Names;
        fn document_symbols(id: DocId) -> Vec<DocumentSymbol>;
        fn workspace_symbols(id: DocId) -> Vec<WorkspaceSymbol>;
        fn workspace_names() -> WorkspaceNames;
        fn locals(id: DocId) -> Locals;
        fn globals(id: DocId) -> Globals;
    }
});

// Inputs
subquery!(Ops, document, InkDocument);
subquery!(Ops, doc_ids, DocIds);
subquery!(Ops, opened_docs, HashSet<DocId>);

subquery!(Ops, document_names, Names, |self, db| {
    names::document_names(&db.document(self.id))
});

subquery!(Ops, locals, Locals, |self, db| {
    let doc = db.document(self.id);
    crate::lsp::ink_visitors::locals::locals(&doc)
});

subquery!(Ops, globals, Globals, |self, db| {
    let doc = db.document(self.id);
    crate::lsp::ink_visitors::globals::globals(&doc)
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
