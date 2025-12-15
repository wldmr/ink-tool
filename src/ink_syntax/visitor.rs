#![allow(unused_variables)] // We provide a few default impls that don't use their arugments

use type_sitter::{IncorrectKind, Node};

/// Can walk a tree of tree-sitter nodes and build up a new structure
pub(crate) trait Visitor<'a, N: Node<'a>>: Sized {
    type State;

    /// Called upon entering a node
    fn visit(&mut self, node: N, state: &mut Self::State) -> VisitInstruction<Self::State>;

    /// What to do when a node can't be parsed
    fn visit_error(&mut self, err: IncorrectKind) -> VisitInstruction<Self::State> {
        VisitInstruction::Ignore
    }

    /// Combine two target values.
    ///
    /// In the default implementation of [`Self::traverse`], `other` is the result of a call to
    /// [`Self::traverse`] on a direct child of `self`.
    fn combine(outer: &mut Self::State, inner: Self::State);

    /// Traverse a document tree.
    ///
    /// Default implementation is depth-first.
    fn traverse(
        &mut self,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        outer_state: &mut Self::State,
    ) {
        use VisitInstruction::*;
        let node = N::try_from_raw(cursor.node());

        let instruction = match node {
            Ok(ok) => self.visit(ok, outer_state),
            Err(err) => self.visit_error(err),
        };

        let (new_state, should_descend) = match instruction {
            Ignore => (None, false),
            Descend => (None, true),
            DescendWith(new) => (Some(new), true),
        };

        if should_descend {
            if let Some(mut inner_state) = new_state {
                traverse_children(self, &mut inner_state, cursor);
                Self::combine(outer_state, inner_state);
            } else {
                traverse_children(self, outer_state, cursor);
            }
        }
    }
}

fn traverse_children<'a, N: Node<'a>, V: Visitor<'a, N>>(
    me: &mut V,
    state: &mut V::State,
    cursor: &mut tree_sitter::TreeCursor<'a>,
) {
    if cursor.goto_first_child() {
        loop {
            me.traverse(cursor, state);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

pub(crate) enum VisitInstruction<T> {
    /// Don't return anything for this node, and don't recurse into children.
    Ignore,
    /// Don't return anything for this node, but recurse into children.
    Descend,
    /// Return a value, but recurse into children first.
    ///
    /// `combine` the outer state with this one on the way up.
    DescendWith(T),
}
