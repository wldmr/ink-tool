use super::{
    document::{DocumentEdit, InkDocument},
    location::{
        self,
        specification::{rank_match, LocationThat},
        Location,
    },
    salsa::{
        common_file_prefix, doc_symbols, locations, uris, workspace_symbols, DbImpl, Doc, Workspace,
    },
};
use crate::{
    ink_syntax::{
        self,
        types::{AllNamed, Usages},
    },
    lsp::salsa::{usages_in_block, usages_in_doc, GetNodeError, NodeSalsa, SalsaUsage},
};
use derive_more::derive::{Display, Error};
use itertools::Itertools;
use line_index::WideEncoding;
use lsp_types::{
    CompletionItem, CompletionItemKind, DocumentSymbol, Position, Uri, WorkspaceSymbol,
};
use salsa::Setter as _;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};
use tap::Pipe as _;
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
#[display("Not a valid position: {}:{}", _0.line, _0.character)]
pub(crate) struct InvalidPosition(#[error(not(source))] pub(crate) Position);

#[derive(Debug, Clone, Display, Error, derive_more::From)]
#[display("Could not go to position")]
pub(crate) enum GotoDefinitionError {
    DocumentNotFound(DocumentNotFound),
    PositionOutOfBounds(InvalidPosition),
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

    pub fn uris(&self) -> Vec<&Uri> {
        uris(&self.db, self.workspace)
    }

    pub fn common_file_prefix(&self) -> String {
        common_file_prefix(&self.db, self.workspace)
    }

    pub fn text(&self, uri: Uri) -> Result<&String, DocumentNotFound> {
        self.workspace
            .docs(&self.db)
            .get(&uri)
            .map(|it| it.text(&self.db))
            .ok_or_else(|| DocumentNotFound(uri))
    }

    pub fn edit<S: AsRef<str> + Into<String>>(&mut self, uri: Uri, edits: Vec<DocumentEdit<S>>) {
        let doc_id = DocId::from(&uri);
        let parser = self.parsers.entry(doc_id).or_insert_with(|| {
            let parser = InkDocument::new(uri.clone(), String::new(), self.workspace.enc(&self.db));
            let mut ws_docs = self.workspace.docs(&self.db).clone();
            ws_docs.insert(
                uri.clone(),
                Doc::new(
                    &self.db,
                    self.workspace,
                    uri.clone(),
                    parser.text.clone(),
                    parser.tree.clone(),
                    parser.lines.clone(),
                    self.workspace.enc(&self.db),
                ),
            );
            self.workspace.set_docs(&mut self.db).to(ws_docs);
            parser
        });
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
                // log::debug!("find {specification}");
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
        from_uri: &Uri,
        from_position: Position,
    ) -> Result<Vec<lsp_types::Location>, GotoDefinitionError> {
        let Some(doc) = self.workspace.docs(&self.db).get(&from_uri) else {
            return Err(DocumentNotFound(from_uri.clone()).into());
        };
        let db = &self.db;
        let (_point, cursor_offset) = doc.ts_point(db, from_position)?;
        // let start_node: Usages<'_> = match doc.get_node_at(&self.db, from_position) {
        //     Ok(usage) => usage,
        //     Err(GetNodeError::InvalidType) => {
        //         log::trace!("Not even a usage, what is wrong with you!");
        //         // not an error, just no definitions to be found
        //         return Ok(Vec::new());
        //     }
        //     Err(GetNodeError::PositionOutOfBounds(e)) => return Err(e.into()),
        // };
        // log::debug!(
        //     "Searching for links for node under cursor: `{}` {:?}",
        //     &doc.text(&self.db)[start_node.byte_range()],
        //     start_node.raw(),
        // );
        usages_in_doc(db, *doc)
            .iter()
            .inspect(|it| log::debug!("found usage {:?}", it.node(db).into_raw()))
            .filter(|usage| usage.node(db).raw().byte_range().contains(&cursor_offset))
            .inspect(|_| log::debug!("which is under cursor"))
            .flat_map(|usage| usage.definition(db))
            .inspect(|def| {
                log::debug!(
                    "with definition {:?} in {}",
                    def.node(db),
                    def.uri(db).path()
                )
            })
            .map(|it| lsp_types::Location {
                uri: it.uri(&self.db).clone(),
                range: doc.lsp_range(db, it.name_node(db).range()),
            })
            .collect_vec()
            .pipe(Ok)
    }

    pub fn goto_references(
        &self,
        from_uri: &Uri,
        from_position: Position,
    ) -> Result<Vec<lsp_types::Location>, GotoDefinitionError> {
        let Some(doc) = self.workspace.docs(&self.db).get(&from_uri) else {
            return Err(DocumentNotFound(from_uri.clone()).into());
        };
        let mut target_node = doc.named_cst_node_at(&self.db, from_position)?;
        log::debug!("Searching for references to target node {target_node:?}");
        if target_node.kind() != ink_syntax::types::Identifier::KIND {
            log::debug!("Not even an identifier, what is wrong with you!");
            return Ok(Vec::new());
        }

        if let Some(parent) = target_node.parent() {
            if parent.kind() == ink_syntax::types::QualifiedName::KIND {
                target_node = parent.downcast::<AllNamed>().unwrap();
                log::debug!("Target is actually a qualified name! {target_node:?}");
            }
        };

        todo!()
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

    #[cfg(test)]
    fn to_ts_range(&self, from_uri: &Uri, loc: lsp_types::Range) -> tree_sitter::Range {
        // only used in tests, so we'll crash liberally
        self.workspace
            .docs(&self.db)
            .get(from_uri)
            .expect("why would you call this with a made-up URI?")
            .ts_range(&self.db, loc)
            .expect("don't call this with an invalid location")
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
            log::debug!(
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

        use super::{new_state, set_content, State};
        use crate::{
            lsp::salsa::{workspace_definitions_by_name, NodeSalsa},
            test_utils::{
                self,
                text_annotations::{Annotation, AnnotationScanner},
                Compact,
            },
        };
        use assert2::assert;
        use itertools::Itertools;
        use lsp_types::Uri;
        use test_case::test_case;
        use type_sitter_lib::Node;

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
                let visibilities_of_definitions =
                    workspace_definitions_by_name(db, state.workspace)
                        .into_iter()
                        .flat_map(|(name, visibilities)| {
                            visibilities.into_iter().map(move |(vis, def)| {
                                let def_lsp_location = lsp_types::Location {
                                    uri: def.uri(db).clone(),
                                    range: def.lsp_range(db),
                                };
                                (def_lsp_location, (vis, name.clone()))
                            })
                        })
                        .into_group_map();

                let mut messages = Vec::new();
                for (usage_uri, usage_ann, reference_kind, names) in &self.references {
                    let usage_lsp_position: lsp_types::Range = usage_ann.text_location.into();
                    let found_definitions = state
                        .goto_definition(&usage_uri, usage_lsp_position.start)
                        .expect("we should be within range");
                    if *reference_kind == ReferenceKind::Unresolved {
                        if !found_definitions.is_empty() {
                            for def in found_definitions {
                                let ts_range = state.to_ts_range(&usage_uri, usage_lsp_position);
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
                                        let span = state.workspace.docs(db)[&other.uri]
                                            .ts_range(db, other.range)
                                            .map(|it| it.start_byte..it.end_byte)
                                            .unwrap();
                                        let (uri, source) = inks.get_key_value(&other.uri).unwrap();
                                        annotate_snippets::Snippet::source(source)
                                            .origin(uri.path().as_str())
                                            .annotation(annotate_snippets::Level::Info.span(span))
                                    })
                                    .collect_vec();
                                let vis_of_target = if let Some((_loc, vec)) =
                                    visibilities_of_definitions.get_key_value(&def_lsp_location)
                                {
                                    vec.iter()
                                        .map(|(vis, name)| {
                                            use crate::lsp::salsa::Visibility::*;
                                            match vis {
                                                Global => {
                                                    let source = &def_ann.full_text;
                                                    let span = def_ann.text_location.byte_range();
                                                    annotate_snippets::Snippet::source(source)
                                                        .origin(def_uri.path().as_str())
                                                        .annotations([
                                                            annotate_snippets::Level::Info
                                                                .span(span.clone())
                                                                .label("has global name"),
                                                            annotate_snippets::Level::Info
                                                                .span(span)
                                                                .label(name),
                                                        ])
                                                }
                                                Inside(scope) => {
                                                    let source = scope.cst(db).text(db);
                                                    let uri = scope.uri(db);
                                                    let span = scope.node(db).byte_range();
                                                    annotate_snippets::Snippet::source(source)
                                                        .origin(uri.path().as_str())
                                                        .annotations([
                                                            annotate_snippets::Level::Info
                                                                .span(span.clone())
                                                                .label("has local name here"),
                                                            annotate_snippets::Level::Info
                                                                .span(span)
                                                                .label(name),
                                                        ])
                                                }
                                                Temp(scope) => {
                                                    let source = scope.cst(db).text(db);
                                                    let uri = scope.uri(db);
                                                    let span = scope.node(db).byte_range();
                                                    annotate_snippets::Snippet::source(source)
                                                        .origin(uri.path().as_str())
                                                        .annotations([
                                                            annotate_snippets::Level::Info
                                                                .span(span.clone())
                                                                .label("has temp name here"),
                                                            annotate_snippets::Level::Info
                                                                .span(span)
                                                                .label(name),
                                                        ])
                                                }
                                            }
                                        })
                                        .collect_vec()
                                } else {
                                    Vec::new()
                                };
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
                                        })
                                        .footer(if vis_of_target.is_empty() {
                                            annotate_snippets::Level::Error
                                                .title("The intended target is not visible anywhere.")
                                        } else {
                                            annotate_snippets::Level::Note
                                                .title("The intended target has the following visibilities:")
                                                .snippets(vis_of_target)
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
                    eprintln!(
                        "all names: {:#?}",
                        workspace_definitions_by_name(db, state.workspace)
                    );
                    // eprintln!(
                    //     "all names: {:#?}",
                    //     workspace_definitions_by_name(db, state.workspace)
                    //         .iter()
                    //         .flat_map(move |(name, defs)| defs
                    //             .into_iter()
                    //             .map(|def| (name.clone(), def)))
                    //         .map(|(name, def)| format!(
                    //             "{name} ({})",
                    //             match def.0 {
                    //                 crate::lsp::salsa::Visibility::Global => "global".to_string(),
                    //                 crate::lsp::salsa::Visibility::Inside(scope) => format!(
                    //                     "local in {}",
                    //                     scope.global_name(db).unwrap_or_else(|| def
                    //                         .1
                    //                         .uri(db)
                    //                         .path()
                    //                         .to_string())
                    //                 ),
                    //                 crate::lsp::salsa::Visibility::Temp(scope) => format!(
                    //                     "temp in {}",
                    //                     scope.global_name(db).unwrap_or_else(|| def
                    //                         .1
                    //                         .uri(db)
                    //                         .path()
                    //                         .to_string())
                    //                 ),
                    //             }
                    //         ))
                    //         .sorted_unstable()
                    //         .collect_vec()
                    // );
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
                checks.add_annotations(uri, annotation_scanner.scan(contents))
            }

            if checks.failed(&state) {
                panic!("Link check for {fs_location} failed.");
            }
        }
    }
}
