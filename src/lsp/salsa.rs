use super::{location::Location, state::InvalidPosition};
use crate::ink_syntax::{
    self, traversal,
    types::{self as ink_types, AllNamed, Identifier, OfInterest},
    Visitor,
};
use itertools::Itertools;
use line_index::{LineCol, LineIndex, WideEncoding};
use lsp_types::{lsif::DocumentSymbolOrRangeBasedVec, DocumentSymbol, Uri, WorkspaceSymbol};
use salsa::{self};
use salsa_block_items::BlockItems;
use std::{
    any::{type_name, type_name_of_val},
    marker::PhantomData,
};
use std::{collections::HashMap, ops::Index as _};
use tap::{Pipe as _, Tap as _};
use type_sitter_lib::{IncorrectKind, Node};

pub mod salsa_block_items;
pub mod salsa_definitions;
pub mod salsa_doc_symbols;
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
        // log::trace!("Event: {event:?}");
    }
}

/// Straight up copied from salsa source code
/// (from the `fallback()` function in the `update` module).
///
/// I don't really understand any of this, but this [comment] gives me hope:
///
/// > It is for data structures that can be included in tracked/interned structs
/// > but which are not, themselves, tracked/interned structs.
///
/// [comment]: https://github.com/salsa-rs/salsa/issues/592#issuecomment-2427901436
macro_rules! impl_salsa_update {
    ($($target:path),+$(,)?) => {
    $(
        unsafe impl<'a> salsa::Update for $target {
            unsafe fn maybe_update(old_pointer: *mut Self, new_value: Self) -> bool {
                // Because everything is owned, this ref is simply a valid `&mut`
                let old_ref: &mut Self = unsafe { &mut *old_pointer };

                if *old_ref != new_value {
                    *old_ref = new_value;
                    true
                } else {
                    // Subtle but important: Eq impls can be buggy or define equality
                    // in surprising ways. If it says that the value has not changed,
                    // we do not modify the existing value, and thus do not have to
                    // update the revision, as downstream code will not see the new value.
                    false
                }
            }
        }
    )+
    };
}

impl_salsa_update![
    ink_types::AllNamed<'a>,
    ink_types::Definitions<'a>,
    ink_types::Redirect<'a>,
    ink_types::Block<'a>,
    ink_types::ScopeBlock<'a>,
    ink_types::Usages<'a>,
];

#[salsa::input]
pub struct Doc {
    pub workspace: Workspace,
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

#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("Could not go to node.")]
pub enum GetNodeError {
    #[display("Node type didn't match")]
    InvalidType,
    PositionOutOfBounds(InvalidPosition),
}

#[salsa::tracked]
impl Doc {
    pub fn get_node_at<'a, T>(
        self,
        db: &'a dyn Db,
        pos: lsp_types::Position,
    ) -> Result<T, GetNodeError>
    where
        T: type_sitter_lib::Node<'a>,
    {
        let (point, _byte) = self.ts_point(db, pos)?;
        self.tree(db)
            .root_node()
            .named_descendant_for_point_range(point, point)
            .ok_or_else(|| InvalidPosition(pos))?
            .pipe(T::try_from_raw)
            .map_err(|_| GetNodeError::InvalidType)
    }

    pub fn named_cst_node_at(
        self,
        db: &dyn Db,
        pos: lsp_types::Position,
    ) -> Result<AllNamed<'_>, InvalidPosition> {
        let (point, _byte) = self.ts_point(db, pos)?;
        self.tree(db)
            .root_node()
            .named_descendant_for_point_range(point, point)
            .and_then(|node| AllNamed::try_from_raw(node).ok())
            .ok_or_else(|| InvalidPosition(pos))
    }

    pub fn ts_point(
        self,
        db: &dyn Db,
        pos: lsp_types::Position,
    ) -> Result<(tree_sitter::Point, usize), InvalidPosition> {
        let lines = self.lines(db);
        let line_col = if let Some(enc) = self.enc(db) {
            let wide = line_index::WideLineCol {
                line: pos.line,
                col: pos.character,
            };
            lines
                .to_utf8(enc, wide)
                .ok_or_else(|| InvalidPosition(pos))?
        } else {
            line_index::LineCol {
                line: pos.line,
                col: pos.character,
            }
        };
        let point = tree_sitter::Point::new(pos.line as usize, pos.character as usize);
        let byte = lines
            .offset(line_col)
            .ok_or_else(|| InvalidPosition(pos))?
            .into();
        Ok((point, byte))
    }

    pub fn ts_range(
        self,
        db: &dyn Db,
        range: lsp_types::Range,
    ) -> Result<tree_sitter::Range, InvalidPosition> {
        let (start_point, start_byte) = self.ts_point(db, range.start)?;
        let (end_point, end_byte) = self.ts_point(db, range.end)?;
        Ok(tree_sitter::Range {
            start_byte,
            end_byte,
            start_point,
            end_point,
        })
    }

    fn lsp_position(self, db: &dyn Db, point: tree_sitter::Point) -> lsp_types::Position {
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

    pub fn lsp_range(self, db: &dyn Db, point: tree_sitter::Range) -> lsp_types::Range {
        lsp_types::Range {
            start: self.lsp_position(db, point.start_point),
            end: self.lsp_position(db, point.end_point),
        }
    }
}

#[salsa::input]
pub(crate) struct Workspace {
    #[return_ref]
    pub(crate) docs: HashMap<Uri, Doc>,
    pub(crate) enc: Option<WideEncoding>,
}

#[salsa::tracked(return_ref)]
pub fn workspace_definitions_by_name<'db>(
    db: &'db dyn Db,
    ws: Workspace,
) -> HashMap<String, Vec<(Visibility<'db>, SalsaDefinition<'db>)>> {
    ws.docs(db)
        .into_iter()
        .flat_map(|(uri, doc)| {
            log::debug!("looking at document: {}", uri.as_str());
            let ink = doc.tree(db).root_node();
            let ink = ink_types::Block::try_from_raw(ink).expect("root node should be Ink");
            let ink = SalsaBlock::new(db, *doc, ink);
            definitions_in_block(db, ink)
        })
        .flat_map(|def| def.names(db))
        .map(|scoped| {
            (
                scoped.name(db).to_string(),
                (scoped.visibility(db), scoped.definition(db)),
            )
        })
        .into_group_map()
}

#[salsa::tracked(return_ref)]
pub fn definitions_in_block<'db>(
    db: &'db dyn Db,
    node: SalsaBlock<'db>,
) -> Vec<SalsaDefinition<'db>> {
    let Some(items) = block_items(db, node) else {
        return Vec::new();
    };
    let BlockItems {
        mut defs, blocks, ..
    } = items;
    for block in blocks {
        defs.extend(definitions_in_block(db, block));
    }
    defs
}

#[salsa::tracked]
pub fn usages_in_doc<'db>(db: &'db dyn Db, doc: Doc) -> Vec<SalsaUsage<'db>> {
    let ink = doc.tree(db).root_node();
    let ink = ink_types::Block::try_from_raw(ink).expect("root node should be Ink");
    let ink = SalsaBlock::new(db, doc, ink);
    usages_in_block(db, ink).to_vec()
}

#[salsa::tracked(return_ref)]
pub fn usages_in_block<'db>(db: &'db dyn Db, node: SalsaBlock<'db>) -> Vec<SalsaUsage<'db>> {
    let Some(block_items) = block_items(db, node) else {
        return Vec::new();
    };
    let BlockItems {
        mut usages, blocks, ..
    } = block_items;
    for block in blocks {
        usages.extend(usages_in_block(db, block));
    }
    usages
}

#[salsa::tracked]
pub fn block_items<'db>(db: &'db dyn Db, node: SalsaBlock<'db>) -> Option<BlockItems<'db>> {
    log::debug!(
        "looking for items in node: {:?} of {}",
        node.node(db),
        node.uri(db).path()
    );
    let mut cursor = node.node(db).raw().walk();
    let mut visitor = salsa_block_items::BlockVisitor::new(db, node);
    visitor.traverse(&mut cursor);
    let items = visitor.finish();
    log::debug!(
        "found {} definitions, {} usages and {} blocks",
        items.defs.len(),
        items.usages.len(),
        items.blocks.len()
    );
    items.has_any_items().then_some(items)
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

/// Takes a type_sitter node type and an identifier,
/// creates a Salsa struct for that node and implements NodeSalsa for it
///
/// The typesitter type needs a single lifetime called `<'db>`.
/// I couldn't figure out a hygenic way to do it otherwise.
/// But neither could Salsa, so maybe that's the best we can do.
macro_rules! salsa_structs_for_nodes {
    ($($ts_type:ty => $salsa_type:ident),+$(,)?) => {
        $(
        #[salsa::tracked]
        pub struct $salsa_type<'db> {
            #[id]
            pub cst: Doc,
            #[id]
            pub node: $ts_type,
        }

        impl<'db> NodeSalsa<'db, $ts_type> for $salsa_type<'db> {
            fn cst(self, db: &'db dyn Db) -> Doc {
                $salsa_type::cst(self, db)
            }
            fn node(self, db: &'db dyn Db) -> $ts_type {
                $salsa_type::node(self, db)
            }
        }
        )+
    };
}

salsa_structs_for_nodes![
    ink_types::Block<'db> => SalsaBlock,
    ink_types::ScopeBlock<'db> => SalsaScope,
    ink_types::AllNamed<'db> => SalsaNode,
    ink_types::Definitions<'db> => SalsaDefinition,
    ink_types::Usages<'db> => SalsaUsage,
];

#[salsa::tracked]
impl<'db> SalsaScope<'db> {
    #[salsa::tracked]
    pub fn local_name(self, db: &'db dyn Db) -> Option<&'db str> {
        use ink_types::ScopeBlock::*;
        let range = match self.node(db) {
            StitchBlock(block) => block.header().map(|it| it.name().byte_range()).ok(),
            KnotBlock(block) => block.header().map(|it| it.name().byte_range()).ok(),
            Ink(_) => None,
        };
        range.map(|it| self.cst(db).text(db).index(it))
    }

    #[salsa::tracked]
    pub fn global_name(self, db: &'db dyn Db) -> Option<String> {
        let Some(my_name) = self.local_name(db) else {
            return None;
        };
        if let Some(parent_name) = self.parent_scope(db).and_then(|it| it.global_name(db)) {
            Some(format!("{parent_name}.{my_name}"))
        } else {
            Some(format!("{my_name}"))
        }
    }

    #[salsa::tracked]
    pub fn parent_scope(self, db: &'db dyn Db) -> Option<Self> {
        traversal::parent(self.node(db))
            .next()
            .map(|block| Self::new(db, self.cst(db), block))
    }

    #[salsa::tracked]
    pub fn child_scopes(self, db: &'db dyn Db) -> Vec<Self> {
        let raw = self.node(db).into_raw();
        raw.children(&mut raw.walk())
            .filter_map(|child| ink_types::ScopeBlock::try_from_raw(child).ok())
            .map(|block| SalsaScope::new(db, self.cst(db), block))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update)]
pub enum Visibility<'db> {
    Global,
    Inside(SalsaScope<'db>),
    Temp(SalsaScope<'db>),
}
impl<'db> Visibility<'db> {
    pub fn visible_to(
        &self,
        db: &'db dyn Db,
        uri: &Uri,
        node: impl type_sitter_lib::Node<'db>,
    ) -> bool {
        let offset = node.start_byte();
        match self {
            Visibility::Global => true,
            Visibility::Inside(scope) => {
                scope.uri(db) == uri && scope.node(db).byte_range().contains(&offset)
            }
            Visibility::Temp(scope) => {
                scope.uri(db) == uri
                    && scope.node(db).byte_range().contains(&offset)
                    && scope // temp only visible until the first subscope starts
                        .child_scopes(db)
                        .first()
                        .map(|first| offset < first.node(db).start_byte())
                        .unwrap_or(true)
            }
        }
    }
}

pub type Name = String;

#[salsa::tracked]
pub struct ScopedDefinition<'db> {
    #[id]
    #[return_ref]
    pub name: Name,
    #[id]
    pub visibility: Visibility<'db>,
    pub definition: SalsaDefinition<'db>,
}

#[salsa::tracked]
impl<'db> SalsaDefinition<'db> {
    #[salsa::tracked]
    pub fn defining_scope(self, db: &'db dyn Db) -> SalsaScope<'db> {
        let this = self.node(db);
        log::info!("trying to find scope for definition {:?}", this.raw());
        traversal::parent(this)
            .next()
            .map(|block| SalsaScope::new(db, self.cst(db), block))
            .expect("a definition must be defined inside _something_")
    }

    pub fn local_name(self, db: &'db dyn Db) -> String {
        let name_node = self.name_node(db);
        self.cst(db).text(db)[name_node.byte_range()].to_string()
    }

    #[salsa::tracked]
    pub fn names(self, db: &'db dyn Db) -> Vec<ScopedDefinition<'db>> {
        let node = self.node(db);
        let local_name = self.local_name(db);
        let def_scope = self.defining_scope(db);
        use ink_types::Definitions::*;
        use ink_types::ScopeBlock::*;
        let names = match node {
            Knot(_) => vec![self.global(db, local_name)],
            Stitch(_) => {
                // either we're inside a knot (which has a name), or we're global
                if let Some(knot_name) = def_scope.parent_scope(db).and_then(|it| it.local_name(db))
                {
                    vec![
                        self.global(db, format!("{knot_name}.{local_name}")),
                        self.inside(db, def_scope, local_name),
                    ]
                } else {
                    vec![self.global(db, local_name)]
                }
            }
            Label(_) => match def_scope.node(db) {
                KnotBlock(_) => {
                    let knot_name = def_scope.local_name(db).expect("knot must have name");
                    vec![
                        self.global(db, format!("{knot_name}.{local_name}")),
                        self.inside(db, def_scope, local_name),
                    ]
                }
                StitchBlock(_) => {
                    let stitch_name = def_scope.local_name(db).expect("stitch must have name");
                    if let Some(knot) = def_scope.parent_scope(db) {
                        let knot_name = knot.local_name(db).expect("knot must have name");
                        vec![
                            self.global(db, format!("{knot_name}.{local_name}")),
                            self.global(db, format!("{knot_name}.{stitch_name}.{local_name}")),
                            self.inside(db, def_scope, local_name),
                        ]
                    } else {
                        vec![
                            self.global(db, format!("{stitch_name}.{local_name}")),
                            self.inside(db, def_scope, local_name),
                        ]
                    }
                }
                Ink(_) => vec![self.global(db, local_name)],
            },
            TempDef(_) => vec![self.temp(db, def_scope, local_name)],
            Param(_) => vec![self.inside(db, def_scope, local_name)],
            List(_) | Global(_) | External(_) => vec![self.global(db, local_name)],
            ListValueDef(value_def) => {
                let parent: ink_types::List<'db> = traversal::parent(value_def)
                    .next()
                    .expect("list value def must be inside a list");
                let list_name = self.text_for(db, parent.name());
                vec![
                    self.global(db, format!("{list_name}.{local_name}",)),
                    self.global(db, local_name),
                ]
            }
        };
        log::debug!(
            "names for {node:?}: {:#?}",
            names
                .iter()
                .map(|it| match it.visibility(db) {
                    Visibility::Global => format!("{} (global)", it.name(db)),
                    Visibility::Inside(salsa_scope) => format!(
                        "{} (local in {})",
                        it.name(db),
                        salsa_scope
                            .local_name(db)
                            .unwrap_or_else(|| salsa_scope.uri(db).path().as_str())
                    ),
                    Visibility::Temp(salsa_scope) => format!(
                        "{} (temp in {})",
                        it.name(db),
                        salsa_scope
                            .local_name(db)
                            .unwrap_or_else(|| salsa_scope.uri(db).path().as_str())
                    ),
                })
                .collect_vec()
        );
        names
    }

    fn global(self, db: &'db dyn Db, name: Name) -> ScopedDefinition<'db> {
        ScopedDefinition::new(db, name.into(), Visibility::Global, self)
    }

    fn inside(self, db: &'db dyn Db, scope: SalsaScope<'db>, name: Name) -> ScopedDefinition<'db> {
        ScopedDefinition::new(db, name.into(), Visibility::Inside(scope), self)
    }

    fn temp(self, db: &'db dyn Db, scope: SalsaScope<'db>, name: Name) -> ScopedDefinition<'db> {
        ScopedDefinition::new(db, name.into(), Visibility::Temp(scope), self)
    }

    pub fn name_node(self, db: &'db dyn Db) -> type_sitter_lib::UntypedNode<'db> {
        match self.node(db) {
            ink_types::Definitions::Knot(knot) => knot.name().upcast(),
            ink_types::Definitions::Label(label) => label.name().upcast(),
            ink_types::Definitions::Stitch(stitch) => stitch.name().upcast(),
            ink_types::Definitions::TempDef(temp_def) => temp_def.name().upcast(),
            ink_types::Definitions::Param(param) => param.value().upcast(),
            ink_types::Definitions::List(list) => list.name().upcast(),
            ink_types::Definitions::External(external) => external.name().upcast(),
            ink_types::Definitions::Global(global) => global.name().upcast(),
            ink_types::Definitions::ListValueDef(list_value_def) => list_value_def.name().upcast(),
        }
    }
}

#[salsa::tracked]
impl<'db> SalsaUsage<'db> {
    /// Construct a usage that encompases the biggest possible node.
    /// (we need this because  qualified names contain identifiers)
    pub fn from_inner_node(db: &'db dyn Db, cst: Doc, inner: ink_types::Usages<'db>) -> Self {
        let outer = std::iter::successors(Some(inner.upcast()), |node| node.parent())
            .map_while(|node| node.downcast().ok())
            .last()
            .unwrap_or(inner);
        Self::new(db, cst, outer)
    }
    #[salsa::tracked]
    pub fn parent_scope(self, db: &'db dyn Db) -> SalsaScope<'db> {
        traversal::parent(self.node(db))
            .next()
            .map(|block| SalsaScope::new(db, self.cst(db), block))
            .expect("a usage must be defined inside _something_")
    }

    #[salsa::tracked(return_ref)]
    pub fn definition(self, db: &'db dyn Db) -> Vec<SalsaDefinition<'db>> {
        let uri = self.uri(db);
        let node = self.node(db);
        let target_name = self.text(db);
        let workspace = self.workspace(db);
        if let Some(defs) = workspace_definitions_by_name(db, workspace).get(target_name) {
            defs.into_iter()
                .filter_map(|(vis, def)| vis.visible_to(db, uri, node).then_some(def))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

pub trait NodeSalsa<'db, T: type_sitter_lib::Node<'db>>: Copy {
    fn cst(self, db: &'db dyn Db) -> Doc;
    fn node(self, db: &'db dyn Db) -> T;

    fn workspace(self, db: &'db dyn Db) -> Workspace {
        self.cst(db).workspace(db)
    }

    fn uri(self, db: &'db dyn Db) -> &'db lsp_types::Uri {
        self.cst(db).uri(db)
    }

    fn lsp_range(self, db: &'db dyn Db) -> lsp_types::Range {
        let ts_range = self.node(db).range();
        self.cst(db).lsp_range(db, ts_range)
    }

    fn text_for(self, db: &'db dyn Db, node: impl type_sitter_lib::Node<'db>) -> &'db str {
        self.cst(db).text(db).index(node.byte_range())
    }

    fn text(self, db: &'db dyn Db) -> &'db str {
        self.text_for(db, self.node(db))
    }
}

#[salsa::tracked]
pub(crate) fn uris<'d>(db: &'d dyn Db, ws: Workspace) -> Vec<&'d Uri> {
    ws.docs(db).keys().sorted_unstable().collect()
}

/// The common part at the start of all the URIs in this workspace.
/// Can be used to remove he "uninteresting" bits for the paths.
#[salsa::tracked]
pub(crate) fn common_file_prefix<'d>(db: &'d dyn Db, ws: Workspace) -> String {
    uris(db, ws)
        .into_iter()
        .map(|it| it.path().to_string())
        .reduce(|acc, next| {
            acc.chars()
                .zip(next.chars())
                .take_while(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect::<String>()
        })
        .unwrap_or_default()
        .tap(|it| log::debug!("Common file name prefix: `{it}``"))
}

#[salsa::accumulator]
pub struct LspDiagnostic {
    doc: Doc,
    diagnostic: lsp_types::Diagnostic,
}
