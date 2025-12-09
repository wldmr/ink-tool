use super::document::{DocumentEdit, InkDocument};
use crate::lsp::salsa::{self, names::Meta, DocId, InkGetters, InkSetters};
use derive_more::derive::{Display, Error};
use line_index::WideEncoding;
use lsp_types::{CompletionItem, DocumentSymbol, Position, Uri, WorkspaceSymbol};
use tap::Tap as _;

// This is quite an abomination, but we have to deal with it.
type DbType = mini_milc::salsa::Salsa<
    // Query
    salsa::Ops,
    // Storagage Index
    salsa::Ops,
    // Storage impl
    mini_milc::storage_impls::HashMapStorage<salsa::Ops>,
>;

pub(crate) struct State {
    pub(crate) db: DbType,
    pub(crate) enc: Option<WideEncoding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Error)]
#[display("Document not found: `{}`", _0.path())]
pub(crate) struct DocumentNotFound(#[error(not(source))] pub(crate) Uri);

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_more::Error)]
#[display("Not a valid position: {}:{}", _0.line, _0.character)]
pub(crate) struct InvalidPosition(#[error(not(source))] pub(crate) Position);

#[derive(Debug, Clone, Display, Error, derive_more::From)]
#[display("Could not go to position")]
pub(crate) enum GotoDefinitionError {
    DocumentNotFound(DocumentNotFound),
    PositionOutOfBounds(InvalidPosition),
}

impl State {
    pub fn new(enc: Option<WideEncoding>, _qualified_names: bool) -> Self {
        Self {
            db: mini_milc::salsa_hashmap(),
            enc,
        }
    }

    pub fn uris(&self) -> Vec<Uri> {
        self.db.doc_ids().values().cloned().collect()
    }

    pub fn common_file_prefix(&self) -> String {
        // TODO: Perfect candite for caching
        self.uris()
            .into_iter()
            .map(|it| it.path().to_string())
            .reduce(|acc, next| {
                acc.chars()
                    .zip(next.chars())
                    .take_while(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect::<String>()
            })
            .unwrap_or_default()
            .tap(|it| log::debug!("Common file name prefix: `{it}``"))
    }

    pub fn text(&self, uri: Uri) -> Result<String, DocumentNotFound> {
        if let Some(id) = self.db.doc_ids().get_id(&uri) {
            Ok(self.db.document(id).text.clone())
        } else {
            Err(DocumentNotFound(uri))
        }
    }

    pub fn edit<S: AsRef<str> + Into<String>>(&mut self, uri: Uri, edits: Vec<DocumentEdit<S>>) {
        // Ensure the document is registered.
        let id: Option<DocId> = self.db.doc_ids().get_id(&uri).clone();
        let id: DocId = id.unwrap_or_else(|| {
            self.db.modify_docs(|docs| docs.insert(uri.clone()));
            self.db.doc_ids().get_id(&uri).unwrap()
        });
        // Now actually modify it.
        self.db.modify_document(
            id,
            || InkDocument::new_empty(self.enc),
            |doc| doc.edit(edits),
        );
    }

    pub fn forget(&mut self, uri: Uri) -> Result<(), DocumentNotFound> {
        let removed = self.db.modify_docs(|it| it.remove(&uri));
        if removed {
            Ok(())
        } else {
            Err(DocumentNotFound(uri))
        }
    }

    /// Return a document symbol for this `uri`. Error on unknown document
    pub fn document_symbols(&self, uri: Uri) -> Result<Option<DocumentSymbol>, DocumentNotFound> {
        if let Some(id) = self.db.doc_ids().get_id(&uri) {
            Ok(self.db.document_symbols(id).clone())
        } else {
            Err(DocumentNotFound(uri))
        }
    }

    pub fn workspace_symbols(&mut self, query: String) -> Vec<WorkspaceSymbol> {
        let query = query.to_lowercase();
        let should_filter = !query.is_empty();
        self.db
            .doc_ids()
            .ids()
            .flat_map(|uri| {
                if let Some(syms) = &*self.db.workspace_symbols(uri) {
                    syms.iter()
                        .filter(|sym| should_filter && sym.name.to_lowercase().contains(&query))
                        .cloned()
                        .collect()
                } else {
                    Vec::new()
                }
            })
            .collect()
    }

    pub fn completions(
        &mut self,
        uri: Uri,
        position: Position,
    ) -> Result<Option<Vec<CompletionItem>>, DocumentNotFound> {
        todo!()
    }

    pub fn goto_definition(
        &self,
        uri: Uri,
        pos: Position,
    ) -> Result<Vec<lsp_types::Location>, GotoDefinitionError> {
        let docs = self.db.doc_ids();
        let Some(docid) = docs.get_id(&uri) else {
            return Err(DocumentNotFound(uri).into());
        };
        let doc = self.db.document(docid);

        let Some(search_terms) = doc.usage_at(pos) else {
            return Ok(Vec::new());
        };

        let ws_names = self.db.workspace_names();
        let mut result = Vec::new();

        for term in search_terms {
            let Some(metas) = ws_names.get(term) else {
                continue;
            };

            let (locals, globals): (Vec<&Meta>, Vec<&Meta>) = metas
                .iter()
                .filter(|it| it.visible_at(docid, pos))
                .partition(|it| it.extent.is_some());

            // Find "most local" thing.
            let local = locals.into_iter().min_by(|a, b| a.cmp_extent(b));
            if let Some(def) = local {
                result.push(lsp_types::Location::new(docs[def.file].clone(), def.site));
            } else {
                // We "allow" ambiguity for globals, since we can't know which definition the user meant
                // (there'll be an error message and they'll have to fix it).
                result.extend(
                    globals
                        .into_iter()
                        .map(|it| lsp_types::Location::new(docs[it.file].clone(), it.site)),
                );
            }
        }

        return Ok(result);
    }

    pub fn goto_references(
        &self,
        from_uri: &Uri,
        from_position: Position,
    ) -> Result<Vec<lsp_types::Location>, GotoDefinitionError> {
        todo!()
    }

    #[cfg(test)]
    fn to_ts_range(&self, uri: Uri, loc: lsp_types::Range) -> tree_sitter::Range {
        // only used in tests, so we'll crash liberally!
        let id = self
            .db
            .doc_ids()
            .get_id(&uri)
            .expect("don't call with with a non-existent document");
        self.db
            .document(id)
            .ts_range(loc)
            .expect("don't call this with an invalid location")
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
        use super::{new_state, set_content, State};
        use crate::{
            lsp::salsa::InkGetters,
            test_utils::{
                self,
                text_annotations::{Annotation, AnnotationScanner},
                Compact,
            },
        };
        use assert2::assert;
        use itertools::Itertools;
        use lsp_types::Uri;
        use std::{
            collections::{BTreeSet, HashMap},
            str::FromStr,
        };
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
                        "references-nothing" => {
                            if !arg.is_empty() {
                                panic!("references-nothing doesn't take arguments, found `{arg}`");
                            }
                            self.references
                                .push((uri, loc, ReferenceKind::Unresolved, Vec::new()));
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

            fn failed(&'a self, state: &'a State) -> bool {
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
                let db = &state.db;

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

                // kind of breaks encapsulation, but just to get going:
                // let visibilities_of_definitions = todo!();

                let mut messages = Vec::new();
                for (usage_uri, usage_ann, reference_kind, names) in &self.references {
                    let usage_lsp_position: lsp_types::Range = usage_ann.text_location.into();
                    let found_definitions = state
                        .goto_definition((*usage_uri).clone(), usage_lsp_position.start)
                        .expect("we should be within range");
                    if *reference_kind == ReferenceKind::Unresolved {
                        if !found_definitions.is_empty() {
                            for def in found_definitions {
                                let ts_range =
                                    state.to_ts_range((*usage_uri).clone(), usage_lsp_position);
                                let byte_range = ts_range.start_byte..ts_range.end_byte;
                                // have to use def_uri, because we can't return a reference to def.uri from this function.
                                let (def_uri, def_src) = inks.get_key_value(&def.uri).unwrap();
                                messages.push(
                                    annotate_snippets::Level::Error
                                        .title("Disallowed definition")
                                        .snippets([
                                            Self::annotation_to_snippet(
                                                &usage_uri,
                                                &usage_ann,
                                                "Expected this usage to be unresolved, …",
                                            ),
                                            annotate_snippets::Snippet::source(def_src)
                                                .origin(def_uri.path().as_str())
                                                // .line_start(def.range.start.line as usize + 1)
                                                .fold(true)
                                                .annotation(
                                                    annotate_snippets::Level::Error
                                                        .span(byte_range)
                                                        .label("… but it links to this definition"),
                                                ),
                                        ]),
                                );
                            }
                        }
                    } else {
                        for name in names {
                            let (def_uri, def_ann) = self.definitions.get(name).expect(
                                "we checked that every reference has a definition, see above",
                            );
                            let def_lsp_location = lsp_types::Location {
                                uri: (*def_uri).clone(),
                                range: def_ann.text_location.into(),
                            };
                            log::debug!(
                                "expecting location for definition {name}: {:#?}",
                                vec![Compact(def_lsp_location.clone())]
                            );
                            log::debug!(
                                "found definitions for {name}: {:#?}",
                                found_definitions.iter().cloned().map(Compact).collect_vec()
                            );

                            let does_reference = found_definitions.contains(&def_lsp_location);

                            if *reference_kind == ReferenceKind::Any && !does_reference {
                                let other_references = found_definitions
                                    .iter()
                                    .map(|other| {
                                        let docid =
                                            db.doc_ids().get_id(&other.uri).expect("must exist");
                                        let span = db
                                            .document(docid)
                                            .ts_range(other.range)
                                            .map(|it| it.start_byte..it.end_byte)
                                            .unwrap();
                                        let (uri, source) = inks.get_key_value(&other.uri).unwrap();
                                        annotate_snippets::Snippet::source(source)
                                            .origin(uri.path().as_str())
                                            .annotation(annotate_snippets::Level::Info.span(span))
                                    })
                                    .collect_vec();
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
                                        ])
                                        .footer(if other_references.is_empty() {
                                            annotate_snippets::Level::Error
                                                .title("But it links nowhere.")
                                        } else {
                                            annotate_snippets::Level::Error
                                                .title("But it only links to these locations:")
                                                .snippets(other_references)
                                        }),
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
                }

                if messages.is_empty() {
                    false
                } else {
                    let renderer = annotate_snippets::Renderer::styled();
                    for message in messages {
                        eprintln!("{}", renderer.render(message));
                    }
                    true
                }
            }

            fn annotation_to_snippet(
                file: &'a Uri,
                ann: &crate::test_utils::text_annotations::Annotation<'a>,
                label: &'a str,
            ) -> annotate_snippets::Snippet<'a> {
                annotate_snippets::Snippet::source(ann.full_text)
                    .origin(file.path().as_str())
                    // .line_start(ann.text_location.start.row as usize + 1)
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
            test_utils::setup_logging(log::LevelFilter::Trace);

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
            let annotation_scanner = AnnotationScanner::new();
            let mut checks = LinkCheck::default();

            for (uri, contents) in &ink_files {
                // parse actual links via tree-sitter (i.e. normally)
                set_content(&mut state, uri.clone(), contents);
                // parse expected links via annotations
                checks.add_annotations(&uri, annotation_scanner.scan(contents))
            }

            if checks.failed(&state) {
                panic!("Link check for {fs_location} failed.");
            }
        }
    }
}
