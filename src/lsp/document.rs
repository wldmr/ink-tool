use crate::ink_syntax::types::{AllNamed, Condition, DivertTarget, Eval, Expr, Redirect};
use crate::lsp::location::{self, specification::LocationThat};
use crate::lsp::salsa::{GetNodeError, Workspace};
use crate::lsp::state::InvalidPosition;
use line_index::{LineCol, LineIndex, WideEncoding, WideLineCol};
use lsp_types::Position;
use milc::Db;
use tap::Pipe as _;
use tree_sitter::Parser;
use type_sitter_lib::Node;

// IMPORTANT: This module (and submodules) should be the only place that knows about tree-sitter types.
// Everthing else works in terms of LSP types.

pub(crate) struct InkDocument {
    pub(crate) tree: tree_sitter::Tree,
    pub(crate) text: String,
    pub(crate) parser: tree_sitter::Parser,
    pub(crate) enc: Option<WideEncoding>,
    pub(crate) lines: line_index::LineIndex,
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

    pub(crate) fn possible_completions(
        &self,
        position: Position,
    ) -> Option<(lsp_types::Range, location::specification::LocationThat)> {
        let offset_at_cursor = self.to_byte(position);
        let root = self.tree.root_node();

        // simply walk to the left (whithin reason) until we hit something the tells us what to complete
        for offset in (0..=offset_at_cursor).rev() {
            // Some special behavior based on the character left of the search position:
            match self.text.get(offset.saturating_sub(1)..offset) {
                Some("\n") => break, // only look until the start of the line, at most
                Some("") => break,   // we're at the beginning of the file; nothing to complete here
                None => continue,    // we're inside a multibyte sequence
                Some(_other) => {
                    // log::debug!("The character left of offset {offset} is `{_other}`");
                }
            };

            let node = root
                .descendant_for_byte_range(offset, offset)
                .expect("the offset must lie within the file, so there must be a node here");
            // log::debug!("found {node} at offset {offset}");

            if node.is_error() || node.is_missing() {
                let text = &self.text[node.byte_range()];
                log::error!("Completion: unhandled `{node}` for `{text}`. Please file a bug.");
                // so that the client also sees it:
                eprintln!("Completion: unhandled `{node}` for `{text}`. Please file a bug.");
            } else if let Ok(target) = DivertTarget::try_from_raw(node) {
                let target = Self::widen_to_full_name(target);
                // Determine the parent. We only complete in very specific cases:
                let parent = target.parent()?;
                let mut result = if Redirect::try_from_raw(*parent.raw()).is_ok() {
                    LocationThat::is_divert_target()
                } else if Expr::try_from_raw(*parent.raw()).is_ok()
                    | Eval::try_from_raw(*parent.raw()).is_ok()
                    | Condition::try_from_raw(*parent.raw()).is_ok()
                {
                    LocationThat::is_named()
                } else {
                    return None;
                };
                if let DivertTarget::Call(call) = target {
                    result &= LocationThat::has_parameters();
                    result &= LocationThat::matches_name(&self.text[call.name().byte_range()]);
                } else {
                    result &= LocationThat::matches_name(&self.text[target.byte_range()]);
                }
                let range = self.lsp_range(&target.range());
                return Some((range, result));
            }
        }
        // For-loop hasn't produced anything, so:
        None
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

    fn to_byte_maybe(&self, pos: lsp_types::Position) -> Option<usize> {
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
        self.lines.offset(pos).map(|it| it.into())
    }

    fn to_byte(&self, pos: lsp_types::Position) -> usize {
        self.to_byte_maybe(pos)
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

    pub fn get_node_at<'a, T>(
        &'a self,
        db: &'a impl Db,
        pos: lsp_types::Position,
    ) -> Result<T, GetNodeError>
    where
        T: type_sitter_lib::Node<'a>,
    {
        let (point, _byte) = self.ts_point(db, pos)?;
        self.tree
            .root_node()
            .named_descendant_for_point_range(point, point)
            .ok_or_else(|| InvalidPosition(pos))?
            .pipe(T::try_from_raw)
            .map_err(|_| GetNodeError::InvalidType)
    }

    pub fn named_cst_node_at(
        &self,
        db: &impl Db,
        pos: lsp_types::Position,
    ) -> Result<AllNamed<'_>, InvalidPosition> {
        let (point, _byte) = self.ts_point(db, pos)?;
        self.tree
            .root_node()
            .named_descendant_for_point_range(point, point)
            .and_then(|node| AllNamed::try_from_raw(node).ok())
            .ok_or_else(|| InvalidPosition(pos))
    }

    pub fn ts_point(
        &self,
        db: &impl Db,
        pos: lsp_types::Position,
    ) -> Result<(tree_sitter::Point, usize), InvalidPosition> {
        let lines = &self.lines;
        let line_col = if let Some(enc) = self.enc(db) {
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

    fn enc(&self, db: &impl Db) -> Option<WideEncoding> {
        *db.get(&Workspace)
    }

    pub fn ts_range(
        &self,
        db: &impl Db,
        range: lsp_types::Range,
    ) -> Result<tree_sitter::Range, InvalidPosition> {
        let (start_point, start_byte) = self.ts_point(db, range.start)?;
        let (end_point, end_byte) = self.ts_point(db, range.end)?;
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
    use super::{DocumentEdit, InkDocument};
    use crate::lsp::location;
    use crate::test_utils::Compact;
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

    // The @ symbol signifies where the cursor is.
    // Random test names for quick re-runs; ugly, but less cumbersome than descriptive or sequential names
    // Many strange corner cases, but the upshot is: We only complete when the cursor is adjacent to identifier characters
    // If we complete, then we narrow it down to divert targets if preceded by a redirect marker.
    // IDEA: We could
    use super::location::specification::LocationThat as Loc;
    #[test_case("@{}", None; "bsj")]
    #[test_case("{@}", None; "lkc")]
    #[test_case("hi{@}", None; "gkf")]
    #[test_case("hi@{}", None; "dpf")]
    #[test_case("{ @}", None; "ixk")]
    #[test_case("{ @ }", None; "ina")]
    #[test_case("{}@", None; "fay")]
    #[test_case("{@x}", Some((range((0, 1), (0, 2)), Loc::is_named() & Loc::matches_name("x"))); "rmo")]
    #[test_case("{x@}", Some((range((0, 1), (0, 2)), Loc::is_named() & Loc::matches_name("x"))); "ivb")]
    #[test_case("{a == @}", None; "pqf")]
    #[test_case("{a + b@}", Some((range((0, 5), (0, 6)), Loc::is_named() & Loc::matches_name("b"))); "hea")]
    #[test_case("{a@ == b}", Some((range((0, 1), (0, 2)), Loc::is_named() & Loc::matches_name("a"))); "qpd")]
    #[test_case("-@>", None; "yug")]
    #[test_case("->@", None; "qgi")]
    #[test_case("-> @", None; "oak")]
    #[test_case("-> @ab", Some((range((0, 3), (0, 5)), Loc::is_divert_target() & Loc::matches_name("ab"))); "pgf")]
    #[test_case("-> ab@", Some((range((0, 3), (0, 5)), Loc::is_divert_target() & Loc::matches_name("ab"))); "uad")]
    #[test_case("-> a@b", Some((range((0, 3), (0, 5)), Loc::is_divert_target() & Loc::matches_name("ab"))); "djg")]
    #[test_case("<- @ab", Some((range((0, 3), (0, 5)), Loc::is_divert_target() & Loc::matches_name("ab"))); "izf")]
    #[test_case("->-> ab@", Some((range((0, 5), (0, 7)), Loc::is_divert_target() & Loc::matches_name("ab"))); "tqz")]
    #[test_case("-> @aa.bb", Some((range((0, 3), (0, 8)), Loc::is_divert_target() & Loc::matches_name("aa.bb"))); "hpp")]
    #[test_case("-> aa@.bb", Some((range((0, 3), (0, 8)), Loc::is_divert_target() & Loc::matches_name("aa.bb"))); "mvz")]
    #[test_case("-> aa.@bb", Some((range((0, 3), (0, 8)), Loc::is_divert_target() & Loc::matches_name("aa.bb"))); "glq")]
    #[test_case("-> aa.b@b", Some((range((0, 3), (0, 8)), Loc::is_divert_target() & Loc::matches_name("aa.bb"))); "uon")]
    #[test_case("-> aa.bb@", Some((range((0, 3), (0, 8)), Loc::is_divert_target() & Loc::matches_name("aa.bb"))); "npt")]
    #[test_case("-> aa.bb()@", Some((range((0, 3), (0, 10)), Loc::is_divert_target() & Loc::matches_name("aa.bb") & Loc::has_parameters())); "sgs")]
    #[test_case("-> aa.b@b(some, param)", Some((range((0, 3), (0, 21)), Loc::is_divert_target() & Loc::matches_name("aa.bb") & Loc::has_parameters())); "xbo")]
    #[test_case("->@\n== knot", None; "iqu")]
    #[test_case("->@\ntext", None; "aho")]
    #[test_case("== text@", None; "no completion in knots")]
    #[test_case("= text@", None; "no completion in stitches")]
    #[test_case("* {text@}", Some((range((0, 3), (0, 7)), Loc::is_named() & Loc::matches_name("text"))); "we do complete in choice conditions")]
    fn completions(txt: &str, expected: Option<(lsp_types::Range, Loc)>) {
        let (doc, caret) = doc_with_caret(txt);
        let actual = doc
            .possible_completions(caret)
            .map(|(r, s)| (Compact(r), location::specification::simplified(s)));
        let expected = expected.map(|(r, s)| (Compact(r), location::specification::simplified(s)));
        pretty_assertions::assert_eq!(actual, expected, "Ink source:\n```\n{txt}\n```");
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
