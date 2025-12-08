pub mod types {
    use type_sitter;
    include!(concat!(env!("OUT_DIR"), "/type_sitter_ink.rs"));
}

pub mod traversal {
    use std::iter::{from_fn, successors};
    use type_sitter::Node as _;

    // Walk up the tree, emitting all nodes of type `Out`
    pub fn parent<'a, In, Out>(start: In) -> impl Iterator<Item = Out> + use<'a, In, Out>
    where
        In: type_sitter::Node<'a>,
        Out: type_sitter::Node<'a>,
    {
        // parents can't be found with cursors because a cursor can't go outside its defining node.
        successors(Some(start.upcast()), |node| node.parent())
            .filter_map(|node| node.downcast::<Out>().ok())
    }

    // Walk down from `start`. Includes initial node (if it is of the `T` type).
    pub fn children<'a, S, T>(start: S) -> impl Iterator<Item = T> + use<'a, S, T>
    where
        S: type_sitter::Node<'a>,
        T: type_sitter::Node<'a>,
    {
        let start = start.into_raw();
        let mut cursor = start.walk();
        let has_children = cursor.goto_first_child();
        let children = from_fn(move || {
            let has_moved = has_children && cursor.goto_next_sibling();
            has_moved.then(|| cursor.node())
        });
        children
            .inspect(|it| log::trace!("found direct child {it:?}"))
            .filter_map(|node| T::try_from_raw(node).ok())
    }

    // Walk down from `start`. Includes initial node (if it is of the `T` type).
    pub fn depth_first<'a, S, T>(start: S) -> impl Iterator<Item = T> + use<'a, S, T>
    where
        S: type_sitter::Node<'a>,
        T: type_sitter::Node<'a>,
    {
        let mut cursor = start.walk();
        let myself = std::iter::once(cursor.node());
        let descendants = from_fn(move || move_depth_first(&mut cursor).then(|| cursor.node()));
        let nodes = myself.chain(descendants);
        nodes.filter_map(|node| node.downcast::<T>().ok())
    }

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
}

mod visitor;
pub(crate) use visitor::Visit;
pub(crate) use visitor::VisitInstruction;
pub(crate) use visitor::Visitor;
