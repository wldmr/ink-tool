use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ColumnId(usize);

impl std::fmt::Debug for ColumnId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

type ColumnWidth = usize;

#[derive(Debug)]
pub struct ColumnWidths(Vec<ColumnWidth>);

impl ColumnWidths {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_column(&mut self) -> ColumnId {
        let new_id = self.0.len();
        self.0.push(0);
        ColumnId(new_id)
    }

    pub fn width(&self, id: &ColumnId) -> ColumnWidth {
        *self
            .0
            .get(id.0)
            .expect("If we gave it out, it should exist")
    }

    pub fn width_at_least(&mut self, id: &ColumnId, width: ColumnWidth) -> bool {
        let entry = self
            .0
            .get_mut(id.0)
            .expect("If we gave it out, it should exist");
        if width >= *entry {
            *entry = width;
            true
        } else {
            false
        }
    }
}

pub struct ColumnStack {
    stack: Vec<ColumnId>,
    current_column: usize,
    widths: ColumnWidths,
}

impl ColumnStack {
    pub fn new() -> Self {
        let mut widths = ColumnWidths::new();
        let first_column_id = widths.new_column();
        let stack = vec![first_column_id];
        Self {
            stack,
            current_column: 0,
            widths,
        }
    }

    pub fn next_column(&mut self) {
        self.current_column += 1;
        if self.current_column == self.stack.len() {
            let new_id = self.widths.new_column();
            self.stack.push(new_id);
        }
    }

    pub fn next_line(&mut self) {
        self.current_column = 0;
    }

    /// remember the current column and return to it every time `next_line()` is called.
    pub fn start_group(&mut self) {
        todo!()
    }

    /// remember the current column and return to it every time `next_line()` is called.
    pub fn end_group(&mut self) {
        todo!()
    }

    pub fn current_column_width_at_least(&mut self, width: usize) {
        let column_id = *self
            .stack
            .get(self.current_column)
            .expect("We should have pushed this");
        self.widths.width_at_least(&column_id, width);
    }
}
