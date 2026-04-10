use std::{
    borrow::Cow,
    collections::{BTreeSet, HashMap, HashSet},
    ops::Range,
    str::FromStr as _,
};

use annotate_snippets::{AnnotationKind, Level, Snippet};
use ink_document::InkDocument;
use ink_tool::lsp::{InkGetters, Ops, State};
use itertools::Itertools as _;
use lsp_types::{Location, Uri};
use mini_milc::Cached;
use text_annotations::{scan_default_annotations, Annotation};
use util::testing::Compact;

type TestResult = Result<(), Cow<'static, str>>;

fn main() -> TestResult {
    let mut state = State::new(None, false);

    let ink_files = walkdir::WalkDir::new("examples/namespacing/")
        .into_iter()
        .map(|file| file.expect("We don't tolerate errors in tests"))
        .filter(|file| file.path().extension().is_some_and(|it| it == "ink"));

    for ink in ink_files {
        let path = ink.path().as_os_str().to_string_lossy();
        let uri = Uri::from_str(&path).unwrap();
        let contents = std::fs::read_to_string(&*path).unwrap();
        state.edit(uri, contents);
    }

    test_goto_definition(&state)
}

fn test_goto_definition(state: &State) -> TestResult {
    let mut definitions: HashMap<&str, (&Uri, Annotation<'_>)> = Default::default();
    let mut references: Vec<(&Uri, Annotation<'_>, BTreeSet<&str>)> = Default::default();
    let doc_ids = state.db.doc_ids();
    let docs: HashMap<&Uri, Cached<Ops, InkDocument>> = doc_ids
        .pairs()
        .map(|(id, uri)| (uri, state.db.document(id)))
        .collect();

    // Sometimes we'll need the bytes from a a location that we don't know the annotation for:
    let bytes = |loc: &Location| -> Range<usize> { docs[&loc.uri].byte_range(loc.range) };

    for (uri, doc) in docs.iter() {
        let annotations = scan_default_annotations(doc.text(..));
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
                    if let Some((existing_uri, existing_ann)) = definitions.get(arg) {
                        return Err(format!(
                            "Duplicate definition for `{}`: {}:{:?} and {}:{:?}",
                            existing_ann.text(),
                            existing_uri.path().as_str(),
                            Compact(existing_ann.text_location),
                            uri.path().as_str(),
                            Compact(loc.text_location)
                        )
                        .into());
                    } else {
                        definitions.insert(arg, (uri, loc));
                    };
                }
                "references" => {
                    assert!(!claim.is_empty(), "references needs at least one parameter");
                    references.push((uri, loc, claim));
                }
                "references-nothing" => {
                    assert!(claim.is_empty(), "references-nothing takes no parameters");
                    references.push((uri, loc, claim));
                }
                _ => continue, // Ignore (might be a claim for another annotation scanner)
            }
        }
    }

    let renderer = annotate_snippets::Renderer::styled()
        .decor_style(annotate_snippets::renderer::DecorStyle::Unicode);
    let mut output = String::new();

    // Insurance against some obviously bad test definitions:
    {
        let defined_names = definitions.keys().copied().collect::<BTreeSet<_>>();
        let referenced_names = references
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

    let inks: HashMap<&Uri, &str> = {
        let defs = definitions
            .iter()
            .map(|(_, (uri, annotation))| (*uri, annotation.full_text));
        let refs = references
            .iter()
            .map(|(uri, annotation, _)| (*uri, annotation.full_text));
        defs.chain(refs).collect()
    };

    for (usage_uri, usage_ann, names) in &references {
        let found_definitions: HashSet<Location> = state
            .goto_definition((*usage_uri).clone(), usage_ann.text_location.start.into())
            .expect("we should be within range")
            .into_iter()
            .collect();

        let expected: HashSet<Location> = names
            .iter()
            .map(|name| definitions[name])
            .map(|(uri, loc)| Location::new(uri.clone(), loc.text_location.into()))
            .collect();

        if found_definitions == expected {
            continue;
        }

        let mut error = Level::ERROR.primary_title("Incorrect links").element(
            Snippet::source(usage_ann.full_text)
                .path(usage_uri.path().as_str())
                .fold(true)
                .annotations([AnnotationKind::Primary
                    .span(usage_ann.text_location.byte_range())
                    .label("This usage …")]),
        );

        let missing_by_file = expected
            .difference(&found_definitions)
            .into_group_map_by(|it| &it.uri);
        for (file, ranges) in missing_by_file {
            error = error.element(
                Snippet::source(inks[file])
                    .path(file.path().as_str())
                    .fold(true)
                    .annotations(ranges.into_iter().map(|loc| {
                        AnnotationKind::Primary
                            .span(bytes(loc))
                            .label("should link here")
                    })),
            );
        }

        let unexpected_by_file = found_definitions
            .difference(&expected)
            .into_group_map_by(|it| &it.uri);
        for (file, ranges) in unexpected_by_file {
            error = error.element(
                Snippet::source(inks[file])
                    .path(file.path().as_str())
                    .fold(true)
                    .annotations(ranges.into_iter().map(|loc| {
                        AnnotationKind::Primary
                            .span(bytes(loc))
                            .label("should NOT link here")
                    })),
            );
        }

        output.push_str(&format!("{}\n\n", renderer.render(&[error])));
    }

    let output = output.trim();
    if !output.is_empty() {
        print!("\n{output}\n");
        Err("Test failed".into())
    } else {
        Ok(())
    }
}
