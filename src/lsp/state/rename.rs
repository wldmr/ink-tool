use derive_more::derive::{Display, Error};
use itertools::Itertools;
use lsp_types::{TextEdit, Uri, WorkspaceEdit};
use std::collections::HashMap;

impl From<RenameError> for lsp_server::ResponseError {
    fn from(value: RenameError) -> Self {
        match value {
            RenameError::LocationError(err) => err.into(),
            RenameError::RenameFailed(message) => Self {
                code: lsp_server::ErrorCode::RequestFailed as i32,
                message,
                data: None,
            },
        }
    }
}

#[derive(Debug, Clone, Display, Error, derive_more::From)]
#[display("Rename Error")]
pub(crate) enum RenameError {
    LocationError(super::GotoLocationError),
    RenameFailed(#[error(not(source))] String),
}

impl super::State {
    pub fn rename_symbol(
        &self,
        uri: Uri,
        pos: lsp_types::Position,
        new_name: impl Into<String>,
    ) -> Result<Option<WorkspaceEdit>, RenameError> {
        let new_name = new_name.into();
        let edits: HashMap<Uri, Vec<lsp_types::TextEdit>> = self
            .goto_references(uri, pos)?
            .into_iter()
            .map(move |it| (it.uri, TextEdit::new(it.range, new_name.clone())))
            .into_group_map();
        let edits = WorkspaceEdit::new(edits);
        Ok(Some(edits))
    }
}

#[cfg(test)]
mod tests {
    fn test_rename(assertion: &str, input: &str, expectation: &str) {
        use crate::lsp::state::tests::{comment_separated_files, new_state};
        use text_annotations::scan_default_annotations;

        // crate::test_utils::setup_logging(log::LevelFilter::Trace);

        let mut state = new_state();
        let mut renames = Vec::new();

        for (uri, text) in comment_separated_files(input).unwrap() {
            for ann in scan_default_annotations(&text) {
                let new_text = match ann.claim().split_once(' ') {
                    Some(("rename-symbol", new_text)) => new_text,
                    _ => continue,
                };
                renames.push((uri.clone(), ann.text_location.start, new_text.to_owned()))
            }
            state.edit(uri, text);
        }

        for (uri, pos, text) in renames {
            let edits = match state.rename_symbol(uri, pos.into(), text) {
                Ok(Some(edits)) => edits.changes.expect("some edits"),
                other => panic!("{other:?}"),
            };
            for (uri, edits) in edits {
                for edit in edits {
                    state.edit(uri.clone(), edit);
                }
            }
        }

        for (uri, text) in comment_separated_files(expectation).unwrap() {
            pretty_assertions::assert_str_eq!(
                state.text(&uri).unwrap(),
                text,
                "{}: {assertion}",
                uri.path().as_str()
            );
        }
    }

    macro_rules! test_rename {
        ($assertion:literal, $ident:ident, $input:expr, $expect:expr) => {
            #[test]
            fn $ident() {
                test_rename($assertion, $input, $expect)
            }
        };

        ($ident:ident, $input:expr, $expect:expr) => {
            test_rename!("", $ident, $input, $expect);
        };
    }

    mod var {
        use super::test_rename;

        test_rename![
            definition,
            "
            VAR var = 1
            //  ^ rename-symbol renamed
            {var}
            ",
            "
            VAR renamed = 1
            //  ^ rename-symbol renamed
            {renamed}
            "
        ];

        test_rename![
            reference,
            "
            VAR var = 1
            {var}
            //^ rename-symbol renamed
            ",
            "
            VAR renamed = 1
            {renamed}
            //^ rename-symbol renamed
            "
        ];
    }

    mod list {
        use super::test_rename;

        test_rename![
            list_definition,
            "
            LIST list = item1, item2
            //          ^ rename-symbol renamed_item1
            //   ^ rename-symbol renamed_list
            {list.item1}
            {list.item2}
            {item1}
            {item2}
            ",
            "
            LIST renamed_list = renamed_item1, item2
            //          ^ rename-symbol renamed_item1
            //   ^ rename-symbol renamed_list
            {renamed_list.renamed_item1}
            {renamed_list.item2}
            {renamed_item1}
            {item2}
            "
        ];

        test_rename![
            list_reference,
            "
            LIST list = item1, item2
            {list.item1}
            //    ^ rename-symbol renamed_item1
            {list.item2}
            // ^ rename-symbol renamed_list
            ",
            "
            LIST renamed_list = renamed_item1, item2
            {renamed_list.renamed_item1}
            //    ^ rename-symbol renamed_item1
            {renamed_list.item2}
            // ^ rename-symbol renamed_list
            "
        ];
    }

    mod knot {
        use super::test_rename;

        test_rename![
            knot_definition,
            "
            // file: def.ink
            === knot ===
            //  ^ rename-symbol new
            = stitch
            - (label) text

            // file: ref.ink
            -> knot.stitch.label
            ",
            "
            // file: def.ink
            === new ===
            //  ^ rename-symbol new
            = stitch
            - (label) text

            // file: ref.ink
            -> new.stitch.label
            "
        ];

        test_rename![
            knot_usage,
            "
            // file: def.ink
            === knot ===
            = stitch
            - (label) text

            // file: ref.ink
            -> knot.stitch.label
            // ^ rename-symbol new
            ",
            "
            // file: def.ink
            === new ===
            = stitch
            - (label) text

            // file: ref.ink
            -> new.stitch.label
            // ^ rename-symbol new
            "
        ];
    }

    mod stitch {
        use super::test_rename;

        test_rename![
            stitch_definition,
            "
            // file: def.ink
            === knot ===
            = stitch
            //^ rename-symbol new
            - (label) text

            // file: ref.ink
            -> knot.stitch.label
            ",
            "
            // file: def.ink
            === knot ===
            = new
            //^ rename-symbol new
            - (label) text

            // file: ref.ink
            -> knot.new.label
            "
        ];

        test_rename![
            stitch_reference,
            "
            // file: def.ink
            === knot ===
            = stitch
            - (label) text

            === other ===
            = stitch
            This is unaffected

            // file: ref.ink
            -> knot.stitch.label
            //      ^ rename-symbol new
            ",
            "
            // file: def.ink
            === knot ===
            = new
            - (label) text

            === other ===
            = stitch
            This is unaffected

            // file: ref.ink
            -> knot.new.label
            //      ^ rename-symbol new
            "
        ];

        test_rename![
            "Renaming a stitch doesn't affect other stitches with the same name",
            stitch_rename_is_specific,
            "
            === knot ===
            = stitch
            //^ rename-symbol renamed
            This is affected

            === other ===
            = stitch
            This is unaffected
            ",
            "
            === knot ===
            = renamed
            //^ rename-symbol renamed
            This is affected

            === other ===
            = stitch
            This is unaffected
            "
        ];
    }

    mod label {
        use super::test_rename;

        test_rename![
            label_definition,
            "
            // file: def.ink
            === knot ===
            = stitch
            - (label) text
            // ^ rename-symbol new

            // file: ref.ink
            -> knot.stitch.label
            ",
            "
            // file: def.ink
            === knot ===
            = stitch
            - (new) text
            // ^ rename-symbol new

            // file: ref.ink
            -> knot.stitch.new
            "
        ];

        test_rename![
            label_reference,
            "
            // file: def.ink
            === knot ===
            = stitch
            - (label) text

            // file: ref.ink
            -> knot.stitch.label
            //             ^ rename-symbol new
            ",
            "
            // file: def.ink
            === knot ===
            = stitch
            - (new) text

            // file: ref.ink
            -> knot.stitch.new
            //             ^ rename-symbol new
            "
        ];
    }

    mod param {
        use super::test_rename;

        test_rename![
            "Renaming does not affect similarly named parameters of subsections",
            param_definition_site1_shadowing,
            "
            === knot(p1, p2) ===
            //       ^ rename-symbol new
            {p1 + p2}
            = stitch(p1)
            //       -- unaffected
            {p1 + p2}
            ",
            "
            === knot(new, p2) ===
            //       ^ rename-symbol new
            {new + p2}
            = stitch(p1)
            //       -- unaffected
            {p1 + p2}
            "
        ];

        test_rename![
            param_usage1,
            "
            === knot(p1, p2) ===
            {p1 + p2}
            //^ rename-symbol new
            = stitch(p1)
            {p1 + p2}
            ",
            "
            === knot(new, p2) ===
            {new + p2}
            //^ rename-symbol new
            = stitch(p1)
            {p1 + p2}
            "
        ];

        test_rename![
            "Renaming does affect all usages (that aren't shadowed) (definition site)",
            param_definition_site2_shadowing,
            "
            === knot(p1, p2) ===
            //           ^ rename-symbol new
            {p1 + p2}
            = stitch(p1)
            {p1 + p2}
            ",
            "
            === knot(p1, new) ===
            //           ^ rename-symbol new
            {p1 + new}
            = stitch(p1)
            {p1 + new}
            "
        ];

        test_rename![
            param_usage2,
            "
            === knot(p1, p2) ===
            {p1 + p2}
            //    ^ rename-symbol new
            = stitch(p1)
            {p1 + p2}
            ",
            "
            === knot(p1, new) ===
            {p1 + new}
            //    ^ rename-symbol new
            = stitch(p1)
            {p1 + new}
            "
        ];
    }
}
