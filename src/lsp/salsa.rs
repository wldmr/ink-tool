use super::{location::Location, state::PositionOutOfBounds};
use crate::{ink_syntax::Visitor as _, lsp::links::Links};
use itertools::Itertools as _;
use line_index::{LineCol, LineIndex, WideEncoding};
use lsp_types::{Diagnostic, DocumentSymbol, Uri, WorkspaceSymbol};
use salsa::{self, Accumulator};
use salsa_links::LinkVisitor;
use std::collections::HashMap;

pub mod salsa_doc_symbols;
pub mod salsa_links;
pub mod salsa_locations;
pub mod salsa_syntax_structs;
pub mod salsa_ws_symbols;

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

#[salsa::input]
pub struct Doc {
    #[return_ref]
    pub uri: Uri,
    #[return_ref]
    pub text: String,
    #[return_ref]
    pub tree: tree_sitter::Tree,
    #[return_ref]
    pub lines: LineIndex,
    pub enc: Option<WideEncoding>,
}

#[salsa::tracked]
impl Doc {
    #[salsa::tracked]
    pub fn named_cst_node_at(
        self,
        db: &dyn Db,
        pos: lsp_types::Position,
    ) -> Result<tree_sitter::Node<'_>, PositionOutOfBounds> {
        let point = self.ts_point(db, pos)?;
        let node = self
            .tree(db)
            .root_node()
            .named_descendant_for_point_range(point, point)
            .expect("point wasn't out of bounds, so this should return _something_");
        Ok(node)
    }

    #[salsa::tracked]
    fn ts_point(
        self,
        db: &dyn Db,
        pos: lsp_types::Position,
    ) -> Result<tree_sitter::Point, PositionOutOfBounds> {
        let mut pos = pos;
        if let Some(enc) = self.enc(db) {
            let wide = line_index::WideLineCol {
                line: pos.line,
                col: pos.character,
            };
            let encoded = self
                .lines(db)
                .to_utf8(enc, wide)
                .expect("Conversion from wide encoding to UTF-8 mustn't fail");
            pos.line = encoded.line;
            pos.character = encoded.col;
        }
        Ok(tree_sitter::Point::new(
            pos.line as usize,
            pos.character as usize,
        ))
    }

    #[salsa::tracked]
    pub fn lsp_position(self, db: &dyn Db, point: tree_sitter::Point) -> lsp_types::Position {
        let native = LineCol {
            line: point.row as u32,
            col: point.column as u32,
        };

        if let Some(enc) = self.enc(db) {
            let wide = self.lines(db).to_wide(enc, native).unwrap();
            lsp_types::Position {
                line: wide.line,
                character: wide.col,
            }
        } else {
            lsp_types::Position {
                line: native.line,
                character: native.col,
            }
        }
    }
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
pub(crate) fn workspace_symbols<'d>(db: &'d dyn Db, workspace: Workspace) -> Vec<WorkspaceSymbol> {
    workspace
        .docs(db)
        .values()
        .flat_map(|&doc| workspace_symbols_for_doc(db, doc))
        .collect()
}

#[salsa::tracked]
pub(crate) fn workspace_symbols_for_doc<'d>(db: &'d dyn Db, doc: Doc) -> Vec<WorkspaceSymbol> {
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

#[salsa::tracked]
pub struct Poi<'db> {
    #[id]
    pub cst: Doc,
    #[id]
    pub range: (tree_sitter::Point, tree_sitter::Point),
}

#[salsa::tracked]
impl<'db> Poi<'db> {
    pub fn from_range(database: &'db dyn Db, cst: Doc, range: tree_sitter::Range) -> Self {
        Poi::new(database, cst, (range.start_point, range.end_point))
    }

    #[salsa::tracked]
    pub fn lsp_range(self, db: &'db dyn Db) -> lsp_types::Range {
        let cst = self.cst(db);
        let (start, end) = self.range(db);
        let start = cst.lsp_position(db, start);
        let end = cst.lsp_position(db, end);
        lsp_types::Range { start, end }
    }

    #[salsa::tracked]
    pub fn cst_node(self, db: &'db dyn Db) -> tree_sitter::Node<'db> {
        let cst = self.cst(db);
        let tree = cst.tree(db);
        let (start, end) = self.range(db);
        tree.root_node()
            .descendant_for_point_range(start, end)
            .expect("this instance shouldn't exist without a valid range")
    }

    #[salsa::tracked]
    pub fn text(self, db: &'db dyn Db) -> &'db str {
        let node = self.cst_node(db);
        let text = self.cst(db).text(db);
        &text[node.byte_range()]
    }
}

#[salsa::tracked]
pub(crate) fn links_for_workspace<'d>(db: &'d dyn Db, ws: Workspace) -> Links<'d, Poi<'d>> {
    let mut links: Links<'d, Poi<'d>> = ws.docs(db).values().map(|doc| find_links(db, *doc)).sum();
    links.resolve();
    for (loc, name) in &links.resolvable {
        LspDiagnostic {
            doc: loc.cst(db),
            diagnostic: Diagnostic {
                range: loc.lsp_range(db),
                severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("ink-tool".to_string()),
                message: format!("Unresolved name '{name}'."),
                related_information: None,
                tags: None,
                data: None,
            },
        }
        .accumulate(db); // TODO: Actually do something with these.
    }
    links
}

#[salsa::tracked]
pub(crate) fn map_of_definitions<'d>(
    db: &'d dyn Db,
    ws: Workspace,
) -> HashMap<Poi<'d>, Vec<Poi<'d>>> {
    links_for_workspace(db, ws)
        .resolved
        .into_iter()
        .into_group_map()
}

/// file -> location -> linked location
#[salsa::tracked]
pub(crate) fn definitions_to_usages<'d>(
    db: &'d dyn Db,
    ws: Workspace,
) -> HashMap<Uri, HashMap<tree_sitter::Range, Vec<(Uri, lsp_types::Range)>>> {
    let uri_map = links_for_workspace(db, ws)
        .resolved
        .into_iter()
        .map(|(def, usage)| {
            let usage_uri = usage.cst(db).uri(db).clone();
            let usage_range = usage.cst_node(db).range();
            let def_uri = def.cst(db).uri(db).clone();
            let def_range = def.lsp_range(db).clone();
            (usage_uri, (usage_range, (def_uri, def_range)))
        })
        .into_group_map();
    let uri_map = uri_map
        .into_iter()
        .map(|(uri, vec)| (uri, vec.into_iter().into_group_map()))
        .collect();
    uri_map
}

#[salsa::tracked]
pub(crate) fn find_links<'d>(db: &'d dyn Db, doc: Doc) -> Links<'d, Poi<'d>> {
    let mut visitor = LinkVisitor::new(doc.text(db));
    visitor.traverse(&mut doc.tree(db).walk());
    let links = visitor.into_links();
    links.transform_locations(|loc| {
        let range = loc.range();
        Poi::new(db, doc, (range.start_point, range.end_point))
    })
}

#[salsa::accumulator]
pub struct LspDiagnostic {
    doc: Doc,
    diagnostic: lsp_types::Diagnostic,
}
