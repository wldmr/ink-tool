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
        uri: &Uri,
        range: &Range,
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
}

// We mimic some basic LSP types to get around the orphan rule.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
// IDEA: Maybe don't store the whole path here and instead just make this an index. Pro: it would become Copy. Con: We could only say what the path is at the Workspace level.
pub(crate) struct FileId(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct FileRange {
    pub(crate) start: FilePos,
    pub(crate) end: FilePos,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct FilePos {
    line: u32,
    character: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct LocationId {
    file: FileId,
    range: FileRange,
}

impl LocationId {
    pub(crate) fn new(uri: &Uri, range: &Range) -> Self {
        Self {
            file: uri.into(),
            range: (*range).into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct Link {
    pub(crate) source: LocationId,
    pub(crate) target: LocationId,
    pub(crate) kind: LinkKind,
}

impl Location {
    pub(crate) fn qualified_name(&self) -> String {
        match self.namespace {
            Some(ref ns) => format!("{}.{}", ns, self.name),
            None => self.name.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) enum LocationKind {
    Knot,
    Stitch,
    Label,
    Variable,
    Function,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

impl From<&lsp_types::Uri> for FileId {
    fn from(value: &lsp_types::Uri) -> Self {
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
