use std::{
    fmt::Display,
    io::{self, Write as _},
    path::Path,
    process::{Command, Stdio},
};

use derive_more::derive::{Display, Error, From};
use serde::Deserialize;
use yansi::Paint;

#[derive(Debug, From, Error)]
pub enum TestFailure {
    IOError(io::Error),
    Serialization(serde_json::Error),
    TestError { message: String },
}

impl Display for TestFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestFailure::IOError(error) => error.fmt(f),
            TestFailure::Serialization(error) => error.fmt(f),
            TestFailure::TestError { message: msg } => f.write_str(&msg),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Communication {
    Compilation {
        #[serde(rename = "compile-success")]
        compile_success: bool,
    },
    Issues {
        issues: Vec<String>,
    },
    Text(Text),
    Choices {
        choices: Vec<Text>,
    },
    NeedInput {
        #[serde(rename = "needInput")]
        need_input: bool,
    },
}

#[derive(Debug, serde::Deserialize, Display)]
#[display("{text}")]
pub struct Text {
    text: String,
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

    let mut choices = run.choices.iter().copied();
    let mut actual = String::new();

    while replies.end().is_err() {
        let reply = Communication::deserialize(&mut replies)?;
        use Communication::*;
        match reply {
            Compilation { .. } => {}
            Issues { issues } => {
                for issue in issues {
                    actual.push_str(&format!("{issue}\n"));
                }
            }
            Text(text) => {
                actual.push_str(&format!("{text}")); // text should take care of newline itself
            }
            Choices { choices } => {
                actual.push_str(&format!("\n"));
                for (n, choice) in choices.into_iter().enumerate() {
                    actual.push_str(&format!("{}. {choice}\n", n + 1));
                }
                actual.push_str(&format!("?> "));
            }
            NeedInput { need_input: true } => {
                let choice = choices.next().ok_or_else(|| TestFailure::TestError {
                    message: format!("Required a choice, but ran out."),
                })?;
                let command = &format!("{choice}\n");
                actual.push_str(&format!("{command}"));
                send.write_all(command.as_bytes())?;
                send.flush()?;
            }
            NeedInput { need_input: false } => {
                break;
            }
        }
    }

    let diff = similar::udiff::unified_diff(
        similar::Algorithm::Myers,
        &actual,
        &run.expected_output,
        2,
        Some((&path_to_ink.to_string_lossy(), "Expected")),
    );

    if diff.trim().is_empty() {
        Ok(())
    } else {
        for line in diff.lines() {
            let line = if line.starts_with("+") {
                line.green()
            } else if line.starts_with("-") {
                line.red()
            } else if line.starts_with("@@") {
                line.bright_blue()
            } else {
                line.primary()
            };
            eprintln!("{line}");
        }
        Err(TestFailure::TestError {
            message: "Unexpected output".to_string(),
        })
    }
}

fn extract_tests(path_to_ink: &Path) -> Vec<TestDescription> {
    Vec::new()
}

#[derive(Debug, Default)]
pub struct TestDescription {
    choices: Vec<u8>,
    expected_output: String,
}
