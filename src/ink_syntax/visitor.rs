use tree_sitter::TreeCursor;
use type_sitter::{IncorrectKind, Node, NodeResult};

pub(crate) enum Visit<T> {
    Enter(T),
    Leave(T),
}

/// Can walk a tree of tree-sitter nodes and build up a new structure
pub(crate) trait Visitor<'a, N: Node<'a>>: Sized {
    fn voyage(&mut self, visit: Visit<N>) -> VisitInstruction<Self> {
        #[allow(deprecated)]
        match visit {
            Visit::Enter(node) => self.visit(node),
            Visit::Leave(node) => {
                self.leave(node);
                VisitInstruction::Ignore
            }
        }
    }
    #[deprecated = "in favor of the voyage methods"]
    /// Called upon entering a node
    fn visit(&mut self, #[allow(unused)] node: N) -> VisitInstruction<Self> {
        VisitInstruction::Descend
    }

    /// Called upon leavig a node
    #[deprecated = "in favor of the voyage methods"]
    fn leave(&mut self, #[allow(unused)] node: N) {
        // no-op by default
    }

    /// What to do when a node can't be parsed
    fn voyage_error(&mut self, visit: Visit<IncorrectKind>) -> VisitInstruction<Self> {
        #[allow(deprecated)]
        match visit {
            Visit::Enter(err) => self.visit_error(err),
            Visit::Leave(err) => {
                self.leave_error(err);
                VisitInstruction::Ignore
            }
        }
    }

    /// What to do when a node can't be parsed
    #[deprecated = "in favor of the voyage methods"]
    fn visit_error(&mut self, #[allow(unused)] err: IncorrectKind) -> VisitInstruction<Self> {
        VisitInstruction::Ignore
    }

    /// What to do when leaving a node that can't be parsed
    #[deprecated = "in favor of the voyage methods"]
    fn leave_error(&mut self, #[allow(unused)] err: IncorrectKind) {
        // no-op by default
    }

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
            Ok(ok) => self.voyage(Visit::Enter(ok)),
            Err(err) => self.voyage_error(Visit::Enter(err)),
        };
        match instruction {
            VisitInstruction::Ignore => None,
            VisitInstruction::Return(leaf) => Some(leaf),
            VisitInstruction::Descend => {
                traverse_children(self, cursor, node);
                None
            }
            VisitInstruction::DescendWith(mut new_parent) => {
                traverse_children(&mut new_parent, cursor, node);
                Some(new_parent)
            }
        }
    }
}

fn traverse_children<'a, N: Node<'a>>(
    me: &mut impl Visitor<'a, N>,
    cursor: &mut TreeCursor<'a>,
    current_node: NodeResult<'a, N>,
) {
    debug_assert_eq!(
        cursor.node(),
        *current_node.raw(),
        "cursor position must correspond to current node"
    );
    if cursor.goto_first_child() {
        loop {
            if let Some(child) = me.traverse(cursor) {
                me.combine(child);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        let _ = match current_node {
            Ok(ok) => me.voyage(Visit::Leave(ok)),
            Err(err) => me.voyage_error(Visit::Leave(err)),
        };
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
