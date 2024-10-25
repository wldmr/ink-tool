use clap::Args;
use ink_tool::{lsp::run_lsp, AppResult};

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
/// Spec-compliant arguments for LSP communication channel
pub(crate) struct Communication {
    /// Communicate over stdin/stdout (currently the only option)
    #[arg(long)]
    stdio: bool,
}

#[derive(Args, Debug)]
pub(crate) struct LspOpt {
    #[command(flatten)]
    communication: Communication,
}

pub(crate) fn lsp(_opt: LspOpt) -> AppResult<()> {
    run_lsp()
}
