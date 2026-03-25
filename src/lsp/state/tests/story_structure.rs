use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use assert2::check;
use indoc::indoc;

use crate::lsp::{
    salsa::{DocId, InkGetters},
    state::{tests::new_state, State},
};

/// Some helpers to make the tests more readable.
impl State {
    fn path(&self, id: impl Into<DocId>) -> String {
        let ids = self.db.doc_ids();
        let uri = ids.get(id.into()).expect("You gave me a DocId!");
        let prefix = self.db.common_path_prefix().len();
        uri.path().as_str()[prefix..].to_string()
    }

    fn story_structure(&self) -> BTreeMap<String, BTreeSet<Result<String, String>>> {
        let mut structure = BTreeMap::new();
        for story in self.db.story_roots().iter().copied() {
            let story_path = self.db.short_path(story.into());
            let imports = self.db.transitive_imports(story);

            let resolved = imports.resolved.keys().map(|it| Ok(self.path(*it)));

            let unresolved = imports
                .unresolved
                .iter()
                .flat_map(|(file, ranges)| ranges.iter().copied().map(|it| (*file, it)))
                .map(|(file, range)| {
                    let doc = self.db.document(file);
                    let text = doc.lsp_text(range);
                    let docpath = self.db.short_path(file);
                    let path = Path::new(story_path.as_str()).parent().unwrap().join(text);
                    Err(format!("{}:{}", docpath.as_str(), path.to_string_lossy()))
                });

            structure.insert(self.path(story), resolved.chain(unresolved).collect());
        }
        structure
    }
}

fn structure<'a>(
    them: impl IntoIterator<Item = (&'a str, Vec<Result<&'a str, &'a str>>)>,
) -> BTreeMap<String, BTreeSet<Result<String, String>>> {
    let mut structure = BTreeMap::new();
    for (story, imports) in them {
        structure.insert(
            story.to_string(),
            imports
                .into_iter()
                .map(|it| match it {
                    Ok(ok) => Ok(ok.to_string()),
                    Err(err) => Err(err.to_string()),
                })
                .collect(),
        );
    }
    structure
}

#[test]
fn single_root_all_files_exist() {
    let state = new_state().with_comment_separated_files(indoc! {"
        // file: main.ink
        INCLUDE chapter1.ink

        // file: chapter1.ink
        INCLUDE chapter1/beginning.ink
        INCLUDE chapter1/middle.ink
        INCLUDE chapter1/end.ink

        // file: chapter1/beginning.ink
        It was a dark and stormy night.

        // file: chapter1/middle.ink
        Detective Sam Malone couldn't see a thing.

        // file: chapter1/end.ink
        “It's too dark and stormy tonight,” he said, and went home.
    "});

    check!(
        state.story_structure()
            == structure([(
                "main.ink",
                vec![
                    Ok("main.ink"),
                    Ok("chapter1.ink"),
                    Ok("chapter1/beginning.ink"),
                    Ok("chapter1/middle.ink"),
                    Ok("chapter1/end.ink"),
                ]
            )])
    );
}

#[test]
fn multiple_roots_all_files_exist() {
    let state = new_state().with_comment_separated_files(indoc! {"
        // file: main.ink
        INCLUDE main/beginning.ink
        INCLUDE main/middle.ink
        INCLUDE main/end.ink
        INCLUDE common/stats.ink


        // file: side/story.ink
        INCLUDE beginning.ink
        INCLUDE middle.ink
        INCLUDE end.ink
        //      ^ Note how the paths are relative to /side/story.ink, not the root directory


        // file: demo.ink
        INCLUDE main/beginning.ink
        INCLUDE side/beginning.ink
        INCLUDE common/stats.ink

        // file: main/beginning.ink
        I wanted a tomato.
        // file: main/middle.ink
        I ate a tomato.
        // file: main/end.ink
        It was good.

        // file: side/beginning.ink
        I was out of tomatoes.
        // file: side/middle.ink
        Luckily the shop was open.
        // file: side/end.ink
        So I went and bought tomatoes.

        // file: common/stats.ink
        VAR tomato_count = 0
    "});

    check!(
        state.story_structure()
            == structure([
                (
                    "main.ink",
                    vec![
                        Ok("main.ink"),
                        Ok("main/beginning.ink"),
                        Ok("main/middle.ink"),
                        Ok("main/end.ink"),
                        Ok("common/stats.ink"),
                    ]
                ),
                (
                    "side/story.ink",
                    vec![
                        Ok("side/story.ink"),
                        Ok("side/beginning.ink"),
                        Ok("side/middle.ink"),
                        Ok("side/end.ink"),
                    ]
                ),
                (
                    "demo.ink",
                    vec![
                        Ok("demo.ink"),
                        Ok("main/beginning.ink"),
                        Ok("side/beginning.ink"),
                        Ok("common/stats.ink"),
                    ]
                ),
            ])
    );
}

#[test]
fn unresolved_imports() {
    let state = new_state().with_comment_separated_files(indoc! {"
        // file: lib/include.ink
        INCLUDE content.ink

        This file can only find `content.ink` for story roots
        that are a sibling files of this very file.

        // file: lib/content.ink
        The actual library content.

        // file: toplevel.ink
        INCLUDE lib/include.ink
        This story will cause an include error.

        // file: lib/test.ink
        INCLUDE include.ink
        This story root will be fine.
    "});

    check!(
        state.story_structure()
            == structure([
                (
                    "toplevel.ink",
                    vec![
                        Ok("toplevel.ink"),
                        Ok("lib/include.ink"),
                        Err("lib/include.ink:content.ink"), // a:b -> file a tried to import path b relative to the root file.
                    ]
                ),
                (
                    "lib/test.ink",
                    vec![
                        Ok("lib/test.ink"),
                        Ok("lib/include.ink"),
                        Ok("lib/content.ink"),
                    ]
                ),
            ])
    );
}

#[test]
fn duplicate_imports() {
    let state = new_state().with_comment_separated_files(indoc! {"
        // file: main.ink
        INCLUDE a.ink
        INCLUDE b.ink

        // file: a.ink
        INCLUDE b.ink
        I require b.

        // file: b.ink
        I am b.
    "});

    check!(
        state.story_structure()
            == structure([("main.ink", vec![Ok("main.ink"), Ok("a.ink"), Ok("b.ink")])])
    );
}

#[test]
fn circular_imports() {
    let state = new_state().with_comment_separated_files(indoc! {"
        // file: a.ink
        INCLUDE b.ink

        // file: b.ink
        INCLUDE c.ink

        // file: c.ink
        INCLUDE a.ink
    "});

    check!(
        state.story_structure()
            == structure([
                ("a.ink", vec![Ok("a.ink"), Ok("b.ink"), Ok("c.ink")]),
                ("b.ink", vec![Ok("b.ink"), Ok("c.ink"), Ok("a.ink")]),
                ("c.ink", vec![Ok("c.ink"), Ok("a.ink"), Ok("b.ink")])
            ])
    );
}

#[test]
fn semi_circular_imports() {
    let state = new_state().with_comment_separated_files(indoc! {"
        // file: a.ink
        INCLUDE b.ink

        // file: b.ink
        INCLUDE c.ink

        // file: c.ink
        INCLUDE b.ink
    "});

    check!(
        state.story_structure()
            == structure([
                ("a.ink", vec![Ok("a.ink"), Ok("b.ink"), Ok("c.ink")]),
                ("b.ink", vec![Ok("b.ink"), Ok("c.ink")]),
                ("c.ink", vec![Ok("c.ink"), Ok("b.ink")])
            ])
    );
}

#[test]
fn circular_and_non_circular_imports() {
    let state = new_state().with_comment_separated_files(indoc! {"
        // file: main.ink
        INCLUDE a.ink

        // file: circle.ink
        INCLUDE b.ink

        // file: a.ink
        Hi, I'm a.

        // file: b.ink
        INCLUDE circle.ink
    "});

    check!(
        state.story_structure()
            == structure([
                ("main.ink", vec![Ok("main.ink"), Ok("a.ink")]),
                ("circle.ink", vec![Ok("circle.ink"), Ok("b.ink")]),
                ("b.ink", vec![Ok("circle.ink"), Ok("b.ink")]),
            ])
    );
}
