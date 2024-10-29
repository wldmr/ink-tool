use lsp_types::{Location, OneOf, SymbolKind, Uri, WorkspaceLocation, WorkspaceSymbol};

use type_sitter_lib::{IncorrectKindCause, Node};

use crate::ink_syntax::{
    types::{AllNamed, GlobalKeyword},
    VisitInstruction, Visitor,
};

use crate::lsp::document::InkDocument;

pub(crate) struct WorkspaceSymbols<'a> {
    uri: &'a Uri,
    doc: &'a InkDocument,
    qualified_names: bool,
    knot: Option<&'a str>,
    stitch: Option<&'a str>,
    pub(crate) sym: Vec<WorkspaceSymbol>,
}

impl<'a> WorkspaceSymbols<'a> {
    pub(crate) fn new(doc: &'a InkDocument, uri: &'a Uri, qualified_names: bool) -> Self {
        Self {
            uri,
            doc,
            qualified_names,
            knot: None,
            stitch: None,
            sym: Vec::new(),
        }
    }

    pub fn global_name(&self, local_name: &str) -> String {
        if self.qualified_names {
            match (self.knot, self.stitch) {
                (None, None) => format!("{local_name}"),
                (None, Some(stitch)) => format!("{stitch}.{local_name}"),
                (Some(knot), None) => format!("{knot}.{local_name}"),
                (Some(knot), Some(stitch)) => format!("{knot}.{stitch}.{local_name}"),
            }
        } else {
            local_name.to_string()
        }
    }

    pub fn location(&self, range: tree_sitter::Range) -> OneOf<Location, WorkspaceLocation> {
        OneOf::Left(Location {
            uri: self.uri.clone(),
            range: self.doc.lsp_range(&range),
        })
    }

    fn add_sym(
        &mut self,
        kind: SymbolKind,
        name: impl AsRef<str> + Into<String>, // so we only allocate when necessary
        location: OneOf<Location, WorkspaceLocation>,
    ) {
        self.sym.push(WorkspaceSymbol {
            name: name.into(),
            kind,
            tags: None,
            container_name: None,
            location,
            data: None,
        });
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
            | AllNamed::CodeStmt(_)
            | AllNamed::Comment(_)
            | AllNamed::CondArm(_)
            | AllNamed::CondBlock(_)
            | AllNamed::Condition(_)
            | AllNamed::ConditionalText(_)
            | AllNamed::Divert(_)
            | AllNamed::DivertTarget(_)
            | AllNamed::Else(_)
            | AllNamed::Eol(_)
            | AllNamed::Eval(_)
            | AllNamed::Expr(_)
            | AllNamed::GatherMark(_)
            | AllNamed::GatherMarks(_)
            | AllNamed::GlobalKeyword(_)
            | AllNamed::Glue(_)
            | AllNamed::Identifier(_)
            | AllNamed::Include(_)
            | AllNamed::Label(_)
            | AllNamed::LineComment(_)
            | AllNamed::ListValueDef(_)
            | AllNamed::ListValueDefs(_)
            | AllNamed::ListValues(_)
            | AllNamed::Logic(_)
            | AllNamed::MultilineAlternatives(_)
            | AllNamed::Number(_)
            | AllNamed::Paragraph(_)
            | AllNamed::Param(_)
            | AllNamed::ParamValue(_)
            | AllNamed::Params(_)
            | AllNamed::Paren(_)
            | AllNamed::Path(_)
            | AllNamed::Postfix(_)
            | AllNamed::QualifiedName(_)
            | AllNamed::Redirect(_)
            | AllNamed::Return(_)
            | AllNamed::String(_)
            | AllNamed::Tag(_)
            | AllNamed::Temp(_)
            | AllNamed::Text(_)
            | AllNamed::Thread(_)
            | AllNamed::TodoComment(_)
            | AllNamed::Tunnel(_)
            | AllNamed::Unary(_) => Ignore,

            // Symbols (== levels) to be created
            AllNamed::Ink(ink) => {
                let name = std::path::Path::new(self.uri.path().as_str())
                    .file_name()
                    .expect("there should be a filename")
                    .to_string_lossy()
                    .to_string();
                self.add_sym(SymbolKind::FILE, name, self.location(ink.range()));
                Descend
            }

            AllNamed::Knot(knot) => {
                let kind = if knot.function().is_some() {
                    SymbolKind::FUNCTION
                } else {
                    SymbolKind::CLASS
                };
                let name_node = knot.name().unwrap();
                let local_name = &self.doc.text[name_node.byte_range()];
                self.add_sym(
                    kind,
                    self.global_name(local_name),
                    self.location(name_node.range()),
                );
                self.knot = Some(local_name);
                self.stitch = None;
                Descend
            }

            AllNamed::Stitch(stitch) => {
                let name_node = stitch.name().unwrap();
                let local_name = &self.doc.text[name_node.byte_range()];
                self.add_sym(
                    SymbolKind::CLASS,
                    self.global_name(local_name),
                    self.location(name_node.range()),
                );
                self.stitch = Some(local_name);
                Descend
            }

            AllNamed::Choice(choice) => {
                if let Some(Ok(label)) = choice.label() {
                    let name_node = label.name().unwrap();
                    let local_name = &self.doc.text[name_node.byte_range()];
                    self.add_sym(
                        SymbolKind::KEY,
                        self.global_name(local_name),
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
                        self.global_name(&self.doc.text[name_node.byte_range()]),
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
                    &self.doc.text[name_node.byte_range()],
                    self.location(name_node.range()),
                );
                Ignore
            }

            AllNamed::List(list) => {
                let name_node = list.name().unwrap();
                let list_name = &self.doc.text[name_node.byte_range()];
                self.add_sym(SymbolKind::ENUM, list_name, self.location(list.range()));
                if let Ok(defs) = list.values() {
                    let mut cursor = defs.walk(); // don't like this, should be able to do this without another cursor
                    let list_values = defs
                        .values(&mut cursor)
                        .flat_map(|def| def.ok())
                        .filter_map(|def| def.name().ok())
                        .map(|identifier| identifier);
                    for identifier in list_values {
                        let value_name = &self.doc.text[identifier.byte_range()];
                        self.add_sym(
                            SymbolKind::ENUM_MEMBER,
                            format!("{list_name}.{value_name}"),
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
                        &self.doc.text[name_node.byte_range()],
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
