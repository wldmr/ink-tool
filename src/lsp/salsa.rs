use super::location::Location;
use crate::ink_syntax::Visitor as _;
use line_index::{LineIndex, WideEncoding};
use lsp_types::{DocumentSymbol, Uri, WorkspaceSymbol};
use std::collections::HashMap;

mod salsa_doc_symbols;
mod salsa_locations;
mod salsa_ws_symbols;

#[salsa::db]
pub(crate) trait Db: salsa::Database {}

#[salsa::db]
impl Db for DbImpl {}

#[salsa::db]
#[derive(Clone)]
pub(crate) struct DbImpl {
    storage: salsa::Storage<Self>,
}

impl DbImpl {
    pub(crate) fn new() -> Self {
        Self {
            storage: Default::default(),
        }
    }
}

#[salsa::db]
impl salsa::Database for DbImpl {
    fn salsa_event(&self, event: &dyn Fn() -> salsa::Event) {
        let _event = event();
        // eprintln!("Event: {event:?}");
    }
}

#[salsa::interned]
pub(crate) struct File<'db> {
    uri: lsp_types::Uri,
}

#[salsa::input]
pub(crate) struct Doc {
    #[return_ref]
    pub(crate) uri: Uri,
    #[return_ref]
    pub(crate) text: String,
    #[return_ref]
    pub(crate) tree: tree_sitter::Tree,
    #[return_ref]
    pub(crate) lines: LineIndex,
    pub(crate) enc: Option<WideEncoding>,
}

#[salsa::input]
pub(crate) struct Workspace {
    #[return_ref]
    pub(crate) docs: HashMap<Uri, Doc>,
    pub(crate) enc: Option<WideEncoding>,
}

#[salsa::tracked]
pub(crate) fn doc_symbols<'d>(db: &'d dyn Db, doc: Doc) -> Option<DocumentSymbol> {
    let mut sym = salsa_doc_symbols::DocumentSymbols::new(db, doc, false);
    sym.traverse(&mut doc.tree(db).walk()).and_then(|it| it.sym)
}

#[salsa::tracked]
pub(crate) fn workspace_symbols<'d>(db: &'d dyn Db, doc: Doc) -> Vec<WorkspaceSymbol> {
    let mut sym = salsa_ws_symbols::WorkspaceSymbols::new(db, doc);
    sym.traverse(&mut doc.tree(db).walk());
    sym.sym
}

#[salsa::tracked]
pub(crate) fn locations<'d>(db: &'d dyn Db, doc: Doc) -> Vec<Location> {
    let mut locations = salsa_locations::LocationVisitor::new(db, doc);
    locations.traverse(&mut doc.tree(db).root_node().walk());
    locations.locs
}
