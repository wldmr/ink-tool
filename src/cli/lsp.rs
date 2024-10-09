use clap::Args;
use ink_tool::{lsp::run_lsp, AppResult};

#[derive(Args, Debug)]
pub(crate) struct LspOpt;

pub(crate) fn lsp(_opt: LspOpt) -> AppResult<()> {
    run_lsp()
}
