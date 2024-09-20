use std::ops::Add;

use crate::util::constrained_value::Constrained;

/// Keeps track of the constraints on spaces and newlines.
///
/// Think of it as Schrödinger's Whitespace: A superpositon of spaces and newlines.
/// What exactly we will output hasn't been decided yet.
pub(crate) struct UndecidedWhitespace {
    pub(crate) space: Constrained,
    pub(crate) newline: Constrained,
}

impl Add for UndecidedWhitespace {
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

impl From<UndecidedWhitespace> for Whitespace {
    fn from(buf: UndecidedWhitespace) -> Self {
        if buf.newline.value() > 0 {
            Whitespace::Newline(buf.newline)
        } else {
            Whitespace::Space(buf.space)
        }
    }
}

impl std::fmt::Debug for UndecidedWhitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(␣{:?}⊕⏎{:?})", self.space, self.newline)
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
