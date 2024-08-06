use tree_sitter::{InputEdit, Node, Point};

#[derive(Debug)]
pub struct Change {
    pub range: InputEdit,
    pub text: String,
}

struct StrInfo {
    newlines: usize,
    len_of_last_line: usize,
}

// I'd feel more comfortable if we'd use an API that makes linebreaks explicit, instead of walking through every string.
fn examine_str(s: &str) -> StrInfo {
    let mut info = StrInfo {
        newlines: 0,
        len_of_last_line: 0,
    };
    for b in s.as_bytes() {
        if *b == ('\n' as u8) {
            info.newlines += 1;
            info.len_of_last_line = 0;
        } else {
            info.len_of_last_line += 1;
        }
    }
    info
}

pub fn replace(node: &Node, text: &str) -> Change {
    let strinfo = examine_str(text);
    let mut new_end_position = node.start_position();
    new_end_position.row += strinfo.newlines;
    new_end_position.column = strinfo.len_of_last_line;
    Change {
        range: InputEdit {
            start_byte: node.start_byte(),
            old_end_byte: node.end_byte(),
            new_end_byte: node.start_byte() + text.len(),
            // TODO get these right
            start_position: node.start_position(),
            old_end_position: node.end_position(),
            new_end_position,
        },
        text: text.to_owned(),
    }
}

pub fn replace_between(left: &Node, right: &Node, text: &str) -> Change {
    let strinfo = examine_str(text);
    let mut new_end_position = left.end_position();
    new_end_position.row += strinfo.newlines;
    new_end_position.column = strinfo.len_of_last_line;
    Change {
        range: InputEdit {
            start_byte: left.end_byte(),
            old_end_byte: right.start_byte(),
            new_end_byte: left.end_byte() + text.len(),
            // TODO get these right
            start_position: left.end_position(),
            old_end_position: right.start_position(),
            new_end_position,
        },
        text: text.to_owned(),
    }
}
