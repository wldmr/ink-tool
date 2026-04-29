use crate::lsp::{
    location::TextRange,
    salsa::{Name, NodeFlag},
};
use enumflags2::{make_bitflags, BitFlags};
use ink_document::ids::{DefId, ScopeId};
use itertools::Itertools;
use lsp_types::{CompletionItem, Position, Uri};
use std::ops::Bound;
use tree_traversal::TreeTraversal;
use type_sitter::Node;

use super::*;

/// Flags that identify a local variable, not visible outside the defining scope.
static LOCALS: BitFlags<NodeFlag, u32> = make_bitflags!(NodeFlag::{Temp | Param});

impl super::State {
    pub fn completions(
        &self,
        uri: &Uri,
        position: Position,
    ) -> Result<Option<Vec<CompletionItem>>, DocumentNotFound> {
        let (doc, this_doc) = self.get_doc_and_id(uri)?;

        let Some(spec) = self.what_to_search_for(&doc, position) else {
            return Ok(None);
        };

        let flags = self.db.node_flags(this_doc);
        let locals = self.db.local_resolutions(this_doc);

        let mut completions = Vec::new();

        // Walk from tightest to widest scope, collecting relevant names along the way.
        for (n, block) in spec.scopes.iter().rev().copied().enumerate() {
            if let Some(names) = locals.in_scope.get(&ScopeId::from(block)) {
                for (name, def) in names {
                    // Text must match, plus: either we're in the innermost scope or we only see non-locals.
                    if name.as_str().contains(spec.search_text)
                        && (n == 0 || !flags[def].intersects(LOCALS))
                    {
                        completions.push(self.completion(this_doc, *name, *def, &spec));
                    }
                }
            }
        }

        // And finally the globals:
        for story in self.db.stories_of(this_doc).iter() {
            let globals = self.db.global_names(*story);
            for ((doc, def), names) in globals.iter() {
                for name in names {
                    if name.as_str().contains(spec.search_text) {
                        let item = self.completion(*doc, *name, *def, &spec);
                        completions.push(item);
                    }
                }
            }
        }

        Ok(Some(completions))
    }

    fn completion(
        &self,
        docid: DocId,
        text: Name,
        def: DefId,
        spec: &SearchSpec,
    ) -> CompletionItem {
        use lsp_types::{CompletionItemKind, CompletionTextEdit, TextEdit};

        let flags = self.db.node_flags(docid)[def];
        let params = self.find_params(flags, docid, def);

        CompletionItem {
            label: text.to_string(),

            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                range: spec.search_text_range.into(),
                new_text: if let Some(params) = &params {
                    format!("{text}{params}")
                } else {
                    format!("{text}")
                },
            })),

            label_details: Some(lsp_types::CompletionItemLabelDetails {
                detail: params,
                description: Some(self.db.short_path(docid).clone()),
            }),

            kind: salsa::match_flags!(match (flags) {
                NodeFlag::Function => CompletionItemKind::FUNCTION,
                NodeFlag::Knot => CompletionItemKind::CLASS,
                NodeFlag::Stitch => CompletionItemKind::METHOD,
                NodeFlag::Label => CompletionItemKind::FIELD,
                NodeFlag::External => CompletionItemKind::INTERFACE,
                NodeFlag::ListItem => CompletionItemKind::ENUM_MEMBER,
                NodeFlag::List => CompletionItemKind::ENUM,
                NodeFlag::Const => CompletionItemKind::CONSTANT,
                NodeFlag::Var => CompletionItemKind::VARIABLE,
            }),

            ..lsp_types::CompletionItem::default()
        }
    }

    fn find_params(&self, flags: BitFlags<NodeFlag>, docid: DocId, def: DefId) -> Option<String> {
        if !flags.contains(NodeFlag::HasParams) {
            return None;
        }
        let doc = self.db.document(docid);
        let range = self.db.node_locations(docid)[def];
        let start = doc.to_byte(range.start.into());
        let end = doc.to_byte(range.end.into());
        if let Some(mut node) = doc.root().named_descendant_for_byte_range(start, end) {
            while let Some(next) = node.next_named_sibling() {
                if let Ok(param) = next.downcast::<ink_syntax::Params>() {
                    return Some(doc.text(param.start_byte()..param.end_byte()).to_string());
                }
                node = next;
            }
        }
        let path = &*self.db.short_path(docid);
        log::warn!("Couldn't find params for {path}:{range}, although the flags indicate there should be some.");
        None
    }

    fn what_to_search_for<'a>(
        &self,
        doc: &'a InkDocument,
        position: Position,
    ) -> Option<SearchSpec<'a>> {
        // We try to infer as much as possible from the text itself, because tree-sitter is
        // quite unpredictabel when it comes to incomplete/erroneous parses. Tree-sitter’s
        // chief objective is to get through the file *somehow*, and achieves this by
        // liberally adding ERROR or MISSING nodes when it’s stumped.
        //
        // This means that we can’t rely on finding an `identifier`, `call`, `redirect`, or
        // any other kind of node to tell us the context of this invocation. And given that
        // completion requests will *most* likely happen on invalid syntax trees
        // (e.g. `-> knot.stitch.<cursor>`), there’s almost no chance of tree-sitter being
        // of any help.

        // In the following search closure: Have we found any word characters?
        let mut found_word_char = false;

        // Determines whether this character can be part of a name, and thus the search string
        let mut is_qname_char = |(_idx, chr): &(usize, char)| {
            let is_word_char = chr.is_alphabetic() || *chr == '_';
            found_word_char |= is_word_char;
            is_word_char || chr.is_numeric() || *chr == '.'
        };

        let actual_cursor = doc.to_byte(position);

        // Be lenient: We allow completions even when the user has typed spaces (This also
        // catches cases where the cursor is at the end of the line, which is really the
        // reason this exists in the first place.)
        let cursor = doc
            .text(..actual_cursor)
            .char_indices()
            .rev()
            .take_while(|(_, chr)| chr.is_whitespace() && *chr != '\n')
            .map(|(idx, _)| idx)
            .last()
            .unwrap_or(actual_cursor);

        // Determine first and last valid (qualified) name characters around cursor.
        let first_byte = doc
            .text(..cursor)
            .char_indices()
            .rev()
            .take_while(&mut is_qname_char)
            .last()
            .map(|(idx, _)| idx)
            .unwrap_or(cursor);
        let first_bound = Bound::Included(first_byte);

        let last_bound = doc
            .text(cursor..)
            .char_indices()
            .take_while(&mut is_qname_char)
            .last()
            .map(|(idx, _)| Bound::Included(cursor + idx)) // because we started from cursor
            .unwrap_or_else(|| Bound::Excluded(cursor)); // Excluded because the cursor is already included in the start.

        let search_text = doc.text((first_bound, last_bound));

        // What we found might be a number, but we don't complete those:
        if !search_text.is_empty() && !found_word_char {
            return None;
        }

        // The closest named node that contains the text we just selected.
        let node = doc
            .root()
            .named_descendant_for_byte_range(first_byte, first_byte + search_text.len())?;

        // Feels wasteful to walk down the tree twice, but it'll do for now.
        let scopes = doc
            .root()
            .depth_first::<ink_syntax::ScopeBlock>()
            .filter(|it| it.contains(&node))
            .collect_vec();

        // We don’t complete text, so if we can determine that we’re definitely not in
        // code, we can just abort.
        if node.kind() == ink_syntax::Text::KIND && !node.is_error() {
            return None;
        }

        Some(SearchSpec {
            node,
            scopes,
            search_text,
            search_text_range: doc
                .lsp_range_from_bytes(first_byte, first_byte + search_text.len())
                .into(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SearchSpec<'a> {
    node: type_sitter::UntypedNode<'a>,
    scopes: Vec<ink_syntax::ScopeBlock<'a>>,
    search_text_range: TextRange,
    search_text: &'a str,
}

#[cfg(test)]
mod tests {
    use super::super::tests::{new_state, uri};
    use super::*;

    use indoc::indoc;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    mod what_to_search_for {
        use assert2::check;
        use util::testing::setup_logging;

        use crate::lsp::state::tests::text_with_caret;

        use super::*;

        fn do_test(text: &str, expectation: fn(Option<SearchSpec<'_>>)) {
            setup_logging(log::LevelFilter::Trace);
            let mut state = new_state();
            let (text, caret) = text_with_caret(text);
            state.edit(uri("main.ink"), text);
            let (doc, _docid) = state.get_doc_and_id(&uri("main.ink")).unwrap();
            let actual = state.what_to_search_for(&doc, caret);
            expectation(actual);
        }

        macro_rules! search_text {
            ($name:ident, $text:expr, |$actual:ident| $expectation:expr) => {
                #[test]
                fn $name() {
                    do_test($text, |$actual| $expectation);
                }
            };
            ($name:ident, $text:expr, $expectation:literal) => {
                #[test]
                fn $name() {
                    do_test($text, |it| {
                        let text = $text;
                        check!(
                            it.map(|it| it.search_text) == ($expectation).into(),
                            "{text}"
                        );
                    });
                }
            };
            ($name:ident, $text:expr, $expectation:expr) => {
                #[test]
                fn $name() {
                    do_test($text, |it| {
                        let text = $text;
                        check!(it == $expectation, "{text}");
                    });
                }
            };
        }

        search_text!(inside_curlies, "{@}", "");
        search_text!(inside_curlies_one_letter, "{@a}", "a");
        search_text!(inside_open_curlies, "{@", "");
        search_text!(inside_open_curlies_with_prefix, "{bla.@", "bla.");
        search_text!(
            inside_open_curlies_with_prefix_and_curlies,
            "{ func@()",
            "func"
        );

        search_text!(redirect, "-> knot.stit@ch.label", "knot.stitch.label");
        search_text!(redirect_unfinished, "-> knot.stit@ch.", "knot.stitch.");
        search_text!(
            redirect_unfinished_space,
            "-> knot.stitch. @",
            "knot.stitch."
        );
        search_text!(redirect_empty, "->@", "");
        search_text!(redirect_empty_space, "-> @", "");
        search_text!(redirect_empty_spaces, "->   @", "");
        search_text!(redirect_empty_with_parens, "-> @()", "");

        search_text!(inside_normal_text, "Just@ text.", None);
        search_text!(inside_normal_text_space, "Just @ text.", None);
        // These next ones are a bit tricky, so we have slightly more paranoid tests. The
        // reason is that, basically, tree-sitter (or our ink-grammar) behaves a little
        // weird at the end of the line when it comes to finding text nodes.
        search_text!(
            inside_normal_text_eol,
            indoc! {"
            == Knot
            This is just text, nothing to complet@
            "},
            None
        );
        search_text!(
            inside_normal_text_eol_space,
            indoc! {"
            == Knot
            This is just text, nothing to complet  @  
            "},
            None
        );
        search_text!(
            inside_normal_text_sol,
            indoc! {"
            == Knot
            @Neither does it do anything at the start of the line.
            "},
            None
        );

        search_text!(dont_complete_numbers, "{1.2@}", None);
        search_text!(
            dont_complete_funky_numbers_either,
            "His IP address was {1.2.@4.0}",
            None
        );
    }

    #[test]
    fn completion_shows_local_names() {
        let text = indoc! {"
            VAR some_var = true

            This {ab}
            //     ^ suggests knot.label_a knot.stitch.label_b knot.label_b

            == knot(param1, param2)

            - (label_a)

            = stitch(param1, param3)

            - (label_b)
        "};
        let mut state = new_state();
        let file = uri("context.ink");
        state.edit(file, text);

        for ann in text_annotations::scan_default_annotations(text) {
            let mut split = ann.claim().split_whitespace();
            let expected = match split.next() {
                Some("suggests") => split
                    .sorted_unstable()
                    .collect_vec()
                    .tap(|it| assert!(!it.is_empty(), "suggests needs at least one parameter")),
                Some("suggests-nothing") => {
                    assert!(
                        split.collect_vec().is_empty(),
                        "suggests-nothing doesn't take params"
                    );
                    Vec::new()
                }
                _ => continue,
            };
            let actual = state.completions(&uri("context.ink"), ann.text_location.start.into());
            let actual = match actual {
                Ok(Some(completions)) => completions
                    .into_iter()
                    .map(|it| it.label)
                    .sorted_unstable()
                    .collect_vec(),
                other => {
                    eprintln!("Expected completions, got {other:?}");
                    Vec::new()
                }
            };
            assert_eq!(expected, actual)
        }
    }
}
