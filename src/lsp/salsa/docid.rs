use std::str::FromStr as _;

use derive_more::derive::{AsRef, Debug, Display, Into};
use lsp_types::Uri;
use ustr::{ustr, Ustr};

use crate::lsp::salsa::subqueries::ink_inventory::ISet;

pub type DocIds = ISet<DocId>;

/// Document ID that is _equivalent_ to an URI, but is Copy.
///
/// Construct it using
///
/// ```rust
/// # use lsp_types::Uri;
/// # use std::str::FromStr as _;
/// # use ink_tool::lsp::salsa::DocId;
/// let uri = Uri::from_str("file:///some/file/or/another").unwrap();
/// let docid = DocId::from(uri);
/// ```
#[derive(
    Default, Display, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Into,
)]
#[debug("DocId({_0})")]
#[display("{_0}")]
pub struct DocId(Ustr);

const FILE_SCHEME: &'static str = "file://";

impl DocId {
    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }

    pub fn path(&self) -> &'static str {
        // FIXME: We don't actually enforce that the URI starts with that prefix
        &self.0.as_str()[FILE_SCHEME.len()..]
    }
}

impl From<Uri> for DocId {
    fn from(value: Uri) -> Self {
        DocId(ustr(value.as_str()))
    }
}

impl<'a> From<&'a Uri> for DocId {
    fn from(value: &'a Uri) -> Self {
        DocId(ustr(value.as_str()))
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
        Uri::from_str(self.as_str()).unwrap()
    }
}

impl<'a> Into<Uri> for &'a DocId {
    fn into(self) -> Uri {
        (*self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Arbitrary;
    use tap::Tap;

    impl Arbitrary for DocId {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            // very simple number based URIs (file:///1/2/3), to sidestep any
            let nums = Vec::<u8>::arbitrary(g).tap_mut(|nums| {
                if nums.is_empty() {
                    nums.push(u8::arbitrary(g))
                }
            });
            let mut uri = nums
                .into_iter()
                .fold(format!("{FILE_SCHEME}"), |mut acc, next| {
                    acc.push('/');
                    acc.push_str(&next.to_string());
                    acc
                });
            uri.push_str(".ink"); // we ensured that nums is not empty.
            let uri = Uri::from_str(&uri).unwrap();
            Self::from(uri)
        }

        // The uris are simple enough, let's not bother with shrinking.
    }
}
