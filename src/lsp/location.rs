use std::str::FromStr;

pub mod specification;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct Location {
    pub(crate) id: LocationId,
    pub(crate) name: String,
    pub(crate) namespace: Option<String>,
    pub(crate) kind: LocationKind,
    pub(crate) link: Vec<Link>,
}

impl Location {
    pub(crate) fn path_as_str(&self) -> &str {
        &self.id.file.0
    }
}

// We mimic some basic LSP types to get around the orphan rule.

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
// IDEA: Maybe don't store the whole path here and instead just make this an index. Pro: it would become Copy. Con: We could only say what the path is at the Workspace level.
pub(crate) struct FileId(String);

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct FileRange {
    start: FilePos,
    end: FilePos,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct FilePos {
    line: u32,
    character: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct LocationId {
    file: FileId,
    position: FileRange,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) struct Link {
    /// Location that this link started from
    from: LocationId,
    /// Either a full resolved location or a string that describes the intended location.
    to: Result<LocationId, String>,
    kind: RelationKind,
}

impl Location {
    pub(crate) fn qualified_name(&self) -> String {
        match self.namespace {
            Some(ref ns) => format!("{}.{}", ns, self.name),
            None => self.name.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
pub(crate) enum RelationKind {
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
impl From<lsp_types::Range> for FileRange {
    fn from(value: lsp_types::Range) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl From<FileRange> for lsp_types::Range {
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

impl From<FileId> for lsp_types::Uri {
    fn from(value: FileId) -> Self {
        lsp_types::Uri::from_str(&value.0).expect("This should have been created from a valid URI")
    }
}

impl From<(&lsp_types::Uri, &lsp_types::Range)> for LocationId {
    fn from((uri, pos): (&lsp_types::Uri, &lsp_types::Range)) -> Self {
        Self {
            file: uri.clone().into(),
            position: pos.clone().into(),
        }
    }
}
