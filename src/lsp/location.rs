use derive_more::{derive::Display, Debug};
use lsp_types::{Range, Uri};
use std::{ops::RangeBounds, str::FromStr};

pub mod specification;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct Location {
    pub(crate) id: LocationId,
    pub(crate) name: String,
    pub(crate) namespace: Option<String>,
    pub(crate) kind: LocationKind,
}

impl Location {
    pub(crate) fn id(&self) -> LocationId {
        self.id.clone()
    }

    pub(crate) fn path_as_str(&self) -> &str {
        &self.id.file.0
    }

    pub(crate) fn new(
        uri: Uri,
        range: Range,
        name: String,
        namespace: Option<String>,
        kind: LocationKind,
    ) -> Self {
        Self {
            id: LocationId::new(uri, range),
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
// IDEA: Maybe don't store the whole path here and instead just make this an index. Pro: it would become Copy. Con: We could only say what the path is at the Workspace level.
#[debug("{_0}")]
pub(crate) struct FileId(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[debug("{start:?}..{end:?}")]
pub(crate) struct FileRange {
    start: FilePos,
    end: FilePos,
}

/// Subtracting two `FilePos`s creates a `FileRange`.
impl FileRange {
    pub(crate) fn new(a: impl Into<FilePos>, b: impl Into<FilePos>) -> Self {
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
pub(crate) struct FilePos {
    line: u32,
    character: u32,
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash)]
#[display("{file:?}:{range:?}")]
pub(crate) struct LocationId {
    file: FileId,
    range: FileRange,
}

impl LocationId {
    pub(crate) fn new(uri: impl Into<FileId>, range: impl Into<FileRange>) -> Self {
        Self {
            file: uri.into(),
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

    pub(crate) fn is_in_file(&self, file: &FileId) -> bool {
        self.file == *file
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct Link {
    pub(crate) source: LocationId,
    pub(crate) target: LocationId,
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

impl From<lsp_types::Position> for FilePos {
    fn from(value: lsp_types::Position) -> Self {
        Self {
            line: value.line,
            character: value.character,
        }
    }
}

impl From<FilePos> for lsp_types::Position {
    fn from(value: FilePos) -> Self {
        Self {
            line: value.line,
            character: value.character,
        }
    }
}

impl From<Range> for FileRange {
    fn from(value: Range) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl From<FileRange> for Range {
    fn from(value: FileRange) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl From<lsp_types::Uri> for FileId {
    fn from(value: lsp_types::Uri) -> Self {
        Self(value.as_str().to_string())
    }
}

impl From<&FileId> for lsp_types::Uri {
    fn from(value: &FileId) -> Self {
        lsp_types::Uri::from_str(&value.0).expect("This should have been created from a valid URI")
    }
}

impl RangeBounds<LocationId> for &Location {
    fn start_bound(&self) -> std::ops::Bound<&LocationId> {
        std::ops::Bound::Included(&self.id)
    }

    fn end_bound(&self) -> std::ops::Bound<&LocationId> {
        std::ops::Bound::Excluded(&self.id)
    }
}

#[cfg(test)]
mod tests {
    mod file_range {
        use crate::{
            lsp::location::{FilePos, FileRange},
            test_utils::in_case,
        };
        use quickcheck::{quickcheck, TestResult};

        quickcheck! {
            fn creating_file_range_is_symmetric(a: FilePos, b: FilePos) -> bool {
                FileRange::new(a, b) == FileRange::new(b, a)
            }

            fn contains_is_asymmetric(a: FileRange, b: FileRange) -> TestResult {
                in_case! { a.contains(&b) => !b.contains(&a) }
            }

            fn range_contains_itself(a: FileRange) -> bool {
                a.contains(&a)
            }

            fn contains_implies_overlaps(a: FileRange, b: FileRange) -> TestResult {
                in_case! { a.contains(&b) => a.overlaps(&b) }
            }

            fn overlaps_is_symmetric(a: FileRange, b: FileRange) -> TestResult {
                in_case! { a.overlaps(&b) => b.overlaps(&a) }
            }

            fn strictly_before_implies_no_overlap(a: FileRange, b: FileRange) -> TestResult {
                in_case! { a.is_strictly_before(&b) => !a.overlaps(&b) }
            }

            fn strictly_after_implies_no_overlap(a: FileRange, b: FileRange) -> TestResult {
                in_case! { a.is_strictly_after(&b) => !a.overlaps(&b) }
            }
        }
    }

    mod location_id {
        use crate::{lsp::location::LocationId, test_utils::in_case};
        use quickcheck::{quickcheck, TestResult};

        quickcheck! {
            fn same_file_contains_matches_range(a: LocationId, b: LocationId) -> bool {
                let mut b = b;
                b.file = a.file.clone();
                a.contains(&b) == a.range.contains(&b.range)
            }

            fn same_file_overlaps_matches_range(a: LocationId, b: LocationId) -> bool {
                let mut b = b;
                b.file = a.file.clone();
                a.overlaps(&b) == a.range.overlaps(&b.range)
            }

            fn same_file_disjoint_matches_range(a: LocationId, b: LocationId) -> bool {
                let mut b = b;
                b.file = a.file.clone();
                a.disjoint(&b) == a.range.disjoint(&b.range)
            }

            fn different_file_implies_no_containment(a: LocationId, b: LocationId) -> TestResult {
                in_case!(a.file != b.file => !a.contains(&b))
            }

            fn different_file_implies_no_overlap(a: LocationId, b: LocationId) -> TestResult {
                in_case!(a.file != b.file => !a.overlaps(&b))
            }

            fn different_file_implies_disjoint(a: LocationId, b: LocationId) -> TestResult {
                in_case!(a.file != b.file => a.disjoint(&b))
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod arbitrary {
    use super::{FileId, FilePos, FileRange, LocationId};
    use quickcheck::Arbitrary;

    impl Arbitrary for FilePos {
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

    impl Arbitrary for FileRange {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            FileRange::new(FilePos::arbitrary(g), FilePos::arbitrary(g))
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

    impl Arbitrary for FileId {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self(String::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(self.0.shrink().map(Self))
        }
    }

    impl Arbitrary for LocationId {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                file: FileId::arbitrary(g),
                range: FileRange::arbitrary(g),
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
