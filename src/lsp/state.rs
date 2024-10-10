use std::collections::HashMap;

use line_index::WideEncoding;
use lsp_types::Uri;

use super::document::{Document, DocumentEdit};

pub(crate) struct ServerState {
    wide_encoding: Option<WideEncoding>,
    parser: tree_sitter::Parser,
    documents: HashMap<Uri, Document>,
}

impl ServerState {
    pub fn new(wide_encoding: Option<WideEncoding>) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_ink::LANGUAGE.into())
            .expect("If this fails, we can't recover.");
        Self {
            parser,
            documents: HashMap::new(),
            wide_encoding,
        }
    }
}

/// Document handling
impl ServerState {
    pub fn open_document(&mut self, uri: Uri, text: String) -> Result<(), String> {
        let new = Document::new(text, self.wide_encoding, &mut self.parser);
        match self.documents.insert(uri, new) {
            Some(_) => Err("Document already open".to_owned()),
            None => Ok(()),
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
        doc.edit(edits, &mut self.parser);
        Ok(())
    }
}
