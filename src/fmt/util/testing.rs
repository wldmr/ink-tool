#[cfg(test)]
macro_rules! in_case {
    ($prereq:expr => $($stmts:stmt);+) => {
        if $prereq {
            quickcheck::TestResult::from_bool({$($stmts)*})
        } else {
            quickcheck::TestResult::discard()
        }
    };
}

#[cfg(test)]
pub(crate) use in_case;
