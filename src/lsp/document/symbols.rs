use builder::SymbolBuilder;
use lsp_types::{DocumentSymbol, SymbolKind};
use type_sitter_lib::{IncorrectKindCause, Node};

use crate::ink_syntax::{
    types::{AllNamed, GlobalKeyword},
    VisitInstruction, Visitor,
};

use crate::lsp::document::InkDocument;

mod builder {
    use lsp_types::{DocumentSymbol, Range, SymbolKind};
    use std::marker::PhantomData;

    #[allow(unused)]
    pub(super) struct Init;
    pub(super) struct NeedsName;
    pub(super) struct NeedsRange;
    pub(super) struct RequiredFieldsFilled;

    pub(super) struct SymbolBuilder<T>(DocumentSymbol, PhantomData<T>);

    impl SymbolBuilder<Init> {
        pub(super) fn new(kind: SymbolKind) -> SymbolBuilder<NeedsName> {
            #[allow(deprecated)]
            SymbolBuilder(
                DocumentSymbol {
                    name: String::new(),
                    detail: None,
                    kind,
                    tags: None,
                    deprecated: None,
                    range: Range::default(),
                    selection_range: Range::default(),
                    children: None,
                },
                PhantomData,
            )
        }
    }

    impl SymbolBuilder<NeedsName> {
        pub(super) fn name(self, name: impl ToString) -> SymbolBuilder<NeedsRange> {
            SymbolBuilder(
                DocumentSymbol {
                    name: name.to_string(),
                    ..self.0
                },
                PhantomData,
            )
        }
    }

    impl SymbolBuilder<NeedsRange> {
        pub(super) fn range(self, range: Range) -> SymbolBuilder<RequiredFieldsFilled> {
            SymbolBuilder(
                DocumentSymbol {
                    range,
                    selection_range: range,
                    ..self.0
                },
                PhantomData,
            )
        }
    }

    impl SymbolBuilder<RequiredFieldsFilled> {
        pub(super) fn selection_range(mut self, range: Range) -> Self {
            self.0.selection_range = range;
            self
        }

        pub(super) fn build(self) -> DocumentSymbol {
            self.0
        }
    }
}

pub(crate) struct DocumentSymbols<'a> {
    doc: &'a InkDocument,
    qualified_names: bool,
    knot: Option<&'a str>,
    stitch: Option<&'a str>,
    pub(super) sym: Option<DocumentSymbol>,
}

impl<'a> DocumentSymbols<'a> {
    pub(crate) fn new(doc: &'a InkDocument, qualified_names: bool) -> Self {
        Self {
            doc,
            qualified_names,
            knot: None,
            stitch: None,
            sym: None,
        }
    }
    fn new_sym(&self, sym: DocumentSymbol) -> Self {
        Self {
            doc: self.doc,
            qualified_names: self.qualified_names,
            knot: self.knot,
            stitch: self.stitch,
            sym: Some(sym),
        }
    }

    fn name(&self, local_name: &str) -> String {
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
}

impl<'tree> Visitor<'tree, AllNamed<'tree>> for DocumentSymbols<'tree> {
    fn visit(&mut self, node: AllNamed) -> VisitInstruction<Self> {
        use VisitInstruction::*;
        match node {
            // recurse into these without creating a new level
            AllNamed::Choice(_)
            | AllNamed::Code(_)
            | AllNamed::Content(_)
            | AllNamed::Gather(_) => Descend,

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
            | AllNamed::Knot(_)
            | AllNamed::Label(_)
            | AllNamed::LineComment(_)
            | AllNamed::ListValueDefs(_)
            | AllNamed::ListValueDef(_)
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
            | AllNamed::Stitch(_)
            | AllNamed::String(_)
            | AllNamed::Tag(_)
            | AllNamed::Text(_)
            | AllNamed::Thread(_)
            | AllNamed::TodoComment(_)
            | AllNamed::Tunnel(_)
            | AllNamed::Unary(_) => Ignore,

            // Symbols (== levels) to be created
            AllNamed::Ink(ink) => DescendWith(
                self.new_sym(
                    SymbolBuilder::new(SymbolKind::FILE)
                        .name("unknown.ink")
                        .range(self.doc.lsp_range(&ink.range()))
                        .build(),
                ),
            ),

            AllNamed::KnotBlock(block) => {
                let knot = if let Ok(knot) = block.header() {
                    knot
                } else {
                    return Descend;
                };
                let kind = if knot.function().is_some() {
                    SymbolKind::FUNCTION
                } else {
                    SymbolKind::CLASS
                };
                let local_name = &self.doc.text[(&knot.name().unwrap()).byte_range()];
                let mut sym = SymbolBuilder::new(kind)
                    .name(self.name(local_name))
                    .range(self.doc.lsp_range(&block.range()))
                    .build();
                if let Some(Ok(params)) = knot.params() {
                    sym.detail = Some(self.doc.text[params.byte_range()].to_owned());
                }
                let mut new = self.new_sym(sym);
                new.knot = Some(local_name);
                new.stitch = None;
                DescendWith(new)
            }

            AllNamed::StitchBlock(block) => {
                let stitch = if let Ok(it) = block.header() {
                    it
                } else {
                    return Descend;
                };
                let local_name = &self.doc.text[(&stitch.name().unwrap()).byte_range()];
                let mut sym = SymbolBuilder::new(SymbolKind::CLASS)
                    .name(self.name(local_name))
                    .range(self.doc.lsp_range(&block.range()))
                    .build();
                if let Some(Ok(params)) = stitch.params() {
                    sym.detail = Some(self.doc.text[params.byte_range()].to_owned());
                }
                let mut new = self.new_sym(sym);
                new.stitch = Some(local_name);
                DescendWith(new)
            }

            AllNamed::External(external) => {
                if let Ok(name_node) = external.name() {
                    let mut sym = SymbolBuilder::new(SymbolKind::INTERFACE)
                        .name(&self.doc.text[name_node.byte_range()])
                        .range(self.doc.lsp_range(&name_node.range()))
                        .build();
                    if let Ok(params) = external.params() {
                        sym.detail = Some(self.doc.text[params.byte_range()].to_owned());
                    }
                    return Return(self.new_sym(sym));
                } else {
                    Ignore
                }
            }

            AllNamed::ChoiceBlock(block) => {
                if let Ok(choice) = block.header() {
                    if let Some(Ok(label)) = choice.label() {
                        let name_node = label.name().unwrap();
                        let mut sym = SymbolBuilder::new(SymbolKind::KEY)
                            .name(self.name(&self.doc.text[name_node.byte_range()]))
                            .range(self.doc.lsp_range(&block.range()))
                            .build();
                        sym.detail = choice
                            .marks()
                            .ok()
                            .map(|marks| &self.doc.text[marks.byte_range()])
                            .map(str::to_string);
                        return DescendWith(self.new_sym(sym));
                    }
                }
                Descend
            }

            AllNamed::GatherBlock(block) => {
                if let Ok(gather) = block.header() {
                    if let Some(Ok(label)) = gather.label() {
                        let name_node = label.name().unwrap();
                        let mut sym = SymbolBuilder::new(SymbolKind::KEY)
                            .name(self.name(&self.doc.text[name_node.byte_range()]))
                            .range(self.doc.lsp_range(&block.range()))
                            .build();
                        sym.detail = gather
                            .gather_marks()
                            .ok()
                            .map(|marks| &self.doc.text[marks.byte_range()])
                            .map(str::to_string);
                        DescendWith(self.new_sym(sym))
                    } else {
                        Descend
                    }
                } else {
                    Descend
                }
            }

            AllNamed::Global(global) => {
                let kind = match global.keyword().unwrap() {
                    GlobalKeyword::Const(_) => SymbolKind::CONSTANT,
                    GlobalKeyword::Var(_) => SymbolKind::VARIABLE,
                };
                let name_node = &global.name().unwrap();
                let sym = SymbolBuilder::new(kind)
                    .name(&self.doc.text[name_node.byte_range()])
                    .range(self.doc.lsp_range(&global.range()))
                    .selection_range(self.doc.lsp_range(&name_node.range()))
                    .build();
                Return(self.new_sym(sym))
            }

            AllNamed::List(list) => {
                let name_node = list.name().unwrap();
                let mut sym = SymbolBuilder::new(SymbolKind::ENUM)
                    .name(&self.doc.text[name_node.byte_range()])
                    .range(self.doc.lsp_range(&list.range()))
                    .selection_range(self.doc.lsp_range(&name_node.range()))
                    .build();
                sym.detail = list.values().ok().map(|defs| {
                    defs.values(&mut defs.walk())
                        .flat_map(|def| def.ok())
                        .filter_map(|def| def.name().ok())
                        .map(|identifier| identifier.byte_range())
                        .map(|range| self.doc.text[range].to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                });
                Return(self.new_sym(sym))
            }

            AllNamed::Temp(temp) => {
                let name_node = &temp.name().unwrap();
                let sym = SymbolBuilder::new(SymbolKind::VARIABLE)
                    .name(self.name(&self.doc.text[name_node.byte_range()]))
                    .range(self.doc.lsp_range(&temp.range()))
                    .selection_range(self.doc.lsp_range(&name_node.range()))
                    .build();
                Return(self.new_sym(sym))
            }
        }
    }

    fn combine(&mut self, child: Self) {
        if let Some(ref mut parent) = self.sym {
            if let Some(child) = child.sym {
                parent.children.get_or_insert_with(Vec::new).push(child);
            }
        } else {
            *self = child;
        }
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
