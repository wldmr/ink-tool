use std::path::PathBuf;

use clap::Args;
use ink_test::TestFailures;
use ink_tool::AppResult;

#[derive(Args, Debug)]
/// Run ink tests
pub(crate) struct TestOpt {
    /// The files to run. Must be ink files with special `TEST` block comments.
    inputs: Vec<PathBuf>,
}

pub(crate) fn test(opt: TestOpt) -> AppResult<()> {
    if opt.inputs.is_empty() {
        return Err("Need at least one file".to_string().into());
    }

    let mut errors = TestFailures::empty();
    for input in opt.inputs {
        if let Err(error) = ink_test::run_tests_in_file(&input) {
            errors += error;
        }
    }

    errors.into_result().map_err(|err| err.into())
}
