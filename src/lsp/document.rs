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

    pub(crate) fn edit(&mut self, edits: Vec<DocumentEdit>) {
        // eprintln!("applying {} edits", edits.len());
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
        // eprintln!("document now is {}", self.text);
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;

    use super::{DocumentEdit, InkDocument};

    /// The important thing here is that each edit's coordinates is relative to the previous edit,
    /// not the initial document.
    #[test]
    fn multiple_edits() {
        let text = "hello world\nhow's it hanging?".to_string();
        let mut document = InkDocument::new(text, None);
        document.edit(vec![
            edit((0, 0), (0, 1), "H"),      // Hello world
            edit((0, 1), (0, 5), "i"),      // Hi world
            edit((0, 3), (0, 8), "gang!"),  // Hi gang!
            edit((1, 0), (1, 1), "H"),      // How's it hanging?
            edit((1, 9), (1, 16), "going"), // How's it going?
        ]);
        assert_str_eq!(document.text, "Hi gang!\nHow's it going?");
    }

    #[test]
    fn giving_no_range_means_replace_all_text() {
        let text = "some text".to_string();
        let mut document = InkDocument::new(text, None);
        document.edit(vec![
            (
                None,
                "some ignored text\nthis will be completely overwritted\nby the next edit"
                    .to_string(),
            ),
            (None, "final version".to_string()),
        ]);
        assert_str_eq!(document.text, "final version");
    }

    #[test]
    fn line_endings_dont_matter() {
        // We'll freely mix Windows and Unix newlines.
        // No \r, because I don't expect old Macs will use this language server.
        let text = "line one\r\nline two\nline three".to_string();
        let mut document = InkDocument::new(text, None);
        document.edit(vec![
            edit((0, 5), (0, 8), "1"),
            edit((1, 5), (1, 8), "2"),
            edit((2, 5), (2, 10), "3"),
        ]);
        assert_str_eq!(document.text, "line 1\r\nline 2\nline 3");
    }

    /// See these articles
    /// * https://fasterthanli.me/articles/the-bottom-emoji-breaks-rust-analyzer#caught-in-the-middle
    /// * https://hsivonen.fi/string-length/
    mod wide_encodings {
        use pretty_assertions::assert_str_eq;
        use tests::edit;

        use super::super::*;

        #[test]
        fn utf8() {
            let text = "🥺🥺".to_string();
            let mut document = InkDocument::new(text, None);
            document.edit(vec![edit((0, 4), (0, 4), " ")]);
            assert_str_eq!(document.text, "🥺 🥺");
        }

        #[test]
        fn utf16() {
            let text = "🥺🥺".to_string();
            let mut document = InkDocument::new(text, Some(WideEncoding::Utf16));
            document.edit(vec![edit((0, 2), (0, 2), " ")]);
            assert_str_eq!(document.text, "🥺 🥺");
        }

        #[test]
        fn utf32() {
            let text = "🥺🥺".to_string();
            let mut document = InkDocument::new(text, Some(WideEncoding::Utf32));
            document.edit(vec![edit((0, 1), (0, 1), " ")]);
            assert_str_eq!(document.text, "🥺 🥺");
        }
    }

    fn edit(from: (u32, u32), to: (u32, u32), text: &str) -> DocumentEdit {
        (
            Some(lsp_types::Range {
                start: lsp_types::Position {
                    line: from.0,
                    character: from.1,
                },
                end: lsp_types::Position {
                    line: to.0,
                    character: to.1,
                },
            }),
            text.to_owned(),
        )
    }
}
