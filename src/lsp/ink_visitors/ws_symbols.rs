use std::ops::Deref;

use ink_document::InkDocument;
use ink_syntax::{AllNamed, GlobalKeyword};
use lsp_types::{Location, OneOf, SymbolKind, Uri, WorkspaceLocation, WorkspaceSymbol};
use tree_traversal::{VisitInstruction, Visitor};
use type_sitter::{IncorrectKindCause, Node};

pub fn from_doc(uri: &Uri, doc: &impl Deref<Target = InkDocument>) -> Vec<WorkspaceSymbol> {
    WorkspaceSymbols::new(uri, doc.deref()).traverse(doc.root())
}

struct WorkspaceSymbols<'a> {
    uri: &'a Uri,
    doc: &'a InkDocument,
    knot: Option<&'a str>,
    stitch: Option<&'a str>,
}

impl<'a> WorkspaceSymbols<'a> {
    fn new(uri: &'a Uri, doc: &'a InkDocument) -> Self {
        Self {
            uri,
            doc,
            knot: None,
            stitch: None,
        }
    }

    fn namespace(&self) -> Option<String> {
        match (self.knot, self.stitch) {
            (None, None) => None,
            (None, Some(stitch)) => Some(format!("{stitch}")),
            (Some(knot), None) => Some(format!("{knot}")),
            (Some(knot), Some(stitch)) => Some(format!("{knot}.{stitch}")),
        }
    }

    fn location(&self, node: impl type_sitter::Node<'a>) -> OneOf<Location, WorkspaceLocation> {
        OneOf::Left(Location {
            uri: self.uri().clone(),
            range: self.lsp_range(node),
        })
    }

    fn uri(&self) -> &Uri {
        &self.uri
    }

    fn lsp_range(&self, node: impl type_sitter::Node<'a>) -> lsp_types::Range {
        self.doc.lsp_range(node.range())
    }
}

/// Return text, if not empty.
///
/// This isn't a method to enable partial borrowing. If it were a method,
/// we'd force `self` to be immutable, but that breaks the mut methods.
fn text_of<'a>(doc: &InkDocument, node: impl type_sitter::Node<'a>) -> Option<&str> {
    let name = doc.node_text(node).trim();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn add_to(
    sym: &mut Vec<WorkspaceSymbol>,
    kind: SymbolKind,
    name: impl Into<String>,
    container_name: Option<String>,
    location: OneOf<Location, WorkspaceLocation>,
) {
    sym.push(WorkspaceSymbol {
        name: name.into(),
        kind,
        tags: None,
        container_name,
        location,
        data: None,
    });
}

impl<'tree> Visitor<'tree, AllNamed<'tree>> for WorkspaceSymbols<'tree> {
    type State = Vec<WorkspaceSymbol>;

    fn visit(&mut self, node: AllNamed, sym: &mut Self::State) -> VisitInstruction<Self::State> {
        // log::trace!("visiting: {}", node.kind());
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
                add_to(sym, SymbolKind::FILE, name, None, self.location(ink));
                Descend
            }

            AllNamed::Knot(knot) => {
                let kind = if knot.function().is_some() {
                    SymbolKind::FUNCTION
                } else {
                    SymbolKind::CLASS
                };
                let name_node = knot.name().unwrap();
                if let Some(local_name) = text_of(self.doc, name_node) {
                    add_to(sym, kind, local_name, None, self.location(name_node));
                    self.knot = Some(local_name);
                    self.stitch = None;
                }
                Descend
            }

            AllNamed::Stitch(stitch) => {
                let name_node = stitch.name().unwrap();
                if let Some(local_name) = text_of(self.doc, name_node) {
                    add_to(
                        sym,
                        SymbolKind::CLASS,
                        local_name,
                        self.namespace(),
                        self.location(name_node),
                    );
                    self.stitch = Some(local_name);
                }
                Descend
            }

            AllNamed::Choice(choice) => {
                if let Some(Ok(label)) = choice.label() {
                    let name_node = label.name().unwrap();
                    if let Some(local_name) = text_of(self.doc, name_node) {
                        add_to(
                            sym,
                            SymbolKind::KEY,
                            local_name,
                            self.namespace(),
                            self.location(name_node),
                        );
                    }
                }
                Ignore
            }

            AllNamed::Gather(gather) => {
                if let Some(Ok(label)) = gather.label() {
                    let name_node = label.name().unwrap();
                    if let Some(name) = text_of(self.doc, name_node) {
                        add_to(
                            sym,
                            SymbolKind::KEY,
                            name,
                            self.namespace(),
                            self.location(name_node),
                        );
                    }
                }
                Ignore
            }

            AllNamed::Global(global) => {
                let kind = match global.keyword().unwrap() {
                    GlobalKeyword::Const(_) => SymbolKind::CONSTANT,
                    GlobalKeyword::Var(_) => SymbolKind::VARIABLE,
                };
                let name_node = global.name().unwrap();
                if let Some(name) = text_of(self.doc, name_node) {
                    add_to(sym, kind, name, None, self.location(name_node));
                }
                Ignore
            }

            AllNamed::List(list) => {
                let name_node = list.name().unwrap();
                if let Some(list_name) = text_of(self.doc, name_node) {
                    add_to(sym, SymbolKind::ENUM, list_name, None, self.location(list));
                    if let Ok(defs) = list.values() {
                        let mut cursor = defs.walk(); // don't like this, should be able to do this without another cursor
                        let list_values = defs
                            .values(&mut cursor)
                            .flat_map(|def| def.ok())
                            .filter_map(|def| def.name().ok())
                            .map(|identifier| identifier);
                        for identifier in list_values {
                            if let Some(value_name) = text_of(self.doc, identifier) {
                                add_to(
                                    sym,
                                    SymbolKind::ENUM_MEMBER,
                                    value_name,
                                    Some(list_name.to_string()),
                                    self.location(identifier),
                                );
                            }
                        }
                    }
                }
                Ignore
            }

            AllNamed::External(external) => {
                if let Ok(name_node) = external.name() {
                    if let Some(name) = text_of(self.doc, name_node) {
                        add_to(
                            sym,
                            SymbolKind::INTERFACE,
                            name,
                            None,
                            self.location(name_node),
                        );
                    }
                }
                Ignore
            }
        }
    }

    fn visit_error(&mut self, err: type_sitter::IncorrectKind) -> VisitInstruction<Self::State> {
        match err.cause() {
            // Error nodes might have children
            IncorrectKindCause::Error => VisitInstruction::Descend,
            // Missing nodes don't have children
            IncorrectKindCause::Missing => VisitInstruction::Ignore,
            // The node couldn't be converted to an AllNamed; unnamed nodes don't have any interesting children
            IncorrectKindCause::OtherKind(_) => VisitInstruction::Ignore,
        }
    }

    fn combine(_: &mut Self::State, _: Self::State) {
        // parent.append(&mut children); // More of a failsafe; we shouldn't actuall nest these.
        unreachable!("We don't have sub-states")
    }
}
