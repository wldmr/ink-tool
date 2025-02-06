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
    let proj = directories::ProjectDirs::from("net", "wldmr", "ink-tool")
        .ok_or("Couldn't find required directories")?;
    let state_dir = proj.state_dir().unwrap_or_else(|| proj.cache_dir());
    std::fs::create_dir_all(state_dir)?;
    let log_file = state_dir.join("ink-tool.log");
    let log_file = log_file
        .to_str()
        .ok_or_else(|| format!("Couldn't create log file `{log_file:?}`"))?;
    ftail::Ftail::new()
        .single_file(log_file, true, log::LevelFilter::max())
        .max_file_size(5)
        .init()?;
    match args.command {
        Commands::Fmt(opt) => fmt::fmt(opt),
        Commands::Lsp(opt) => lsp::lsp(opt),
    }
}
