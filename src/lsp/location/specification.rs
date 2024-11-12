use super::{FileId, Location, LocationKind};
use itertools::Itertools;
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
    And(Vec<Self>),
    Or(Vec<Self>),
    // IDEA: Not(Box<Self>)
    IsInFile(FileId),
    IsLocation(LocationKind),
    HasParameters,
    MatchesName(String),
    VisibleInNamespace(String),
}

impl std::fmt::Display for LocationThat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LocationThat::*;
        match self {
            And(items) => match items.len() {
                0 => panic!("Empty And!"),
                1 => items[0].fmt(f),
                _ => write!(f, "({})", items.iter().join(" & ")),
            },
            Or(items) => match items.len() {
                0 => panic!("Empty Or!"),
                1 => items[0].fmt(f),
                _ => write!(f, "({})", items.iter().join(" | ")),
            },
            IsInFile(file) => write!(f, "file={}", file.0),
            IsLocation(kind) => match kind {
                LocationKind::Knot => f.write_str("knot"),
                LocationKind::Stitch => f.write_str("stitch"),
                LocationKind::Label => f.write_str("label"),
                LocationKind::Variable => f.write_str("variable"),
                LocationKind::Function => f.write_str("function"),
            },
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
        use LocationThat::*;
        // Coalesce adjacent `And`s. This makes keeping the expressions tidy a little easier.
        let items = match (self, rhs) {
            (And(mut items), And(others)) => {
                items.extend(others.into_iter());
                items
            }
            (And(mut items), other) | (other, And(mut items)) => {
                items.push(other);
                items
            }
            (l, r) => vec![l, r],
        };
        And(items)
    }
}

impl std::ops::BitAndAssign for LocationThat {
    fn bitand_assign(&mut self, rhs: Self) {
        let mut self_placeholder = Self::is_knot();
        std::mem::swap(self, &mut self_placeholder);
        *self = self_placeholder & rhs
    }
}

impl std::ops::BitOr for LocationThat {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        use LocationThat::*;
        let items = match (self, rhs) {
            (Or(mut items), Or(others)) => {
                items.extend(others.into_iter());
                items
            }
            (Or(mut items), other) | (other, Or(mut items)) => {
                items.push(other);
                items
            }
            (l, r) => vec![l, r],
        };
        Or(items)
    }
}

impl std::ops::BitOrAssign for LocationThat {
    fn bitor_assign(&mut self, rhs: Self) {
        let mut self_placeholder = Self::is_knot();
        std::mem::swap(self, &mut self_placeholder);
        *self = self_placeholder | rhs
    }
}

/// Construction
impl LocationThat {
    pub(crate) fn is_in_file(file: impl Into<FileId>) -> Self {
        Self::IsInFile(file.into())
    }

    pub(crate) fn is_knot() -> LocationThat {
        Self::IsLocation(LocationKind::Knot)
    }
    pub(crate) fn is_function() -> LocationThat {
        Self::IsLocation(LocationKind::Function)
    }
    pub(crate) fn is_stitch() -> LocationThat {
        Self::IsLocation(LocationKind::Stitch)
    }
    pub(crate) fn is_label() -> LocationThat {
        Self::IsLocation(LocationKind::Label)
    }

    pub(crate) fn is_variable() -> LocationThat {
        Self::IsLocation(LocationKind::Variable)
    }

    pub(crate) fn is_divert_target() -> LocationThat {
        Self::is_knot() | Self::is_stitch() | Self::is_label()
    }

    pub(crate) fn is_named() -> LocationThat {
        Self::is_divert_target() | Self::is_variable() | Self::is_function()
    }

    pub(crate) fn has_parameters() -> LocationThat {
        Self::HasParameters
    }

    pub(crate) fn matches_name(s: impl Into<String>) -> LocationThat {
        Self::MatchesName(s.into())
    }
}

/// Simplify a specification. Mostly used for comparing two specs.
// NOTE: This isn't proper normalization (neither DNF nor CNF), it just sorts and deduplicates the existing structure.
// Together with our ad-hoc merging during construction this seems to work well enough for now.
// I suspect that'll change once we introduce negation.
#[cfg(test)]
pub(crate) fn simplified(spec: LocationThat) -> LocationThat {
    use std::ops::{BitAnd, BitOr};

    match spec {
        // Same pattern for And and Or: Sort and deduplicate like items.
        LocationThat::And(items) => items
            .into_iter()
            .map(simplified)
            .sorted_unstable()
            .dedup()
            .reduce(BitAnd::bitand)
            .expect("`And` should not be empty"),
        LocationThat::Or(items) => items
            .into_iter()
            .map(simplified)
            .sorted_unstable()
            .dedup()
            .reduce(BitOr::bitor)
            .expect("`Or` should not be empty"),
        _ => spec,
    }
}

/// All the URIs in `spec`, if any.
pub(crate) fn extract_uris(spec: &LocationThat) -> Option<Vec<Uri>> {
    match spec {
        LocationThat::And(items) | LocationThat::Or(items) => {
            let merged: Vec<Uri> = items.iter().filter_map(extract_uris).flatten().collect();
            if merged.is_empty() {
                None
            } else {
                Some(merged)
            }
        }
        LocationThat::IsInFile(path) => Some(vec![path.into()]),

        LocationThat::IsLocation(_)
        | LocationThat::HasParameters
        | LocationThat::MatchesName(_)
        | LocationThat::VisibleInNamespace(_) => None,
    }
}

/// How well `loc` matches `spec`. Higher numbers are better, zero means no match.
pub(crate) fn rank_match(spec: &LocationThat, loc: &Location) -> usize {
    match spec {
        LocationThat::And(items) => items
            .into_iter()
            .map(|spec| rank_match(spec, loc))
            .min()
            .unwrap_or(0),
        LocationThat::Or(items) => items
            .into_iter()
            .map(|spec| rank_match(spec, loc))
            .max()
            .unwrap_or(0),
        LocationThat::IsInFile(uri) => usize::from(loc.id.file == *uri),
        LocationThat::IsLocation(kind) => usize::from(loc.kind == *kind),
        LocationThat::MatchesName(query) => {
            if loc.qualified_name().contains(query) {
                query.len()
            } else {
                0
            }
        }
        LocationThat::VisibleInNamespace(_) => todo!(),
        LocationThat::HasParameters => usize::from(loc.name.ends_with(")")), // OMG, so dirty.
    }
}

pub(crate) fn matches(spec: &LocationThat, loc: &Location) -> bool {
    rank_match(spec, loc) != 0
}

#[cfg(test)]
// Need to implement it manually, because derive_quickcheck_arbitrary::Arbitrary seems to overflow the stack due to recursion.
impl quickcheck::Arbitrary for LocationThat {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        use strum::VariantArray as _;
        let kind = g.choose(LocationThatDiscriminants::VARIANTS).unwrap();
        match kind {
            LocationThatDiscriminants::And => {
                LocationThat::arbitrary(g) & LocationThat::arbitrary(g)
            }
            LocationThatDiscriminants::Or => {
                LocationThat::arbitrary(g) | LocationThat::arbitrary(g)
            }
            LocationThatDiscriminants::IsInFile => LocationThat::IsInFile(FileId::arbitrary(g)),
            LocationThatDiscriminants::IsLocation => {
                LocationThat::IsLocation(LocationKind::arbitrary(g))
            }
            LocationThatDiscriminants::HasParameters => LocationThat::HasParameters,
            LocationThatDiscriminants::MatchesName => {
                LocationThat::MatchesName(String::arbitrary(g))
            }
            LocationThatDiscriminants::VisibleInNamespace => {
                LocationThat::VisibleInNamespace(String::arbitrary(g))
            }
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        use std::ops::{BitAnd, BitOr};
        use LocationThat::*;
        match self {
            And(items) => Box::new(
                items
                    .shrink()
                    .flat_map(|each| each.into_iter().reduce(BitAnd::bitand)),
            ),
            Or(items) => Box::new(
                items
                    .shrink()
                    .flat_map(|each| each.into_iter().reduce(BitOr::bitor)),
            ),
            IsInFile(file) => Box::new(file.shrink().map(IsInFile)),
            MatchesName(name) => Box::new(name.shrink().map(MatchesName)),
            VisibleInNamespace(ns) => Box::new(ns.shrink().map(VisibleInNamespace)),
            IsLocation(_) | HasParameters => quickcheck::empty_shrinker(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    mod simplification {
        use crate::{
            lsp::location::specification::{simplified, LocationThat},
            test_utils::check_eq,
        };
        use quickcheck::{quickcheck, TestResult};

        quickcheck! {
            fn order_doesnt_matter_and(a: LocationThat, b: LocationThat) -> TestResult {
                check_eq!(
                    simplified(a.clone() & b.clone()),
                    simplified(b.clone() & a.clone())
                )
            }
            fn order_doesnt_matter_or(a: LocationThat, b: LocationThat) -> TestResult {
                check_eq!(
                    simplified(a.clone() | b.clone()),
                    simplified(b.clone() | a.clone())
                )
            }

            fn duplication_is_removed_and(a: LocationThat) -> TestResult {
                check_eq!(
                    simplified(a.clone() & a.clone()),
                    simplified(a.clone() & a.clone())
                )
            }
            fn duplication_is_removed_or(a: LocationThat) -> TestResult{
                check_eq!(
                    simplified(a.clone() | a.clone()),
                    simplified(a.clone() | a.clone())
                )
            }

        }
    }
}
