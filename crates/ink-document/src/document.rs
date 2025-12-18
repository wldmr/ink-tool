use crate::doc_symbols::DocumentSymbols;
use crate::ids::{self, NodeId, UsageInfo};
use crate::names::{Name, Names};
use crate::traversal::{self, parent};
use crate::types;
use crate::visitor::Visitor;
use crate::ws_symbols::WorkspaceSymbols;
use crate::{doc_symbols, Meta};
use derive_more::derive::{Display, Error, From};
use line_index::{LineCol, LineIndex, WideEncoding, WideLineCol};
use lsp_types::{Position, Uri};
use tap::Pipe as _;
use tree_sitter::Parser;
use type_sitter::{Node, UntypedNode};

// IMPORTANT: This module (and submodules) should be the only place that knows about tree-sitter types.
// Everthing else works in terms of LSP types.

pub struct InkDocument {
    tree: tree_sitter::Tree,
    text: String,
    parser: tree_sitter::Parser,
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

pub type Usages = Vec<(String, lsp_types::Range)>;

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

pub type DocumentEdit<S> = (Option<lsp_types::Range>, S);

pub struct UsageUnderCursor<'a> {
    pub usage: ids::Usage,
    pub range: lsp_types::Range,
    pub terms: Vec<&'a str>,
}

pub struct DefinitionUnderCursor<'a> {
    pub range: lsp_types::Range,
    pub term: &'a str,
}

/// Public API
impl InkDocument {
    pub fn new(text: String, enc: Option<WideEncoding>) -> Self {
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

    pub fn new_empty(enc: Option<WideEncoding>) -> Self {
        Self::new(String::new(), enc)
    }

    pub fn edit<S: AsRef<str> + Into<String>>(&mut self, edits: Vec<DocumentEdit<S>>) {
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

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn root(&self) -> types::Ink<'_> {
        self.tree
            .root_node()
            .pipe(types::Ink::try_from_raw)
            .expect("Root node must be Ink")
    }

    pub fn byte_range(&self, range: lsp_types::Range) -> std::ops::Range<usize> {
        let start = self.to_byte(range.start);
        let end = self.to_byte(range.end);
        start..end
    }

    pub fn definition_at(&self, pos: Position) -> Option<DefinitionUnderCursor<'_>> {
        let node: types::Definitions = self.thing_under_cursor(pos)?;
        let site = match node {
            types::Definitions::External(external) => external.name().upcast(),
            types::Definitions::Global(global) => global.name().upcast(),
            types::Definitions::Knot(knot) => knot.name().upcast(),
            types::Definitions::Label(label) => label.name().upcast(),
            types::Definitions::List(list) => list.name().upcast(),
            types::Definitions::ListValueDef(lvd) => lvd.name().upcast(),
            types::Definitions::Param(param) => match param.value().ok()? {
                types::ParamValue::Divert(divert) => divert.target().upcast(),
                types::ParamValue::Identifier(identifier) => identifier.upcast(),
            },
            types::Definitions::Stitch(stitch) => stitch.name().upcast(),
            types::Definitions::TempDef(temp_def) => temp_def.name().upcast(),
        };

        Some(DefinitionUnderCursor {
            range: self.lsp_range(site.range()),
            term: self.node_text(site),
        })
    }

    pub fn usage_at(&self, pos: Position) -> Option<UsageUnderCursor<'_>> {
        let byte_pos = self.to_byte(pos);
        let node: types::DivertTarget = self.thing_under_cursor(pos)?;

        // The “terms” that this usage references. A qualified name can refer to multiple
        // search terms, namely each level in its hierarchy. So the name `foo.bar.baz`
        // potentially contains the terms
        //
        // - `foo`
        // - `foo.bar`, and
        // - `foo.bar.baz`
        //
        // “Potentially”, because we only return the names that end to the *right* of the
        // cursor. This is because we assume that the user is being specific when they
        // place the cursor over the last part of a qualified name.
        //
        // Example:
        //
        //     knot.stitch.label
        //            ^
        //
        // would look for “knot.stitch” and “knot.stitch.label”, but not “knot”.
        let mut terms: Vec<&str> = Default::default();
        let mut extract_terms_from_qname = |qname: types::QualifiedName| {
            let start = qname.start_byte();
            for ident in qname.identifiers(&mut qname.walk()) {
                let end = ident.end_byte();
                if end >= byte_pos {
                    terms.push(&self.text[start..end]);
                };
            }
        };

        let find_redirect_kind = |node: UntypedNode| {
            parent::<_, types::Redirect>(node)
                .next()
                .map(|it| match it {
                    types::Redirect::Divert(_) => ids::RedirectKind::Divert,
                    types::Redirect::Thread(_) => ids::RedirectKind::Thread,
                    types::Redirect::Tunnel(tunnel) => {
                        if tunnel.target().is_some() {
                            ids::RedirectKind::NamedTunnelReturn
                        } else {
                            ids::RedirectKind::Tunnel
                        }
                    }
                })
        };

        let (usage, range) = match node {
            types::DivertTarget::Call(call) => {
                match call.name().ok() {
                    Some(types::Usages::Identifier(ident)) => terms.push(self.node_text(ident)),
                    Some(types::Usages::QualifiedName(qname)) => extract_terms_from_qname(qname),
                    _ => {}
                };
                (
                    ids::Usage(
                        NodeId::new(call.name()),
                        UsageInfo {
                            redirect: find_redirect_kind(call.upcast()),
                            params: true,
                        },
                    ),
                    call.name().range(),
                )
            }
            types::DivertTarget::Identifier(ident) => {
                terms.push(self.node_text(ident));
                (
                    ids::Usage(
                        NodeId::new(ident),
                        UsageInfo {
                            redirect: find_redirect_kind(ident.upcast()),
                            params: false,
                        },
                    ),
                    ident.range(),
                )
            }
            types::DivertTarget::QualifiedName(qname) => {
                extract_terms_from_qname(qname);
                (
                    ids::Usage(
                        NodeId::new(qname),
                        UsageInfo {
                            redirect: find_redirect_kind(qname.upcast()),
                            params: false,
                        },
                    ),
                    qname.range(),
                )
            }
        };

        Some(UsageUnderCursor {
            usage,
            range: self.lsp_range(range),
            terms,
        })
    }

    pub fn usages(&self) -> Usages {
        let ink = type_sitter::UntypedNode::new(self.tree.root_node());
        traversal::depth_first::<_, types::Usages>(ink)
            .map(|node| match node {
                types::Usages::Identifier(ident) => ident.range(),
                types::Usages::QualifiedName(qname) => qname.range(),
            })
            .map(|range| {
                (
                    self.text[range.start_byte..range.end_byte].to_owned(),
                    self.lsp_range(range),
                )
            })
            .collect::<Usages>()
    }

    pub fn names(&self) -> Vec<(Name, Meta)> {
        Names::new(self).traverse(self.root())
    }

    pub fn doc_symbols(&self) -> Vec<lsp_types::DocumentSymbol> {
        DocumentSymbols::new(self)
            .traverse_with_state(self.root(), doc_symbols::dummy_file_symbol())
            .children
            .unwrap()
    }

    pub fn workspace_symbols(&self, uri: &Uri) -> Vec<lsp_types::WorkspaceSymbol> {
        WorkspaceSymbols::new(&uri, self).traverse(self.root())
    }
}

/// Private Helpers
impl InkDocument {
    /// Translate editor position into an underlying tree node of a given type.
    ///
    /// This is its own function because tree-sitter doesn’t consider a cursor at the
    /// “end” of of a node (like an Identifier) as inside that node, while the user
    /// typically would. So we encapsulate that search here.
    fn thing_under_cursor<'a, N: type_sitter::Node<'a>>(&'a self, pos: Position) -> Option<N> {
        let byte_pos = self.to_byte(pos);
        let root = self.tree.root_node();

        // We’ll try to find the biggest interesting node that surrounds the cursor
        // position. Why the biggest? Because an Identifier is a part of a QualifiedName,
        // and we want the latter if it is there.
        root.named_descendant_for_byte_range(byte_pos, byte_pos)
            .map(UntypedNode::new)
            .and_then(|node| parent::<_, N>(node).last())
            .or_else(|| {
                // If we couldn’t find anything interesting at pos, try one byte to the left. This
                // is to catch the (rather common) cases where the cursor is at the end of a word.
                // For example, a cursor `@` at the end of an eval `{please_compl@}` would not be
                // found to refer to `please_compl` if we didn’t account for this.
                let one_to_the_left = byte_pos.checked_sub(1)?;
                root.named_descendant_for_byte_range(one_to_the_left, one_to_the_left)
                    .map(UntypedNode::new)
                    .and_then(|node| parent::<_, N>(node).last())
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

    use super::{DocumentEdit, InkDocument};
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
