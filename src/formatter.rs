use std::fmt::{Debug, Display};

use tree_sitter::TreeCursor;
use tree_sitter_ink::node_types::{
    self as ty, anon_unions,
    lib::{ExtraOr, IncorrectKind, TypedNode},
    Identifier, ListValueDef,
};

use crate::config::FormatConfig;

pub trait InkFmt<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult;
}

impl<'a> InkFmt<'a> for ty::Ink<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        if let Some(block) = self.content() {
            block?.inkfmt(f)?
        }
        let mut cursor = f.new_cursor();
        for block in self.stitchs(&mut cursor) {
            f.fmt_ok(block)?
        }
        for block in self.knots(&mut cursor) {
            f.fmt_ok(block)?
        }
        Ok(())
    }
}

impl<'a> InkFmt<'a> for ty::ContentBlock<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        for child in self.children(&mut f.cursor.clone()) {
            f.fmt_ok(child)?;
        }
        f.newline()
    }
}

impl<'a> InkFmt<'a> for ty::StitchBlock<'a> {
    fn inkfmt(&self, formatter: &mut InkFormatter<'a>) -> InkFmtResult {
        formatter.newline()?;
        self.header()?.inkfmt(formatter)?;
        formatter.newline()?;
        if let Some(content) = self.content() {
            content?.inkfmt(formatter)?
        }
        formatter.newline()
    }
}

impl<'a> InkFmt<'a> for ty::KnotBlock<'a> {
    fn inkfmt(&self, formatter: &mut InkFormatter<'a>) -> InkFmtResult {
        formatter.newline()?;
        self.header()?.inkfmt(formatter)?;
        formatter.newline()?;
        if let Some(content) = self.content() {
            content?.inkfmt(formatter)?
        }
        formatter.newline()
    }
}

impl<'a> InkFmt<'a> for ty::Knot<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.str("=== ")?;
        f.fmt_ok(self.name())?;
        if self.function().is_some() {
            f.str("function ")?;
        }
        f.fmt_some(self.params())?;
        f.str(" ===")?;
        f.newline()
    }
}

impl<'a> InkFmt<'a> for ty::Stitch<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.str("= ")?;
        f.fmt_ok(self.name())?;
        f.fmt_some(self.params())?;
        f.newline()
    }
}

impl<'a> InkFmt<'a> for ty::Binary<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.fmt_ok(self.left())?;
        f.space()?;
        f.copy(&self.op()?)?;
        f.space()?;
        f.fmt_ok(self.right())
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
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        match &mut self.name()? {
            ty::anon_unions::Identifier_QualifiedName::Identifier(it) => it.inkfmt(f),
            ty::anon_unions::Identifier_QualifiedName::QualifiedName(it) => it.inkfmt(f),
        }?;

        f.chr('(')?;
        f.fmt_some(self.args())?;
        f.chr(')')
    }
}

impl<'a> InkFmt<'a> for ty::ListValues<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        let mut cursor = f.new_cursor();
        let mut children = self
            .node()
            .children(&mut cursor)
            .filter(|it| {
                eprintln!("{}", it.kind());
                it.kind() != ","
            })
            .map(|it| <ExtraOr<'a, Identifier>>::try_from(it))
            .peekable();
        while let Some(child) = children.next() {
            match child? {
                ExtraOr::Extra(extra) => handle_extra(&extra, f)?,
                ExtraOr::Regular(regular) => f.fmt(regular)?,
            }
            if let Some(Ok(ExtraOr::Regular(_))) = children.peek() {
                f.str(", ")?
            }
        }
        Ok(())
    }
}

impl<'a> InkFmt<'a> for ty::List<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.str("LIST ")?;
        f.fmt_ok(self.name())?;
        f.str(" = ")?;
        if is_multiline(self) {
            let range = &self.name()?.byte_range().len();
            f.indent_by_spaces(8 + range);
            f.fmt_ok(self.values())?;
            f.unindent();
        } else {
            f.fmt_ok(self.values())?;
        }
        f.newline()
    }
}

impl<'a> InkFmt<'a> for ty::ListValueDefs<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        let mut cursor = f.new_cursor();
        let is_multiline = self.start_position().row != self.end_position().row;
        let mut values = self
            .node()
            .children(&mut cursor)
            .filter(|it| it.kind() != ",")
            .map(|it| <ExtraOr<'a, ListValueDef>>::try_from(it))
            .peekable();
        while let Some(value) = values.next() {
            match value? {
                ExtraOr::Extra(extra) => handle_extra(&extra, f)?,
                ExtraOr::Regular(regular) => f.fmt(regular)?,
            }
            if let Some(Ok(ExtraOr::Regular(_))) = values.peek() {
                if is_multiline {
                    f.str(",")?;
                    f.newline()?
                } else {
                    f.str(", ")?
                }
            }
        }
        Ok(())
    }
}

impl<'a> InkFmt<'a> for ty::ListValueDef<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        let mut cursor = f.new_cursor();
        let has_parens = self.parens(&mut cursor).next().is_some();

        if has_parens {
            f.chr('(')?;
        }

        f.fmt_ok(self.name())?;

        if let Some(value) = self.value() {
            f.str(" = ")?;
            f.fmt_ok(value)?;
        }

        if has_parens {
            f.chr(')')?;
        }

        Ok(())
    }
}

impl<'a> InkFmt<'a> for ty::Args<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Paragraph<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::TodoComment<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Identifier<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::QualifiedName<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Divert<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Boolean<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Number<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.trim(*self) // No idea why this node includes whitespace. The regex doesn't include it.
    }
}
impl<'a> InkFmt<'a> for ty::Postfix<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::String<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Unary<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Paren<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Params<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Param<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::ParamValue<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Code<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::ChoiceBlock<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::GatherBlock<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::External<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Choice<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Global<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}
impl<'a> InkFmt<'a> for ty::Include<'a> {
    fn inkfmt(&self, f: &mut InkFormatter<'a>) -> InkFmtResult {
        f.copy(self)
    }
}

impl<'a> InkFmt<'a> for ty::Assignment<'a> {
    fn inkfmt(&self, formatter: &mut InkFormatter<'a>) -> InkFmtResult {
        self.name()?.inkfmt(formatter)?;

        formatter.copy(&self.op()?)?;
        self.value()?.inkfmt(formatter)
    }
}

impl<'a, T: InkFmt<'a>> InkFmt<'a> for ExtraOr<'a, T> {
    fn inkfmt(&self, formatter: &mut InkFormatter<'a>) -> InkFmtResult {
        match self {
            ExtraOr::Extra(node) => handle_extra(node, formatter),
            ExtraOr::Regular(regular) => regular.inkfmt(formatter),
        }
    }
}

fn handle_extra<'t>(node: &tree_sitter::Node<'t>, f: &mut InkFormatter<'t>) -> InkFmtResult {
    if let Ok(comment) = ty::Comment::try_from(*node) {
        let start = &f.input[comment.start_byte()..comment.start_byte() + 2];
        f.copy(&comment)?;
        if start == "//" {
            f.newline()?;
        }
    }
    Ok(())
}

impl<'a> InkFmt<'a>
    for anon_unions::ChoiceBlock_Code_External_GatherBlock_Global_Include_List_Paragraph_TodoComment<
        'a,
    >
{
    fn inkfmt(&self, formatter: &mut super::InkFormatter<'a>) -> InkFmtResult {
        match self {
            anon_unions::ChoiceBlock_Code_External_GatherBlock_Global_Include_List_Paragraph_TodoComment::ChoiceBlock(it) => it.inkfmt(formatter),
            anon_unions::ChoiceBlock_Code_External_GatherBlock_Global_Include_List_Paragraph_TodoComment::Code(it) => it.inkfmt(formatter),
            anon_unions::ChoiceBlock_Code_External_GatherBlock_Global_Include_List_Paragraph_TodoComment::External(it) => it.inkfmt(formatter),
            anon_unions::ChoiceBlock_Code_External_GatherBlock_Global_Include_List_Paragraph_TodoComment::GatherBlock(it) => it.inkfmt(formatter),
            anon_unions::ChoiceBlock_Code_External_GatherBlock_Global_Include_List_Paragraph_TodoComment::Global(it) => it.inkfmt(formatter),
            anon_unions::ChoiceBlock_Code_External_GatherBlock_Global_Include_List_Paragraph_TodoComment::Include(it) => it.inkfmt(formatter),
            anon_unions::ChoiceBlock_Code_External_GatherBlock_Global_Include_List_Paragraph_TodoComment::List(it) => it.inkfmt(formatter),
            anon_unions::ChoiceBlock_Code_External_GatherBlock_Global_Include_List_Paragraph_TodoComment::Paragraph(it) => it.inkfmt(formatter),
            anon_unions::ChoiceBlock_Code_External_GatherBlock_Global_Include_List_Paragraph_TodoComment::TodoComment(it) => it.inkfmt(formatter),
        }
    }
}

impl<'a> InkFmt<'a> for anon_unions::Identifier_QualifiedName<'a> {
    fn inkfmt(&self, formatter: &mut super::InkFormatter<'a>) -> InkFmtResult {
        match self {
            anon_unions::Identifier_QualifiedName::Identifier(it) => it.inkfmt(formatter),
            anon_unions::Identifier_QualifiedName::QualifiedName(it) => it.inkfmt(formatter),
        }
    }
}

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
            "{}:{}: Expected {}, Found {}",
            value.node.start_position().row + 1,
            value.node.start_position().column + 1,
            value.kind,
            value.actual_kind()
        ))
    }
}

type InkFmtResult = Result<(), InkFmtError>;

pub struct InkFormatter<'t> {
    input: &'t str,
    output: String,
    indents: Vec<String>,
    current_indent: String,
    cursor: TreeCursor<'t>,
    _config: FormatConfig,
}

impl<'t> InkFormatter<'t> {
    pub fn new(input: &'t str, cursor: TreeCursor<'t>, config: FormatConfig) -> Self {
        Self {
            input,
            _config: config,
            indents: Vec::new(),
            current_indent: String::new(),
            cursor,
            output: String::with_capacity(input.len()),
        }
    }

    pub fn into_string(self) -> String {
        self.output
    }
}

impl<'t> InkFormatter<'t> {
    fn copy<'n, N: TypedNode<'t>>(&mut self, node: &N) -> InkFmtResult {
        self.output.push_str(&self.input[node.byte_range()]);
        Ok(())
    }

    /// Some nodes seem to come with whitespace, even though the grammar doesn't allow it. Not sure why. :-/
    fn trim<'n, N: ty::lib::TypedNode<'n>>(&mut self, node: N) -> InkFmtResult {
        self.output.push_str(&self.input[node.byte_range()].trim());
        Ok(())
    }

    fn fmt<T: InkFmt<'t>>(&mut self, node: T) -> InkFmtResult {
        node.inkfmt(self)
    }

    fn fmt_ok<T: InkFmt<'t>>(&mut self, node: Result<T, IncorrectKind>) -> InkFmtResult {
        node?.inkfmt(self)
    }

    fn fmt_some<T: InkFmt<'t>>(&mut self, node: Option<Result<T, IncorrectKind>>) -> InkFmtResult {
        if let Some(node) = node {
            node?.inkfmt(self)
        } else {
            Ok(())
        }
    }

    fn newline(&mut self) -> InkFmtResult {
        self.chr('\n')?;
        self.str(&self.current_indent.clone())
    }

    fn space(&mut self) -> InkFmtResult {
        self.chr(' ')
    }

    fn str(&mut self, str: &'_ str) -> InkFmtResult {
        self.output.push_str(str);
        Ok(())
    }

    fn chr(&mut self, chr: char) -> InkFmtResult {
        self.output.push(chr);
        Ok(())
    }

    fn new_cursor(&self) -> TreeCursor<'t> {
        self.cursor.clone()
    }

    fn indent_by(&mut self, s: String) {
        self.indents.push(s);
        self.current_indent = self.indents.join("");
    }

    fn indent_by_spaces(&mut self, count: usize) {
        self.indent_by(" ".repeat(count));
    }

    fn unindent(&mut self) {
        self.indents.pop();
        self.current_indent = self.indents.join("");
    }
}

fn is_multiline<'t, T>(node: &T) -> bool
where
    T: TypedNode<'t>,
{
    node.start_position().row != node.end_position().row
}
