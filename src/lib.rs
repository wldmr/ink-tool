use std::error::Error;

pub mod fmt;
pub mod ink_syntax;
pub mod lsp;

pub type AppResult<T> = Result<T, Box<dyn Error + Sync + Send>>;
