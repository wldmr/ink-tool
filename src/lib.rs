use std::error::Error;

pub mod fmt;
pub mod ink_syntax;
pub mod lsp;
#[cfg(test)]
pub(crate) mod test_utils;

pub type AppResult<T> = Result<T, Box<dyn Error + Sync + Send>>;
