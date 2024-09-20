use std::ops::Add;

use crate::util::constrained_value::Constrained;

/// Keeps track of the constraints on spaces and newlines.
///
/// Think of it as Schrödinger's Whitespace: A superpositon of spaces and newlines.
/// What exactly we will output hasn't been decided yet.
#[derive(Clone, Copy)]
pub(crate) struct Undecided {
    pub(crate) space: Constrained,
    pub(crate) newline: Constrained,
}

impl Add for Undecided {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.space += rhs.space;
        self.newline += rhs.newline;
        self
    }
}

/// "Decided" Whitespace. Either space(s) or newline(s).
pub(super) enum Whitespace {
    Space(Constrained),
    Newline(Constrained),
}

/// The actual decision: If there are required newlines, collapse to newlines, otherwise collapse to spaces.
impl From<Undecided> for Whitespace {
    fn from(source: Undecided) -> Self {
        if source.newline.value() > 0 {
            Whitespace::Newline(source.newline)
        } else {
            Whitespace::Space(source.space)
        }
    }
}

impl std::fmt::Debug for Undecided {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(␣{:?}||⏎{:?})", self.space, self.newline)
    }
}

impl std::fmt::Debug for Whitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Whitespace::Space(constraint) => write!(f, "␣{:?}", constraint),
            Whitespace::Newline(constraint) => write!(f, "⏎{:?}", constraint),
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, TestResult};
    use quickcheck_macros::quickcheck;

    use crate::util::testing::in_case;

    use super::*;

    impl Arbitrary for Undecided {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                space: Constrained::arbitrary(g),
                newline: Constrained::arbitrary(g),
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let shrunk_space = self.space.shrink().map(|it| Self {
                space: it,
                newline: self.newline,
            });
            let shrunk_newline = self.newline.shrink().map(|it| Self {
                space: self.space,
                newline: it,
            });
            Box::new(
                Iterator::chain(shrunk_space, shrunk_newline)
                    .collect::<Vec<_>>()
                    .into_iter(),
            )
        }
    }

    #[quickcheck]
    fn if_contains_newlines_then_collapse_to_newlines(undecided: Undecided) -> TestResult {
        in_case! { undecided.newline.value() != 0 =>
            match Whitespace::from(undecided) {
                Whitespace::Newline(repeats) => repeats == undecided.newline,
                _ => false
            }
        }
    }

    #[quickcheck]
    fn if_no_newlines_then_collapse_to_spaces(undecided: Undecided) -> TestResult {
        in_case! { undecided.newline.value() == 0 =>
            match Whitespace::from(undecided) {
                Whitespace::Space(repeats) => repeats == undecided.space,
                _ => false
            }
        }
    }
}
