use std::collections::HashMap;

use lsp_types::Uri;

use super::document;

pub struct ServerState {
    pub(crate) parser: tree_sitter::Parser,
    pub(crate) documents: HashMap<Uri, document::Document>,
}

impl ServerState {
    pub fn new() -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_ink::LANGUAGE.into())
            .expect("If this fails, we can't recover.");
        Self {
            parser,
            documents: HashMap::new(),
        }
    }
}
