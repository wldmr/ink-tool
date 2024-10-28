use super::document::{DocumentEdit, InkDocument};
use line_index::WideEncoding;
use lsp_types::{DocumentSymbolResponse, Uri};
use std::collections::HashMap;

pub(crate) struct State {
    wide_encoding: Option<WideEncoding>,
    documents: HashMap<Uri, InkDocument>,
    qualified_names: bool,
}

impl State {
    pub fn new(wide_encoding: Option<WideEncoding>, qualified_names: bool) -> Self {
        Self {
            documents: HashMap::new(),
            wide_encoding,
            qualified_names,
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

    pub fn document_symbols(&self, uri: &Uri) -> Result<Option<DocumentSymbolResponse>, String> {
        if let Some(doc) = self.documents.get(uri) {
            Ok(doc.symbols(self.qualified_names))
        } else {
            Err(format!("Unknown document '{}'", uri.path().as_str()))
        }
    }
}
