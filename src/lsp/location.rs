use lsp_types::Uri;
use std::ops::Range;

pub mod specification;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub(crate) enum LocationKind {
    Knot,
    Stitch,
    Label,
    Variable,
    Function,
}
