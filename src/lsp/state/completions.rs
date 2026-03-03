use lsp_types::{CompletionItem, Position, Uri};
use std::ops::Bound;
use tree_traversal::TreeTraversal;
use type_sitter::Node;

use super::*;

impl super::State {
    pub fn completions(
        &self,
        uri: &Uri,
        position: Position,
    ) -> Result<Option<Vec<CompletionItem>>, DocumentNotFound> {
        let (doc, this_doc) = self.get_doc_and_id(uri)?;

        let spec = self.what_to_search_for(&doc, position);

        let Some(usage) = doc.usage_at(position) else {
            return Ok(Default::default());
        };

        let ws_names = self.db.workspace_names();
        use ink_document::ids::DefinitionInfo::*;
        use lsp_types::CompletionItemKind;
        Ok(Some(
            ws_names
                .iter()
                .filter(|(key, _)| key.contains(usage.term))
                .flat_map(|(key, metas)| metas.iter().map(move |meta| (key, meta)))
                .filter(|(_, (docid, meta))| {
                    meta.is_global() || (*docid == this_doc && meta.is_locally_visible_at(position))
                })
                .map(|(key, (_, meta))| CompletionItem {
                    label: key.clone(),
                    label_details: None,
                    kind: Some(match meta.id.info() {
                        Section { .. } => CompletionItemKind::MODULE,
                        Subsection { .. } => CompletionItemKind::CLASS,
                        Function => CompletionItemKind::FUNCTION,
                        External => CompletionItemKind::INTERFACE,
                        Var => CompletionItemKind::VARIABLE, // TODO: Differentiate between VAR and CONST
                        Const => CompletionItemKind::CONSTANT, // TODO: Differentiate between VAR and CONST
                        List => CompletionItemKind::ENUM,
                        ListItem { .. } => CompletionItemKind::ENUM_MEMBER,
                        Label => CompletionItemKind::PROPERTY,
                        Param { .. } => CompletionItemKind::VARIABLE,
                        Temp => CompletionItemKind::UNIT,
                    }),
                    // TODO: Fetch actual definition
                    detail: Some(match meta.id.info() {
                        Section { stitch, params } => {
                            format!(
                                "{} {key}{}",
                                if stitch { "=" } else { "==" },
                                if params { "(…)" } else { "" }
                            )
                        }
                        Subsection { params, .. } => {
                            format!("= {key}{}", if params { "(…)" } else { "" })
                        }
                        Function => format!("== function {key}(…)"),
                        External => format!("EXTERNAL {key}(…)"),
                        Var => format!("VAR {key} = …"),
                        Const => format!("CONST {key} = …"),
                        List => format!("LIST {key} = …"),
                        ListItem { .. } => format!("LIST … = … {key}, "),
                        Label => format!("({key}) // label"),
                        Param { .. } => format!("param // parameter"),
                        Temp => format!("~ temp {key} = …"),
                    }),
                    // TODO: Fetch actual docs
                    documentation: None,
                    deprecated: None,
                    preselect: None,
                    sort_text: None,
                    filter_text: None,
                    insert_text: None,
                    insert_text_format: None,
                    insert_text_mode: None,
                    text_edit: Some(lsp_types::CompletionTextEdit::Edit(lsp_types::TextEdit {
                        range: usage.range,
                        new_text: key.to_owned(),
                    })),
                    additional_text_edits: None,
                    command: None,
                    commit_characters: None,
                    data: None,
                    tags: None,
                })
                .collect(),
        ))
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

        let actual_cursor = doc.to_byte(position);
        // Be lenient: We allow completions even when the user has typed spaces
        let mut found_word_char = false;
        let mut check_qname = |(_idx, chr): &(usize, char)| {
            if chr.is_alphabetic() || *chr == '_' {
                found_word_char = true;
                true
            } else {
                chr.is_numeric() || *chr == '.'
            }
        };
        let cursor = doc
            .text(..actual_cursor)
            .char_indices()
            .rev()
            .take_while(|(_, chr)| chr.is_whitespace() && *chr != '\n')
            .map(|(idx, _)| idx)
            .last()
            .unwrap_or(actual_cursor);
        // Determine first and last valid (Qualified) name characters around cursor.
        let first = doc
            .text(..cursor)
            .char_indices()
            .rev()
            .take_while(&mut check_qname)
            .last()
            .map(|(idx, _)| idx)
            .unwrap_or(cursor);
        let first = Bound::Included(first);

        let last = doc
            .text(cursor..)
            .char_indices()
            .take_while(&mut check_qname)
            .last()
            .map(|(idx, _)| Bound::Included(cursor + idx)) // because we started from cursor
            .unwrap_or_else(|| Bound::Excluded(cursor)); // Excluded because the cursor is already included in the start.

        let search_text = doc.text((first, last));

        // What we found might be a number, but we don't complete those:
        if !search_text.is_empty() && !found_word_char {
            return None;
        }

        let mut node = doc.root().upcast();
        let mut block = None;
        loop {
            log::trace!("{node:?}");
            // We don’t complete text, so if we can determine that we’re definitely not in
            // code, we can just abort.
            if !node.has_error() && node.kind() == ink_syntax::Text::KIND {
                return None;
            }
            if let Ok(it) = node.downcast::<ink_syntax::ScopeBlock>() {
                block = Some(it);
            }
            node = match node.first_child_for_byte(cursor) {
                Some(node) if node.byte_range().contains(&cursor) => node,
                _ => break,
            };
        }
        Some(SearchSpec { block, search_text })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SearchSpec<'a> {
    block: Option<ink_syntax::ScopeBlock<'a>>,
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
            let mut state = new_state();
            let (text, caret) = text_with_caret(text);
            state.edit(uri("main.ink"), text);
            let (doc, _docid) = state.get_doc_and_id(&uri("main.ink")).unwrap();
            let actual = state.what_to_search_for(&doc, caret);
            expectation(actual);
        }

        macro_rules! search_text {
            ($name:ident, $text:literal, |$actual:ident| $expectation:expr) => {
                #[test]
                fn $name() {
                    do_test($text, |$actual| $expectation);
                }
            };
            ($name:ident, $text:literal, $expectation:literal) => {
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
            ($name:ident, $text:literal, $expectation:expr) => {
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
