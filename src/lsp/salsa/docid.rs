use std::str::FromStr as _;

use derive_more::derive::{AsRef, Debug, Display, Into};
use lsp_types::Uri;
use ustr::{ustr, Ustr};

use crate::lsp::salsa::subqueries::ink_inventory::ISet;

pub type DocIds = ISet<DocId>;

#[derive(
    Default, Display, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Into,
)]
#[debug("DocId({_0})")]
#[display("{_0}")]
pub struct DocId(Ustr);

impl DocId {
    pub fn new(uri: &Uri) -> Self {
        uri.as_str().into()
    }

    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }

    pub fn path(&self) -> &'static str {
        pub(crate) const PREFIX: usize = "file:///".len();
        &self.0.as_str()[PREFIX..]
    }
}

impl<T: AsRef<str>> From<T> for DocId {
    fn from(value: T) -> Self {
        DocId(ustr(value.as_ref()))
    }
}

impl Into<&'static str> for DocId {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

impl Into<String> for DocId {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl Into<Uri> for DocId {
    fn into(self) -> Uri {
        Uri::from_str(self.0.as_str()).unwrap()
    }
}

impl<'a> Into<Uri> for &'a DocId {
    fn into(self) -> Uri {
        Uri::from_str(self.0.as_str()).unwrap()
    }
}
