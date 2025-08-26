use super::state::InvalidPosition;
use crate::{
    ink_syntax::Visitor,
    lsp::{
        document::InkDocument,
        salsa::{salsa_doc_symbols::DocumentSymbols, salsa_ws_symbols::WorkspaceSymbols},
    },
};
use line_index::WideEncoding;
use lsp_types::{DocumentSymbol, Uri, WorkspaceSymbol};
use milc::{Db, Input, Query};
use std::collections::BTreeMap;

mod salsa_doc_symbols;
mod salsa_ws_symbols;

#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("Could not go to node.")]
pub enum GetNodeError {
    #[display("Node type didn't match")]
    InvalidType,
    PositionOutOfBounds(InvalidPosition),
}

pub(crate) type Docs = BTreeMap<Uri, InkDocument>;

#[derive(Debug, Hash, Clone, Copy)]
pub(crate) struct Workspace;

impl Workspace {
    pub(crate) fn new(db: &mut impl Db, enc: Option<WideEncoding>) -> Self {
        Workspace.set(db, enc);
        Workspace
    }
}

impl Input<Docs> for Workspace {}
impl Input<Option<WideEncoding>> for Workspace {}

/// Ad-hoc wrapper for foreign types so we can implement queries for them.
#[derive(Debug, Hash, Clone, derive_more::From)]
pub(crate) struct My<T>(pub(crate) T);

impl Query<Option<DocumentSymbol>> for My<Uri> {
    fn value(&self, db: &impl Db) -> Option<DocumentSymbol> {
        let docs = db.get::<Docs>(&Workspace);
        let doc = docs.get(&self.0)?;
        let mut syms = DocumentSymbols::new(doc, false);
        let mut cursor = doc.tree.root_node().walk();
        let syms = syms.traverse(&mut cursor)?;
        syms.sym
    }
}

pub(crate) fn workspace_symbols<'d>(db: &'d impl Db) -> Vec<WorkspaceSymbol> {
    db.get::<Docs>(&Workspace)
        .keys()
        .cloned()
        .flat_map(|uri| db.get::<Option<Vec<WorkspaceSymbol>>>(&My(uri)).clone())
        .flat_map(|them| them)
        .collect()
}

impl Query<Option<Vec<WorkspaceSymbol>>> for My<Uri> {
    fn value(&self, db: &impl Db) -> Option<Vec<WorkspaceSymbol>> {
        let doc = db.get::<Docs>(&Workspace);
        let doc = doc.get(&self.0)?;
        let mut syms = WorkspaceSymbols::new(&self.0, doc);
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
