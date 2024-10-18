use super::document::{DocumentEdit, InkDocument};
use line_index::WideEncoding;
use lsp_types::Uri;
use std::collections::HashMap;

pub(crate) struct State {
    wide_encoding: Option<WideEncoding>,
    documents: HashMap<Uri, InkDocument>,
}

impl State {
    pub fn new(wide_encoding: Option<WideEncoding>) -> Self {
        Self {
            documents: HashMap::new(),
            wide_encoding,
        }
    }

    pub fn edit(&mut self, uri: Uri, edits: Vec<DocumentEdit>) -> Result<(), String> {
        let entry = self
            .documents
            .entry(uri)
            .or_insert(InkDocument::new(String::new(), self.wide_encoding));
        entry.edit(edits);
        Ok(())
    }

    pub fn forget(&mut self, uri: Uri) -> Result<(), String> {
        match self.documents.remove(&uri) {
            Some(_) => Ok(()),
            None => Err(format!("Document {} not known", uri.path())),
        }
    }
}
