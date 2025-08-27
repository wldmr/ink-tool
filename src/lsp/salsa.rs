use super::state::InvalidPosition;
use crate::{
    ink_syntax::Visitor,
    lsp::{
        document::InkDocument,
        salsa::{salsa_doc_symbols::DocumentSymbols, salsa_ws_symbols::WorkspaceSymbols},
    },
};
use lsp_types::{DocumentSymbol, Uri, WorkspaceSymbol};
use milc::{Db, Input, Query};
use std::collections::BTreeSet;

mod salsa_doc_symbols;
mod salsa_ws_symbols;

#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("Could not go to node.")]
pub enum GetNodeError {
    #[display("Node type didn't match")]
    InvalidType,
    PositionOutOfBounds(InvalidPosition),
}

pub(crate) type Docs = BTreeSet<DocId>;

#[derive(Debug, Hash, Clone, Copy)]
pub(crate) struct Workspace;

impl Input<Docs> for Workspace {}

/// Ad-hoc wrapper for foreign types so we can implement queries for them.
#[derive(
    Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::From, derive_more::Into,
)]
pub(crate) struct DocId(Uri);

impl DocId {
    pub(crate) fn uri(&self) -> &Uri {
        &self.0
    }
}

impl Input<InkDocument> for DocId {}

impl Query<Option<DocumentSymbol>> for DocId {
    fn value(&self, db: &impl Db) -> Option<DocumentSymbol> {
        let doc = db.get::<InkDocument>(self);
        let mut syms = DocumentSymbols::new(&doc, false);
        let mut cursor = doc.tree.root_node().walk();
        let syms = syms.traverse(&mut cursor)?;
        syms.sym
    }
}

pub(crate) fn workspace_symbols<'d>(db: &'d impl Db) -> Vec<WorkspaceSymbol> {
    db.get::<Docs>(&Workspace)
        .iter()
        .flat_map(|docid| db.get::<Option<Vec<WorkspaceSymbol>>>(docid).clone())
        .flat_map(|them| them)
        .collect()
}

impl Query<Option<Vec<WorkspaceSymbol>>> for DocId {
    fn value(&self, db: &impl Db) -> Option<Vec<WorkspaceSymbol>> {
        let doc = db.get::<InkDocument>(self);
        let mut syms = WorkspaceSymbols::new(&self.0, &doc);
        let mut cursor = doc.tree.root_node().walk();
        syms.traverse(&mut cursor); // TODO: inconsistent usage with document symbols
        Some(syms.sym)
    }
}

pub type Name = String;

pub struct LspDiagnostic {
    doc: InkDocument,
    diagnostic: lsp_types::Diagnostic,
}
