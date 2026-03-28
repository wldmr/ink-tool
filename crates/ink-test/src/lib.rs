use std::{
    fmt::Display,
    io,
    path::Path,
    process::{Command, Stdio},
};

use derive_more::derive::{Error, From};
use serde::Deserialize;

#[derive(Debug, From, Error)]
pub enum TestFailure {
    IOError(io::Error),
    Serialization(serde_json::Error),
    UnexpectedOutput { expected: String, actual: String },
}

impl Display for TestFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestFailure::IOError(error) => error.fmt(f),
            TestFailure::Serialization(error) => error.fmt(f),

            TestFailure::UnexpectedOutput { expected, actual } => {
                let diff = similar::udiff::unified_diff(
                    similar::Algorithm::Myers,
                    &expected,
                    &actual,
                    2,
                    None,
                );
                f.write_str(&diff)
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged, rename_all_fields(deserialize = "kebab-case"))]
pub enum Communication {
    Compilation { compile_success: bool },
    Issues { issues: Vec<String> },
}

pub fn run_test(path_to_ink: &Path, run: TestDescription) -> Result<(), TestFailure> {
    let mut player = Command::new("inklecate")
        .arg("-pj")
        .arg(path_to_ink)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    let mut send = player
        .stdin
        .take()
        .ok_or(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "Could not connect to inklecate's stdin",
        ))
        .map(io::BufWriter::new)?;

    let recv = player
        .stdout
        .take()
        .ok_or(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "Could not connect to inklecate's stdout",
        ))
        .map(io::BufReader::new)?;

    let mut replies = serde_json::Deserializer::from_reader(recv);

    while replies.end().is_err() {
        let reply = Communication::deserialize(&mut replies)?;
        println!("{reply:?}");
    }

    Ok(())
}

fn extract_tests(path_to_ink: &Path) -> Vec<TestDescription> {
    Vec::new()
}

#[derive(Debug, Default)]
pub struct TestDescription {
    choices: Vec<u8>,
    expected_output: String,
}
