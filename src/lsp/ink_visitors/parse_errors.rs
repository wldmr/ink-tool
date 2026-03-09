use ink_document::InkDocument;
use lsp_types::{Diagnostic, DiagnosticSeverity};
use std::hint::unreachable_unchecked;
use tree_traversal::{VisitInstruction, Visitor};
use type_sitter::{Node, UntypedNode};

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

impl<'a> Visitor<'a, UntypedNode<'a>> for ParseErrorsVisitor<'a> {
    type State = ParseErrors;

    fn visit(
        &mut self,
        node: UntypedNode<'a>,
        state: &mut Self::State,
    ) -> VisitInstruction<Self::State> {
        use VisitInstruction::{Descend, Ignore};

        if node.is_error() || node.is_missing() {
            state.push(Diagnostic {
                range: self.doc.lsp_range(node.range()),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some(String::from("ink-tool")),
                // tree-sitter makes it very hard to be specific here, so we don't even try.
                message: String::from("Syntax error"),
                ..Diagnostic::default()
            });
            Ignore
        } else if node.has_error() {
            // One of our descendants is the actual error.
            Descend
        } else {
            // No errors here, move on.
            Ignore
        }
    }

    fn combine(_: &mut Self::State, _: Self::State) {
        // SAFETY: We don't use substates
        unsafe { unreachable_unchecked() }
    }
}
