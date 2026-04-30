use std::{
    ffi::OsStr,
    fmt::{Display, Write as _},
    io::{self, Read, Write as _},
    iter::{chain, once},
    path::Path,
    process::{Command, Stdio},
    string::FromUtf8Error,
    time::{Duration, Instant},
};

use derive_more::derive::{Debug, Error, From};
use ink_document::InkDocument;
use similar::TextDiff;
use tree_traversal::TreeTraversal;
use type_sitter::Node as _;
use yansi::Paint;

#[derive(Debug, Error)]
pub enum TestFailure {
    #[debug("{_0}")]
    IOError(io::Error),
    #[debug("{_0}")]
    Utf8(FromUtf8Error),
    #[debug("{message}")]
    TestError { message: String, output: String },
}

impl From<io::Error> for TestFailure {
    fn from(err: io::Error) -> Self {
        TestFailure::IOError(err)
    }
}

impl From<FromUtf8Error> for TestFailure {
    fn from(err: FromUtf8Error) -> Self {
        TestFailure::Utf8(err)
    }
}

impl From<String> for TestFailure {
    fn from(message: String) -> Self {
        TestFailure::TestError {
            message,
            output: String::new(),
        }
    }
}

impl Display for TestFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestFailure::IOError(error) => error.fmt(f),
            TestFailure::Utf8(error) => error.fmt(f),
            TestFailure::TestError { message: msg, .. } => f.write_str(&msg),
        }
    }
}

#[derive(From, Error)]
pub struct TestFailures {
    failures: Vec<TestFailure>,
}

impl TestFailures {
    pub fn empty() -> Self {
        Self {
            failures: Default::default(),
        }
    }

    pub fn into_result(self) -> Result<(), TestFailures> {
        if self.failures.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }

    pub fn eprint(&self) {
        for fail in &self.failures {
            if let TestFailure::TestError { output, .. } = fail {
                let output = output.trim();
                eprintln!("\n{output}\n");
            }
        }
    }
}

impl std::ops::AddAssign for TestFailures {
    fn add_assign(&mut self, mut rhs: Self) {
        self.failures.append(&mut rhs.failures);
    }
}

impl Display for TestFailures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.failures).finish()
    }
}

impl std::fmt::Debug for TestFailures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.failures).finish()
    }
}

pub fn run_tests_in_file(path_to_ink: &Path) -> Result<(), TestFailures> {
    let mut failures = Vec::new();
    let tests = extract_tests(path_to_ink).map_err(|it| vec![it])?;

    let mut stdout = std::io::stdout().lock();
    for test in tests {
        _ = write!(
            stdout,
            "{}:{} {} ",
            path_to_ink.to_string_lossy(),
            test.line,
            test.name.blue(),
        );
        match run_test(path_to_ink, test) {
            Err(failure) => {
                _ = writeln!(stdout, "{}", "fail".red());
                failures.push(failure);
            }
            Ok(_) => {
                _ = writeln!(stdout, "{}", "pass".green());
            }
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

    static TIMEOUT: Duration = Duration::from_secs(1);
    let instant = Instant::now();
    let mut timed_out = false;
    loop {
        match recv.read(&mut buf)? {
            0 => break,
            n => {
                static PROMPT: &[u8] = &[b'?', b'>', b' '];
                actual.extend_from_slice(&buf[..n]);
                if actual.ends_with(PROMPT) {
                    let Some(choice) = choices.next() else {
                        break;
                    };
                    let command = format!("{choice}\n").into_bytes();
                    actual.extend_from_slice(&command);
                    send.write_all(&command)?;
                    send.flush()?;
                }

                if Instant::now() - instant > TIMEOUT {
                    timed_out = true;
                    break;
                }
            }
        }
    }
    player.kill()?;
    player.wait()?;

    let actual = String::from_utf8(actual)?;
    let actual = if timed_out {
        // If we've timed out, we're likely in an infinite loop, so we truncate the output.
        let max = actual.ceil_char_boundary(run.expected_output.len() + 150);
        &actual[..max]
    } else {
        &actual[..]
    };

    let command_header = chain(once(command.get_program()), command.get_args())
        .collect::<Vec<_>>()
        .join(OsStr::new(" "))
        .to_string_lossy()
        .to_string();
    let expectation_header = format!("Test {}", run.name);

    let diff = TextDiff::from_lines(actual, &run.expected_output)
        .unified_diff()
        .context_radius(3)
        .header(&command_header, &expectation_header)
        .to_string();

    if diff.trim().is_empty() {
        Ok(())
    } else {
        let mut output = String::new();
        for line in diff.lines() {
            _ = writeln!(
                &mut output,
                "{}",
                if line.starts_with("+") {
                    line.green()
                } else if line.starts_with("-") {
                    line.red()
                } else if line.starts_with("@@") {
                    line.bright_blue()
                } else {
                    line.primary()
                }
            );
        }
        let message = format!(
            "Unexpected output{}",
            if timed_out { " after timeout" } else { "" },
        );
        Err(TestFailure::TestError { message, output })
    }
}

fn extract_tests(path_to_ink: &Path) -> Result<Vec<TestDescription>, TestFailure> {
    let string = std::fs::read_to_string(path_to_ink)?;
    let document = InkDocument::new(string, None);
    let mut tests = Vec::new();
    let mut n = 0;
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
        n += 1;
        let (declaration, expectation) = content
            .split_once('\n')
            .ok_or_else(|| format!("Incorrect test syntax: {node_text}"))?;
        let name = &declaration[KEYWORD.len()..].trim();
        let line = comment.start_position().row + 1;
        let name = if name.is_empty() {
            format!("Test {n}")
        } else {
            format!("{name}")
        };
        let input = expectation
            .lines()
            .filter_map(|it| it.starts_with("?> ").then(|| &it[3..]))
            .map(str::to_string)
            .collect();
        tests.push(TestDescription {
            name,
            line,
            input,
            expected_output: expectation.to_string(),
        });
    }
    if tests.is_empty() {
        Err(format!("No tests found in file {}", path_to_ink.to_string_lossy()).into())
    } else {
        Ok(tests)
    }
}

#[derive(Debug, Default)]
pub struct TestDescription {
    name: String,
    line: usize,
    input: Vec<String>,
    expected_output: String,
}
