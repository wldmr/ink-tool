use derive_more::derive::{Display, Error};
use lsp_types::{Uri, WorkspaceEdit};
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
        Ok(Some(WorkspaceEdit::new(HashMap::new())))
    }
}

#[cfg(test)]
mod tests {
    use crate::lsp::state::{
        tests::{comment_separated_files, new_state, uri},
        State,
    };
    use assert2::check;
    use indoc::indoc;
    use lsp_types::{Position, Uri};

    fn do_rename(init: &str, (uri, pos, new): (Uri, Position, &str)) -> State {
        let mut state = new_state().with_comment_separated_files(init);

        let edits = state
            .rename_symbol(uri, pos, new)
            .expect("no errors")
            .expect("some workspace edit");

        // TRIPWIRE: Symbol renaming shouldn't result in any complicated file-related operations.
        check!(edits.document_changes == None);
        check!(edits.change_annotations == None);

        let edits = edits.changes.expect("There should be changes");
        for (uri, edits) in edits {
            state.edits(uri, edits);
        }

        state
    }

    fn test_rename_symbol(input: &str, rename: (Uri, Position, &str), expectation: &str) {
        let state = do_rename(input, rename);

        for (uri, text) in comment_separated_files(expectation).unwrap() {
            check!(state.text(uri).unwrap() == text);
        }
    }

    #[test]
    fn knot() {
        let input = indoc! {"
            // file: def.ink
            === knot ===
            = stitch
            - (label) text

            // file: ref.ink
            -> knot.stitch.label
        "};
        let expectation = indoc! {"
            // file: def.ink
            === new ===
            = stitch
            - (label) text

            // file: ref.ink
            -> new.stitch.label
        "};

        test_rename_symbol(
            input,
            (uri("def.ink"), Position::new(0, 4), "new"), // at definition site
            expectation,
        );

        test_rename_symbol(
            input,
            (uri("ref.ink"), Position::new(0, 3), "new"), // at usage site
            expectation,
        );
    }

    #[test]
    fn stitch() {
        let input = indoc! {"
            // file: def.ink
            === knot ===
            = stitch
            - (label) text

            // file: ref.ink
            -> knot.stitch.label
        "};
        let expectation = indoc! {"
            // file: def.ink
            === knot ===
            = new
            - (label) text

            // file: ref.ink
            -> knot.new.label
        "};

        test_rename_symbol(
            input,
            (uri("def.ink"), Position::new(1, 2), "new"), // at definition site
            expectation,
        );

        test_rename_symbol(
            input,
            (uri("ref.ink"), Position::new(0, 3), "new"), // at usage site
            expectation,
        );
    }

    #[test]
    fn label() {
        let input = indoc! {"
            // file: def.ink
            === knot ===
            = stitch
            - (label) text

            // file: ref.ink
            -> knot.stitch.label
        "};
        let expectation = indoc! {"
            // file: def.ink
            === knot ===
            = stitch
            - (new) text

            // file: ref.ink
            -> knot.stitch.new
        "};

        test_rename_symbol(
            input,
            (uri("def.ink"), Position::new(1, 2), "new"), // at definition site
            expectation,
        );

        test_rename_symbol(
            input,
            (uri("ref.ink"), Position::new(0, 3), "new"), // at usage site
            expectation,
        );
    }

    #[test]
    fn param() {
        let input = indoc! {"
            === knot(p1, p2) ===
            {p1 + p2}
            = stitch(p1)
            {p1 + p2}
        "};

        {
            // renaming p1 of knot (respects name shadowing)
            let expectation = indoc! {"
                === knot(new, p2) ===
                {new + p2}
                = stitch(p1)
                {p1 + p2}
            "};
            test_rename_symbol(
                input,
                (uri("main.ink"), Position::new(0, 9), "new"), // at definition site
                expectation,
            );
            test_rename_symbol(
                input,
                (uri("main.ink"), Position::new(1, 1), "new"), // at usage site
                expectation,
            );
        }

        {
            // renaming p1 of stitch (respects name shadowing)
            let expectation = indoc! {"
                === knot(p1, p2) ===
                {p1 + p2}
                = stitch(new)
                {new + p2}
            "};
            test_rename_symbol(
                input,
                (uri("main.ink"), Position::new(2, 2), "new"), // at definition site
                expectation,
            );
            test_rename_symbol(
                input,
                (uri("main.ink"), Position::new(3, 1), "new"), // at usage site
                expectation,
            );
        }

        {
            // renaming shared p2 (no shadowing)
            let expectation = indoc! {"
                === knot(p1, new) ===
                {p1 + new}
                = stitch(p2)
                {p1 + new}
            "};
            test_rename_symbol(
                input,
                (uri("main.ink"), Position::new(0, 13), "new"), // at def site
                expectation,
            );

            test_rename_symbol(
                input,
                (uri("main.ink"), Position::new(1, 6), "new"), // at usage site 1
                expectation,
            );
            test_rename_symbol(
                input,
                (uri("main.ink"), Position::new(3, 6), "new"), // at usage site 2
                expectation,
            );
        }
    }
}
