use clap::{command, Parser, Subcommand};
use ink_tool::AppResult;

pub(crate) mod fmt;
pub(crate) mod lsp;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Fmt(fmt::FmtOpt),
    Lsp(lsp::LspOpt),
}

pub(crate) fn main() -> AppResult<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Fmt(opt) => fmt::fmt(opt),
        Commands::Lsp(opt) => lsp::lsp(opt),
    }
}
