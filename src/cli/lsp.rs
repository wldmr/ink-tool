use std::io;

use clap::Args;
use ink_tool::{lsp::run_lsp, AppResult};

use super::main;

#[derive(Args, Debug)]
pub(crate) struct LspOpt;

pub(crate) fn lsp(opt: LspOpt) -> AppResult<()> {
    run_lsp()
}
