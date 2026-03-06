use ink_document::InkDocument;
use ink_syntax::AllNamed;
use lsp_types::{Diagnostic, DiagnosticSeverity};
use std::hint::unreachable_unchecked;
use tree_traversal::{VisitInstruction, Visitor};
use type_sitter::{IncorrectKind, Node};

pub type ParseErrors = Vec<Diagnostic>;

pub fn parse_errors(doc: &InkDocument) -> Vec<Diagnostic> {
    ParseErrorsVisitor::new(doc).traverse(doc.root())
}

struct ParseErrorsVisitor<'a> {
    doc: &'a InkDocument,
}

impl<'a> ParseErrorsVisitor<'a> {
    fn new(doc: &'a InkDocument) -> Self {
        Self { doc }
    }
}

impl<'a> Visitor<'a, AllNamed<'a>> for ParseErrorsVisitor<'a> {
    type State = ParseErrors;

    fn visit(
        &mut self,
        node: AllNamed<'a>,
        _state: &mut Self::State,
    ) -> VisitInstruction<Self::State> {
        use VisitInstruction::{Descend, Ignore};
        if node.has_error() {
            Descend
        } else {
            Ignore
        }
    }

    fn visit_error_with_state(
        &mut self,
        err: IncorrectKind,
        state: &mut Self::State,
    ) -> VisitInstruction<Self::State> {
        if err
            .node
            .raw()
            .children(&mut err.node.raw().walk())
            .any(|child| child.has_error())
        {
            return VisitInstruction::Descend;
        }

        let mut diag = Diagnostic {
            range: self.doc.lsp_range(err.node.range()),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some(String::from("ink-tool")),
            message: String::from("Syntax error"),
            ..Diagnostic::default()
        };

        match err.cause() {
            type_sitter::IncorrectKindCause::Error => {}
            type_sitter::IncorrectKindCause::Missing => {
                diag.message = format!("Missing {}", err.actual_kind());
            }
            type_sitter::IncorrectKindCause::OtherKind(_) => return VisitInstruction::Descend,
        };

        state.push(diag);

        VisitInstruction::Ignore
    }

    fn combine(_: &mut Self::State, _: Self::State) {
        // SAFETY: We don't use substates
        unsafe { unreachable_unchecked() }
    }
}
