mod symbols;

use line_index::{LineCol, LineIndex, WideEncoding, WideLineCol};
use lsp_types::{DocumentSymbol, WorkspaceSymbol};
use symbols::{document_symbol::DocumentSymbols, workspace_symbol::WorkspaceSymbols};
use tree_sitter::Parser;

use crate::ink_syntax::Visitor as _;

// IMPORTANT: This module (and submodules) should be the only place that knows about tree-sitter types.
// Everthing else works in terms of LSP types.

pub(crate) struct InkDocument {
    tree: tree_sitter::Tree,
    text: String,
    parser: tree_sitter::Parser,
    enc: Option<WideEncoding>,
    lines: line_index::LineIndex,
    doc_symbols_cache: Option<DocumentSymbol>,
    ws_symbols_cache: Option<Vec<WorkspaceSymbol>>,
}

pub(crate) type DocumentEdit = (Option<lsp_types::Range>, String);

/// Public API
impl InkDocument {
    pub(crate) fn new(text: String, enc: Option<WideEncoding>) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_ink::LANGUAGE.into())
            .expect("setting the language mustn't fail");
        let tree = parser
            .parse(&text, None)
            .expect("can only return None with timeout, cancellation flag or missing language");
        let lines = LineIndex::new(&text);
        Self {
            parser,
            tree,
            lines,
            enc,
            text,
            doc_symbols_cache: None,
            ws_symbols_cache: None,
        }
    }

    pub(crate) fn edit(&mut self, edits: Vec<DocumentEdit>) {
        // eprintln!("applying {} edits", edits.len());
        for (range, new_text) in edits.into_iter() {
            let edit = range.map(|range| self.input_edit(range, &new_text));
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
        self.doc_symbols_cache = None;
        self.ws_symbols_cache = None;
        // eprintln!("document now is {}", self.text);
    }

    pub(crate) fn symbols(&mut self, qualified_symbol_names: bool) -> Option<DocumentSymbol> {
        if self.doc_symbols_cache.is_none() {
            let new = DocumentSymbols::new(&self, qualified_symbol_names)
                .traverse(&mut self.tree.walk())
                .and_then(|it| it.sym);
            self.doc_symbols_cache = new;
        }
        self.doc_symbols_cache.clone()
    }

    pub(crate) fn workspace_symbols(
        &mut self,
        uri: &lsp_types::Uri,
        qualified_symbol_names: bool,
    ) -> Option<Vec<WorkspaceSymbol>> {
        if self.ws_symbols_cache.is_none() {
            let mut symbls = WorkspaceSymbols::new(self, uri, qualified_symbol_names);
            symbls.traverse(&mut self.tree.walk());
            self.ws_symbols_cache = Some(symbls.sym);
        }
        self.ws_symbols_cache.clone()
    }
}

/// Private Helpers
impl InkDocument {
    fn input_edit(&self, range: lsp_types::Range, new_text: &str) -> tree_sitter::InputEdit {
        let start_byte = self.to_byte(range.start);
        let old_end_byte = self.to_byte(range.end);
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

    fn to_byte(&self, pos: lsp_types::Position) -> usize {
        let lsp_types::Position {
            line,
            character: col,
        } = pos;
        let pos = if let Some(enc) = self.enc {
            self.lines
                .to_utf8(enc, WideLineCol { line, col })
                .expect("Conversion from wide to UTF-8 mustn't fail")
        } else {
            LineCol { line, col }
        };
        self.lines
            .offset(pos)
            .expect("LineCol must correspond to an offset")
            .into()
    }

    fn lsp_position(&self, point: tree_sitter::Point) -> lsp_types::Position {
        let native = LineCol {
            line: point.row as u32,
            col: point.column as u32,
        };

        if let Some(enc) = self.enc {
            let wide = self.lines.to_wide(enc, native).unwrap();
            lsp_types::Position {
                line: wide.line,
                character: wide.col,
            }
        } else {
            lsp_types::Position {
                line: native.line,
                character: native.col,
            }
        }
    }

    fn lsp_range(&self, node: &tree_sitter::Range) -> lsp_types::Range {
        let start = self.lsp_position(node.start_point);
        let end = self.lsp_position(node.end_point);
        lsp_types::Range { start, end }
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
