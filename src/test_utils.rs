macro_rules! check_eq {
    ($a:expr, $b:expr) => {
        if $a == $b {
            quickcheck::TestResult::passed()
        } else {
            quickcheck::TestResult::error(format!(
                "Expected\n{}\n  to equal\n{}\n  but found that\n{:?}\n  is not equal to\n{:?}",
                stringify!($a).replace(".clone()", ""),
                stringify!($b).replace(".clone()", ""),
                $a,
                $b
            ))
        }
    };
}

macro_rules! in_case {
    ($prereq:expr => $($stmts:stmt);+) => {
        if $prereq {
            quickcheck::TestResult::from_bool({$($stmts)*})
        } else {
            quickcheck::TestResult::discard()
        }
    };
}

pub(crate) use check_eq;
pub(crate) use in_case;
