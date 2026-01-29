use crate::lsp::{salsa::InkGetters as _, state::GotoLocationError};
use lsp_types::{Position, Uri};

impl super::State {
    pub fn goto_references(
        &self,
        from_uri: Uri,
        from_position: Position,
    ) -> Result<Vec<lsp_types::Location>, GotoLocationError> {
        let docs = self.db.doc_ids();
        let this_doc = docs
            .get_id(&from_uri)
            .ok_or_else(|| DocumentNotFound(from_uri.clone()))?;
        let doc = self.db.document(this_doc);

        let Some(usage) = doc.usage_at(from_position) else {
            return Ok(Vec::new());
        };

        log::trace!(
            r#"finding references for usage "{}" ({}:{}|{}-{}|{})"#,
            usage.term,
            from_uri.path().as_estr(),
            usage.range.start.line,
            usage.range.start.character,
            usage.range.end.line,
            usage.range.end.character,
        );

        let definitions = self
            .db
            .workspace_names()
            .iter()
            .filter_map(|(name, metas)| (usage.term == name).then_some(metas))
            .flatten()
            .filter(|(doc_id, meta)| {
                meta.is_global()
                    || (*doc_id == this_doc && meta.is_locally_visible_at(from_position))
            })
            .map(|(doc_id, meta)| lsp_types::Location::new(docs[*doc_id].clone(), meta.site.into()))
            .collect();

        Ok(definitions)
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
    use lsp_types::Uri;
    use std::{
        collections::{BTreeSet, HashMap},
        str::FromStr,
    };
    use tap::Pipe;
    use test_case::test_case;
    use text_annotations::{scan_default_annotations, Annotation};
    use util::testing::Compact;

    #[derive(Debug, Default)]
    struct LinkCheck<'a> {
        definitions: HashMap<&'a str, (&'a Uri, Annotation<'a>)>,
        references: Vec<(&'a Uri, Annotation<'a>, ReferenceKind, Vec<&'a str>)>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ReferenceKind {
        Any,
        Not,
        Nothing,
    }

    impl<'a> LinkCheck<'a> {
        fn add_annotations(
            &mut self,
            uri: &'a Uri,
            annotations: impl IntoIterator<Item = Annotation<'a>>,
        ) {
            for loc in annotations {
                let claim = loc.claim().split_whitespace().collect_vec();
                if claim.is_empty() {
                    continue;
                };
                match claim[0] {
                    "defines" => {
                        assert!(claim.len() == 2, "defines takes exactly one argument");
                        let arg = &claim[1];
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
                            self.definitions.insert(claim[1], (uri, loc));
                        };
                    }
                    "references-nothing" => {
                        assert!(
                            claim.len() == 1,
                            "references-nothing doesn't take arguments"
                        );
                        self.references
                            .push((uri, loc, ReferenceKind::Nothing, Vec::new()));
                    }
                    "references" => {
                        assert!(claim.len() > 1, "references takes at least one argument");
                        let names = claim[1..].to_vec();
                        self.references.push((uri, loc, ReferenceKind::Any, names));
                    }
                    "references-not" => {
                        assert!(
                            claim.len() > 1,
                            "references-not takes at least one argument"
                        );
                        let names = claim[1..].to_vec();
                        self.references.push((uri, loc, ReferenceKind::Not, names));
                    }
                    _ => continue, // Ignore (might be a claim for another annotation scanner)
                }
            }
        }

        fn check(&'a self, state: &'a State) {
            // Insurance against some obviously bad test definitions:
            {
                let defined_names = self.definitions.keys().copied().collect::<BTreeSet<_>>();
                let referenced_names = self
                    .references
                    .iter()
                    .flat_map(|(_, _, _, names)| names.iter())
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
                    .map(|(uri, annotation, _, _)| (*uri, annotation.full_text));
                defs.chain(refs).collect()
            };

            let mut messages = Vec::new();
            use annotate_snippets::{Level, Snippet};

            for (usage_uri, usage_ann, reference_kind, names) in &self.references {
                let usage_lsp_position: lsp_types::Range = usage_ann.text_location.into();
                let found_definitions = state
                    .goto_definition((*usage_uri).clone(), usage_lsp_position.start)
                    .expect("we should be within range");
                let per_file_defs = |level: Level| {
                    let per_file = found_definitions
                        .iter()
                        .into_group_map_by(|it| it.uri.clone());
                    per_file
                        .into_iter()
                        .map(|(uri, defs)| {
                            // have to use def_uri, because we can't return a reference to def.uri from this function.
                            let (def_uri, def_src) = inks.get_key_value(&uri).unwrap();
                            Snippet::source(def_src)
                                .origin(def_uri.path().as_str())
                                .fold(true)
                                .annotations(defs.into_iter().map(|def| {
                                    let range = state.byte_range_of(&uri, def.range);
                                    level.span(range)
                                }))
                        })
                        .collect_vec()
                };
                if matches!(reference_kind, ReferenceKind::Nothing) {
                    if !found_definitions.is_empty() {
                        messages.push(
                            Level::Error
                                .title("Disallowed definitions")
                                .snippet(Self::annotation_to_snippet(
                                    &usage_uri,
                                    &usage_ann,
                                    "Expected this usage to be unresolved, …",
                                ))
                                .footer(
                                    Level::Info
                                        .title("… but it resolves to:")
                                        .snippets(per_file_defs(Level::Error)),
                                ),
                        )
                    }
                } else {
                    for name in names {
                        let (def_uri, def_ann) = self
                            .definitions
                            .get(name)
                            .expect("we checked that every reference has a definition, see above");
                        let range = def_ann.text_location.into();
                        let does_reference = found_definitions
                            .iter()
                            .any(|def| def.uri == **def_uri && def.range == range);

                        if matches!(reference_kind, ReferenceKind::Any) && !does_reference {
                            messages.push(
                                Level::Error
                                    .title("Required reference not found")
                                    .snippets([
                                        Self::annotation_to_snippet(
                                            &usage_uri,
                                            &usage_ann,
                                            "Expected this usage …",
                                        ),
                                        Self::annotation_to_snippet(
                                            &def_uri,
                                            &def_ann,
                                            "… to reference this definition.",
                                        ),
                                    ])
                                    .footer(per_file_defs(Level::Warning).pipe(|defs| {
                                        if defs.is_empty() {
                                            Level::Error.title("But it links nowhere.")
                                        } else {
                                            Level::Error
                                                .title("But it only links to these locations:")
                                                .snippets(defs)
                                        }
                                    })),
                            );
                        } else if matches!(reference_kind, ReferenceKind::Not) && does_reference {
                            messages.push(
                                Level::Error.title("Forbidden reference found").snippets([
                                    Self::annotation_to_snippet(
                                        &usage_uri,
                                        &usage_ann,
                                        "Expected this usage …",
                                    ),
                                    Self::annotation_to_snippet(
                                        &def_uri,
                                        &def_ann,
                                        "… to not reference this definition, but it does.",
                                    ),
                                ]),
                            );
                        }
                    }
                }
            }

            if !messages.is_empty() {
                let renderer = annotate_snippets::Renderer::styled();
                for message in messages {
                    eprintln!("{}", renderer.render(message));
                }
                panic!("check failed");
            }
        }

        fn annotation_to_snippet(
            file: &'a Uri,
            ann: &text_annotations::Annotation<'a>,
            label: &'a str,
        ) -> annotate_snippets::Snippet<'a> {
            annotate_snippets::Snippet::source(ann.full_text)
                .origin(file.path().as_str())
                .fold(true)
                .annotations([
                    annotate_snippets::Level::Error
                        .span(ann.text_location.byte_range())
                        .label(label),
                    annotate_snippets::Level::Info
                        .span(ann.claim_location.byte_range())
                        .label("due to this claim"),
                ])
        }
    }

    #[test_case("examples/links/forward_declarations.ink")]
    #[test_case("examples/links/temp_vars.ink")]
    #[test_case("examples/links/lists.ink")]
    #[test_case("examples/links/labels.ink")]
    #[test_case("examples/links/shadowing.ink")]
    #[test_case("examples/links/self-reference.ink")]
    #[test_case("examples/links/ambiguous/")]
    #[test_case("examples/links/knots_and_stitches/")]
    fn test_links(fs_location: &str) {
        util::testing::setup_logging(log::LevelFilter::Trace);

        let ink_files = walkdir::WalkDir::new(fs_location)
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
        let mut checks = LinkCheck::default();

        for (uri, contents) in &ink_files {
            // parse actual links via tree-sitter (i.e. normally)
            state.edit(uri.clone(), contents);
            // parse expected links via annotations
            checks.add_annotations(&uri, scan_default_annotations(contents))
        }

        checks.check(&state);
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

        let mut state = new_state();
        let mut checks = LinkCheck::default();

        let main_uri = uri("main.ink");
        state.edit(main_uri.clone(), text);
        checks.add_annotations(&main_uri, scan_default_annotations(text));

        checks.check(&state);
    }

    mod labels {
        use crate::lsp::state::tests::{new_state, uri};
        use assert2::check;
        use itertools::Itertools;
        use std::collections::HashMap;
        use text_annotations::{scan_default_annotations, TextRegion};

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
            let state = new_state().with_comment_separated_files(TEXT);

            let loc = named_ranges::<lsp_types::Range>(TEXT);

            let defs = state
                .goto_references(&uri("main.ink"), loc["label-ref1"].start)
                .unwrap()
                .into_iter()
                .map(|it| it.range)
                .sorted_unstable_by(|a, b| a.start.cmp(&b.start).then_with(|| a.end.cmp(&b.end)))
                .inspect(|it| eprintln!("{it:#?}"))
                .collect_vec();

            check!(defs == vec![loc["label-ref1"], loc["label-ref2"], loc["label"]]);
            // check!(defs[0].range == loc["label-ref1"]);
            // check!(defs[1].range == loc["label-ref2"]);
            // check!(defs[2].range == loc["label"]);
        }
    }
}
