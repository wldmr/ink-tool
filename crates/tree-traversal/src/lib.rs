mod visitor;

pub use visitor::{VisitInstruction, Visitor};

use std::iter::from_fn;
use type_sitter::{Node, UntypedNode};

/// Extension trait to facilate moving around a type_sitter tree.
///
/// Also fills in some gaps in APIs that [`tree_sitter::Node`] has that are
/// missing from [`type_sitter::Node`].
pub trait TreeTraversal<'a>: Node<'a> {
    /// All `T`-nodes between `self` and `node`, if `node` is a descendant of of `self`
    fn descend_to<T: Node<'a>>(self, target: impl Node<'a>) -> impl Iterator<Item = T> {
        let first = Some(self.upcast());
        let start = target.start_byte();
        let path = std::iter::successors(first, move |node| node.first_child_for_byte(start));
        path.filter_map(|node| node.downcast().ok())
    }

    /// All `T`-nodes between `node` and `self`, if `self` is a descendant of `node`
    ///
    /// (this is the reverse of [`Self::down_to`])
    fn ascend_to<T: Node<'a>>(self, root: impl Node<'a>) -> impl Iterator<Item = T> {
        // Treesitter nodes don’t really know their parents, so we scan downwards from the
        // root, and then just return the reverse.
        root.descend_to(self).collect::<Vec<_>>().into_iter().rev()
    }

    /// Typed variant of the normal `node.parent()` method.
    ///
    /// Equally inefficient. Consider [`up_to()`] if you want to walk up many parents.
    fn parent_of_type<T: Node<'a>>(self) -> Option<T> {
        self.parent().and_then(|it| it.downcast().ok())
    }

    fn child(&self, idx: usize) -> Option<UntypedNode<'a>> {
        self.raw().child(idx).map(UntypedNode::new)
    }

    fn child_of_type<T: Node<'a>>(&self, idx: usize) -> Option<T> {
        self.child(idx).and_then(|it| it.downcast().ok())
    }

    fn contains(&self, target: &impl Node<'a>) -> bool {
        self.start_byte() <= target.start_byte() && target.end_byte() <= self.end_byte()
    }

    /// Typesitter should have had this to begin with.
    fn descendant_for_point_range(
        self,
        start: type_sitter::Point,
        end: type_sitter::Point,
    ) -> Option<UntypedNode<'a>> {
        type_sitter::raw::Node::descendant_for_point_range(self.raw(), start, end)
            .map(UntypedNode::from)
    }

    fn descendant_containing(self, node: impl Node<'a>) -> Option<UntypedNode<'a>> {
        self.descendant_for_point_range(node.start_position(), node.end_position())
    }

    fn first_child_for_byte(self, byte: usize) -> Option<UntypedNode<'a>> {
        self.raw().first_child_for_byte(byte).map(UntypedNode::new)
    }

    /// Walk down from `self`. Includes `self` (if it is of the `T` type).
    fn depth_first<T: Node<'a>>(self) -> impl Iterator<Item = T> {
        let mut cursor = self.walk();
        let myself = std::iter::once(cursor.node());
        let descendants = from_fn(move || move_depth_first(&mut cursor).then(|| cursor.node()));
        let nodes = myself.chain(descendants);
        nodes.filter_map(|node| node.downcast::<T>().ok())
    }
}
impl<'a, N: Node<'a>> TreeTraversal<'a> for N {}

fn move_depth_first<'a>(cursor: &mut type_sitter::TreeCursor<'a>) -> bool {
    // Walking down: Child or next sibling is a node we haven't seen before.
    if cursor.goto_first_child() || cursor.goto_next_sibling() {
        return true;
    }
    // Walking up: We've seen each parent before (on the way down), but not its next sibling:
    while cursor.goto_parent() {
        if cursor.goto_next_sibling() {
            return true;
        }
    }
    // No more parents means we're done.
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use indoc::indoc;
    use type_sitter::Point;

    #[test]
    fn descend_ascend() {
        const TEXT: &str = indoc! {"
            === Knot ===

            Text

            = Stitch

            Text {true}
        "};
        const INSIDE_TRUE: Point = Point { row: 6, column: 7 };
        const EXPECTED_PATH_DOWN: [&str; 7] = [
            "ink",
            "knot_block",
            "stitch_block",
            "paragraph",
            "eval",
            "boolean",
            "true",
        ];

        let tree = {
            let language = tree_sitter_ink::LANGUAGE.into();
            let mut parser = type_sitter::Parser::<ink_syntax::Ink>::new(&language).unwrap();
            parser.parse(TEXT, None).unwrap()
        };
        let root = tree.root_node().unwrap();

        let target = root
            .descendant_for_point_range(INSIDE_TRUE, INSIDE_TRUE)
            .unwrap();

        let path_down: Vec<&str> = root
            .descend_to::<type_sitter::UntypedNode>(target)
            .map(|it| it.kind())
            .collect();
        check!(path_down == EXPECTED_PATH_DOWN);

        let path_up: Vec<&str> = root
            .descend_to::<type_sitter::UntypedNode>(target)
            .map(|it| it.kind())
            .collect();
        let expected_path_up = EXPECTED_PATH_DOWN.into_iter().rev().collect::<Vec<_>>();
        check!(path_up == expected_path_up);

        let scopes: Vec<&str> = root
            .descend_to::<ink_syntax::ScopeBlock>(target)
            .map(|it| it.kind())
            .collect();
        check!(
            scopes == ["ink", "knot_block", "stitch_block"],
            "filtered by type"
        );
    }
}
