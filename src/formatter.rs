use std::{fmt::Display, ops::Range};

use tree_sitter::TreeCursor;
use tree_sitter_ink::{
    node_types::lib::ExtraOr,
    node_types::{
        self as ty,
        lib::{OptionNodeResultExt, TypedNode},
    },
};

use crate::config::FormatConfig;

pub trait ByteRange {
    fn byte_range(&self) -> Range<usize>;
}

pub trait InkFmt<'a>: TypedNode<'a> {
    fn inkfmt(&self, formatter: &mut InkFormatter<'a>) -> InkFmtResult {
        formatter.copy(*self);
        Ok(())
    }
}

impl<'a> InkFmt<'a> for ty::Ink<'a> {
    fn inkfmt(&self, formatter: &mut InkFormatter<'a>) -> InkFmtResult {
        if let Some(block) = self.content() {
            block?.inkfmt(formatter)?
        }
        let stitches: Vec<_> = self.stitchs(&mut formatter.cursor).collect();
        for block in stitches {
            block?.inkfmt(formatter);
        }
        let stitches: Vec<_> = self.knots(&mut formatter.cursor).collect();
        for block in stitches {
            block?.inkfmt(formatter);
        }
        Ok(())
    }
}

impl<'a> InkFmt<'a> for ty::Binary<'a> {
    fn inkfmt(&self, formatter: &mut InkFormatter<'a>) -> InkFmtResult {
        self.left()?.inkfmt(formatter)?;
        formatter.str(" ")?;
        formatter.copy(self.op()?);
        formatter.str(" ")?;
        self.left()?.inkfmt(formatter)
    }
}

impl<'a> InkFmt<'a> for ty::Expr<'a> {
    fn inkfmt(&self, formatter: &mut InkFormatter<'a>) -> InkFmtResult {
        match self {
            ty::Expr::Binary(it) => it.inkfmt(formatter),
            ty::Expr::Boolean(it) => it.inkfmt(formatter),
            ty::Expr::Call(it) => it.inkfmt(formatter),
            ty::Expr::Divert(it) => it.inkfmt(formatter),
            ty::Expr::Identifier(it) => it.inkfmt(formatter),
            ty::Expr::ListValues(it) => it.inkfmt(formatter),
            ty::Expr::Number(it) => it.inkfmt(formatter),
            ty::Expr::Paren(it) => it.inkfmt(formatter),
            ty::Expr::Postfix(it) => it.inkfmt(formatter),
            ty::Expr::QualifiedName(it) => it.inkfmt(formatter),
            ty::Expr::String(it) => it.inkfmt(formatter),
            ty::Expr::Unary(it) => it.inkfmt(formatter),
        }
    }
}

impl<'a> InkFmt<'a> for ty::Call<'a> {
    fn inkfmt(&self, formatter: &mut InkFormatter<'a>) -> InkFmtResult {
        match &mut self.name()? {
            ty::anon_unions::Identifier_QualifiedName::Identifier(it) => it.inkfmt(formatter),
            ty::anon_unions::Identifier_QualifiedName::QualifiedName(it) => it.inkfmt(formatter),
        }?;

        formatter.chr('(')?;
        self.args()
            .expect2("There must be Args here")
            .inkfmt(formatter)?;
        formatter.chr(')')
    }
}

impl<'a> InkFmt<'a> for ty::Args<'a> {}
impl<'a> InkFmt<'a> for ty::Identifier<'a> {}
impl<'a> InkFmt<'a> for ty::QualifiedName<'a> {}
impl<'a> InkFmt<'a> for ty::Divert<'a> {}
impl<'a> InkFmt<'a> for ty::Boolean<'a> {}
impl<'a> InkFmt<'a> for ty::ListValues<'a> {}
impl<'a> InkFmt<'a> for ty::Number<'a> {}
impl<'a> InkFmt<'a> for ty::Postfix<'a> {}
impl<'a> InkFmt<'a> for ty::String<'a> {}
impl<'a> InkFmt<'a> for ty::Unary<'a> {}
impl<'a> InkFmt<'a> for ty::Paren<'a> {}
impl<'a> InkFmt<'a> for ty::ContentBlock<'a> {}
impl<'a> InkFmt<'a> for ty::KnotBlock<'a> {}
impl<'a> InkFmt<'a> for ty::StitchBlock<'a> {}

impl<'a, T: TypedNode<'a>> InkFmt<'a> for ExtraOr<'a, T> {}

#[derive(Debug)]
pub struct InkFmtError(String);

impl Display for InkFmtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<ty::lib::IncorrectKind<'_>> for InkFmtError {
    fn from(value: ty::lib::IncorrectKind) -> Self {
        InkFmtError(format!(
            "Expected: {}, Found: {}",
            value.kind,
            value.actual_kind()
        ))
    }
}

type InkFmtResult = Result<(), InkFmtError>;

pub struct InkFormatter<'t> {
    input: &'t str,
    output: String,
    cursor: TreeCursor<'t>,
    config: FormatConfig,
}

impl<'t> InkFormatter<'t> {
    pub fn new(input: &'t str, cursor: TreeCursor<'t>, config: FormatConfig) -> Self {
        Self {
            input,
            config,
            cursor,
            output: String::new(),
        }
    }

    pub fn into_string(self) -> String {
        self.output
    }
}

impl<'t> InkFormatter<'t> {
    fn copy<'n, N: ty::lib::TypedNode<'n>>(&mut self, node: N) {
        self.output.push_str(&self.input[node.byte_range()]);
    }

    fn newline(&mut self) -> InkFmtResult {
        self.chr('\n');
        Ok(())
    }

    fn str(&mut self, str: &'_ str) -> InkFmtResult {
        self.output.push_str(str);
        Ok(())
    }

    fn chr(&mut self, chr: char) -> InkFmtResult {
        self.output.push(chr);
        Ok(())
    }
}
