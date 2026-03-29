use std::{
    fmt::Display,
    io::{self, Write as _},
    path::Path,
    process::{Command, Stdio},
};

use derive_more::derive::{AsRef, Display, Error, From};
use ink_document::InkDocument;
use serde::Deserialize;
use similar::TextDiff;
use tree_traversal::TreeTraversal;
use type_sitter::Node as _;
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

#[derive(Debug, From, Error)]
pub struct TestFailures {
    failures: Vec<TestFailure>,
}

impl Display for TestFailures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.failures).finish()
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
    Other(serde_json::Value),
}

#[derive(Debug, serde::Deserialize, Display)]
#[display("{text}")]
pub struct Text {
    text: String,
}

pub fn run_tests_in_file(path_to_ink: &Path) -> Result<(), TestFailures> {
    let mut failures = Vec::new();
    let tests = extract_tests(path_to_ink).map_err(|it| vec![it])?;

    for test in tests {
        if let Err(failure) = run_test(path_to_ink, test) {
            failures.push(failure);
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.into())
    }
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

    let mut choices = run.input.iter();
    let mut actual = Actual::default();

    #[derive(Debug, Default, From, AsRef)]
    struct Actual(String);
    impl Actual {
        /// Wrapper around String::push_str so that we can echo what we record to the user.
        fn record(&mut self, format: impl AsRef<str>) {
            let format = format.as_ref();
            eprint!("{format}");
            self.0.push_str(format);
        }
    }

    while replies.end().is_err() {
        let reply = Communication::deserialize(&mut replies)?;
        use Communication::*;
        match reply {
            Compilation { .. } => {}
            Issues { issues } => {
                for issue in issues {
                    actual.record(format!("{issue}\n"));
                }
            }
            Text(text) => {
                actual.record(&format!("{text}")); // text should take care of newline itself
            }
            Choices { choices } => {
                actual.record(format!("\n"));
                for (n, choice) in choices.into_iter().enumerate() {
                    actual.record(format!("{}: {choice}\n", n + 1));
                }
                actual.record(format!("?> "));
            }
            NeedInput { need_input: true } => {
                let choice = choices.next().ok_or_else(|| TestFailure::TestError {
                    message: format!("Required a choice, but ran out."),
                })?;
                let command = format!("{choice}\n");
                actual.record(&command);
                send.write_all(command.as_bytes())?;
                send.flush()?;
            }
            NeedInput { need_input: false } => {
                break;
            }
            Other(value) => {
                eprintln!("Unexpected reply from the ink runner: {value}");
            }
        }
    }

    let diff = TextDiff::from_lines(actual.as_ref(), &run.expected_output)
        .unified_diff()
        .context_radius(3)
        .header(
            &format!("inkclecate -p {}", path_to_ink.to_string_lossy()),
            &format!("Test {}", run.name),
        )
        .to_string();

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

fn extract_tests(path_to_ink: &Path) -> Result<Vec<TestDescription>, TestFailure> {
    let string = std::fs::read_to_string(path_to_ink)?;
    let document = InkDocument::new(string, None);
    let mut tests = Vec::new();
    for comment in document.root().depth_first::<ink_syntax::BlockComment>() {
        let node_text = document.node_text(comment);
        let content = node_text
            .trim_start_matches("/*")
            .trim_start_matches("*")
            .trim_start()
            .trim_end_matches("*/")
            .trim_end_matches("*");

        static KEYWORD: &'static str = "TEST";
        if !content.starts_with(KEYWORD) {
            continue;
        }
        let (declaration, expectation) = content
            .split_once('\n')
            .ok_or_else(|| format!("Incorrect test sytax: {node_text}"))?;
        let name = &declaration[KEYWORD.len()..].trim();
        let line = comment.start_position().row + 1;
        let name = if name.is_empty() {
            format!("starting on line {line}")
        } else {
            format!("{name} starting on line {line}")
        };
        let input = expectation
            .lines()
            .filter_map(|it| it.starts_with("?> ").then(|| &it[3..]))
            .map(str::to_string)
            .collect();
        tests.push(TestDescription {
            name,
            input,
            expected_output: expectation.to_string(),
        });
    }
    if tests.is_empty() {
        Err(TestFailure::TestError {
            message: format!("No tests found in file {}", path_to_ink.to_string_lossy()),
        })
    } else {
        Ok(tests)
    }
}

#[derive(Debug, Default)]
pub struct TestDescription {
    name: String,
    input: Vec<String>,
    expected_output: String,
}
