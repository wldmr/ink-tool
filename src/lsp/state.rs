use std::collections::{hash_map::Entry, HashMap};

use line_index::WideEncoding;
use lsp_types::Uri;

use super::document::{DocumentEdit, InkDocument};

pub(crate) struct ServerState {
    wide_encoding: Option<WideEncoding>,
    documents: HashMap<Uri, InkDocument>,
}

impl ServerState {
    pub fn new(wide_encoding: Option<WideEncoding>) -> Self {
        Self {
            documents: HashMap::new(),
            wide_encoding,
        }
    }
}

/// Document handling
impl ServerState {
    pub fn open_document(&mut self, uri: Uri, text: String) -> Result<(), String> {
        match self.documents.entry(uri) {
            Entry::Vacant(vacant) => {
                vacant.insert(InkDocument::new(text, self.wide_encoding));
                Ok(())
            }
            Entry::Occupied(old) => Err(format!("Document '{}' already open", old.key().as_str())),
        }
    }

    pub fn close_document(&mut self, uri: Uri) -> Result<(), String> {
        match self.documents.remove(&uri) {
            Some(_) => Ok(()),
            None => Err(format!("Document '{}' wasn't open", uri.as_str())),
        }
    }

    pub fn edit_document(&mut self, uri: Uri, edits: Vec<DocumentEdit>) -> Result<(), String> {
        let doc = self
            .documents
            .get_mut(&uri)
            .ok_or_else(|| format!("Document not open '{}'", uri.as_str()))?;
        doc.edit(edits);
        Ok(())
    }
}
