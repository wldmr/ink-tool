use super::state::InvalidPosition;
use crate::{
    ink_syntax::Visitor,
    lsp::{
        document::InkDocument,
        salsa::{
            salsa_doc_symbols::DocumentSymbols as DocSymVisitor,
            salsa_ws_symbols::WorkspaceSymbols as WsSymVisitor,
        },
    },
};
use itertools::Itertools;
use lsp_types::{DocumentSymbol, Uri, WorkspaceSymbol};
use mini_milc::{composite_query, subquery, Db};
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
#[derive(Default, Debug, derive_more::Deref, derive_more::DerefMut)]
pub(crate) struct Docs(HashSet<DocId>);

composite_query! {
    #[derive(Hash)]
    pub(crate) enum Ops -> OpsV;

    #[derive(Hash)]
    struct Workspace -> Docs {;}

    #[derive(Hash)]
    struct Document -> InkDocument {(pub DocId);}

    #[derive(Hash)]
    struct DocumentSymbols -> Option<DocumentSymbol> {(pub DocId);}

    #[derive(Hash)]
    struct WorkspaceSymbols -> Option<Vec<WorkspaceSymbol>> {(pub DocId);}
}

impl PartialEq for Docs {
    fn eq(&self, other: &Self) -> bool {
        // :(
        self.0
            .iter()
            .sorted_unstable()
            .zip(other.0.iter().sorted_unstable())
            .all(|(a, b)| a == b)
    }
}

subquery!(Ops, Workspace, Docs);
subquery!(Ops, Document, InkDocument);

subquery!(Ops, DocumentSymbols, Option<DocumentSymbol>, |self, db| {
    let doc = db.get(Document(self.0.clone()));
    let mut syms = DocSymVisitor::new(&*doc, false);
    let mut cursor = doc.tree.root_node().walk();
    let syms = syms.traverse(&mut cursor).unwrap();
    syms.sym
});

subquery!(
    Ops,
    WorkspaceSymbols,
    Option<Vec<WorkspaceSymbol>>,
    |self, db| {
        let ws = db.get(Workspace);
        let doc = db.get(Document(self.0.clone()));
        let uri = ws.get(&self.0).unwrap();
        let mut syms = WsSymVisitor::new(uri, &*doc);
        let mut cursor = doc.tree.root_node().walk();
        syms.traverse(&mut cursor);

        Some(syms.sym)
    }
);

pub(crate) fn workspace_symbols(db: &impl Db<Ops>) -> Vec<WorkspaceSymbol> {
    db.get(Workspace)
        .iter()
        .flat_map(|uri| db.get(WorkspaceSymbols(uri.clone())).clone())
        .flat_map(|them| them)
        .collect()
}

pub type Name = String;

pub struct LspDiagnostic {
    doc: DocId,
    diagnostic: lsp_types::Diagnostic,
}
