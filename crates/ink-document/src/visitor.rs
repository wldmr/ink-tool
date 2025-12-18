#![allow(unused_variables)] // We provide a few default impls that don't use their arugments

use type_sitter::{IncorrectKind, Node};

/// Can walk a tree of tree-sitter nodes and build up a new structure
pub(crate) trait Visitor<'a, N: Node<'a>>: Sized {
    type State;

    /// Called upon entering a node
    fn visit(&mut self, node: N, state: &mut Self::State) -> VisitInstruction<Self::State>;

    /// What to do when a node can’t be parsed.
    ///
    /// The default implementation will simply descend (because child values might still
    /// be relevant, even if this one is an error). Override this method if you want to
    /// shortcut the traversal under certain circumstances.
    fn visit_error(&mut self, err: IncorrectKind) -> VisitInstruction<Self::State> {
        VisitInstruction::Descend
    }

    /// Merge the `inner` into the `outer` state.
    fn combine(outer: &mut Self::State, inner: Self::State);

    /// Traverse a document tree, converting the given state to a new one.
    ///
    /// Default implementation is depth-first.
    fn traverse_with_cursor_and_state(
        &mut self,
        cursor: &mut type_sitter::TreeCursor<'a>,
        mut state: Self::State,
    ) -> Self::State {
        use VisitInstruction::*;
        let node = cursor.node();

        let instruction = match node.downcast() {
            Ok(ok) => self.visit(ok, &mut state),
            Err(err) => self.visit_error(err),
        };

        let (inner_state, should_descend) = match instruction {
            Ignore => (None, false),
            Descend => (None, true),
            DescendWith(inner_state) => (Some(inner_state), true),
        };

        if should_descend {
            if let Some(inner_state) = inner_state {
                let inner_state = traverse_children(self, inner_state, cursor);
                Self::combine(&mut state, inner_state);
            } else {
                state = traverse_children(self, state, cursor);
            }
        }
        state
    }

    /// Convenience method to use an existing cursor, and create the default start state.
    #[inline]
    fn traverse_with_cursor(&mut self, cursor: &mut type_sitter::TreeCursor<'a>) -> Self::State
    where
        Self::State: Default,
    {
        self.traverse_with_cursor_and_state(cursor, Self::State::default())
    }

    /// Convenience method to start traversing at a node, with the default start state.
    ///
    /// If you already have a cursor, see [`Self::traverse_with_cursor_and_state`].
    #[inline]
    fn traverse(&mut self, node: impl type_sitter::Node<'a>) -> Self::State
    where
        Self::State: Default,
    {
        self.traverse_with_cursor_and_state(&mut node.walk(), Self::State::default())
    }

    /// Convenience method to start traversing at a node, with a customo start state.
    ///
    /// If you already have a cursor, see [`Self::traverse_with_cursor_and_state`].
    #[inline]
    fn traverse_with_state(
        &mut self,
        node: impl type_sitter::Node<'a>,
        start: Self::State,
    ) -> Self::State {
        self.traverse_with_cursor_and_state(&mut node.walk(), start)
    }
}

fn traverse_children<'a, N: Node<'a>, V: Visitor<'a, N>>(
    me: &mut V,
    mut state: V::State,
    cursor: &mut type_sitter::TreeCursor<'a>,
) -> V::State {
    if cursor.goto_first_child() {
        loop {
            state = me.traverse_with_cursor_and_state(cursor, state);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
    state
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
