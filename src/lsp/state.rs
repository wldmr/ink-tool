use super::{
    document::{DocumentEdit, InkDocument},
    location::{self, rank_match, Location},
};
use line_index::WideEncoding;
use lsp_types::{
    CompletionItem, CompletionItemKind, DocumentSymbol, Position, Uri, WorkspaceSymbol,
};
use std::collections::HashMap;

pub(crate) struct State {
    wide_encoding: Option<WideEncoding>,
    documents: HashMap<Uri, InkDocument>,
    qualified_names: bool,
}

#[derive(Debug, thiserror::Error)]
#[error("Document not found: `{}`", .0.path())]
pub(crate) struct DocumentNotFound(pub(crate) Uri);

impl State {
    pub fn new(wide_encoding: Option<WideEncoding>, qualified_names: bool) -> Self {
        Self {
            documents: HashMap::new(),
            wide_encoding,
            qualified_names,
        }
    }

    pub fn edit(&mut self, uri: Uri, edits: Vec<DocumentEdit>) {
        let entry = self
            .documents
            .entry(uri)
            .or_insert_with(|| InkDocument::new(String::new(), self.wide_encoding));
        entry.edit(edits);
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
            .iter_mut()
            .filter_map(|(uri, doc)| doc.workspace_symbols(uri))
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
                let Some(specification) = doc.possible_completions(position) else {
                    return Ok(None);
                };
                // eprintln!("find {specification}");
                let completions = self.find_locations(specification).map(Into::into).collect();
                Ok(Some(completions))
            }
            None => Err(DocumentNotFound(uri)),
        }
    }

    fn find_locations(
        &self,
        spec: location::LocationThat,
    ) -> impl Iterator<Item = location::Location> {
        let uris: Vec<&Uri> = (&spec)
            .try_into()
            .unwrap_or_else(|_| self.documents.keys().collect());
        let mut locs = Vec::new();
        for uri in uris {
            let doc = self
                .documents
                .get(&uri)
                .expect("we mustn't get uris that we don't know");
            let doc_locs = doc
                .locations(&uri)
                .map(|loc| (rank_match(&spec, &loc), loc))
                .filter(|(rank, _)| *rank > 0);
            locs.extend(doc_locs);
        }
        locs.sort_unstable_by_key(|(rank, _)| *rank);
        locs.into_iter().map(|(_, location)| location)
    }
}

impl From<Location> for CompletionItem {
    fn from(value: Location) -> Self {
        let file = value.file.path().as_str();
        Self {
            label: value.name,
            detail: match value.namespace {
                Some(ns) => Some(format!("{}: {ns}", file)),
                None => Some(file.to_string()),
            },
            kind: Some(match value.kind {
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
}
