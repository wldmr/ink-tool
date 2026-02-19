use clap::{command, Parser, Subcommand};
use ink_tool::AppResult;
use std::path::PathBuf;

pub(crate) mod fmt;
pub(crate) mod lsp;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// log to this file instead of stderr
    #[arg(long)]
    log_file: Option<PathBuf>,
    /// minimum log level
    #[arg(long, value_enum, default_value_t)]
    log_level: LogLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            LogLevel::Debug
        } else {
            LogLevel::Info
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    Fmt(fmt::FmtOpt),
    Lsp(lsp::LspOpt),
}

pub(crate) fn main() -> AppResult<()> {
    let args = Cli::parse();
    setup_logging(&args)?;
    match args.command {
        Commands::Fmt(opt) => fmt::fmt(opt),
        Commands::Lsp(opt) => lsp::lsp(opt),
    }
}

fn setup_logging(args: &Cli) -> AppResult<()> {
    let level = match args.log_level {
        LogLevel::Off => log::LevelFilter::Off,
        LogLevel::Error => log::LevelFilter::Error,
        LogLevel::Warn => log::LevelFilter::Warn,
        LogLevel::Info => log::LevelFilter::Info,
        LogLevel::Debug => log::LevelFilter::Debug,
        LogLevel::Trace => log::LevelFilter::Trace,
    };
    let mut dispatch = fern::Dispatch::new()
        .level(log::LevelFilter::Error)
        .level_for("ink_tool", level)
        .level_for("ink_document", level)
        .level_for("ink_syntax", level)
        .level_for("tree_traversal", level)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} {} {}",
                humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        });

    if let Some(ref log_file) = args.log_file {
        dispatch = dispatch.chain(fern::log_file(log_file)?)
    } else {
        dispatch = dispatch.chain(std::io::stderr());
    }

    dispatch.apply()?;
    Ok(())
}
