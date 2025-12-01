pub mod text_annotations;
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

/// Wrapper to enable a more compact debug representation for tests.
#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) struct Compact<T>(pub(crate) T);

impl std::fmt::Debug for Compact<lsp_types::Location> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{:?}",
            self.0.uri.path().as_str(),
            Compact(self.0.range)
        )
    }
}

impl std::fmt::Debug for Compact<lsp_types::Range> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}-{}:{}",
            self.0.start.line, self.0.start.character, self.0.end.line, self.0.end.character
        )
    }
}

impl std::fmt::Debug for Compact<tree_sitter::Range> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}-{}:{}",
            self.0.start_point.row,
            self.0.start_point.column,
            self.0.end_point.row,
            self.0.end_point.column
        )
    }
}

impl std::fmt::Debug for Compact<text_annotations::TextRegion> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}-{}:{}",
            self.0.start.row, self.0.start.col, self.0.end.row, self.0.end.col
        )
    }
}

pub fn setup_logging(min_level: log::LevelFilter) {
    _ = fern::Dispatch::new()
        .level(log::LevelFilter::Error)
        .level_for("ink_tool", min_level)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(std::io::stderr())
        .apply();
}
