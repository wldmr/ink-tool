use std::error::Error;

pub mod fmt;
pub mod lsp;

pub mod ink_syntax {
    pub mod types {
        use type_sitter_lib as type_sitter;

        include!(concat!(env!("OUT_DIR"), "/type_sitter_ink.rs"));
    }

    mod visitor;
    pub use visitor::Visitor;
}

pub type AppResult<T> = Result<T, Box<dyn Error + Sync + Send>>;
