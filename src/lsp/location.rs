#![allow(dead_code)] // TODO: remove this once we've settled on a design

use crate::lsp::idset::Id;
use derive_more::{derive::Display, Debug};
use lsp_types::{Range, Uri};
use std::ops::RangeBounds;

pub mod specification;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct Location {
    pub(crate) file_range: FileTextRange,
    pub(crate) name: String,
    pub(crate) namespace: Option<String>,
    pub(crate) kind: LocationKind,
}

impl Location {
    pub(crate) fn id(&self) -> FileTextRange {
        self.file_range
    }

    pub(crate) fn new(
        uri: Id<Uri>,
        range: Range,
        name: String,
        namespace: Option<String>,
        kind: LocationKind,
    ) -> Self {
        Self {
            file_range: FileTextRange::new(uri, range),
            name,
            namespace,
            kind,
        }
    }

    pub(crate) fn qualified_name(&self) -> String {
        match self.namespace {
            Some(ref ns) => format!("{}.{}", ns, self.name),
            None => self.name.clone(),
        }
    }
}

// We mimic some basic LSP types to get around the orphan rule.

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[debug("{start:?}..{end:?}")]
pub(crate) struct TextRange {
    start: TextPos,
    end: TextPos,
}

/// Subtracting two `FilePos`s creates a `FileRange`.
impl TextRange {
    pub(crate) fn new(a: impl Into<TextPos>, b: impl Into<TextPos>) -> Self {
        let a = a.into();
        let b = b.into();
        if a <= b {
            Self { start: a, end: b }
        } else {
            Self { start: b, end: a }
        }
    }

    pub(crate) fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Is there a gap between the two ranges?
    pub(crate) fn disjoint(&self, other: &Self) -> bool {
        self.is_strictly_before(other) || self.is_strictly_after(other)
    }

    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        !self.disjoint(other) && (!self.is_directly_before(other) || !self.is_directly_after(other))
    }

    pub(crate) fn is_strictly_before(&self, other: &Self) -> bool {
        self.end < other.start
    }

    pub(crate) fn is_strictly_after(&self, other: &Self) -> bool {
        self.start > other.end
    }

    pub(crate) fn is_directly_before(&self, other: &Self) -> bool {
        self.end == other.start
    }

    pub(crate) fn is_directly_after(&self, other: &Self) -> bool {
        self.start == other.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
#[debug("{line}|{character}")]
pub(crate) struct TextPos {
    line: u32,
    character: u32,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
#[display("{file:?}:{range:?}")]
pub(crate) struct FileTextRange {
    file: Id<Uri>,
    range: TextRange,
}

impl FileTextRange {
    pub(crate) fn new(file: Id<Uri>, range: impl Into<TextRange>) -> Self {
        Self {
            file,
            range: range.into(),
        }
    }

    pub(crate) fn disjoint(&self, other: &Self) -> bool {
        self.file != other.file || self.range.disjoint(&other.range)
    }

    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        self.file == other.file && self.range.overlaps(&other.range)
    }

    pub(crate) fn contains(&self, other: &Self) -> bool {
        self.file == other.file && self.range.contains(&other.range)
    }

    pub(crate) fn is_in_file(&self, file: Id<Uri>) -> bool {
        self.file == file
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct Link {
    pub(crate) source: FileTextRange,
    pub(crate) target: FileTextRange,
    pub(crate) kind: LinkKind,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy, Hash)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) enum LocationKind {
    Knot,
    Stitch,
    Label,
    Variable,
    Function,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) enum LinkKind {
    ValueDefinedBy,
    TypeDefinedBy,
    RedirectTo,
}

// *** Conversion to/from lsp_types

impl From<lsp_types::Position> for TextPos {
    fn from(value: lsp_types::Position) -> Self {
        Self {
            line: value.line,
            character: value.character,
        }
    }
}

impl From<TextPos> for lsp_types::Position {
    fn from(value: TextPos) -> Self {
        Self {
            line: value.line,
            character: value.character,
        }
    }
}

impl From<Range> for TextRange {
    fn from(value: Range) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl From<TextRange> for Range {
    fn from(value: TextRange) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl RangeBounds<FileTextRange> for &Location {
    fn start_bound(&self) -> std::ops::Bound<&FileTextRange> {
        std::ops::Bound::Included(&self.file_range)
    }

    fn end_bound(&self) -> std::ops::Bound<&FileTextRange> {
        std::ops::Bound::Excluded(&self.file_range)
    }
}

#[cfg(test)]
mod tests {
    mod file_range {
        use crate::{
            lsp::location::{TextPos, TextRange},
            test_utils::in_case,
        };
        use quickcheck::{quickcheck, TestResult};

        quickcheck! {
            fn creating_file_range_is_symmetric(a: TextPos, b: TextPos) -> bool {
                TextRange::new(a, b) == TextRange::new(b, a)
            }

            fn contains_is_asymmetric(a: TextRange, b: TextRange) -> TestResult {
                in_case! { a.contains(&b) => !b.contains(&a) }
            }

            fn range_contains_itself(a: TextRange) -> bool {
                a.contains(&a)
            }

            fn contains_implies_overlaps(a: TextRange, b: TextRange) -> TestResult {
                in_case! { a.contains(&b) => a.overlaps(&b) }
            }

            fn overlaps_is_symmetric(a: TextRange, b: TextRange) -> TestResult {
                in_case! { a.overlaps(&b) => b.overlaps(&a) }
            }

            fn strictly_before_implies_no_overlap(a: TextRange, b: TextRange) -> TestResult {
                in_case! { a.is_strictly_before(&b) => !a.overlaps(&b) }
            }

            fn strictly_after_implies_no_overlap(a: TextRange, b: TextRange) -> TestResult {
                in_case! { a.is_strictly_after(&b) => !a.overlaps(&b) }
            }
        }
    }

    mod location_id {
        use crate::{lsp::location::FileTextRange, test_utils::in_case};
        use quickcheck::{quickcheck, TestResult};

        quickcheck! {
            fn same_file_contains_matches_range(a: FileTextRange, b: FileTextRange) -> bool {
                let mut b = b;
                b.file = a.file.clone();
                a.contains(&b) == a.range.contains(&b.range)
            }

            fn same_file_overlaps_matches_range(a: FileTextRange, b: FileTextRange) -> bool {
                let mut b = b;
                b.file = a.file.clone();
                a.overlaps(&b) == a.range.overlaps(&b.range)
            }

            fn same_file_disjoint_matches_range(a: FileTextRange, b: FileTextRange) -> bool {
                let mut b = b;
                b.file = a.file.clone();
                a.disjoint(&b) == a.range.disjoint(&b.range)
            }

            fn different_file_implies_no_containment(a: FileTextRange, b: FileTextRange) -> TestResult {
                in_case!(a.file != b.file => !a.contains(&b))
            }

            fn different_file_implies_no_overlap(a: FileTextRange, b: FileTextRange) -> TestResult {
                in_case!(a.file != b.file => !a.overlaps(&b))
            }

            fn different_file_implies_disjoint(a: FileTextRange, b: FileTextRange) -> TestResult {
                in_case!(a.file != b.file => a.disjoint(&b))
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod arbitrary {
    use super::{FileTextRange, TextPos, TextRange};
    use crate::lsp::idset;
    use quickcheck::Arbitrary;

    impl Arbitrary for TextPos {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                line: u8::arbitrary(g) as u32, // smaller numbers for readability
                character: u8::arbitrary(g) as u32,
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut vec = Vec::new();
            for line in self.line.shrink() {
                for character in self.character.shrink() {
                    vec.push(Self { line, character }) // Cartesian product?! Ouch!
                }
            }
            Box::new(vec.into_iter())
        }
    }

    impl Arbitrary for TextRange {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            TextRange::new(TextPos::arbitrary(g), TextPos::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut vec = Vec::new();
            for start in self.start.shrink() {
                for end in self.end.shrink() {
                    vec.push(Self { start, end }) // Cartesian product?! Ouch!
                }
            }
            Box::new(vec.into_iter())
        }
    }

    impl Arbitrary for FileTextRange {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                file: idset::Id::arbitrary(g),
                range: TextRange::arbitrary(g),
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut result = Vec::new();
            for file in self.file.shrink() {
                for range in self.range.shrink() {
                    result.push(Self {
                        file: file.clone(),
                        range,
                    })
                }
            }
            return Box::new(result.into_iter());
        }
    }
}
