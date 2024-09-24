use crate::fmt::util::constrained_value::Constrained;

mod formatters;
mod whitespace;

pub(crate) use formatters::layout::Layout;
pub(crate) use formatters::tracing::Tracing;

/// The operations to format a document with.
pub trait Formatting {
    fn indent(&mut self);
    fn dedent(&mut self);
    fn align_indent_to_current_column(&mut self);

    fn space(&mut self, repeats: impl Into<Constrained>);
    fn line(&mut self, repeats: impl Into<Constrained>);

    fn text(&mut self, s: &str);
}

impl<T: Formatting> Formatting for &mut T {
    fn indent(&mut self) {
        (*self).indent()
    }

    fn dedent(&mut self) {
        (*self).dedent()
    }

    fn align_indent_to_current_column(&mut self) {
        (*self).align_indent_to_current_column()
    }

    fn space(&mut self, repeats: impl Into<Constrained>) {
        (*self).space(repeats)
    }

    fn line(&mut self, repeats: impl Into<Constrained>) {
        (*self).line(repeats)
    }

    fn text(&mut self, s: &str) {
        (*self).text(s)
    }
}
