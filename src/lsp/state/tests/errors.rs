use crate::lsp::{
    salsa::InkGetters,
    state::{tests::new_state, State},
};
use annotate_snippets::{AnnotationKind, Group, Level, Snippet};
use itertools::Itertools;
use lsp_types::Uri;
use std::str::FromStr;
use text_annotations::scan_default_annotations;

fn test_errors(state: &State) {
    let doc_ids = state.db.doc_ids();

    let renderer = annotate_snippets::Renderer::styled()
        .decor_style(annotate_snippets::renderer::DecorStyle::Unicode);
    let mut output = String::new();
    let mut render = |msg: &[Group<'_>]| output.push_str(&format!("{}\n\n", renderer.render(msg)));

    for id in doc_ids.iter().copied() {
        let doc = state.db.document(id);
        let text = doc.text(..);
        let path = id.path();

        'annotations: for ann in scan_default_annotations(doc.text(..)) {
            let mut claim = ann.claim().split_whitespace();

            let Some(keyword) = claim.next() else {
                continue;
            };

            let expected = ann.claim()[keyword.len()..].trim();
            let expected_lsp_range: lsp_types::Range = ann.text_location.into();
            let expected_pos_range = expected_lsp_range.start..=expected_lsp_range.end;

            let expect_diagostic = match keyword.to_lowercase().as_str() {
                "diagnostic" | "diagnostics" => true,
                "no-diagnostic" | "no-diagnostics" => false,
                _ => continue,
            };

            let file_diagnostics = state.db.file_diagnostics(id);
            let actual = file_diagnostics
                .iter()
                .filter(|it| {
                    let range = it.range.start..=it.range.end;
                    range.contains(expected_pos_range.start())
                        || range.contains(expected_pos_range.end())
                        || expected_pos_range.contains(range.start())
                        || expected_pos_range.contains(range.end())
                })
                .collect_vec();

            // Saying `diagnostic` without a message means "there should be something here, but I don't care about the message".
            if expect_diagostic && expected.is_empty() {
                if actual.is_empty() {
                    render(
                        &[Level::ERROR.primary_title("No diagnostics found").element(
                            Snippet::source(text).path(id.path()).fold(true).annotation(
                                AnnotationKind::Primary
                                    .span(ann.text_location.byte_range())
                                    .label(
                                        "Expected to find *some* diagnostic here, but none found.",
                                    ),
                            ),
                        )],
                    );
                }
            } else if expect_diagostic && !expected.is_empty() {
                let mut groups = vec![Level::ERROR
                    .primary_title(format!("Wrong diagnostic"))
                    .element(
                        Snippet::source(text).path(path).fold(true).annotation(
                            AnnotationKind::Primary
                                .span(ann.text_location.byte_range())
                                .label("this range"),
                        ),
                    )
                    .element(
                        Level::NOTE
                            .with_name("expected diagnostic")
                            .message(expected),
                    )];
                for diag in actual {
                    let thing = match (
                        diag.message.contains(expected),
                        diag.range == expected_lsp_range,
                    ) {
                        (true, true) => continue 'annotations,
                        (true, false) => Level::WARNING
                            .secondary_title("Found correct message, but different text span")
                            .element(Snippet::source(text).path(path).annotation(
                                AnnotationKind::Primary.span(doc.byte_range(diag.range)),
                            )),
                        (false, _) => Level::INFO
                            .secondary_title("Found other diagnostic")
                            .element(
                                Snippet::source(text).path(path).annotation(
                                    AnnotationKind::Primary
                                        .span(doc.byte_range(diag.range))
                                        .label(&diag.message),
                                ),
                            ),
                    };

                    groups.push(thing);
                }

                if groups.len() > 1 {
                    render(&groups);
                }
            } else if !expect_diagostic && expected.is_empty() {
                if !actual.is_empty() {
                    let mut group = vec![Level::ERROR
                        .primary_title("No diagnostics expected")
                        .element(
                            Snippet::source(text).path(id.path()).fold(true).annotation(
                                AnnotationKind::Primary
                                    .span(ann.text_location.byte_range())
                                    .label("Expected to find *no* diagnostic here, but …"),
                            ),
                        )];
                    for diag in actual {
                        group.push(
                            Level::WARNING.secondary_title("found diagnostic").element(
                                Snippet::source(text).path(path).annotation(
                                    AnnotationKind::Primary
                                        .span(doc.byte_range(diag.range))
                                        .label(&diag.message),
                                ),
                            ),
                        );
                    }
                    render(&group);
                }
            } else if !expect_diagostic && !expected.is_empty() {
                // we expect no diagnostic to contain that message
                let mut group = vec![Level::ERROR
                    .primary_title("Unexpected diagnostic found")
                    .element(
                        Snippet::source(text).path(id.path()).fold(true).annotation(
                            AnnotationKind::Primary
                                .span(ann.text_location.byte_range())
                                .label("Expected to find *no* diagnostic that match"),
                        ),
                    )
                    .element(
                        Level::NOTE
                            .with_name("unexpected diagnostic")
                            .message(expected),
                    )];

                let matches = actual
                    .into_iter()
                    .filter(|diag| diag.message.contains(expected))
                    .map(|diag| {
                        Level::WARNING
                            .secondary_title("but found diagnostic")
                            .element(
                                Snippet::source(text).path(path).annotation(
                                    AnnotationKind::Primary
                                        .span(doc.byte_range(diag.range))
                                        .label(&diag.message),
                                ),
                            )
                    });
                group.extend(matches);

                if group.len() > 1 {
                    render(&group)
                }
            }
        }
    }

    let output = output.trim();
    if !output.is_empty() {
        panic!("\n{output}\n");
    }
}

#[test]
fn errors() {
    let mut state = new_state();

    let ink_files = walkdir::WalkDir::new("examples/")
        .into_iter()
        .map(|file| file.expect("We don't tolerate errors in tests"))
        .filter(|file| file.path().extension().is_some_and(|it| it == "ink"));

    for ink in ink_files {
        let path = ink.path().as_os_str().to_string_lossy();
        let uri = Uri::from_str(&path).unwrap();
        let contents = std::fs::read_to_string(&*path).unwrap();
        state.edit(uri, contents);
    }

    test_errors(&state);
}
