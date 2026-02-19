use crate::fmt::{constrained_value::Constrained, formatting::Formatting};

/// Keep a log of formatting instructions.
///
/// Useful for finding out how different layers of formatters change the instructions.
#[derive(Debug)]
pub(crate) struct Tracing<I> {
    pub(crate) downstream: I,
    pub(crate) trace: String,
}

impl<T> Tracing<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self {
            downstream: inner,
            trace: String::new(),
        }
    }

    pub(crate) fn sep_space(&mut self) {
        if !self.trace.is_empty() && !self.trace.ends_with('\n') {
            self.trace.push(' ')
        }
    }
}

impl<T: Formatting> Formatting for Tracing<T> {
    fn indent(&mut self) {
        self.downstream.indent();
        self.sep_space();
        self.trace.push('›');
    }

    fn dedent(&mut self) {
        self.downstream.dedent();
        self.sep_space();
        self.trace.push('‹');
    }

    fn align_indent_to_current_column(&mut self) {
        self.downstream.align_indent_to_current_column();
        self.sep_space();
        self.trace.push('»');
    }

    fn space(&mut self, repeats: impl Into<Constrained>) {
        let repeats: Constrained = repeats.into();
        self.sep_space();
        self.trace.push_str(&format!("␣{:?}", repeats));
        self.downstream.space(repeats);
    }

    fn line(&mut self, repeats: impl Into<Constrained>) {
        let repeats: Constrained = repeats.into();
        self.downstream.line(repeats);
        self.sep_space();
        self.trace.push_str(&format!("⏎{:?}", repeats));
        self.trace.push('\n');
    }

    fn text(&mut self, s: &str) {
        self.downstream.text(s);
        self.sep_space();
        self.trace.push('\'');
        self.trace.push_str(s);
        self.trace.push('\'');
    }
}
