use crate::fmt::{
    formatting::{whitespace, Formatting},
    util::constrained_value::Constrained,
};

use std::fmt::Debug;

/// Takes care of actually layouting a document (spacing, indentation, etc)
#[derive(Debug)]
pub(crate) struct Layout<T> {
    downstream: T,
    buffer_item: Option<Bufferable>,
    indents: Vec<usize>,
    relative_indent: i8,
    current_column: usize,
    alignment: Option<Alignment>,
}

/// When aligning, we need to wait for all the content to come in before we can decide how many spaces to add.
#[derive(Debug)]
enum Alignment {
    Tentative,
    Determined(usize),
}

impl<T: Formatting> Formatting for Layout<T> {
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
        self.handle_next_bufferable({
            Bufferable::Whitespace({
                whitespace::Undecided {
                    space: repeats.into(),
                    newline: Constrained::new(),
                }
            })
        })
    }

    fn line(&mut self, repeats: impl Into<Constrained>) {
        self.handle_next_bufferable({
            Bufferable::Whitespace({
                whitespace::Undecided {
                    space: Constrained::new(),
                    newline: repeats.into(),
                }
            })
        })
    }

    fn text(&mut self, s: &str) {
        self.handle_next_bufferable(Bufferable::Text(s.to_owned()));
    }
}

impl<T: Formatting> Layout<T> {
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

    pub(crate) fn handle_next_bufferable(&mut self, next: Bufferable) {
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
            (B::Whitespace(a), B::Whitespace(b)) => B::Whitespace(a + b),
            (B::Whitespace(undecided), text @ B::Text(_)) => {
                match whitespace::Whitespace::from(undecided) {
                    whitespace::Whitespace::Space(spaces) => {
                        self.downstream.space(spaces);
                        self.current_column += spaces.value() as usize;
                        text
                    }
                    whitespace::Whitespace::Newline(newlines) => {
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
            (B::Text(l), B::Text(r)) => B::Text(l + &r),
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

/// Buffer to hold the next piece of content.
///
/// This enables us to look at all the whitespace constraints before deciding what to actually output
/// (how many spaces or newlines).
pub(crate) enum Bufferable {
    Whitespace(whitespace::Undecided),
    Text(String),
}

impl std::fmt::Debug for Bufferable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bufferable::Whitespace(it) => it.fmt(f),
            Bufferable::Text(it) => write!(f, "'{:?}'", it),
        }
    }
}
