use std::fmt::{Debug, Write};

pub trait InkFmt {
    fn indent(&mut self);
    fn dedent(&mut self);
    fn align_indent_to_current_column(&mut self);

    fn space(&mut self, max_repeats: usize);
    fn antispace(&mut self);
    fn line(&mut self, max_repeats: usize);

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

    fn space(&mut self, max_repeats: usize) {
        (*self).space(max_repeats)
    }

    fn antispace(&mut self) {
        (*self).antispace()
    }

    fn line(&mut self, max_repeats: usize) {
        (*self).line(max_repeats)
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
    relative_indent: isize,
    current_column: usize,
    aligment: Option<Alignment>,
}

impl<T: Default> Default for InkFormatter<T> {
    fn default() -> Self {
        Self {
            downstream: T::default(),
            buffer_item: None,
            indents: vec![0],
            relative_indent: 0,
            current_column: 0,
            aligment: None,
        }
    }
}

enum Bufferable {
    Antispace,
    Space(BufData),
    Line(BufData),
    Text(String),
}

impl std::fmt::Debug for Bufferable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bufferable::Antispace => f.write_char('⁀'),
            Bufferable::Space(it) => {
                for _ in 0..it.repeats {
                    f.write_char('␣')?;
                }
                Ok(())
            }
            Bufferable::Line(it) => {
                for _ in 0..it.repeats {
                    f.write_char('⏎')?;
                }
                Ok(())
            }
            Bufferable::Text(it) => f.write_fmt(format_args!("'{:?}'", it)),
        }
    }
}

#[derive(Debug)]
struct BufData {
    repeats: usize,
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
        self.aligment = Some(Alignment::Tentative);
    }

    fn space(&mut self, repeats: usize) {
        self.handle_next_bufferable(Bufferable::Space(BufData { repeats }))
    }

    fn antispace(&mut self) {
        self.handle_next_bufferable(Bufferable::Antispace)
    }

    fn line(&mut self, repeats: usize) {
        self.handle_next_bufferable(Bufferable::Line(BufData { repeats }))
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
            aligment: None,
        }
    }

    fn handle_next_bufferable(&mut self, next: Bufferable) {
        if self.buffer_item.is_none() {
            self.buffer_item = Some(next);
            return;
        }

        let next_is_text = matches!(next, Bufferable::Text(_));

        let mut buf = self
            .buffer_item
            .as_mut()
            .expect("We just checked that it's filled");

        use Bufferable::*;
        match (&mut buf, &next) {
            // Antispace eats any space
            (Antispace, Antispace) => {}
            (Antispace, Space(_)) => {}
            (Space(_), Antispace) => *buf = next,

            // Antispace is replaced by any non-space
            (Antispace, _) => *buf = next,

            (Space(a), Space(b)) => {
                if a.repeats < b.repeats {
                    a.repeats = b.repeats;
                }
            }
            (Space { .. }, Line { .. }) => {
                // spaces at line ends are dropped
                self.buffer_item = Some(next)
            }

            (Line { .. }, Space { .. } | Antispace) => {
                // spaces at line starts are dropped
            }

            (Line(a), Line(b)) => {
                if a.repeats < b.repeats {
                    a.repeats = b.repeats;
                }
            }

            (Space(sp), Text(_)) => {
                let spaces = sp.repeats;
                self.downstream.space(spaces);
                self.current_column += spaces;
                *buf = next;
            }

            (Line(line), Text(_)) => {
                // new text after a line break. This is where we must handle indentation, and only here!
                let newlines = line.repeats;
                self.downstream.line(newlines);
                if self.relative_indent > 0 {
                    let next_indent = match self.aligment {
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
                self.aligment = None;
                let column_for_next_line = *self.indents.last().expect("this shouldn't be empty");
                self.downstream.space(column_for_next_line);
                self.current_column = column_for_next_line;

                *buf = next;
            }
            (Text(l), Text(r)) => l.push_str(r),
            (Text(t), _) => {
                self.current_column += t.len();
                self.downstream.text(t);
                *buf = next;
            }
        }
        if next_is_text && matches!(self.aligment, Some(Alignment::Tentative)) {
            self.aligment = Some(Alignment::Determined(self.current_column));
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
        self.trace.push('→');
    }

    fn dedent(&mut self) {
        self.downstream.dedent();
        self.sep_space();
        self.trace.push('←');
    }

    fn align_indent_to_current_column(&mut self) {
        self.downstream.align_indent_to_current_column();
        self.sep_space();
        self.trace.push('|');
    }

    fn space(&mut self, max_repeats: usize) {
        self.downstream.space(max_repeats);
        self.sep_space();
        for _ in 0..max_repeats {
            self.trace.push('␣');
        }
    }

    fn antispace(&mut self) {
        self.downstream.antispace();
        self.sep_space();
        self.trace.push('⁀');
    }

    fn line(&mut self, max_repeats: usize) {
        self.downstream.line(max_repeats);
        self.sep_space();
        for _ in 0..max_repeats {
            self.trace.push('⏎');
        }
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

    fn space(&mut self, max_repeats: usize) {
        for _ in 0..max_repeats {
            self.push(' ')
        }
    }

    fn antispace(&mut self) {}

    fn line(&mut self, max_repeats: usize) {
        for _ in 0..max_repeats {
            self.push('\n')
        }
    }

    fn text(&mut self, s: &str) {
        self.push_str(s)
    }
}
