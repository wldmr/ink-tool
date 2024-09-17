use std::fmt::Debug;

use crate::util::constrained_value::Constrained;

pub trait InkFmt {
    fn indent(&mut self);
    fn dedent(&mut self);
    fn align_indent_to_current_column(&mut self);

    fn space(&mut self, repeats: impl Into<Constrained>);
    fn line(&mut self, repeats: impl Into<Constrained>);

    fn text(&mut self, s: &str);
}

impl<T: InkFmt> InkFmt for &mut T {
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

#[derive(Debug)]
pub struct InkFormatter<T> {
    pub downstream: T,
    buffer_item: Option<Bufferable>,
    indents: Vec<usize>,
    relative_indent: i8,
    current_column: usize,
    alignment: Option<Alignment>,
}

enum Bufferable {
    Whitespace(Vec<Whitespace>),
    Text(String),
}

enum Whitespace {
    Space(Constrained),
    Newline(Constrained),
}

fn collapse_whitespace(input: Vec<Whitespace>) -> Whitespace {
    let mut spaces = Constrained::new();
    let mut newlines = Constrained::new();
    for item in input {
        match item {
            Whitespace::Space(constraint) => spaces.combine_mut(constraint),
            Whitespace::Newline(constraint) => newlines.combine_mut(constraint),
        }
    }
    if newlines.value() > 0 {
        Whitespace::Newline(newlines)
    } else {
        Whitespace::Space(spaces)
    }
}

impl std::fmt::Debug for Bufferable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bufferable::Whitespace(it) => it.fmt(f),
            Bufferable::Text(it) => write!(f, "'{:?}'", it),
        }
    }
}

impl std::fmt::Debug for Whitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Whitespace::Space(constraint) => write!(f, "␣{:?}", constraint),
            Whitespace::Newline(constraint) => write!(f, "⏎{:?}", constraint),
        }
    }
}

impl<T: InkFmt> InkFmt for InkFormatter<T> {
    fn indent(&mut self) {
        self.relative_indent += 1;
    }

    fn dedent(&mut self) {
        self.relative_indent -= 1;
    }

    fn align_indent_to_current_column(&mut self) {
        self.relative_indent += 1;
        self.alignment = Some(Alignment::Tentative);
    }

    fn space(&mut self, repeats: impl Into<Constrained>) {
        // wrapping into the vec here seems inefficient.
        self.handle_next_bufferable(Bufferable::Whitespace(vec![Whitespace::Space(
            repeats.into(),
        )]))
    }

    fn line(&mut self, repeats: impl Into<Constrained>) {
        self.handle_next_bufferable(Bufferable::Whitespace(vec![Whitespace::Newline(
            repeats.into(),
        )]))
    }

    fn text(&mut self, s: &str) {
        self.handle_next_bufferable(Bufferable::Text(s.to_owned()));
    }
}

#[derive(Debug)]
enum Alignment {
    Tentative,
    Determined(usize),
}

impl<T: InkFmt> InkFormatter<T> {
    pub fn new(downstream: T) -> Self {
        Self {
            downstream,
            buffer_item: None,
            indents: vec![0],
            relative_indent: 0,
            current_column: 0,
            alignment: None,
        }
    }

    fn handle_next_bufferable(&mut self, next: Bufferable) {
        if self.buffer_item.is_none() {
            self.buffer_item = Some(next);
            return;
        }

        let next_is_text = matches!(next, Bufferable::Text(_));

        let buf = self
            .buffer_item
            .take()
            .expect("We just checked that it's filled");

        use Bufferable as B;
        self.buffer_item = Some(match (buf, next) {
            (B::Whitespace(mut a), B::Whitespace(b)) => {
                a.extend(b);
                B::Whitespace(a)
            }
            (B::Whitespace(ws), text @ B::Text(_)) => {
                match collapse_whitespace(ws) {
                    Whitespace::Space(spaces) => {
                        self.downstream.space(spaces);
                        self.current_column += spaces.value() as usize;
                        text
                    }
                    Whitespace::Newline(newlines) => {
                        // new text after a line break. This is where we must handle indentation, and only here!
                        self.downstream.line(newlines);
                        if self.relative_indent > 0 {
                            let next_indent = match self.alignment {
                                Some(Alignment::Determined(column)) => column,
                                Some(Alignment::Tentative) => {
                                    eprintln!("Tentative alignment at newline. Is this a bug?");
                                    *self.indents.last().expect("this shouldn't be empty") + 4
                                }
                                None => *self.indents.last().expect("this shouldn't be empty") + 4,
                            };
                            for _ in 0..self.relative_indent {
                                self.indents.push(next_indent);
                            }
                        } else if self.relative_indent < 0 {
                            for _ in self.relative_indent..0 {
                                self.indents.pop();
                            }
                        }
                        self.relative_indent = 0;
                        self.alignment = None;
                        let column_for_next_line =
                            *self.indents.last().expect("this shouldn't be empty");
                        self.downstream.space(column_for_next_line);
                        self.current_column = column_for_next_line;

                        text
                    }
                }
            }
            (B::Text(mut l), B::Text(r)) => {
                l.push_str(&r);
                B::Text(l)
            }
            (B::Text(t), whitespace @ B::Whitespace(_)) => {
                self.current_column += t.len();
                self.downstream.text(&t);
                whitespace
            }
        });
        if next_is_text && matches!(self.alignment, Some(Alignment::Tentative)) {
            self.alignment = Some(Alignment::Determined(self.current_column));
        }
    }
}

#[derive(Debug)]
pub struct Tracing<I> {
    pub downstream: I,
    pub trace: String,
}

impl<T> Tracing<T> {
    pub fn new(inner: T) -> Self {
        Self {
            downstream: inner,
            trace: String::new(),
        }
    }

    fn sep_space(&mut self) {
        if !self.trace.is_empty() && !self.trace.ends_with('\n') {
            self.trace.push(' ')
        }
    }
}

impl<T: InkFmt> InkFmt for Tracing<T> {
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

impl InkFmt for &mut String {
    fn indent(&mut self) {}

    fn dedent(&mut self) {}

    fn align_indent_to_current_column(&mut self) {}

    fn space(&mut self, repeats: impl Into<Constrained>) {
        let repeats: Constrained = repeats.into();
        for _ in 0..repeats.value() {
            self.push(' ')
        }
    }

    fn line(&mut self, repeats: impl Into<Constrained>) {
        let repeats: Constrained = repeats.into();
        for _ in 0..repeats.value() {
            self.push('\n')
        }
    }

    fn text(&mut self, s: &str) {
        self.push_str(s)
    }
}
