use tree_sitter::TreeCursor;
use type_sitter_lib::{IncorrectKind, Node};

/// Can walk a tree of tree-sitter nodes and build up a new structure
pub(crate) trait Visitor<'a, N: Node<'a>>: Sized {
    /// Visit a node
    fn visit(&mut self, node: N) -> VisitInstruction<Self>;

    /// What to do when a node can't be parsed
    fn visit_error(&mut self, err: IncorrectKind) -> VisitInstruction<Self>;

    /// Combine two target values.
    ///
    /// In the default implementation of [`Self::traverse`], `other` is the result of a call to
    /// [`Self::traverse`] on a direct child of `self`.
    fn combine(&mut self, other: Self);

    /// Traverse a document tree.
    ///
    /// Default implementation is depth-first.
    fn traverse(&mut self, cursor: &mut tree_sitter::TreeCursor<'a>) -> Option<Self> {
        let node = N::try_from_raw(cursor.node());
        let instruction = match node {
            Ok(ok) => self.visit(ok),
            Err(err) => self.visit_error(err),
        };
        match instruction {
            VisitInstruction::Ignore => None,
            VisitInstruction::Return(leaf) => Some(leaf),
            VisitInstruction::Descend => {
                traverse_children(self, cursor);
                None
            }
            VisitInstruction::DescendWith(mut new_parent) => {
                traverse_children(&mut new_parent, cursor);
                Some(new_parent)
            }
        }
    }
}

fn traverse_children<'a, N: Node<'a>>(me: &mut impl Visitor<'a, N>, cursor: &mut TreeCursor<'a>) {
    if cursor.goto_first_child() {
        loop {
            if let Some(child) = me.traverse(cursor) {
                me.combine(child);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

pub(crate) enum VisitInstruction<T> {
    // Don't return anything for this node, and don't recurse into children.
    Ignore,
    /// Don't return anything for this node, but recurse into children.
    Descend,
    /// Return a value, but don't recurse into children.
    Return(T),
    /// Return a value, but recurse into children first.
    DescendWith(T),
}
