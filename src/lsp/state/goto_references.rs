use crate::lsp::{
    salsa::InkGetters,
    state::{DocumentNotFound, GotoLocationError},
};
use lsp_types::{Location, Position, Uri};

impl super::State {
    pub fn goto_references(
        &self,
        from_uri: Uri,
        from_position: Position,
    ) -> Result<Vec<Location>, GotoLocationError> {
        let docs = self.db.doc_ids();
        let docid = docs
            .get_id(&from_uri)
            .ok_or_else(|| DocumentNotFound(from_uri.clone()))?;
        let doc = self.db.document(docid);

        let mut references = Vec::new();

        if let Some(usage) = doc.usage_at(from_position) {
            let def = self.db.definition_of(docid, usage.range.into());
            for (def_doc, def) in def.iter().copied() {
                let usages = self.db.usages_of(def_doc, def);
                references.extend(
                    usages
                        .iter()
                        .copied()
                        .map(|(docid, range)| Location::new(docs[docid].clone(), range.into())),
                );
            }
        }
        Ok(references)
    }
}

#[cfg(test)]
mod tests {
    use crate::lsp::state::{
        tests::{new_state, uri},
        State,
    };
    use assert2::assert;
    use itertools::Itertools;
    use lsp_types::{Location, Uri};
    use std::{
        collections::{BTreeSet, HashMap, HashSet},
        ops::Range,
        str::FromStr,
    };
    use text_annotations::{scan_default_annotations, Annotation};
    use util::testing::Compact;

    struct LinkCheck<'a> {
        state: &'a State,
        definitions: HashMap<&'a str, (&'a Uri, Annotation<'a>)>,
        references: Vec<(&'a Uri, Annotation<'a>, BTreeSet<&'a str>)>,
    }

    // XXX: This is actually testing `goto-definition` not references!
    // But OK for now. Goto references is the inverse and should come from the same sources. Remedy later.
    impl<'a> LinkCheck<'a> {
        fn new(state: &'a State) -> Self {
            Self {
                state,
                definitions: Default::default(),
                references: Default::default(),
            }
        }

        fn add_annotations(
            &mut self,
            uri: &'a Uri,
            annotations: impl IntoIterator<Item = Annotation<'a>>,
        ) {
            for loc in annotations {
                let mut claim = loc.claim().split_whitespace();
                let Some(keyword) = claim.next() else {
                    continue;
                };

                let claim = claim.collect::<BTreeSet<_>>();

                match keyword {
                    "defines" => {
                        assert!(claim.len() == 1, "defines takes exactly one argument");
                        let arg = claim.first().unwrap();
                        if let Some((existing_uri, existing_ann)) = self.definitions.get(arg) {
                            panic!(
                                "Duplicate definition for `{}`: {}:{:?} and {}:{:?}",
                                existing_ann.text(),
                                existing_uri.path().as_str(),
                                Compact(existing_ann.text_location),
                                uri.path().as_str(),
                                Compact(loc.text_location)
                            );
                        } else {
                            self.definitions.insert(arg, (uri, loc));
                        };
                    }
                    "references" => {
                        assert!(!claim.is_empty(), "references needs at least one parameter");
                        self.references.push((uri, loc, claim));
                    }
                    "references-nothing" => {
                        assert!(claim.is_empty(), "references-nothing takes no parameters");
                        self.references.push((uri, loc, claim));
                    }
                    _ => continue, // Ignore (might be a claim for another annotation scanner)
                }
            }
        }

        fn check(&'a self) {
            use annotate_snippets::{Level, Snippet};
            let renderer = annotate_snippets::Renderer::styled();
            let mut output = String::new();
            // Insurance against some obviously bad test definitions:
            {
                let defined_names = self.definitions.keys().copied().collect::<BTreeSet<_>>();
                let referenced_names = self
                    .references
                    .iter()
                    .flat_map(|(_, _, names)| names)
                    .copied()
                    .collect::<BTreeSet<_>>();
                assert!(
                    defined_names == referenced_names,
                    "We don't want dangling references or definitions in tests."
                );
                assert!(
                    defined_names.len() != 0,
                    "There should be at least one reference in a test"
                );
            }

            let inks: HashMap<&'a Uri, &'a str> = {
                let defs = self
                    .definitions
                    .iter()
                    .map(|(_, (uri, annotation))| (*uri, annotation.full_text));
                let refs = self
                    .references
                    .iter()
                    .map(|(uri, annotation, _)| (*uri, annotation.full_text));
                defs.chain(refs).collect()
            };

            for (usage_uri, usage_ann, names) in &self.references {
                let found_definitions: HashSet<Location> = self
                    .state
                    .goto_definition((*usage_uri).clone(), usage_ann.text_location.start.into())
                    .expect("we should be within range")
                    .into_iter()
                    .collect();

                let expected: HashSet<Location> = names
                    .iter()
                    .map(|name| self.definitions[name])
                    .map(|(uri, loc)| Location::new(uri.clone(), loc.text_location.into()))
                    .collect();

                if found_definitions == expected {
                    continue;
                }

                let mut error = Level::Error.title("Incorrect links").snippet(
                    Snippet::source(usage_ann.full_text)
                        .origin(usage_uri.path().as_str())
                        .fold(true)
                        .annotations([Level::Info
                            .span(usage_ann.text_location.byte_range())
                            .label("This usage …")]),
                );

                let missing_by_file = expected
                    .difference(&found_definitions)
                    .into_group_map_by(|it| &it.uri);
                for (file, ranges) in missing_by_file {
                    error = error.snippet(
                        Snippet::source(inks[file])
                            .origin(file.path().as_str())
                            .fold(true)
                            .annotations(ranges.into_iter().map(|loc| {
                                Level::Error.span(self.bytes(loc)).label("should link here")
                            })),
                    );
                }

                let unexpected_by_file = found_definitions
                    .difference(&expected)
                    .into_group_map_by(|it| &it.uri);

                for (file, ranges) in unexpected_by_file {
                    error = error.snippet(
                        Snippet::source(inks[file])
                            .origin(file.path().as_str())
                            .fold(true)
                            .annotations(ranges.into_iter().map(|loc| {
                                Level::Error
                                    .span(self.bytes(loc))
                                    .label("should NOT link here")
                            })),
                    );
                }

                output.push_str(&format!("{}", renderer.render(error)));
            }

            if !output.is_empty() {
                panic!("\n{output}\n");
            }
        }

        fn bytes(&self, loc: &Location) -> Range<usize> {
            let (doc, _) = self.state.get_doc_and_id(&loc.uri).unwrap();
            doc.byte_range(loc.range)
        }
    }

    #[test]
    fn namespacing() {
        let ink_files = walkdir::WalkDir::new("examples/namespacing/")
            .into_iter()
            .filter_ok(|it| it.path().extension().is_some_and(|it| it == "ink"))
            .map_ok(|it| {
                let path = it.path().as_os_str().to_string_lossy();
                let uri = Uri::from_str(&path).unwrap();
                let contents = std::fs::read_to_string(&*path).unwrap();
                (uri, contents)
            })
            .collect::<Result<HashMap<_, _>, _>>()
            .unwrap();

        let mut state = new_state();
        for (uri, contents) in &ink_files {
            // parse actual links via tree-sitter (i.e. normally)
            state.edit(uri.clone(), contents);
        }

        let mut checks = LinkCheck::new(&state);

        for (uri, contents) in &ink_files {
            // parse expected links via annotations
            checks.add_annotations(&uri, scan_default_annotations(contents))
        }

        checks.check();
    }

    #[test]
    fn goto_reference_arg() {
        use indoc::indoc;
        let text = indoc! {r"
                VAR number = 1
                //  ^^^^^^ defines number
                {  raise(number)  }
                //       ^^^^^^ references number
                == function raise(ref arg)
                ~ arg = arg + 1
            "};

        let main_uri = uri("main.ink");
        let mut state = new_state();
        state.edit(main_uri.clone(), text);
        let mut checks = LinkCheck::new(&state);

        checks.add_annotations(&main_uri, scan_default_annotations(text));

        checks.check();
    }

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
}
