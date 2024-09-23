use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};

use clap::{command, Args, Parser, Subcommand};
use ink_fmt::format;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Fmt(FmtOpt),
}

#[derive(Args, Debug)]
/// Format ink files or STDIN
struct FmtOpt {
    /// The file(s) to format.
    ///
    /// Formatting happens in-place, unless --output is given.
    ///
    /// If omitted, take read from STDIN and output to STDOUT.
    ///
    /// Can be a single file or a directory. If directory format all Ink files found recursively.
    input: Option<PathBuf>,

    /// Where to output the formatted result. Overwrites any existing files.
    ///
    /// If formatting a single file, this is the path of the output file.
    /// If formatting a directory, this is the output directory for the formatted files
    /// (which will be created if necessary).
    /// The file structure will mirror the files in the INPUT directory.
    ///
    /// You wouldn't normally use this option. It's mainly for comparing the input to the output, i.e. for debugging.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Fmt(opts) => fmt(opts),
    }
}

fn fmt(opt: FmtOpt) -> std::io::Result<()> {
    match &opt.input {
        None => {
            let mut source = String::new();
            std::io::stdin().lock().read_to_string(&mut source)?;
            let formatted = format(source);
            std::io::stdout().lock().write_all(formatted.as_bytes())?
        }
        Some(indir) if indir.is_dir() => {
            let walk = walkdir::WalkDir::new(indir).follow_links(true);
            let entries: Result<Vec<_>, _> = walk
                .into_iter()
                .filter_entry(|it| {
                    it.path().is_dir() || it.path().extension().is_some_and(|ext| ext == "ink")
                })
                .collect();

            match entries {
                Ok(entries) => {
                    for entry in entries {
                        if entry.path().is_dir() {
                            continue;
                        }
                        let inpath = entry.into_path();
                        let outpath = match &opt.output {
                            Some(prefix) => {
                                if !prefix.exists() {
                                    std::fs::create_dir(prefix)?;
                                }
                                let unique_part = inpath
                                    .strip_prefix(indir)
                                    .expect("indir should be a prefix of inpath");
                                prefix.join(unique_part)
                            }
                            None => inpath.clone(),
                        };
                        fmt_single(&inpath, &outpath)?;
                    }
                }
                Err(e) => Err(e.into_io_error().expect("what else could it be?"))?,
            }
        }
        Some(inpath) => {
            if let Some(outpath) = &opt.output {
                fmt_single(&inpath, &outpath)?;
            } else {
                fmt_single(&inpath, &inpath)?;
            }
        }
    }
    Ok(())
}

fn fmt_single(input: &Path, output: &Path) -> std::io::Result<()> {
    eprintln!("Formatting {}", output.display());
    let source = std::fs::read_to_string(input)?;
    let formatted = format(source);
    std::fs::write(output, formatted)
}
