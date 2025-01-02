use super::{
    document::{DocumentEdit, InkDocument},
    location::{
        self,
        specification::{rank_match, LocationThat},
        Location,
    },
    salsa::{doc_symbols, locations, workspace_symbols, DbImpl, Doc, Workspace},
};
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

#[derive(Debug, Clone, Display, Error)]
#[display("Document not found: `{}`", _0.path())]
pub(crate) struct DocumentNotFound(#[error(not(source))] pub(crate) Uri);

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

    pub fn edit(&mut self, uri: Uri, edits: Vec<DocumentEdit>) {
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
}
