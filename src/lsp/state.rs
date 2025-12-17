use crate::lsp::{
    idset::Id,
    salsa::{self, DocId, InkGetters, InkSetters},
};
use derive_more::derive::{Display, Error};
use ink_document::{DocumentEdit, InkDocument, Meta};
use line_index::WideEncoding;
use lsp_types::{CompletionItem, DocumentSymbol, Position, Uri, WorkspaceSymbol};
use mini_milc::Cached;
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
pub(crate) enum GotoLocationError {
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
            Ok(self.db.document(id).text().to_owned())
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
    pub fn document_symbols(&self, uri: Uri) -> Result<Vec<DocumentSymbol>, DocumentNotFound> {
        if let Some(id) = self.db.doc_ids().get_id(&uri) {
            Ok(self.db.document_symbols(id).clone())
        } else {
            Err(DocumentNotFound(uri))
        }
    }

    pub fn workspace_symbols(&self, query: String) -> Vec<WorkspaceSymbol> {
        let query = query.trim().to_lowercase();
        let no_filter = query.is_empty();
        let mut syms = Vec::new();
        for id in self.db.doc_ids().ids() {
            for sym in self.db.workspace_symbols(id).iter() {
                if no_filter || sym.name.to_lowercase().contains(&query) {
                    syms.push(sym.clone());
                }
            }
        }
        syms
    }

    pub fn completions(
        &self,
        uri: Uri,
        position: Position,
    ) -> Result<Option<Vec<CompletionItem>>, DocumentNotFound> {
        let (doc, this_doc) = self.get_doc_and_id(uri)?;

        let Some(search_terms) = doc.usage_at(position) else {
            return Ok(Default::default());
        };

        let Some(longest) = search_terms.terms.into_iter().max_by_key(|it| it.len()) else {
            return Ok(Default::default());
        };
        log::debug!("Trying completion for '{longest}'.");

        let ws_names = self.db.workspace_names();
        use ink_document::ids::DefinitionInfo::*;
        use lsp_types::CompletionItemKind;
        Ok(Some(
            ws_names
                .iter()
                .filter(|(key, _)| key.contains(longest))
                .flat_map(|(key, metas)| metas.iter().map(move |meta| (key, meta)))
                .filter(|(_, (docid, meta))| {
                    meta.is_global() || (*docid == this_doc && meta.is_locally_visible_at(position))
                })
                .map(|(key, (_, meta))| lsp_types::CompletionItem {
                    label: key.clone(),
                    label_details: None,
                    kind: Some(match meta.id.info() {
                        ToplevelScope { .. } => CompletionItemKind::MODULE,
                        SubScope { .. } => CompletionItemKind::CLASS,
                        Function => CompletionItemKind::FUNCTION,
                        External => CompletionItemKind::INTERFACE,
                        Var => CompletionItemKind::VARIABLE, // TODO: Differentiate between VAR and CONST
                        Const => CompletionItemKind::CONSTANT, // TODO: Differentiate between VAR and CONST
                        List => CompletionItemKind::ENUM,
                        ListItem { .. } => CompletionItemKind::ENUM_MEMBER,
                        Label => CompletionItemKind::PROPERTY,
                        Param { .. } => CompletionItemKind::VARIABLE,
                        Temp => CompletionItemKind::UNIT,
                    }),
                    // TODO: Fetch actual definition
                    detail: Some(match meta.id.info() {
                        ToplevelScope { stitch, params } => {
                            format!(
                                "{} {key}{}",
                                if stitch { "=" } else { "==" },
                                if params { "(…)" } else { "" }
                            )
                        }
                        SubScope { params, .. } => {
                            format!("= {key}{}", if params { "(…)" } else { "" })
                        }
                        Function => format!("== function {key}(…)"),
                        External => format!("EXTERNAL {key}(…)"),
                        Var => format!("VAR {key} = …"),
                        Const => format!("CONST {key} = …"),
                        List => format!("LIST {key} = …"),
                        ListItem { .. } => format!("LIST … = … {key}, "),
                        Label => format!("({key}) // label"),
                        Param { .. } => format!("param // parameter"),
                        Temp => format!("~ temp {key} = …"),
                    }),
                    // TODO: Fetch actual docs
                    documentation: None,
                    deprecated: None,
                    preselect: None,
                    sort_text: None,
                    filter_text: None,
                    insert_text: None,
                    insert_text_format: None,
                    insert_text_mode: None,
                    text_edit: Some(lsp_types::CompletionTextEdit::Edit(lsp_types::TextEdit {
                        range: search_terms.range,
                        new_text: key.to_owned(),
                    })),
                    additional_text_edits: None,
                    command: None,
                    commit_characters: None,
                    data: None,
                    tags: None,
                })
                .collect(),
        ))
    }

    pub fn goto_definition(
        &self,
        uri: Uri,
        pos: Position,
    ) -> Result<Vec<lsp_types::Location>, GotoLocationError> {
        let (doc, this_doc) = self.get_doc_and_id(uri)?;
        let docs = self.db.doc_ids();

        let Some(search_terms) = doc.usage_at(pos) else {
            return Ok(Vec::new());
        };

        let ws_names = self.db.workspace_names();
        let mut result = Vec::new();

        for term in search_terms.terms {
            let Some(metas) = ws_names.get(term) else {
                continue;
            };

            let (globals, locals): (Vec<&(DocId, Meta)>, Vec<&(DocId, Meta)>) = metas
                .iter()
                .filter(|(docid, meta)| {
                    meta.is_global() || (*docid == this_doc && meta.is_locally_visible_at(pos))
                })
                .partition(|(_, meta)| meta.is_global());

            // Find "most local" thing.
            let local = locals.into_iter().min_by(|a, b| a.1.cmp_extent(&b.1));
            if let Some((file, def)) = local {
                result.push(lsp_types::Location::new(docs[*file].clone(), def.site));
            } else {
                // We "allow" ambiguity for globals, since we can't know which definition the user meant
                // (there'll be an error message and they'll have to fix it).
                result.extend(
                    globals
                        .into_iter()
                        .copied()
                        .map(|(file, def)| lsp_types::Location::new(docs[file].clone(), def.site)),
                );
            }
        }

        return Ok(result);
    }

    pub fn goto_references(
        &self,
        from_uri: Uri,
        from_position: Position,
    ) -> Result<Vec<lsp_types::Location>, GotoLocationError> {
        let (doc, this_doc) = self.get_doc_and_id(from_uri)?;
        let docs = self.db.doc_ids();
        let mut result = Default::default();
        let Some(def) = doc.definition_at(from_position) else {
            return Ok(result);
        };
        let doc_names = self.db.document_names(this_doc);
        let my_names = doc_names
            .iter()
            .filter(|(_name, meta)| meta.site == def.range);

        let ws_usages = self.db.workspace_usages();
        for (name, meta) in my_names {
            if let Some(usages) = ws_usages.get(name) {
                for (file, range) in usages.iter().copied() {
                    if meta.is_global()
                        || (this_doc == file && meta.is_locally_visible_at(range.start))
                    {
                        let uri = docs[file].clone();
                        result.push(lsp_types::Location { uri, range });
                    }
                }
            }
        }
        Ok(result)
    }

    #[cfg(test)]
    fn byte_range_of(&self, uri: &Uri, loc: lsp_types::Range) -> std::ops::Range<usize> {
        // only used in tests, so we'll crash liberally!
        let id = self
            .db
            .doc_ids()
            .get_id(uri)
            .expect("don't call with with a non-existent document");
        self.db.document(id).byte_range(loc)
    }

    fn get_doc_and_id(
        &self,
        uri: Uri,
    ) -> Result<(Cached<'_, salsa::Ops, InkDocument>, Id<Uri>), DocumentNotFound> {
        let Some(id) = self.db.doc_ids().get_id(&uri) else {
            return Err(DocumentNotFound(uri));
        };
        let doc = self.db.document(id);
        Ok((doc, id))
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
        use itertools::Itertools;
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
                    .sorted_unstable()
                    .collect::<Vec<_>>(),
                vec!["one", "some_var", "two"]
            );
        }
    }

    mod links {
        use super::{new_state, set_content, State};
        use crate::test_utils::{
            self,
            text_annotations::{Annotation, AnnotationScanner},
            Compact,
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
                            let (def_uri, def_ann) = self.definitions.get(name).expect(
                                "we checked that every reference has a definition, see above",
                            );
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
                            } else if matches!(reference_kind, ReferenceKind::Not) && does_reference
                            {
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
                ann: &crate::test_utils::text_annotations::Annotation<'a>,
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

            checks.check(&state);
        }
    }
}
