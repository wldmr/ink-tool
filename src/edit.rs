use tree_sitter::{InputEdit, Node, Point};

#[derive(Debug)]
pub struct Change {
    pub range: InputEdit,
    pub text: String,
}

pub fn replace(node: &Node, text: &str) -> Change {
    Change {
        range: InputEdit {
            start_byte: node.start_byte(),
            old_end_byte: node.end_byte(),
            new_end_byte: node.start_byte() + text.len(),
            // Since we're not going the reuse the tree, we won't spend any time getting these right:
            start_position: node.start_position(),
            old_end_position: node.start_position(),
            new_end_position: node.start_position(),
        },
        text: text.to_owned(),
    }
}

pub fn replace_range(start: usize, end: usize, text: &str) -> Change {
    Change {
        range: InputEdit {
            start_byte: start,
            old_end_byte: end,
            new_end_byte: start + text.len(),
            // Since we're not going the reuse the tree, we won't spend any time getting these right:
            start_position: Point::new(0, 0),
            old_end_position: Point::new(0, 0),
            new_end_position: Point::new(0, 0),
        },
        text: text.to_owned(),
    }
}
