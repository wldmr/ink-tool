use super::{
    document::{DocumentEdit, InkDocument},
    links::Links,
    location::{
        self,
        specification::{rank_match, LocationThat},
        Location,
    },
};
use derive_more::derive::{Display, Error};
use line_index::WideEncoding;
use lsp_types::{
    CompletionItem, CompletionItemKind, DocumentSymbol, Position, Uri, WorkspaceSymbol,
};
use std::collections::HashMap;

pub(crate) struct State {
    wide_encoding: Option<WideEncoding>,
    documents: HashMap<Uri, InkDocument>,
    links: Links, // Sure would be nice for this to be reactive
    qualified_names: bool,
}

#[derive(Debug, Display, Error)]
#[display("Document not found: `{}`", _0.path())]
pub(crate) struct DocumentNotFound(#[error(not(source))] pub(crate) Uri);

impl State {
    pub fn new(wide_encoding: Option<WideEncoding>, qualified_names: bool) -> Self {
        Self {
            documents: HashMap::new(),
            links: Links::new(),
            wide_encoding,
            qualified_names,
        }
    }

    pub fn edit(&mut self, uri: Uri, edits: Vec<DocumentEdit>) {
        if !self.documents.contains_key(&uri) {
            self.documents.insert(
                uri.clone(),
                InkDocument::new(uri.clone(), String::new(), self.wide_encoding),
            );
        }
        let doc = self
            .documents
            .get_mut(&uri)
            .expect("we just made sure it exists");
        let new_locations = doc.edit(edits);
        for loc in &new_locations {
            self.links.remove_any(&loc.file_range);
        }
        for _loc in new_locations {
            // todo!()
        }
    }

    pub fn forget(&mut self, uri: Uri) -> Result<(), DocumentNotFound> {
        match self.documents.remove(&uri) {
            Some(_) => Ok(()),
            None => Err(DocumentNotFound(uri)),
        }
    }

    /// Return a document symbol for this `uri`. Error on unknown document
    pub fn document_symbols(
        &mut self,
        uri: Uri,
    ) -> Result<Option<DocumentSymbol>, DocumentNotFound> {
        match self.documents.get_mut(&uri) {
            Some(doc) => Ok(doc.symbols(self.qualified_names)),
            None => Err(DocumentNotFound(uri)),
        }
    }

    pub fn workspace_symbols(&mut self, query: String) -> Vec<WorkspaceSymbol> {
        let symbols = self
            .documents
            .values_mut() // mut because of caching. feels weird, but at least it's honest.
            .filter_map(InkDocument::workspace_symbols)
            .flatten();
        if query.is_empty() {
            symbols.collect()
        } else {
            let query = query.to_lowercase();
            symbols
                .filter(|sym| sym.name.to_lowercase().contains(&query))
                .collect()
        }
    }
    pub fn completions(
        &mut self,
        uri: Uri,
        position: Position,
    ) -> Result<Option<Vec<CompletionItem>>, DocumentNotFound> {
        match self.documents.get_mut(&uri) {
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
        let uris: Vec<Uri> = location::specification::extract_uris(&spec)
            .unwrap_or_else(|| self.documents.keys().cloned().collect());
        let mut locs = Vec::new();
        for uri in uris {
            let doc = self
                .documents
                .get(&uri)
                .expect("we mustn't get uris that we don't know");
            let doc_locs = doc
                .locations()
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
        use super::{set_content, uri};
        use crate::lsp::state::{self, tests::text_with_caret};
        use pretty_assertions::assert_eq;

        #[test]
        fn state() {
            let mut state = state::State::new(None, true);
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
            set_content(&mut state, uri("test.ink"), contents);
            let doc = state.documents.get(&uri("test.ink")).unwrap();
            eprintln!(
                "completions: {:?}",
                doc.possible_completions(caret).unwrap()
            );
            let completions = state.completions(uri("test.ink"), caret).unwrap().unwrap();
            assert_eq!(
                completions
                    .into_iter()
                    .map(|it| it.label)
                    .collect::<Vec<_>>(),
                vec!["some_var", "one", "two"]
            );
        }
    }
}
