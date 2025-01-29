use super::{
    document::{DocumentEdit, InkDocument},
    location::{
        self,
        specification::{rank_match, LocationThat},
        Location,
    },
    salsa::{doc_symbols, locations, workspace_symbols, DbImpl, Doc, Workspace},
};
use crate::{ink_syntax, lsp::salsa::definitions_to_usages};
use derive_more::derive::{Display, Error};
use line_index::WideEncoding;
use lsp_types::{
    CompletionItem, CompletionItemKind, DocumentSymbol, Position, Uri, WorkspaceSymbol,
};
use salsa::Setter as _;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};
use type_sitter_lib::Node;

/// A way to identify Documents hat is Copy (instead of just clone, like Uri).
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct DocId(u64);
impl From<&Uri> for DocId {
    fn from(uri: &Uri) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();
        (*uri).hash(&mut hasher);
        DocId(hasher.finish())
    }
}

pub(crate) struct State {
    parsers: HashMap<DocId, InkDocument>,
    workspace: Workspace,
    db: DbImpl,
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Error)]
#[display("Document not found: `{}`", _0.path())]
pub(crate) struct DocumentNotFound(#[error(not(source))] pub(crate) Uri);

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_more::Error)]
#[display("Position out of bounds: {}:{}", _0.line, _0.character)]
pub(crate) struct PositionOutOfBounds(#[error(not(source))] pub(crate) Position);

#[derive(Debug, Clone, Display, Error, derive_more::From)]
#[display("Could not go to position")]
pub(crate) enum GotoDefinitionError {
    DocumentNotFound(DocumentNotFound),
    PositionOutOfBounds(PositionOutOfBounds),
}

impl State {
    pub fn new(wide_encoding: Option<WideEncoding>, qualified_names: bool) -> Self {
        let db = DbImpl::new();
        let workspace = Workspace::new(&db, Default::default(), wide_encoding);
        Self {
            db,
            parsers: Default::default(),
            workspace,
        }
    }

    pub fn edit<S: AsRef<str> + Into<String>>(&mut self, uri: Uri, edits: Vec<DocumentEdit<S>>) {
        let doc_id = DocId::from(&uri);
        if !self.parsers.contains_key(&doc_id) {
            self.parsers.insert(
                doc_id,
                InkDocument::new(uri.clone(), String::new(), self.workspace.enc(&self.db)),
            );
            let parser = &self.parsers[&doc_id];
            let mut ws_docs = self.workspace.docs(&self.db).clone();
            ws_docs.insert(
                uri.clone(),
                Doc::new(
                    &self.db,
                    uri.clone(),
                    parser.text.clone(),
                    parser.tree.clone(),
                    parser.lines.clone(),
                    self.workspace.enc(&self.db),
                ),
            );
            self.workspace.set_docs(&mut self.db).to(ws_docs);
        }
        let parser = self
            .parsers
            .get_mut(&doc_id)
            .expect("we just made sure it exists");
        parser.edit(edits);
        let doc = *self
            .workspace
            .docs(&self.db)
            .get(&uri)
            .expect("just made sure");
        doc.set_text(&mut self.db).to(parser.text.clone());
        doc.set_tree(&mut self.db).to(parser.tree.clone());
        doc.set_lines(&mut self.db).to(parser.lines.clone());
    }

    pub fn forget(&mut self, uri: Uri) -> Result<(), DocumentNotFound> {
        let doc_id = DocId::from(&uri);
        match self.parsers.remove(&doc_id) {
            Some(_) => {
                let mut ws_docs = self.workspace.docs(&self.db).clone();
                ws_docs.remove(&uri);
                self.workspace.set_docs(&mut self.db).to(ws_docs);
                Ok(())
            }
            None => Err(DocumentNotFound(uri)),
        }
    }

    /// Return a document symbol for this `uri`. Error on unknown document
    pub fn document_symbols(
        &mut self,
        uri: Uri,
    ) -> Result<Option<DocumentSymbol>, DocumentNotFound> {
        match self.workspace.docs(&self.db).get(&uri) {
            Some(&doc) => Ok(doc_symbols(&self.db, doc)),
            None => Err(DocumentNotFound(uri)),
        }
    }

    pub fn workspace_symbols(&mut self, query: String) -> Vec<WorkspaceSymbol> {
        let mut symbols = workspace_symbols(&self.db, self.workspace);
        if query.is_empty() {
            symbols
        } else {
            let query = query.to_lowercase();
            symbols.retain(|sym| sym.name.to_lowercase().contains(&query));
            symbols
        }
    }

    pub fn completions(
        &mut self,
        uri: Uri,
        position: Position,
    ) -> Result<Option<Vec<CompletionItem>>, DocumentNotFound> {
        match self.parsers.get_mut(&DocId::from(&uri)) {
            Some(doc) => {
                let Some((range, specification)) = doc.possible_completions(position) else {
                    return Ok(None);
                };
                // eprintln!("find {specification}");
                let completions = self
                    .find_locations(specification)
                    .map(|loc| to_completion_item(range, loc))
                    .collect();
                Ok(Some(completions))
            }
            None => Err(DocumentNotFound(uri)),
        }
    }

    fn find_locations(&self, spec: LocationThat) -> impl Iterator<Item = location::Location> {
        let mut locs = Vec::new();
        for (_uri, &doc) in self.workspace.docs(&self.db) {
            let doc_locs = locations(&self.db, doc)
                .into_iter()
                .map(|loc| (rank_match(&spec, &loc), loc))
                .filter(|(rank, _)| *rank > 0);
            locs.extend(doc_locs);
        }
        locs.sort_unstable_by_key(|(rank, _)| *rank);
        locs.into_iter().map(|(_, location)| location)
    }
}

fn to_completion_item(_range: lsp_types::Range, loc: Location) -> CompletionItem {
    CompletionItem {
        label: loc.name.clone(),
        detail: match loc.namespace {
            Some(ref ns) => Some(format!("{}: {ns}", loc.path_as_str())),
            None => Some(loc.path_as_str().to_owned()),
        },
        kind: Some(match loc.kind {
            location::LocationKind::Knot => CompletionItemKind::CLASS,
            location::LocationKind::Stitch => CompletionItemKind::METHOD,
            location::LocationKind::Label => CompletionItemKind::FIELD,
            location::LocationKind::Variable => CompletionItemKind::VARIABLE,
            location::LocationKind::Function => CompletionItemKind::FUNCTION,
        }),
        label_details: None,
        documentation: None,
        deprecated: None,
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: None,
        insert_text_format: None,
        insert_text_mode: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        commit_characters: None,
        data: None,
        tags: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_state() -> State {
        State::new(None, true)
    }

    fn uri(name: &str) -> Uri {
        <Uri as std::str::FromStr>::from_str(&format!("file://tmp/{name}")).unwrap()
    }

    fn set_content(state: &mut State, uri: Uri, contents: impl Into<String>) {
        state.edit(uri, vec![(None, contents.into())]);
    }

    fn text_with_caret(input: &str) -> (String, lsp_types::Position) {
        let mut row = 0;
        let mut col = 0;
        for (idx, chr) in input.char_indices() {
            match chr {
                '@' => {
                    let pos = Position::new(row, col);
                    let mut output = input.to_string();
                    output.remove(idx);
                    return (output, pos);
                }
                '\n' => {
                    row += 1;
                    col = 0;
                }
                _ => {
                    col += 1;
                }
            }
        }
        panic!("There should have been an '@' in there somewhere.");
    }

    mod completions {
        use super::{set_content, tests::text_with_caret, uri};
        use crate::lsp::state;
        use pretty_assertions::assert_eq;

        #[test]
        fn state() {
            let mut state = super::new_state();
            set_content(
                &mut state,
                uri("context.ink"),
                "
                VAR some_var = true

                == one
                text

                == two
                text
            ",
            );
            let (contents, caret) = text_with_caret("{o@}");
            let uri = uri("test.ink");
            set_content(&mut state, uri.clone(), contents);
            let doc = state.parsers.get(&state::DocId::from(&uri)).unwrap();
            eprintln!(
                "completions: {:?}",
                doc.possible_completions(caret).unwrap()
            );
            let completions = state.completions(uri, caret).unwrap().unwrap();
            assert_eq!(
                completions
                    .into_iter()
                    .map(|it| it.label)
                    .collect::<Vec<_>>(),
                vec!["some_var", "one", "two"]
            );
        }
    }

    mod links {
        use std::{
            collections::{HashMap, HashSet},
            str::FromStr,
        };

        use super::{new_state, set_content};
        use crate::{
            lsp::{links::Links, salsa::links_for_workspace, state::tests::annotation_to_snippet},
            test_utils::text_annotations::{Annotation, AnnotationScanner, TextPos, TextRegion},
        };
        use assert2::assert;
        use itertools::Itertools;
        use lsp_types::Uri;
        use test_case::test_case;

        pub fn annotations_to_links<'a>(
            annotations: impl IntoIterator<Item = Annotation<'a>>,
        ) -> Links<'a, Annotation<'a>> {
            use itertools::Either;
            use std::convert::identity;
            let (provided_names, resolvable): (Vec<_>, Vec<_>) = annotations
                .into_iter()
                .filter_map(|it| {
                    let Some((keyword, name)) = it.claim().split_whitespace().collect_tuple()
                    else {
                        return None;
                    };
                    match keyword {
                        "defines" => Some(Either::Left((name.to_string(), it))),
                        "references" => Some(Either::Right((it, name))),
                        _ => None, // Ignore (might be a claim for another annotation scanner)
                    }
                })
                .partition_map(identity);
            let provided_names = provided_names.into_iter().into_group_map();
            Links {
                resolved: Vec::new(),
                resolvable,
                provided_names,
            }
        }

        #[test_case("examples/links/forward_declarations.ink")]
        #[test_case("examples/links/temp_vars.ink")]
        #[test_case("examples/links/ambiguous/")]
        #[test_case("examples/links/knots_and_stitches/")]
        #[test_case("examples/links/labels/")]
        fn test_links(fs_location: &str) {
            // GIVEN
            let inks = walkdir::WalkDir::new(fs_location)
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
            let annotation_scanner = AnnotationScanner::new();
            let mut expected_links = Links::default();

            for (uri, contents) in &inks {
                // parse via tree-sitter (i.e. normally)
                set_content(&mut state, uri.clone(), contents);
                // parse annotations
                expected_links += annotations_to_links(annotation_scanner.scan(contents))
                    .transform_locations(|it| (uri, it));
            }
            expected_links.resolve();

            // insurance against some obviously bad test definitions:
            {
                assert!(
                    expected_links.resolved.len() >= 1,
                    "There should be at least one expected link."
                );
                let referenced_names = expected_links
                    .resolvable
                    .iter()
                    .map(|(_, name)| *name)
                    .sorted_unstable()
                    .unique()
                    .collect::<Vec<_>>();
                let defined_names = expected_links
                    .provided_names
                    .iter()
                    .map(|(name, _)| name.as_str())
                    .sorted_unstable()
                    .unique()
                    .collect::<Vec<_>>();
                assert!(
                    defined_names == referenced_names,
                    "We don't want dangling references or definitions in tests."
                );
            }

            // WHEN
            let transform_locations = links_for_workspace(&state.db, state.workspace)
                .transform_locations(|it| {
                    let node = it.cst_node(&state.db);
                    let interval = TextRegion {
                        start: TextPos {
                            byte: node.start_byte(),
                            row: node.start_position().row as u32,
                            col: node.start_position().column as u32,
                        },
                        end: TextPos {
                            byte: node.end_byte(),
                            row: node.end_position().row as u32,
                            col: node.end_position().column as u32,
                        },
                    };
                    let uri = it.cst(&state.db).uri(&state.db);
                    (uri, interval)
                });
            let actual_links: HashSet<_> = (transform_locations).resolved.into_iter().collect();

            // THEN
            let expected_links: HashSet<_> = expected_links.resolved.into_iter().collect();
            let num_expected_links = expected_links.len();
            let mut found_references = HashSet::new();
            let mut messages = Vec::new();
            for ((def_uri, def_ann), (usage_uri, usage_ann)) in expected_links {
                let expected = (
                    (def_uri, def_ann.text_location),
                    (usage_uri, usage_ann.text_location),
                );
                if actual_links.contains(&expected) {
                    found_references.insert(expected);
                } else {
                    messages.push(
                        annotate_snippets::Level::Error
                            .title("Required reference not found")
                            .snippets([
                                annotation_to_snippet(
                                    &usage_uri,
                                    &usage_ann,
                                    "Expected this usage …",
                                ),
                                annotation_to_snippet(
                                    &def_uri,
                                    &def_ann,
                                    "… to reference this definition.",
                                ),
                            ]),
                    );
                }
            }

            let renderer = annotate_snippets::Renderer::styled();
            if !messages.is_empty() {
                for message in messages {
                    eprintln!("{}", renderer.render(message));
                }
                let all_links = actual_links.into_iter().into_group_map();
                for ((uri, region), usages) in all_links {
                    let mut message = annotate_snippets::Level::Info.title("Found link").snippet(
                        annotate_snippets::Snippet::source(&inks[uri])
                            .origin(uri.path().as_str())
                            .line_start(region.start.col as usize)
                            .fold(true)
                            .annotation(
                                annotate_snippets::Level::Info
                                    .span(region.byte_range())
                                    .label("this definition"),
                            ),
                    );
                    for (i, (uri, region)) in usages.into_iter().enumerate() {
                        message = message.snippet(
                            annotate_snippets::Snippet::source(&inks[uri])
                                .origin(uri.path().as_str())
                                .line_start(region.start.col as usize)
                                .fold(true)
                                .annotation(
                                    annotate_snippets::Level::Info
                                        .span(region.byte_range())
                                        .label(if i == 0 { "is used here" } else { "and here" }),
                                ),
                        );
                    }

                    eprintln!("{}", renderer.render(message));
                }
                panic!(
                    "Expected {num_expected_links} reference(s) in {fs_location} \
                    but found {} references in links.",
                    found_references.len()
                );
            }
        }
    }

    fn annotation_to_snippet<'a: 'text, 'text>(
        file: &'a Uri,
        ann: &crate::test_utils::text_annotations::Annotation<'text>,
        label: &'a str,
    ) -> annotate_snippets::Snippet<'text> {
        annotate_snippets::Snippet::source(ann.full_text)
            .origin(file.path().as_str())
            .line_start(ann.text_location.start.col as usize)
            .fold(true)
            .annotations([
                annotate_snippets::Level::Error
                    .span(ann.text_location.byte_range())
                    .label(label),
                annotate_snippets::Level::Help
                    .span(ann.claim_location.byte_range())
                    .label("due to this claim"),
            ])
    }
}
