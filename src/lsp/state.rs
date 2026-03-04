use crate::lsp::{
    idset::Id,
    salsa::{self, DocId, InkGetters, InkSetters},
};
use derive_more::derive::{Display, Error};
use ink_document::{DocumentEdit, InkDocument};
use line_index::WideEncoding;
use lsp_types::{DocumentSymbol, Position, Uri, WorkspaceSymbol};
use mini_milc::Cached;
use tap::Tap as _;

mod completions;
mod goto_definition;
mod goto_references;
mod rename;

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
#[display("Could not go to position: {}", self)]
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

    pub fn text(&self, uri: &Uri) -> Result<String, DocumentNotFound> {
        if let Some(id) = self.db.doc_ids().get_id(uri) {
            Ok(self.db.document(id).full_text())
        } else {
            Err(DocumentNotFound(uri.clone()))
        }
    }

    pub fn is_open(&mut self, uri: &Uri) -> Result<bool, DocumentNotFound> {
        let Some(id) = self.db.doc_ids().get_id(&uri) else {
            return Err(DocumentNotFound(uri.clone()));
        };
        Ok(self.db.opened_docs().contains(&id))
    }

    pub fn open(&mut self, uri: Uri) {
        let id = self.get_or_new_docid(uri);
        self.db.modify_opened(|docs| docs.insert(id));
    }

    pub fn close(&mut self, uri: Uri) {
        let id = self.get_or_new_docid(uri);
        self.db.modify_opened(|docs| docs.remove(&id));
    }

    pub fn edit<'a, E: Into<DocumentEdit>>(&mut self, uri: Uri, edit: E) {
        self.edits(uri, [edit]);
    }

    pub fn edits<'a, E: Into<DocumentEdit>>(
        &mut self,
        uri: Uri,
        edits: impl IntoIterator<Item = E>,
    ) {
        let id = self.get_or_new_docid(uri);
        // Now actually modify it.
        self.db.modify_document(
            id,
            || InkDocument::new_empty(self.enc),
            |doc| doc.edits(edits),
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

    fn get_or_new_docid(&mut self, uri: Uri) -> Id<Uri> {
        let id: Option<DocId> = self.db.doc_ids().get_id(&uri).clone();
        let id: DocId = id.unwrap_or_else(|| {
            self.db.modify_docs(|docs| docs.insert(uri.clone()));
            self.db.doc_ids().get_id(&uri).unwrap()
        });
        id
    }

    fn get_doc_and_id(
        &self,
        uri: &Uri,
    ) -> Result<(Cached<'_, salsa::Ops, InkDocument>, Id<Uri>), DocumentNotFound> {
        let Some(id) = self.db.doc_ids().get_id(uri) else {
            return Err(DocumentNotFound(uri.clone()));
        };
        let doc = self.db.document(id);
        Ok((doc, id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(super) fn new_state() -> State {
        State::new(None, true)
    }

    impl State {
        pub(super) fn with_comment_separated_files(mut self, files: impl AsRef<str>) -> Self {
            for (uri, text) in comment_separated_files(files.as_ref()).unwrap() {
                self.edit(uri, text);
            }
            self
        }
    }

    pub(super) fn comment_separated_files(texts: &str) -> Result<Vec<(Uri, String)>, String> {
        static PREFIX: &'static str = "// file:";
        static SUFFIX: &'static str = ".ink";

        let mut current_uri = uri("main.ink");
        let mut text = String::new();
        let mut vec = Vec::new();
        for line in texts.trim().lines() {
            let line = line.trim();
            if line.starts_with(PREFIX) && line.ends_with(SUFFIX) {
                let Some((_, filename)) = line.split_once(PREFIX) else {
                    return Err(format!("Could not parse file name comment line : `{line}`"));
                };
                if !text.is_empty() {
                    let file_text = std::mem::take(&mut text);
                    vec.push((current_uri.clone(), file_text));
                }
                current_uri = uri(filename.trim());
            } else {
                text.push_str(line);
                text.push('\n');
            }
        }
        vec.push((current_uri, text)); // pick up the last stragglers
        Ok(vec)
    }

    pub(super) fn uri(name: &str) -> Uri {
        <Uri as std::str::FromStr>::from_str(&format!("file://tmp/{name}")).unwrap()
    }

    pub(super) fn text_with_caret(input: &str) -> (String, lsp_types::Position) {
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
}
