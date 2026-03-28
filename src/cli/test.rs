use std::path::PathBuf;

use clap::Args;
use ink_tool::AppResult;

#[derive(Args, Debug)]
/// Test an ink file
pub(crate) struct TestOpt {
    /// The file to run.
    input: PathBuf,
}

pub(crate) fn test(opt: TestOpt) -> AppResult<()> {
    ink_test::run_test(&opt.input, Default::default())?;
    Ok(())
}
