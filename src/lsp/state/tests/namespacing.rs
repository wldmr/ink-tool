mod labels {
    use crate::lsp::state::tests::{new_state, uri};
    use assert2::check;
    use itertools::Itertools;
    use std::collections::HashMap;
    use text_annotations::{scan_default_annotations, TextRegion};
    use util::testing::setup_logging;

    const TEXT: &str = indoc::indoc! {r"
                -> knot.stitch.label
                //             ^^^^^ location label-ref1
                //      ^^^^^^ location stitch-ref1
                // ^^^^ location knot-ref1

                -> knot.label
                //      ^^^^^ location label-ref2
                // ^^^^ location knot-ref2

                == knot
                // ^^^^ location knot
                =  stitch
                // ^^^^^^ location stitch
                - (label)
                // ^^^^^ location label
            "};

    fn named_ranges<'a, T: From<TextRegion> + std::fmt::Debug>(
        text: &'a str,
    ) -> HashMap<&'a str, T> {
        scan_default_annotations(text)
            .filter_map(|ann| match ann.claim().split_once(' ') {
                Some(("location", name)) => Some((name.trim(), ann.text_location.into())),
                _ => None,
            })
            .into_grouping_map()
            .reduce(|first, key, second| {
                panic!("Found multiple occurrences of `{key}`: {first:?}, {second:?}")
            })
    }

    #[test]
    fn goto_definition_label() {
        setup_logging(log::LevelFilter::Debug);
        let state = new_state().with_comment_separated_files(TEXT);

        let loc = named_ranges::<lsp_types::Range>(TEXT);

        let defs = state
            .goto_references(uri("main.ink"), loc["label-ref1"].start)
            .unwrap()
            .into_iter()
            .map(|it| it.range)
            .sorted_unstable_by(|a, b| a.start.cmp(&b.start).then_with(|| a.end.cmp(&b.end)))
            .collect_vec();

        check!(defs == vec![loc["label-ref1"], loc["label-ref2"], loc["label"]]);
        // check!(defs[0].range == loc["label-ref1"]);
        // check!(defs[1].range == loc["label-ref2"]);
        // check!(defs[2].range == loc["label"]);
    }
}
