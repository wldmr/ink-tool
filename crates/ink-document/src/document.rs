use derive_more::derive::{Display, Error, From};
use ink_syntax::{self as syntax, Identifier, Ink};
use line_index::{LineCol, LineIndex, WideEncoding, WideLineCol};
use lsp_types::Position;
use tree_traversal::TreeTraversal;
use type_sitter::Node;

/// Encapsulates Parsing/editing an Ink file.
///
/// This is mostly a convenience wrapper to not have to deal with encodings. Other
/// than that it is actually fairly porous: It takes LSP types and exposes
/// tree-sitter/type-sitter types. This is intentional; it is a bridge between the
/// LSP world and tree-sitter world. Full encapsulation would mean not knowing about
/// LSP and not telling about tree-sitter/type-sitter. This would require recreating
/// a lot of the niceties that those libraries bring, and that is just wasted
/// effort.
///
/// It is unlikely that we’ll move away from tree-sitter or LSP, so we won’t bother
/// hiding it.
pub struct InkDocument {
    tree: type_sitter::Tree<Ink<'static>>,
    text: String,
    parser: type_sitter::Parser<Ink<'static>>,
    enc: Option<WideEncoding>,
    lines: line_index::LineIndex,
}

#[derive(Debug, Clone, Display, Error, From)]
#[display("Could not go to node.")]
pub enum GetNodeError {
    #[display("Node type didn't match")]
    InvalidType,
    PositionOutOfBounds(InvalidPosition),
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Error)]
#[display("Not a valid position: {}:{}", _0.line, _0.character)]
pub struct InvalidPosition(#[error(not(source))] pub(crate) Position);

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

/// Our own type capturing partial or full-file edits.
///
/// Mostly for convenience in terms of conversion and some readability.
pub enum DocumentEdit {
    Whole(String),
    Part(lsp_types::Range, String),
}

/// This is the main conversion for the LSP
impl<'a> From<lsp_types::TextDocumentContentChangeEvent> for DocumentEdit {
    fn from(value: lsp_types::TextDocumentContentChangeEvent) -> Self {
        match value.range {
            Some(range) => DocumentEdit::Part(range, value.text),
            None => DocumentEdit::Whole(value.text),
        }
    }
}

/// This is the main conversion for the LSP
impl<'a> From<lsp_types::TextEdit> for DocumentEdit {
    fn from(value: lsp_types::TextEdit) -> Self {
        DocumentEdit::Part(value.range, value.new_text)
    }
}

/// This conversion is for the file watcher, where we always read in the complete file.
impl From<String> for DocumentEdit {
    fn from(value: String) -> Self {
        DocumentEdit::Whole(value)
    }
}

// The following impls are to make testing a little more convenient.

impl<'a> From<&'a str> for DocumentEdit {
    fn from(value: &'a str) -> Self {
        DocumentEdit::Whole(value.into())
    }
}

impl<'a> From<&'a String> for DocumentEdit {
    fn from(value: &'a String) -> Self {
        DocumentEdit::Whole(value.into())
    }
}

impl<'a> From<(lsp_types::Range, &'a str)> for DocumentEdit {
    fn from(value: (lsp_types::Range, &'a str)) -> Self {
        DocumentEdit::Part(value.0, value.1.into())
    }
}

impl From<(lsp_types::Range, String)> for DocumentEdit {
    fn from(value: (lsp_types::Range, String)) -> Self {
        DocumentEdit::Part(value.0, value.1.into())
    }
}

impl<'a> From<(Option<lsp_types::Range>, &'a str)> for DocumentEdit {
    fn from((range, text): (Option<lsp_types::Range>, &'a str)) -> Self {
        match range {
            Some(range) => DocumentEdit::Part(range, text.into()),
            None => DocumentEdit::Whole(text.into()),
        }
    }
}

impl From<(std::ops::Range<(u32, u32)>, &str)> for DocumentEdit {
    fn from((range, text): (std::ops::Range<(u32, u32)>, &str)) -> DocumentEdit {
        DocumentEdit::Part(
            lsp_types::Range {
                start: lsp_types::Position {
                    line: range.start.0,
                    character: range.start.1,
                },
                end: lsp_types::Position {
                    line: range.end.0,
                    character: range.end.1,
                },
            },
            text.to_owned(),
        )
    }
}

/// The identifier under the cursor, plus optionally any *preceding* identifiers.
#[derive(Debug, PartialEq, Eq)]
pub struct IdentUnderCursor<'a> {
    /// Specific identifier that the cursor was at
    pub ident: Identifier<'a>,
    /// Range of `ident`
    pub range: lsp_types::Range,
    /// The “most specific” qualified name that includes the cursor.
    ///
    /// That means for `a.b.c`, if the cursor was on `b`, then `term` == `a.b`.
    ///
    /// Used to disabmbiguate between `this.name` and `that.name`
    pub term: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DefinitionUnderCursor<'a> {
    pub range: lsp_types::Range,
    pub term: &'a str,
}

/// Public API
impl InkDocument {
    pub fn new(text: String, enc: Option<WideEncoding>) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_ink::LANGUAGE.into())
            .expect("setting the language mustn't fail");
        let mut parser = type_sitter::Parser::wrap(parser);
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

    pub fn new_empty(enc: Option<WideEncoding>) -> Self {
        Self::new(String::new(), enc)
    }

    pub fn edit(&mut self, edit: impl Into<DocumentEdit>) {
        let modified_tree = match edit.into() {
            DocumentEdit::Part(range, text) => {
                let edit = self.input_edit(range, &text);
                self.text
                    .replace_range(edit.start_byte..edit.old_end_byte, &text);
                self.tree.edit(&edit);
                Some(&self.tree)
            }
            DocumentEdit::Whole(text) => {
                self.text = text;
                None
            }
        };
        self.tree = self
            .parser
            .parse(&self.text, modified_tree)
            .expect("parsing must work");
        self.lines = LineIndex::new(&self.text);
    }

    pub fn edits<'a, E: Into<DocumentEdit>>(&mut self, edits: impl IntoIterator<Item = E>) {
        edits.into_iter().for_each(|edit| self.edit(edit));
    }

    /// The full text of the file, as an owned string.
    pub fn full_text(&self) -> String {
        // NOTE: We don’t give out slices because there’s a some chance we’ll replace the
        // underlying string with something more editing-efficient (like a Rope or even
        // just a `Vec<String>`), which likely isn't contiguous.
        self.text.to_owned()
    }

    pub fn root(&self) -> syntax::Ink<'_> {
        self.tree.root_node().expect("Root node must be Ink")
    }

    pub fn byte_range(&self, range: lsp_types::Range) -> std::ops::Range<usize> {
        let start = self.to_byte(range.start);
        let end = self.to_byte(range.end);
        start..end
    }

    /// Find the identifier under the cursor at position `pos`
    ///
    /// Regarding the `term` that this usage references: A qualified name can refer to
    /// multiple search terms, namely each level in its hierarchy. So the name
    /// `foo.bar.baz` potentially contains the terms
    ///
    /// - `foo`
    /// - `foo.bar`, and
    /// - `foo.bar.baz`
    ///
    /// “Potentially”, because we only return the name that ends to the right of the
    /// cursor. This is because we assume that the user is being specific when they
    /// place the cursor over the last part of a qualified name.
    ///
    /// Example:
    ///
    /// ``` ink
    /// -> knot.stitch.label
    /// //        ^ cursor here
    /// ```
    ///
    /// would look for “knot.stitch”, but not “knot” or “knot.stitch.label”.
    ///
    /// ``` rust
    /// # use ink_document::*;
    /// # use lsp_types::Position;
    /// let doc = InkDocument::new(String::from("-> knot.stitch.label"), None);
    /// assert_eq!(doc.usage_at(Position::new(0,  3)).map(|it| it.term), Some("knot"));
    /// assert_eq!(doc.usage_at(Position::new(0,  9)).map(|it| it.term), Some("knot.stitch"));
    /// assert_eq!(doc.usage_at(Position::new(0, 20)).map(|it| it.term), Some("knot.stitch.label"));
    /// ```
    ///
    /// The search considers the the cursor to be “on the left” of the character
    /// position, and therefore “touching” the identifier if placed on the character
    /// immediately to the right. In most editors, if the edit cursor at the end of a
    /// word, the user will consider that positon to be “on” the identifier, so we honor
    /// that.
    ///
    /// ``` rust
    /// # use ink_document::*;
    /// # use lsp_types::Position;
    /// let text = "{a.b} etc";
    ///
    /// let doc = InkDocument::new(String::from(text), None);
    ///
    /// // helper to show the text at column `n` and the term found there
    /// let char_and_term = |n: u32| {
    ///     (
    ///         &text[n as usize..(n+1) as usize],
    ///         doc.usage_at(Position::new(0, n)).map(|it| it.term)
    ///     )
    /// };
    ///
    /// assert_eq!(char_and_term(0), ("{", None));
    /// assert_eq!(char_and_term(1), ("a", Some("a")));
    /// assert_eq!(char_and_term(2), (".", Some("a")));
    /// assert_eq!(char_and_term(3), ("b", Some("a.b")));
    /// assert_eq!(char_and_term(4), ("}", Some("a.b")));
    /// assert_eq!(char_and_term(5), (" ", None));
    /// ```
    pub fn usage_at(&self, pos: Position) -> Option<IdentUnderCursor<'_>> {
        let usage = self.thing_under_cursor::<syntax::Usages>(pos)?;
        let usage = usage.parent_of_type::<syntax::Usages>().unwrap_or(usage); // widen to catch qualified names
        self.individual_idents(usage).find(|it| it.range.end >= pos)
    }

    /// Iterate over all usages in this document, including each *individual* part of every qualified name.
    pub fn usages(&self) -> impl Iterator<Item = IdentUnderCursor<'_>> {
        self.root()
            .depth_first::<syntax::Usages>()
            .flat_map(|it| self.individual_idents(it))
    }

    /// The identifier(s) under the cursor (for qualified names, gives each individual identifier)
    #[inline]
    fn individual_idents<'a>(
        &'a self,
        usage: syntax::Usages<'a>,
    ) -> impl Iterator<Item = IdentUnderCursor<'a>> {
        UsageIter::new(self, usage)
    }
}

struct UsageIter<'a> {
    doc: &'a InkDocument,
    usage: syntax::Usages<'a>,
    next_idx: usize,
}

impl<'a> Iterator for UsageIter<'a> {
    type Item = IdentUnderCursor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.usage {
            syntax::Usages::Identifier(ident) if self.next_idx == 0 => {
                self.next_idx += 1; // So that next time we return None.
                let start_byte = ident.start_byte();
                Some(IdentUnderCursor {
                    ident,
                    range: self.doc.lsp_range(ident.range()),
                    term: &self.doc.text[start_byte..ident.end_byte()],
                })
            }
            syntax::Usages::QualifiedName(qualified_name) => {
                let start_byte = qualified_name.start_byte();
                let next_child_indexes = self.next_idx..qualified_name.raw().child_count();
                for idx in next_child_indexes {
                    self.next_idx = idx + 1; // keep track of where we are
                    if let Some(ident) = self.usage.child_of_type::<syntax::Identifier>(idx) {
                        return Some(IdentUnderCursor {
                            ident,
                            range: self.doc.lsp_range(ident.range()),
                            term: &self.doc.text[start_byte..ident.end_byte()],
                        });
                    }
                }
                None
            }
            _ => None,
        }
    }
}

impl<'a> UsageIter<'a> {
    fn new(doc: &'a InkDocument, usage: syntax::Usages<'a>) -> Self {
        Self {
            usage,
            doc,
            next_idx: 0,
        }
    }
}

/// Private Helpers
impl InkDocument {
    /// Translate editor position into an underlying tree node of a given type.f t
    ///
    /// This is its own function because tree-sitter doesn’t consider a cursor at the
    /// “end” of of a node (like an Identifier) as inside that node, while the user
    /// typically would. So we encapsulate that search here.
    #[inline]
    fn thing_under_cursor<'a, N: type_sitter::Node<'a>>(&'a self, pos: Position) -> Option<N> {
        let byte_pos = self.to_byte(pos);
        let root = self.tree.root_node();
        let root = root.raw(); // annoyingly, type-sitter doesn't have any "descendant" methods.

        root.named_descendant_for_byte_range(byte_pos, byte_pos)
            .and_then(|node| N::try_from_raw(node).ok())
            .or_else(|| {
                // If we couldn’t find anything interesting at pos, try one byte to the left. This
                // is to catch the (rather common) cases where the cursor is at the end of a word.
                // For example, a cursor `@` at the end of an eval `{please_compl@}` would not be
                // found to refer to `please_compl` if we didn’t account for this.
                let one_to_the_left = byte_pos.checked_sub(1)?;
                root.named_descendant_for_byte_range(one_to_the_left, one_to_the_left)
                    .and_then(|node| N::try_from_raw(node).ok())
            })
    }

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

    pub fn node_text<'a, N: Node<'a>>(&self, n: N) -> &str {
        &self.text[n.byte_range()]
    }

    pub fn lsp_range(&self, range: tree_sitter::Range) -> lsp_types::Range {
        let start = self.lsp_position(range.start_point);
        let end = self.lsp_position(range.end_point);
        lsp_types::Range { start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::InkDocument;
    use line_index::WideEncoding;
    use pretty_assertions::assert_str_eq;
    use test_case::test_case;

    /// The important thing here is that each edit's coordinates is relative to the previous edit,
    /// not the initial document.
    #[test]
    fn multiple_edits() {
        let text = "hello world\nhow's it hanging?".to_string();
        let mut document = new_doc(text, None);
        document.edits([
            ((0, 0)..(0, 1), "H"),      // Hello world
            ((0, 1)..(0, 5), "i"),      // Hi world
            ((0, 3)..(0, 8), "gang!"),  // Hi gang!
            ((1, 0)..(1, 1), "H"),      // How's it hanging?
            ((1, 9)..(1, 16), "going"), // How's it going?
        ]);
        assert_str_eq!(document.text, "Hi gang!\nHow's it going?");
    }

    #[test]
    fn giving_no_range_means_replace_all_text() {
        let text = "some text".to_string();
        let mut document = new_doc(text, None);
        document.edits([
            "some ignored text\nthis will be completely overwritted\nby the next edit",
            "final version",
        ]);
        assert_str_eq!(document.text, "final version");
    }

    #[test]
    fn line_endings_dont_matter() {
        // We'll freely mix Windows and Unix newlines.
        // No \r, because I don't expect old Macs will use this language server.
        let text = "line one\r\nline two\nline three".to_string();
        let mut document = new_doc(text, None);
        document.edits([
            ((0, 5)..(0, 8), "1"),
            ((1, 5)..(1, 8), "2"),
            ((2, 5)..(2, 10), "3"),
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
        document.edit(((0, code_units)..(0, code_units), " "));
        pretty_assertions::assert_str_eq!(document.text, "🥺 🥺");
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

    mod usages {
        use crate::InkDocument;
        use assert2::check;
        use indoc::indoc;
        use std::collections::HashMap;
        use text_annotations::scan_default_annotations;

        #[test]
        fn label() {
            let text = indoc! {"
                { knot.stitch.label }
                //|  | |    | ^^^^^ label
                //|  | ^^^^^^ stitch
                //^^^^ knot
            "};

            let mut doc = InkDocument::new_empty(None);
            doc.edit(text);
            let locs: HashMap<&str, lsp_types::Range> = scan_default_annotations(text)
                .map(|ann| (ann.text(), ann.text_location.into()))
                .collect();

            let knot_usage = doc.usage_at(locs["knot"].start).unwrap();
            let stitch_usage = doc.usage_at(locs["stitch"].start).unwrap();
            let label_usage = doc.usage_at(locs["label"].start).unwrap();

            check!(knot_usage.term == "knot");
            check!(knot_usage.range == locs["knot"]);

            check!(stitch_usage.term == "knot.stitch");
            check!(stitch_usage.range == locs["stitch"]);

            check!(label_usage.term == "knot.stitch.label");
            check!(label_usage.range == locs["label"]);
        }
    }
}
