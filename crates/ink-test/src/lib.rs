use std::{
    ffi::OsStr,
    fmt::Display,
    io::{self, Read, Write as _},
    iter::{chain, once},
    path::Path,
    process::{Command, Stdio},
    string::FromUtf8Error,
};

use derive_more::derive::{Error, From};
use ink_document::InkDocument;
use similar::TextDiff;
use tree_traversal::TreeTraversal;
use type_sitter::Node as _;
use yansi::Paint;

#[derive(Debug, From, Error)]
pub enum TestFailure {
    IOError(io::Error),
    Utf8(FromUtf8Error),
    TestError { message: String },
}

impl Display for TestFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestFailure::IOError(error) => error.fmt(f),
            TestFailure::Utf8(error) => error.fmt(f),
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
    let mut command = Command::new("inklecate");
    command
        .arg("-p")
        .arg(path_to_ink)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped());

    let mut player = command.spawn()?;

    let mut send = player
        .stdin
        .take()
        .ok_or(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "Could not connect to inklecate's stdin",
        ))
        .map(io::BufWriter::new)?;

    let mut recv = player
        .stdout
        .take()
        .ok_or(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "Could not connect to inklecate's stdout",
        ))
        .map(io::BufReader::new)?;

    let mut choices = run.input.iter();

    let mut buf = [0u8; 1024];
    let mut actual = Vec::new();

    loop {
        match recv.read(&mut buf)? {
            0 => break,
            n => {
                static PROMPT: &[u8] = &[b'?', b'>', b' '];
                actual.extend_from_slice(&buf[..n]);
                if actual.ends_with(PROMPT) {
                    let choice = choices
                        .next()
                        .ok_or_else(|| "Required a choice, but ran out.".to_string())?;
                    let command = format!("{choice}\n").into_bytes();
                    actual.extend_from_slice(&command);
                    send.write_all(&command)?;
                    send.flush()?;
                }
            }
        }
    }

    let actual = String::from_utf8(actual)?;

    let command_header = chain(once(command.get_program()), command.get_args())
        .collect::<Vec<_>>()
        .join(OsStr::new(" "))
        .to_string_lossy()
        .to_string();
    let expectation_header = format!("Test {}", run.name);

    let diff = TextDiff::from_lines(&actual, &run.expected_output)
        .unified_diff()
        .context_radius(3)
        .header(&command_header, &expectation_header)
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
        Err("Unexpected output".to_string().into())
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
            format!("\"{name}\" starting on line {line}")
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
