use super::{
    document::{DocumentEdit, InkDocument},
    location::{
        self,
        specification::{rank_match, LocationThat},
        Location,
    },
    salsa::{doc_symbols, locations, workspace_symbols, DbImpl, Doc, Workspace},
};
use crate::{ink_syntax, lsp::salsa::usages_to_definitions};
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

    pub fn goto_definition(
        &self,
        from_uri: Uri,
        from_position: Position,
    ) -> Result<Vec<lsp_types::Location>, GotoDefinitionError> {
        let Some(doc) = self.workspace.docs(&self.db).get(&from_uri) else {
            return Err(DocumentNotFound(from_uri).into());
        };
        let target_node = doc.named_cst_node_at(&self.db, from_position)?;
        eprintln!("Searching for links for target node {target_node:?}");
        if target_node.kind() != ink_syntax::types::Identifier::KIND {
            eprintln!("Not even an identifier, what is wrong with you!");
            return Ok(Vec::new());
        }
        let usg2def = usages_to_definitions(&self.db, self.workspace);
        let Some(map) = usg2def.get(&from_uri) else {
            return Ok(Vec::new());
        };
        let mut defs = map.get(&target_node.range()).cloned().unwrap_or_default();
        if let Some(parent) = target_node.parent() {
            if parent.kind() == ink_syntax::types::QualifiedName::KIND {
                defs.extend(
                    map.get(&parent.range())
                        .cloned()
                        .unwrap_or_default()
                        .into_iter(),
                );
            }
        }
        Ok(defs
            .into_iter()
            .map(|(uri, range)| lsp_types::Location { uri, range })
            .collect())
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
        use std::{collections::HashMap, str::FromStr};

        use super::{new_state, set_content};
        use crate::{
            lsp::salsa::links_for_workspace,
            test_utils::{
                text_annotations::{Annotation, AnnotationScanner, TextRegion},
                Compact,
            },
        };
        use assert2::assert;
        use itertools::Itertools;
        use lsp_types::Uri;
        use test_case::test_case;

        #[derive(Debug, Default)]
        struct LinkCheck<'a> {
            definitions: HashMap<&'a str, (&'a Uri, Annotation<'a>)>,
            references: Vec<(&'a Uri, Annotation<'a>, ReferenceKind, Vec<&'a str>)>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum ReferenceKind {
            Any,
            None,
            Unresolved,
        }

        impl<'a> LinkCheck<'a> {
            fn add_annotations(
                &mut self,
                uri: &'a Uri,
                annotations: impl IntoIterator<Item = Annotation<'a>>,
            ) {
                for loc in annotations {
                    if loc.claim().trim().starts_with("references-nothing") {
                        self.references
                            .push((uri, loc, ReferenceKind::Unresolved, Vec::new()));
                        continue;
                    }
                    let Some((keyword, arg)) = loc.claim().split_once(char::is_whitespace) else {
                        continue;
                    };
                    match keyword {
                        "defines" => {
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
                            let names = arg.trim().split_whitespace().collect_vec();
                            if names.is_empty() {
                                panic!("references must have at least one argument");
                            }
                            self.references.push((uri, loc, ReferenceKind::Any, names));
                        }
                        "references-not" => {
                            let names = arg.trim().split_whitespace().collect_vec();
                            if names.is_empty() {
                                panic!("references-not must have at least one argument");
                            }
                            self.references.push((uri, loc, ReferenceKind::None, names));
                        }
                        _ => continue, // Ignore (might be a claim for another annotation scanner)
                    }
                }
            }

            fn check(
                &self,
                links: &crate::lsp::links::Links<'a, (&'a Uri, TextRegion)>,
            ) -> Vec<annotate_snippets::Message> {
                // Insurance against some obviously bad test definitions:
                {
                    let defined_names = self
                        .definitions
                        .keys()
                        .map(|it| *it)
                        .sorted_unstable()
                        .unique()
                        .collect::<Vec<_>>();
                    let referenced_names = self
                        .references
                        .iter()
                        .flat_map(|(_, _, _, names)| names.iter().map(|it| *it))
                        .sorted_unstable()
                        .unique()
                        .collect::<Vec<_>>();
                    assert!(
                        defined_names == referenced_names,
                        "We don't want dangling references or definitions in tests."
                    );
                    assert!(
                        defined_names.len() != 0,
                        "There should be at least one reference in a test"
                    );
                }

                let inks: HashMap<&'a Uri, &'a str> = self
                    .references
                    .iter()
                    .map(|(uri, annotation, _, _)| (*uri, annotation.full_text))
                    .collect();

                let mut messages = Vec::new();
                for (usage_uri, usage_ann, reference_kind, names) in &self.references {
                    let usage = (*usage_uri, usage_ann.text_location);
                    if *reference_kind == ReferenceKind::Unresolved {
                        let definitions = links.definitions(&usage).collect_vec();
                        if !definitions.is_empty() {
                            for (def_uri, def_region) in definitions {
                                messages.push(
                                    annotate_snippets::Level::Error
                                        .title("Disallowed definition")
                                        .snippets([
                                            Self::annotation_to_snippet(
                                                &usage_uri,
                                                &usage_ann,
                                                "Expected this usage to be unresolved, …",
                                            ),
                                            annotate_snippets::Snippet::source(&inks[def_uri])
                                                .origin(def_uri.path().as_str())
                                                .line_start(def_region.start.col as usize)
                                                .fold(true)
                                                .annotation(
                                                    annotate_snippets::Level::Error
                                                        .span(def_region.byte_range())
                                                        .label("… but it links to this definition"),
                                                ),
                                        ]),
                                );
                            }
                        }
                    }
                    for name in names {
                        let (def_uri, def_ann) = self
                            .definitions
                            .get(name)
                            .expect("we checked that every reference has a definition, see above");
                        let expected = ((*def_uri, def_ann.text_location), usage);
                        let does_reference = links.resolved.contains(&expected);
                        if *reference_kind == ReferenceKind::Any && !does_reference {
                            messages.push(
                                annotate_snippets::Level::Error
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
                                    ]),
                            );
                        } else if *reference_kind == ReferenceKind::None && does_reference {
                            messages.push(
                                annotate_snippets::Level::Error
                                    .title("Forbidden reference found")
                                    .snippets([
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

                if !messages.is_empty() {
                    // Add existing links as additional context
                    let all_links = links
                        .resolved
                        .iter()
                        .cloned()
                        .into_grouping_map()
                        .collect::<std::collections::HashSet<_>>();
                    if all_links.is_empty() {
                        messages.push(
                            annotate_snippets::Level::Warning.title("No resolved links found!"),
                        );
                    }
                    for ((uri, region), usages) in all_links {
                        let mut message =
                            annotate_snippets::Level::Help.title("Found link").snippet(
                                annotate_snippets::Snippet::source(&inks[uri])
                                    .origin(uri.path().as_str())
                                    .line_start(region.start.col as usize)
                                    .fold(true)
                                    .annotation(
                                        annotate_snippets::Level::Help
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
                                            .label(if i == 0 {
                                                "is used here"
                                            } else {
                                                "and here"
                                            }),
                                    ),
                            );
                        }
                        messages.push(message);
                    }
                }

                messages
            }

            fn annotation_to_snippet(
                file: &'a Uri,
                ann: &crate::test_utils::text_annotations::Annotation<'a>,
                label: &'a str,
            ) -> annotate_snippets::Snippet<'a> {
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

        #[test_case("examples/links/forward_declarations.ink")]
        #[test_case("examples/links/temp_vars.ink")]
        #[test_case("examples/links/lists.ink")]
        #[test_case("examples/links/labels.ink")]
        #[test_case("examples/links/shadowing.ink")]
        #[test_case("examples/links/ambiguous/")]
        #[test_case("examples/links/knots_and_stitches/")]
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
            let mut checks = LinkCheck::default();

            for (uri, contents) in &inks {
                // parse actual links via tree-sitter (i.e. normally)
                set_content(&mut state, uri.clone(), contents);
                // parse expected links via annotations
                checks.add_annotations(uri, annotation_scanner.scan(contents))
            }

            // WHEN
            let actual_links =
                links_for_workspace(&state.db, state.workspace).transform_locations(|it| {
                    let uri = it.cst(&state.db).uri(&state.db);
                    let region = TextRegion::from(it.cst_node(&state.db));
                    (uri, region)
                });

            // THEN
            let messages = checks.check(&actual_links);
            if !messages.is_empty() {
                let renderer = annotate_snippets::Renderer::styled();
                for message in messages {
                    eprintln!("{}", renderer.render(message));
                }
                panic!("Link check failed for {fs_location}");
            }
        }
    }
}
