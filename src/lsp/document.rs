use line_index::{LineCol, LineIndex, WideEncoding};

use super::tree::ink_parser;

pub(crate) struct InkDocument {
    parser: tree_sitter::Parser,
    tree: tree_sitter::Tree,
    enc: Option<WideEncoding>,
    lines: line_index::LineIndex,
    text: String,
}

pub(crate) type DocumentEdit = (Option<lsp_types::Range>, String);

/// Public API
impl InkDocument {
    pub(crate) fn new(text: String, enc: Option<WideEncoding>) -> Self {
        let mut parser = ink_parser();
        let tree = parser
            .parse(&text, None)
            .expect("parsing should always work");
        let lines = LineIndex::new(&text);
        Self {
            parser,
            tree,
            lines,
            enc,
            text,
        }
    }

    pub fn edit(&mut self, edits: Vec<DocumentEdit>) {
        eprintln!("applying {} edits", edits.len());
        for (range, new_text) in edits.into_iter() {
            let edit = range.map(|range| self.edit_range(range, &new_text));
            let modified_tree = if let Some(edit) = edit {
                self.text
                    .replace_range(edit.start_byte..edit.old_end_byte, &new_text);
                self.tree.edit(&edit);
                Some(&self.tree)
            } else {
                self.text = new_text;
                None
            };
            self.tree = self
                .parser
                .parse(&self.text, modified_tree)
                .expect("parsing must work");
            self.lines = LineIndex::new(&self.text);
        }
        eprintln!("document now is {}", self.text);
    }
}

/// Private Helpers
impl InkDocument {
    fn edit_range(&self, range: lsp_types::Range, new_text: &str) -> tree_sitter::InputEdit {
        let start = self.to_line_col(range.start);
        let end = self.to_line_col(range.end);

        let start_byte = self.to_byte(start);
        let old_end_byte = self.to_byte(end);
        let new_end_byte = start_byte + new_text.bytes().len();

        tree_sitter::InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,

            /* https://github.com/tree-sitter/tree-sitter/discussions/1793#discussioncomment-3094712
            > So if you never plan to read from a tree again after editing it,
            > except to re-parse and create a new tree, you can actually pass
            > bogus row/column information if you want, and re-parsing will still work fine.
            */
            start_position: tree_sitter::Point::new(0, 0),
            old_end_position: tree_sitter::Point::new(0, 0),
            new_end_position: tree_sitter::Point::new(0, 0),
        }
    }

    fn to_line_col(&self, pos: lsp_types::Position) -> LineCol {
        if let Some(enc) = self.enc {
            let wide = line_index::WideLineCol {
                line: pos.line,
                col: pos.character,
            };
            self.lines
                .to_utf8(enc, wide)
                .expect("Conversion from wide to UTF-8 mustn't fail")
        } else {
            LineCol {
                line: pos.line,
                col: pos.character,
            }
        }
    }

    fn to_byte(&self, pos: LineCol) -> usize {
        self.lines.offset(pos).expect("mustn't fail, dammit").into()
    }
}
