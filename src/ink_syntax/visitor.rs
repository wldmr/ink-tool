use std::ops::ControlFlow;
use type_sitter_lib::{IncorrectKindCause, Node, NodeResult};

#[allow(unused)]
pub trait Visitor<'a, T: Node<'a>> {
    type Output;

    fn visit(&mut self, node: NodeResult<T>) -> ControlFlow<Self::Output>;

    fn leave(&mut self, node: NodeResult<T>) {}

    fn walk(&mut self, cursor: &mut tree_sitter::TreeCursor<'a>) {
        let node = T::try_from_raw(cursor.node());
        let skip = node.is_err_and(|it| matches!(it.cause(), IncorrectKindCause::OtherKind(_)));
        if skip || self.visit(node).is_continue() {
            if cursor.goto_first_child() {
                loop {
                    self.walk(cursor);
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }

            if !skip {
                self.leave(node);
            }
        }
    }
}
