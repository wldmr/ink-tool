use crate::ink_syntax::types::{
    AllNamed, DivertTarget, Identifier, Ink, Knot, KnotBlock, ScopeBlock, StitchBlock, Usages,
};
use crate::ink_syntax::{self, traversal};
use crate::lsp::salsa::GetNodeError;
use crate::lsp::state::InvalidPosition;
use line_index::{LineCol, LineIndex, WideEncoding, WideLineCol};
use lsp_types::Position;
use tap::Pipe as _;
use tree_sitter::Parser;
use type_sitter::{Node, UntypedNamedNode, UntypedNode};

// IMPORTANT: This module (and submodules) should be the only place that knows about tree-sitter types.
// Everthing else works in terms of LSP types.

pub(crate) struct InkDocument {
    pub(crate) tree: tree_sitter::Tree,
    pub(crate) text: String,
    pub(crate) parser: tree_sitter::Parser,
    pub(crate) enc: Option<WideEncoding>,
    pub(crate) lines: line_index::LineIndex,
}

impl Default for InkDocument {
    fn default() -> Self {
        // TODO: This will silently lead to errors. We should panic instead.
        InkDocument::new_empty(None) // NOTE: Workaround for `Default` requirement. Must set actual encoding before editing!
    }
}

impl std::fmt::Debug for InkDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InkDocument")
            .field("tree", &self.tree)
            .field("text", &format!("[{} bytes]", self.text.len()))
            .field("enc", &self.enc)
            .finish_non_exhaustive()
    }
}

impl PartialEq for InkDocument {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl std::hash::Hash for InkDocument {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text.hash(state);
    }
}

pub(crate) type DocumentEdit<S> = (Option<lsp_types::Range>, S);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PointOfInterest {
    bytes: (usize, usize),
}

pub type DefinionsSearch<'a> = Vec<&'a str>;

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
        }
    }

    pub(crate) fn new_empty(enc: Option<WideEncoding>) -> Self {
        Self::new(String::new(), enc)
    }

    pub(crate) fn edit<S: AsRef<str> + Into<String>>(&mut self, edits: Vec<DocumentEdit<S>>) {
        // log::trace!("applying {} edits", edits.len());
        for (range, new_text) in edits.into_iter() {
            let edit = range.map(|range| self.input_edit(range, new_text.as_ref()));
            let modified_tree = if let Some(edit) = edit {
                self.text
                    .replace_range(edit.start_byte..edit.old_end_byte, new_text.as_ref());
                self.tree.edit(&edit);
                Some(&self.tree)
            } else {
                self.text = new_text.into();
                None
            };
            self.tree = self
                .parser
                .parse(&self.text, modified_tree)
                .expect("parsing must work");
            self.lines = LineIndex::new(&self.text);
        }
    }

    pub fn usage_at(&self, pos: Position) -> Option<DefinionsSearch<'_>> {
        let byte_pos = self.to_byte(pos);
        let node = self
            .tree
            .root_node()
            .named_descendant_for_byte_range(byte_pos, byte_pos)
            .map(UntypedNamedNode::try_from_raw)?;
        let node = traversal::parent::<_, Usages>(node).last()?;
        let mut search = DefinionsSearch::default();
        match node {
            Usages::Identifier(ident) => {
                search.push(self.text_of(ident));
            }
            Usages::QualifiedName(qname) => {
                // Look for the "substrings" that end right of the cursor.
                //
                //     knot.stitch.label
                //            ^
                //
                // would look for "knot.stitch" and "knot.stitch.label",
                // but not "knot".
                let start = qname.start_byte();
                for ident in qname.identifiers(&mut qname.walk()) {
                    let end = ident.end_byte();
                    if end >= byte_pos {
                        search.push(&self.text[start..end]);
                    }
                }
            }
        };

        Some(search)
    }
}

/// Private Helpers
impl InkDocument {
    fn input_edit(&self, range: lsp_types::Range, new_text: &str) -> tree_sitter::InputEdit {
        let start_byte = self.to_byte(range.start);
        let old_end_byte = self.to_byte(range.end);
        let new_end_byte = start_byte + new_text.len();

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
            .map(|it| it.into())
            .expect("LineCol must correspond to an offset")
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

    fn text_of<'a, N: Node<'a>>(&self, n: N) -> &str {
        &self.text[n.byte_range()]
    }

    pub(super) fn lsp_range(&self, node: &tree_sitter::Range) -> lsp_types::Range {
        let start = self.lsp_position(node.start_point);
        let end = self.lsp_position(node.end_point);
        lsp_types::Range { start, end }
    }

    /// Walk up the parents of this node; return the largest node that can be a (potentially qualified) name of something.
    fn widen_to_full_name<'t>(this: DivertTarget<'t>) -> DivertTarget<'t> {
        let maybe_parent = this
            .parent()
            .map(|it| DivertTarget::try_from_raw(*it.raw()));
        if let Some(Ok(parent)) = maybe_parent {
            Self::widen_to_full_name(parent)
        } else {
            this
        }
    }

    pub fn get_node_at<'a, T>(&'a self, pos: lsp_types::Position) -> Result<T, GetNodeError>
    where
        T: type_sitter::Node<'a>,
    {
        let (point, _byte) = self.ts_point(pos)?;
        self.tree
            .root_node()
            .named_descendant_for_point_range(point, point)
            .ok_or_else(|| InvalidPosition(pos))?
            .pipe(T::try_from_raw)
            .map_err(|_| GetNodeError::InvalidType)
    }

    pub fn named_cst_node_at(
        &self,
        pos: lsp_types::Position,
    ) -> Result<AllNamed<'_>, InvalidPosition> {
        let (point, _byte) = self.ts_point(pos)?;
        self.tree
            .root_node()
            .named_descendant_for_point_range(point, point)
            .and_then(|node| AllNamed::try_from_raw(node).ok())
            .ok_or_else(|| InvalidPosition(pos))
    }

    pub fn ts_point(
        &self,
        pos: lsp_types::Position,
    ) -> Result<(tree_sitter::Point, usize), InvalidPosition> {
        let lines = &self.lines;
        let line_col = if let Some(enc) = self.enc {
            let wide = line_index::WideLineCol {
                line: pos.line,
                col: pos.character,
            };
            lines
                .to_utf8(enc, wide)
                .ok_or_else(|| InvalidPosition(pos))?
        } else {
            line_index::LineCol {
                line: pos.line,
                col: pos.character,
            }
        };
        let point = tree_sitter::Point::new(pos.line as usize, pos.character as usize);
        let byte = lines
            .offset(line_col)
            .ok_or_else(|| InvalidPosition(pos))?
            .into();
        Ok((point, byte))
    }

    pub fn ts_range(&self, range: lsp_types::Range) -> Result<tree_sitter::Range, InvalidPosition> {
        let (start_point, start_byte) = self.ts_point(range.start)?;
        let (end_point, end_byte) = self.ts_point(range.end)?;
        Ok(tree_sitter::Range {
            start_byte,
            end_byte,
            start_point,
            end_point,
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::lsp::salsa;

    use super::{DocumentEdit, InkDocument};
    use indoc::indoc;
    use line_index::WideEncoding;
    use pretty_assertions::assert_str_eq;
    use test_case::test_case;

    /// The important thing here is that each edit's coordinates is relative to the previous edit,
    /// not the initial document.
    #[test]
    fn multiple_edits() {
        let text = "hello world\nhow's it hanging?".to_string();
        let mut document = new_doc(text, None);
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
        let mut document = new_doc(text, None);
        document.edit(vec![
            (
                None,
                "some ignored text\nthis will be completely overwritted\nby the next edit",
            ),
            (None, "final version"),
        ]);
        assert_str_eq!(document.text, "final version");
    }

    #[test]
    fn line_endings_dont_matter() {
        // We'll freely mix Windows and Unix newlines.
        // No \r, because I don't expect old Macs will use this language server.
        let text = "line one\r\nline two\nline three".to_string();
        let mut document = new_doc(text, None);
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
    #[test_case(None,                      4; "Width of emoji in UTF-8")]
    #[test_case(Some(WideEncoding::Utf16), 2; "Width of emoji in UTF-16")]
    #[test_case(Some(WideEncoding::Utf32), 1; "Width of emoji in UTF-32")]
    fn wide_encodings(enc: Option<WideEncoding>, code_units: u32) {
        let text = "🥺🥺".to_string();
        let mut document = new_doc(text, enc);
        document.edit(vec![edit((0, code_units), (0, code_units), " ")]);
        pretty_assertions::assert_str_eq!(document.text, "🥺 🥺");
    }

    fn range(from: (u32, u32), to: (u32, u32)) -> lsp_types::Range {
        lsp_types::Range {
            start: lsp_types::Position {
                line: from.0,
                character: from.1,
            },
            end: lsp_types::Position {
                line: to.0,
                character: to.1,
            },
        }
    }

    fn edit(from: (u32, u32), to: (u32, u32), text: &str) -> DocumentEdit<&str> {
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
            text,
        )
    }

    /// Creates a UTF-8 encoded document and an LSP `Position` based on where the first `@` symbol is.
    /// Panics if there is no `@` symbol.
    fn doc_with_caret(input: &str) -> (InkDocument, lsp_types::Position) {
        let mut row = 0;
        let mut col = 0;
        // Generating positons this way only works for UTF-8!
        // For other encodings we'd need to look at InkDocument internals, which we don't want.
        for (idx, chr) in input.char_indices() {
            match chr {
                '@' => {
                    let pos = lsp_types::Position::new(row, col);
                    let mut output = input.to_string();
                    output.remove(idx);
                    return (new_doc(output, None), pos);
                }
                '\n' => {
                    row += 1;
                    col = 0;
                }
                _ => {
                    col += 1;
                }
            }
        }
        panic!("There should have been an '@' in there somewhere.");
    }

    fn new_doc(text: impl Into<String>, enc: Option<WideEncoding>) -> InkDocument {
        InkDocument::new(text.into(), enc)
    }
}
