use crate::{
    ink_syntax::{
        types::{AllNamed, GlobalKeyword},
        VisitInstruction, Visitor,
    },
    lsp::{document::InkDocument, salsa::DocId},
};
use builder::SymbolBuilder;
use line_index::{LineCol, LineIndex, WideEncoding};
use lsp_types::{DocumentSymbol, Range, SymbolKind};
use type_sitter::{IncorrectKindCause, Node};

// IDEA: Maybe this shouldn't return LSP types?

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub(in crate::lsp::salsa) struct DocumentSymbolsQ(pub DocId);

impl mini_milc::Subquery<super::Ops, Vec<DocumentSymbol>> for DocumentSymbolsQ {
    fn value(
        &self,
        db: &impl mini_milc::Db<super::Ops>,
        old: mini_milc::Old<Vec<DocumentSymbol>>,
    ) -> mini_milc::Updated<Vec<DocumentSymbol>> {
        use crate::lsp::salsa::InkGetters as _;
        let doc = db.document(self.0.clone());
        let mut syms = DocumentSymbols::new(&*doc);
        let mut cursor = doc.tree.root_node().walk();
        let mut dummy = SymbolBuilder::new(SymbolKind::FILE)
            .name("dummy.ink")
            .range(Range::default())
            .build();
        syms.traverse(&mut cursor, &mut dummy);
        old.update(dummy.children.unwrap_or_default())
    }
}

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
        pub(super) fn name(self, name: impl Into<String>) -> SymbolBuilder<NeedsRange> {
            SymbolBuilder(
                DocumentSymbol {
                    name: name.into(),
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

        pub(super) fn maybe_detail(mut self, detail: Option<String>) -> Self {
            self.0.detail = detail;
            self
        }

        pub(super) fn build(self) -> DocumentSymbol {
            self.0
        }
    }
}

pub(crate) fn lsp_position(
    lines: &LineIndex,
    enc: Option<WideEncoding>,
    point: tree_sitter::Point,
) -> lsp_types::Position {
    let native = LineCol {
        line: point.row as u32,
        col: point.column as u32,
    };

    if let Some(enc) = enc {
        let wide = lines.to_wide(enc, native).unwrap();
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

pub(crate) fn lsp_range(
    lines: &LineIndex,
    enc: Option<WideEncoding>,
    node: &tree_sitter::Range,
) -> lsp_types::Range {
    let start = lsp_position(lines, enc, node.start_point);
    let end = lsp_position(lines, enc, node.end_point);
    lsp_types::Range { start, end }
}

pub(super) struct DocumentSymbols<'a> {
    text: &'a str,
    lines: &'a LineIndex,
    enc: Option<WideEncoding>,
}

impl<'a> DocumentSymbols<'a> {
    pub(crate) fn new(doc: &'a InkDocument) -> Self {
        Self {
            text: &doc.text,
            lines: &doc.lines,
            enc: doc.enc,
        }
    }

    /// add a child to the topmost parent
    fn push_sym(&mut self, parent: &mut DocumentSymbol, sym: DocumentSymbol) {
        parent.children.get_or_insert_default().push(sym);
    }

    /// Return the name of this symbol (if the local name isn’t empty)
    ///
    /// We have to make this check because, the following ink with an empty label:
    ///
    /// ``` ink
    /// - () Hey
    /// ```
    ///
    /// would crash the server (or the client process?) in VSCode because it returned an
    /// empty string (which evidently isn’t allowed).
    ///
    /// Returns the fully qualified name (i.e. the concatenation of all its parents’
    /// names plus the local name) if [`Self::qualified_names`] is `true`.
    fn text(&self, n: impl Node<'a>) -> Option<String> {
        let name = self.text[n.byte_range()].trim();
        if name.is_empty() {
            return None;
        }
        Some(name.to_owned())
    }

    fn lsp_range(&self, range: &tree_sitter::Range) -> lsp_types::Range {
        lsp_range(&self.lines, self.enc, range)
    }
}

impl<'tree> Visitor<'tree, AllNamed<'tree>> for DocumentSymbols<'tree> {
    type State = DocumentSymbol;

    fn visit(
        &mut self,
        node: AllNamed,
        parent: &mut Self::State,
    ) -> VisitInstruction<DocumentSymbol> {
        use VisitInstruction::*;
        match node {
            // recurse into these without creating a new level
            AllNamed::Choice(_)
            | AllNamed::Code(_)
            | AllNamed::Content(_)
            | AllNamed::Gather(_)
            | AllNamed::ListValueDefs(_) => Descend,

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
            | AllNamed::Knot(_)
            | AllNamed::Label(_)
            | AllNamed::LineComment(_)
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
            | AllNamed::Stitch(_)
            | AllNamed::String(_)
            | AllNamed::Tag(_)
            | AllNamed::Text(_)
            | AllNamed::Thread(_)
            | AllNamed::TodoComment(_)
            | AllNamed::Tunnel(_)
            | AllNamed::Unary(_) => Ignore,

            // Symbols (== levels) to be created
            AllNamed::Ink(_) => Descend,

            AllNamed::KnotBlock(block) => {
                let Ok(knot) = block.header() else {
                    return Descend;
                };

                let kind = knot
                    .function()
                    .map(|_| SymbolKind::FUNCTION)
                    .unwrap_or(SymbolKind::CLASS);

                // Insert a dummy name if needed.
                // Otherwise we'd have to somehow keep track of whether or not
                // we started a new level here
                let name = self
                    .text(knot.name())
                    .unwrap_or_else(|| String::from("DUMMY KNOT"));

                let sym = SymbolBuilder::new(kind)
                    .name(name)
                    .range(self.lsp_range(&block.range()))
                    .maybe_detail(knot.params().and_then(|params| self.text(params)))
                    .build();

                DescendWith(sym)
            }

            AllNamed::StitchBlock(block) => {
                let Ok(stitch) = block.header() else {
                    return Descend;
                };
                let name = self
                    .text(stitch.name())
                    .unwrap_or_else(|| String::from("DUMMY STITCH"));

                let sym = SymbolBuilder::new(SymbolKind::CLASS)
                    .name(name)
                    .range(self.lsp_range(&block.range()))
                    .maybe_detail(stitch.params().and_then(|params| self.text(params)))
                    .build();

                DescendWith(sym)
            }

            AllNamed::External(external) => {
                if let Some(name) = self.text(external.name()) {
                    let sym = SymbolBuilder::new(SymbolKind::INTERFACE)
                        .name(name)
                        .range(self.lsp_range(&external.name().range()))
                        .maybe_detail(external.params().ok().and_then(|params| self.text(params)))
                        .build();
                    self.push_sym(parent, sym);
                }
                Ignore
            }

            AllNamed::ChoiceBlock(block) => {
                if let Ok(choice) = block.header() {
                    if let Some(Ok(label)) = choice.label() {
                        if let Some(name) = self.text(label.name()) {
                            let sym = SymbolBuilder::new(SymbolKind::KEY)
                                .name(name)
                                .range(self.lsp_range(&block.range()))
                                .maybe_detail(
                                    choice.marks().ok().and_then(|marks| self.text(marks)),
                                )
                                .build();
                            return DescendWith(sym);
                        }
                    }
                }
                Descend
            }

            AllNamed::GatherBlock(block) => {
                if let Ok(gather) = block.header() {
                    if let Some(Ok(label)) = gather.label() {
                        let name_node = label.name();
                        if let Some(name) = self.text(name_node) {
                            let sym = SymbolBuilder::new(SymbolKind::KEY)
                                .name(name)
                                .range(self.lsp_range(&block.range()))
                                .maybe_detail(
                                    gather
                                        .gather_marks()
                                        .ok()
                                        .and_then(|marks| self.text(marks)),
                                )
                                .build();
                            return DescendWith(sym);
                        }
                    }
                }
                Descend
            }

            AllNamed::Global(global) => {
                let kind = match global.keyword() {
                    Ok(GlobalKeyword::Const(_)) => SymbolKind::CONSTANT,
                    Ok(GlobalKeyword::Var(_)) => SymbolKind::VARIABLE,
                    Err(_) => SymbolKind::NULL,
                };
                if let Some(name) = self.text(global.name()) {
                    let sym = SymbolBuilder::new(kind)
                        .name(name)
                        .range(self.lsp_range(&global.range()))
                        .selection_range(self.lsp_range(&global.name().range()))
                        .build();
                    self.push_sym(parent, sym);
                }
                Ignore
            }

            AllNamed::List(list) => {
                let name_node = list.name();
                let name = self
                    .text(name_node)
                    .unwrap_or_else(|| String::from("DUMMY LIST"));
                let sym = SymbolBuilder::new(SymbolKind::ENUM)
                    .name(name)
                    .range(self.lsp_range(&list.range()))
                    .selection_range(self.lsp_range(&name_node.range()))
                    .build();

                DescendWith(sym)
            }

            AllNamed::ListValueDef(def) => {
                let name_node = def.name();
                if let Some(name) = self.text(name_node) {
                    let mut sym = SymbolBuilder::new(SymbolKind::ENUM_MEMBER)
                        .name(name)
                        .range(self.lsp_range(&def.range()))
                        .selection_range(self.lsp_range(&name_node.range()))
                        .build();
                    sym.detail = match (def.value(), def.lparen()) {
                        (None, None) => None,
                        (None, Some(_)) => Some("()".to_string()),
                        (Some(value), None) => {
                            Some(format!("= {}", &self.text[value.byte_range()]))
                        }
                        (Some(value), Some(_)) => {
                            Some(format!("(= {})", &self.text[value.byte_range()]))
                        }
                    };
                    self.push_sym(parent, sym);
                }
                Ignore
            }

            AllNamed::TempDef(temp) => {
                let name_node = temp.name();
                if let Some(name) = self.text(name_node) {
                    let sym = SymbolBuilder::new(SymbolKind::VARIABLE)
                        .name(name)
                        .range(self.lsp_range(&temp.range()))
                        .selection_range(self.lsp_range(&name_node.range()))
                        .build();
                    self.push_sym(parent, sym);
                }
                Ignore
            }
        }
    }

    fn combine(parent: &mut Self::State, other: Self::State) {
        parent.children.get_or_insert_default().push(other);
    }

    fn visit_error(&mut self, err: type_sitter::IncorrectKind) -> VisitInstruction<DocumentSymbol> {
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
