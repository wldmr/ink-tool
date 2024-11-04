use lsp_types::Uri;
use std::ops::{Deref, Range};

#[derive(Debug)]
pub(crate) struct Location {
    pub(crate) file: Uri,
    pub(crate) name: String,
    pub(crate) namespace: Option<String>,
    pub(crate) byte_range: Range<usize>,
    pub(crate) kind: LocationKind,
}

impl Location {
    pub(crate) fn qualified_name(&self) -> String {
        match self.namespace {
            Some(ref ns) => format!("{}.{}", ns, self.name),
            None => self.name.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum LocationKind {
    Knot,
    Stitch,
    Label,
    Variable,
    Function,
}

pub(crate) enum LocationThat {
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    IsInFile(Uri),
    IsKnot,
    IsStitch,
    IsLabel,
    IsVariable,
    IsFunction,
    MatchesName(String),
    VisibleInNamespace(String),
}

impl std::fmt::Display for LocationThat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! parens_if {
            ($expr:expr, $pat:pat) => {
                if matches!($expr, $pat) {
                    format!("({})", $expr)
                } else {
                    format!("{}", $expr)
                }
            };
        }

        use LocationThat::*;
        match self {
            And(a, b) => {
                let a = parens_if!(**a, Or(_, _));
                let b = parens_if!(**b, Or(_, _));
                write!(f, "{a} & {b}")
            }
            Or(a, b) => {
                let a = parens_if!(**a, And(_, _));
                let b = parens_if!(**b, And(_, _));
                write!(f, "{a} | {b}")
            }
            IsInFile(uri) => write!(f, "file={}", uri.path().as_str()),
            IsKnot => f.write_str("is(Knot)"),
            IsStitch => f.write_str("is(Knot)"),
            IsLabel => f.write_str("is(Label)"),
            IsVariable => f.write_str("is(Variable)"),
            IsFunction => f.write_str("is(Function)"),
            MatchesName(query) => write!(f, "name~={query}"),
            VisibleInNamespace(ns) => write!(f, "namespace={ns}"),
        }
    }
}

impl std::ops::BitAnd for LocationThat {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl std::ops::BitOr for LocationThat {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

/// Construction
impl LocationThat {
    pub(crate) fn and(self, other: Self) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }

    pub(crate) fn or(self, other: Self) -> Self {
        Self::Or(Box::new(self), Box::new(other))
    }

    pub(crate) fn is_divert_target() -> LocationThat {
        Self::IsKnot | Self::IsStitch | Self::IsLabel
    }

    pub(crate) fn is_identifier() -> LocationThat {
        Self::is_divert_target() | Self::IsVariable | Self::IsFunction
    }
}

/// Interpretation Helpers
impl<'uri, 'loc: 'uri> TryFrom<&'loc LocationThat> for Vec<&'uri Uri> {
    type Error = ();
    fn try_from(value: &'loc LocationThat) -> Result<Self, Self::Error> {
        match value {
            LocationThat::And(a, b) | LocationThat::Or(a, b) => {
                if let Ok(mut a) = Self::try_from(a.deref()) {
                    if let Ok(b) = Self::try_from(b.deref()) {
                        a.extend(b.into_iter());
                    }
                    Ok(a)
                } else {
                    Self::try_from(b.deref())
                }
            }
            LocationThat::IsInFile(uri) => Ok(vec![uri]),

            LocationThat::IsKnot
            | LocationThat::IsStitch
            | LocationThat::IsLabel
            | LocationThat::IsVariable
            | LocationThat::IsFunction
            | LocationThat::MatchesName(_)
            | LocationThat::VisibleInNamespace(_) => Err(()),
        }
    }
}

pub(crate) fn rank_match(spec: &LocationThat, loc: &Location) -> usize {
    match spec {
        LocationThat::And(a, b) => {
            let a = rank_match(a, loc);
            if a == 0 {
                return 0;
            }
            let b = rank_match(b, loc);
            if b == 0 {
                return 0;
            }
            a + b
        }
        LocationThat::Or(a, b) => rank_match(a, loc).max(rank_match(b, loc)),
        LocationThat::IsInFile(uri) if uri == &loc.file => 1,
        LocationThat::IsKnot if loc.kind == LocationKind::Knot => 1,
        LocationThat::IsFunction if loc.kind == LocationKind::Function => 1,
        LocationThat::IsStitch if loc.kind == LocationKind::Stitch => 1,
        LocationThat::IsLabel if loc.kind == LocationKind::Label => 1,
        LocationThat::IsVariable if loc.kind == LocationKind::Variable => 1,
        LocationThat::MatchesName(query) if loc.qualified_name().contains(query) => query.len(),
        LocationThat::VisibleInNamespace(_) => todo!(),
        _ => 0,
    }
}
