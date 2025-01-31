use crate::ink_syntax::{
    types::{AllNamed, GlobalKeyword},
    VisitInstruction, Visitor,
};
use lsp_types::{Location, OneOf, SymbolKind, Uri, WorkspaceLocation, WorkspaceSymbol};
use type_sitter_lib::{IncorrectKindCause, Node};

use super::{salsa_doc_symbols::lsp_range, Db, Doc};

pub(super) struct WorkspaceSymbols<'a> {
    db: &'a dyn Db,
    doc: Doc,
    knot: Option<&'a str>,
    stitch: Option<&'a str>,
    pub(super) sym: Vec<WorkspaceSymbol>,
}

impl<'a> WorkspaceSymbols<'a> {
    pub(crate) fn new(db: &'a dyn Db, doc: Doc) -> Self {
        Self {
            db,
            doc,
            knot: None,
            stitch: None,
            sym: Vec::new(),
        }
    }

    pub fn namespace(&self) -> Option<String> {
        match (self.knot, self.stitch) {
            (None, None) => None,
            (None, Some(stitch)) => Some(format!("{stitch}")),
            (Some(knot), None) => Some(format!("{knot}")),
            (Some(knot), Some(stitch)) => Some(format!("{knot}.{stitch}")),
        }
    }

    fn location(&self, range: tree_sitter::Range) -> OneOf<Location, WorkspaceLocation> {
        OneOf::Left(Location {
            uri: self.uri().clone(),
            range: self.lsp_range(&range),
        })
    }

    fn add_sym(
        &mut self,
        kind: SymbolKind,
        name: impl Into<String>,
        container_name: Option<String>,
        location: OneOf<Location, WorkspaceLocation>,
    ) {
        self.sym.push(WorkspaceSymbol {
            name: name.into(),
            kind,
            tags: None,
            container_name,
            location,
            data: None,
        });
    }

    fn uri(&self) -> &Uri {
        self.doc.uri(self.db)
    }

    fn lsp_range(&self, range: &tree_sitter::Range) -> lsp_types::Range {
        lsp_range(self.doc.lines(self.db), self.doc.enc(self.db), range)
    }

    fn text(&self, byte_range: std::ops::Range<usize>) -> &'a str {
        &self.doc.text(self.db)[byte_range]
    }
}

impl<'tree> Visitor<'tree, AllNamed<'tree>> for WorkspaceSymbols<'tree> {
    fn visit(&mut self, node: AllNamed) -> VisitInstruction<Self> {
        // eprintln!("visiting: {}", node.kind());
        use VisitInstruction::*;
        match node {
            // recurse into these without creating a new level
            AllNamed::ChoiceBlock(_)
            | AllNamed::Code(_)
            | AllNamed::Content(_)
            | AllNamed::GatherBlock(_)
            | AllNamed::KnotBlock(_)
            | AllNamed::StitchBlock(_) => Descend,

            // children of these don't have interesting symbols, so don't recurse
            AllNamed::AltArm(_)
            | AllNamed::Alternatives(_)
            | AllNamed::Args(_)
            | AllNamed::Assignment(_)
            | AllNamed::Binary(_)
            | AllNamed::BlockComment(_)
            | AllNamed::Boolean(_)
            | AllNamed::Call(_)
            | AllNamed::ChoiceMark(_)
            | AllNamed::ChoiceMarks(_)
            | AllNamed::ChoiceOnly(_)
            | AllNamed::CondArm(_)
            | AllNamed::CondBlock(_)
            | AllNamed::Condition(_)
            | AllNamed::ConditionalText(_)
            | AllNamed::Divert(_)
            | AllNamed::Else(_)
            | AllNamed::Eol(_)
            | AllNamed::Eval(_)
            | AllNamed::Expr(_)
            | AllNamed::GatherMark(_)
            | AllNamed::GatherMarks(_)
            | AllNamed::Glue(_)
            | AllNamed::Identifier(_)
            | AllNamed::Include(_)
            | AllNamed::Label(_)
            | AllNamed::LineComment(_)
            | AllNamed::ListValueDef(_)
            | AllNamed::ListValueDefs(_)
            | AllNamed::ListValues(_)
            | AllNamed::MultilineAlternatives(_)
            | AllNamed::Number(_)
            | AllNamed::Paragraph(_)
            | AllNamed::Param(_)
            | AllNamed::Params(_)
            | AllNamed::Paren(_)
            | AllNamed::Path(_)
            | AllNamed::Postfix(_)
            | AllNamed::QualifiedName(_)
            | AllNamed::Return(_)
            | AllNamed::String(_)
            | AllNamed::Tag(_)
            | AllNamed::TempDef(_)
            | AllNamed::Text(_)
            | AllNamed::Thread(_)
            | AllNamed::TodoComment(_)
            | AllNamed::Tunnel(_)
            | AllNamed::Unary(_) => Ignore,

            // Symbols (== levels) to be created
            AllNamed::Ink(ink) => {
                let name = std::path::Path::new(self.uri().path().as_str())
                    .file_name()
                    .expect("there should be a filename")
                    .to_string_lossy()
                    .to_string();
                self.add_sym(SymbolKind::FILE, name, None, self.location(ink.range()));
                Descend
            }

            AllNamed::Knot(knot) => {
                let kind = if knot.function().is_some() {
                    SymbolKind::FUNCTION
                } else {
                    SymbolKind::CLASS
                };
                let name_node = knot.name().unwrap();
                let local_name = self.text(name_node.byte_range());
                self.add_sym(kind, local_name, None, self.location(name_node.range()));
                self.knot = Some(local_name);
                self.stitch = None;
                Descend
            }

            AllNamed::Stitch(stitch) => {
                let name_node = stitch.name().unwrap();
                let local_name = self.text(name_node.byte_range());
                self.add_sym(
                    SymbolKind::CLASS,
                    local_name,
                    self.namespace(),
                    self.location(name_node.range()),
                );
                self.stitch = Some(local_name);
                Descend
            }

            AllNamed::Choice(choice) => {
                if let Some(Ok(label)) = choice.label() {
                    let name_node = label.name().unwrap();
                    let local_name = self.text(name_node.byte_range());
                    self.add_sym(
                        SymbolKind::KEY,
                        local_name,
                        self.namespace(),
                        self.location(name_node.range()),
                    );
                }
                Ignore
            }

            AllNamed::Gather(gather) => {
                if let Some(Ok(label)) = gather.label() {
                    let name_node = label.name().unwrap();
                    self.add_sym(
                        SymbolKind::KEY,
                        self.text(name_node.byte_range()),
                        self.namespace(),
                        self.location(name_node.range()),
                    );
                }
                Ignore
            }

            AllNamed::Global(global) => {
                let kind = match global.keyword().unwrap() {
                    GlobalKeyword::Const(_) => SymbolKind::CONSTANT,
                    GlobalKeyword::Var(_) => SymbolKind::VARIABLE,
                };
                let name_node = &global.name().unwrap();
                self.add_sym(
                    kind,
                    self.text(name_node.byte_range()),
                    None,
                    self.location(name_node.range()),
                );
                Ignore
            }

            AllNamed::List(list) => {
                let name_node = list.name().unwrap();
                let list_name = self.text(name_node.byte_range());
                self.add_sym(
                    SymbolKind::ENUM,
                    list_name,
                    None,
                    self.location(list.range()),
                );
                if let Ok(defs) = list.values() {
                    let mut cursor = defs.walk(); // don't like this, should be able to do this without another cursor
                    let list_values = defs
                        .values(&mut cursor)
                        .flat_map(|def| def.ok())
                        .filter_map(|def| def.name().ok())
                        .map(|identifier| identifier);
                    for identifier in list_values {
                        let value_name = self.text(identifier.byte_range());
                        self.add_sym(
                            SymbolKind::ENUM_MEMBER,
                            value_name,
                            Some(list_name.to_string()),
                            self.location(identifier.range()),
                        );
                    }
                }
                Ignore
            }

            AllNamed::External(external) => {
                if let Ok(name_node) = external.name() {
                    self.add_sym(
                        SymbolKind::INTERFACE,
                        self.text(name_node.byte_range()),
                        None,
                        self.location(name_node.range()),
                    );
                }
                Ignore
            }
        }
    }

    fn combine(&mut self, _child: Self) {
        // not needed
    }

    fn visit_error(&mut self, err: type_sitter_lib::IncorrectKind) -> VisitInstruction<Self> {
        match err.cause() {
            // Error nodes might have children
            IncorrectKindCause::Error => VisitInstruction::Descend,
            // Missing nodes don't have children
            IncorrectKindCause::Missing => VisitInstruction::Ignore,
            // The node couldn't be converted to an AllNamed; unnamed nodes don't have any interesting children
            IncorrectKindCause::OtherKind(_) => VisitInstruction::Ignore,
        }
    }
}
