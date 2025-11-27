use super::state::InvalidPosition;
use crate::lsp::{
    document::InkDocument,
    salsa::{salsa_doc_symbols::DocumentSymbolsQ, salsa_ws_symbols::WorkspaceSymbolsQ},
};
use lsp_types::{DocumentSymbol, Uri, WorkspaceSymbol};
use mini_milc::{composite_query, subquery, Cached, Db, HasChanged};
use std::collections::HashSet;

mod salsa_doc_symbols;
mod salsa_ws_symbols;

#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("Could not go to node.")]
pub enum GetNodeError {
    #[display("Node type didn't match")]
    InvalidType,
    PositionOutOfBounds(InvalidPosition),
}

pub(crate) type DocId = Uri;
pub(crate) type Docs = HashSet<DocId>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct WorkspaceDocsQ;

composite_query! {
    #[derive(Hash)]
    pub enum Ops -> OpsV;

    use DocId -> InkDocument,
        WorkspaceDocsQ -> Docs,
        DocumentSymbolsQ -> Option<DocumentSymbol>,
        WorkspaceSymbolsQ -> Option<Vec<WorkspaceSymbol>>;
}

// Inputs
subquery!(Ops, DocId, InkDocument);
subquery!(Ops, WorkspaceDocsQ, Docs);

// Extension traits
pub trait InkGetters: Db<Ops> {
    fn docs(&self) -> Cached<'_, Ops, Docs> {
        self.get(WorkspaceDocsQ)
    }

    fn document(&self, id: DocId) -> Cached<'_, Ops, InkDocument> {
        self.get(id)
    }

    fn document_symbols(&self, id: DocId) -> Cached<'_, Ops, Option<DocumentSymbol>> {
        self.get(salsa_doc_symbols::DocumentSymbolsQ(id))
    }

    fn workspace_symbols(&self, id: DocId) -> Cached<'_, Ops, Option<Vec<WorkspaceSymbol>>> {
        self.get(salsa_ws_symbols::WorkspaceSymbolsQ(id))
    }
}
impl<D: Db<Ops>> InkGetters for D {}

pub trait InkSetters: Db<Ops> {
    fn modify_docs<C: HasChanged>(&mut self, f: impl FnOnce(&mut Docs) -> C) -> bool {
        self.modify(WorkspaceDocsQ, f)
    }

    fn modify_document<C: HasChanged>(
        &mut self,
        id: DocId,
        default: impl FnOnce() -> InkDocument,
        update: impl FnOnce(&mut InkDocument) -> C,
    ) -> bool {
        self.modify_with_default(id, default, update)
    }
}
impl<D: Db<Ops>> InkSetters for D {}

pub struct LspDiagnostic {
    doc: DocId,
    diagnostic: lsp_types::Diagnostic,
}
