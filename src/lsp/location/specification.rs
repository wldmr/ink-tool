use std::{
    ops::{BitAnd, BitOr, Deref as _},
    str::FromStr as _,
};

use super::{Location, LocationKind};

use itertools::Itertools as _;
use lsp_types::Uri;

// Ord impls are used for normalization
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(derive(strum::VariantArray))
)]
pub(crate) enum LocationThat {
    // IDEA: The And/Or part, including Display/Debug/BitAnd/etc might work well as a little library (`Specify<LocationThat>`)
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    // IDEA: Not(Box<Self>)
    IsInFile(String),
    IsKnot,
    IsStitch,
    IsLabel,
    IsVariable,
    /// We treat a function this as distinct from a knot
    IsFunction,
    /// Additional
    HasParameters,
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
            IsInFile(file) => write!(f, "file={}", file),
            IsKnot => f.write_str("knot"),
            IsStitch => f.write_str("stitch"),
            IsLabel => f.write_str("label"),
            IsVariable => f.write_str("variable"),
            IsFunction => f.write_str("function"),
            HasParameters => f.write_str("parameters"),
            MatchesName(query) => write!(f, "name~={query}"),
            VisibleInNamespace(ns) => write!(f, "namespace={ns}"),
        }
    }
}

impl std::fmt::Debug for LocationThat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            std::fmt::Display::fmt(self, f)
        } else {
            std::fmt::Display::fmt(self, f)
        }
    }
}

impl std::ops::BitAnd for LocationThat {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::And(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::BitAndAssign for LocationThat {
    fn bitand_assign(&mut self, rhs: Self) {
        let mut self_placeholder = Self::IsKnot;
        std::mem::swap(self, &mut self_placeholder);
        *self = self_placeholder & rhs
    }
}

impl std::ops::BitOr for LocationThat {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Or(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::BitOrAssign for LocationThat {
    fn bitor_assign(&mut self, rhs: Self) {
        let mut self_placeholder = Self::IsKnot;
        std::mem::swap(self, &mut self_placeholder);
        *self = self_placeholder | rhs
    }
}

/// Construction
impl LocationThat {
    pub(crate) fn is_divert_target() -> LocationThat {
        Self::IsKnot | Self::IsStitch | Self::IsLabel
    }

    pub(crate) fn is_named() -> LocationThat {
        Self::is_divert_target() | Self::IsVariable | Self::IsFunction
    }

    pub(crate) fn matches_name(s: impl Into<String>) -> LocationThat {
        Self::MatchesName(s.into())
    }
}

/// Normalize a specification. Mostly used for comparing two specs.
// IDEA: Could we also improve performance with this?
pub(crate) fn normalized(spec: LocationThat) -> LocationThat {
    /// Normalize & sort items at the same level and join them into a result. Local helper to reduce boilerplate.
    fn normalize_join(
        items: impl IntoIterator<Item = LocationThat>,
        joiner: fn(LocationThat, LocationThat) -> LocationThat,
    ) -> LocationThat {
        items
            .into_iter()
            .map(normalized)
            .sorted_unstable()
            .dedup()
            .reduce(joiner)
            .unwrap()
    }

    use LocationThat::*;
    match spec {
        // Same pattern for And and Or: Distribute normalization through like items.
        And(l, r) => match (*l, *r) {
            (And(a, b), And(c, d)) => normalize_join([*a, *b, *c, *d], BitAnd::bitand),
            (And(a, b), c) => normalize_join([*a, *b, c], BitAnd::bitand),
            (a, And(b, c)) => normalize_join([a, *b, *c], BitAnd::bitand),
            (l, r) => normalize_join([l, r], BitAnd::bitand),
        },
        Or(l, r) => match (*l, *r) {
            (Or(a, b), Or(c, d)) => normalize_join([*a, *b, *c, *d], BitOr::bitor),
            (Or(a, b), c) => normalize_join([*a, *b, c], BitOr::bitor),
            (a, Or(b, c)) => normalize_join([a, *b, *c], BitOr::bitor),
            (l, r) => normalize_join([l, r], BitOr::bitor),
        },
        _ => spec,
    }
}

/// All the URIs in `spec`, if any.
pub(crate) fn extract_uris(spec: &LocationThat) -> Option<Vec<Uri>> {
    match spec {
        LocationThat::And(a, b) | LocationThat::Or(a, b) => {
            if let Some(mut a) = extract_uris(a.deref()) {
                if let Some(b) = extract_uris(b.deref()) {
                    a.extend(b.into_iter());
                }
                Some(a)
            } else {
                extract_uris(b.deref())
            }
        }
        LocationThat::IsInFile(path) => Some(vec![Uri::from_str(&path).unwrap()]),

        LocationThat::IsKnot
        | LocationThat::IsStitch
        | LocationThat::IsLabel
        | LocationThat::IsVariable
        | LocationThat::IsFunction
        | LocationThat::HasParameters
        | LocationThat::MatchesName(_)
        | LocationThat::VisibleInNamespace(_) => None,
    }
}

/// How well `loc` matches `spec`. Higher numbers are better, zero means no match.
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
        LocationThat::IsInFile(uri) if uri == loc.file.as_str() => 1,
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

#[cfg(test)]
pub(crate) mod tests {
    mod normalization {
        use itertools::Itertools;
        use quickcheck::{empty_shrinker, quickcheck, Arbitrary};

        use crate::lsp::location::specification::{normalized, LocationThat};

        impl Arbitrary for LocationThat {
            fn arbitrary(g: &mut quickcheck::Gen) -> Self {
                use super::super::LocationThatDiscriminants;
                use strum::VariantArray as _;
                let kind = g.choose(LocationThatDiscriminants::VARIANTS).unwrap();
                match kind {
                    LocationThatDiscriminants::And => {
                        LocationThat::arbitrary(g) & LocationThat::arbitrary(g)
                    }
                    LocationThatDiscriminants::Or => {
                        LocationThat::arbitrary(g) | LocationThat::arbitrary(g)
                    }
                    LocationThatDiscriminants::IsInFile => LocationThat::IsInFile("f".to_string()),
                    LocationThatDiscriminants::IsKnot => LocationThat::IsKnot,
                    LocationThatDiscriminants::IsStitch => LocationThat::IsStitch,
                    LocationThatDiscriminants::IsLabel => LocationThat::IsLabel,
                    LocationThatDiscriminants::IsVariable => LocationThat::IsVariable,
                    LocationThatDiscriminants::IsFunction => LocationThat::IsFunction,
                    LocationThatDiscriminants::HasParameters => LocationThat::HasParameters,
                    LocationThatDiscriminants::MatchesName => {
                        LocationThat::MatchesName("n".to_string())
                    }
                    LocationThatDiscriminants::VisibleInNamespace => {
                        LocationThat::VisibleInNamespace("s".to_string())
                    }
                }
            }

            fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
                use LocationThat::*;
                match self {
                    And(a, b) => {
                        // start with just a or b
                        let mut vec = vec![(**a).clone(), (**b).clone()];
                        // then try And(a', b') where a' and 'b are shrunken versions
                        let shrunken = (**a)
                            .shrink()
                            .cartesian_product((**b).shrink().collect_vec().into_iter())
                            .map(|(a, b)| a & b);
                        vec.extend(shrunken);
                        Box::new(vec.into_iter())
                    }
                    Or(a, b) => {
                        let mut vec = vec![(**a).clone(), (**b).clone()];
                        let shrunken = (**a)
                            .shrink()
                            .cartesian_product((**b).shrink().collect_vec().into_iter())
                            .map(|(a, b)| a | b);
                        vec.extend(shrunken);
                        Box::new(vec.into_iter())
                    }
                    IsInFile(file) => Box::new(file.shrink().map(IsInFile)),
                    MatchesName(name) => Box::new(name.shrink().map(MatchesName)),
                    VisibleInNamespace(ns) => Box::new(ns.shrink().map(VisibleInNamespace)),
                    IsKnot | IsStitch | IsLabel | IsVariable | IsFunction | HasParameters => {
                        empty_shrinker()
                    }
                }
            }
        }

        quickcheck! {
            fn order_doesnt_matter_and(a: LocationThat, b: LocationThat) -> bool {
                normalized(a.clone() & b.clone()) == normalized(b & a)
            }
            fn order_doesnt_matter_or(a: LocationThat, b: LocationThat) -> bool {
                normalized(a.clone() | b.clone()) == normalized(b | a)
            }

            // BUG: These tend to fail; normalization is still a bit buggy.
            fn duplication_is_removed_and(a: LocationThat) -> bool {
                normalized(a.clone() & a.clone()) == normalized(a)
            }
            fn duplication_is_removed_or(a: LocationThat) -> bool {
                normalized(a.clone() | a.clone()) == normalized(a)
            }

        }
    }
}
