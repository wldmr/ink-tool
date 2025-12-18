#![recursion_limit = "512"]

use std::error::Error;

pub mod fmt;
pub mod lsp;
#[cfg(test)]
pub(crate) mod test_utils;

pub type AppResult<T> = Result<T, Box<dyn Error + Sync + Send>>;
