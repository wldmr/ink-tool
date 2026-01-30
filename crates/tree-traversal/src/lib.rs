mod visitor;

use std::iter::from_fn;
use type_sitter::Node;

pub use visitor::{VisitInstruction, Visitor};

/// Walk up from `start`, ending at `root`, emitting all nodes of type `Out`
///
/// Includes `start` if it is of type `Out`.
pub fn parents<'a, Out: Node<'a>>(root: impl Node<'a>, start: impl Node<'a>) -> Parents<'a, Out> {
    let mut cursor = root.walk();
    cursor.0.reset_to(&start.walk().0); // typesitter doesn't have a reset_to method. :-/
    Parents {
        // Include the start node if it already matches the type.
        next: cursor
            .node()
            .downcast()
            .ok()
            .or_else(|| next_parent(&mut cursor)),
        cursor,
    }
}

/// Iterator that walks all parents of type `T`
pub struct Parents<'a, T: Node<'a>> {
    next: Option<T>,
    cursor: type_sitter::TreeCursor<'a>,
}

impl<'a, T: Node<'a>> Iterator for Parents<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        std::mem::replace(&mut self.next, next_parent(&mut self.cursor))
    }
}

fn next_parent<'a, T: Node<'a>>(cursor: &mut type_sitter::TreeCursor<'a>) -> Option<T> {
    while cursor.goto_parent() {
        if let Ok(found) = cursor.node().downcast() {
            return Some(found);
        }
    }
    None
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
