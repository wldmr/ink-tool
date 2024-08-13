use std::mem;

use crate::{
    format_item::Align,
    formatter::columns::{ColumnId, ColumnWidths},
};

use super::format_item::FormatItem;

#[derive(Debug)]
pub struct Formatter(Vec<FormatItem>);

/// Abstract ID of a column
pub mod columns;

impl Formatter {
    pub(super) fn new_from_items(items: Vec<FormatItem>) -> Self {
        Self(items)
    }

    pub fn normalize(&mut self) {
        // dbg!("before normalization", &self.0);

        // This merging could be done directly while building the outputs,
        // but taking it in steps will help with debugging.
        let mut original = mem::take(&mut self.0).into_iter();

        if let Some(mut accumulator) = original.next() {
            while let Some(output) = original.next() {
                match accumulator.merge(output) {
                    Ok(merged) => accumulator = merged,
                    Err((left, right)) => {
                        self.0.push(left);
                        accumulator = right;
                    }
                }
            }
        }

        // dbg!("spaces collapsed", &self.0);

        // Insert leading aligns on linebreak
        // columns in consecutive lines share the same width
        let mut original = mem::take(&mut self.0).into_iter();
        let mut widths = ColumnWidths::new(); // keep track of all columns ever seen

        let mut virtual_columns: Vec<Vec<ColumnId>> = Vec::new();
        let mut columns: Vec<ColumnId> = Vec::new(); // on top of the virtual columns
        let mut column_index = 0; // again, on top
        let mut text_width = 0;
        while let Some(mut item) = original.next() {
            // dbg!(&item);
            if let FormatItem::Align(Align { ref mut column, .. }) = item {
                assert_eq!(*column, None);
                let column_id = if column_index < columns.len() {
                    *columns
                        .get(column_index)
                        .expect("we just checked that it exists")
                } else {
                    let new_column_id = widths.new_column();
                    columns.push(new_column_id);
                    new_column_id
                };
                widths.width_at_least(&column_id, text_width);
                *column = Some(column_id);
                text_width = 0;
                column_index += 1;
                self.0.push(item);
            } else if let FormatItem::AlignmentStart = item {
                virtual_columns.push(mem::take(&mut columns));
                column_index = 0;
                text_width = 0;
                self.0.push(item);
            } else if let FormatItem::AlignmentEnd = item {
                columns = virtual_columns
                    .pop()
                    .expect("Pushs and Pops should be balanced");
                // columns = Vec::new();
                column_index = 0;
                text_width = 0;
                self.0.push(item);
            } else if is_linebreak(&item) {
                column_index = 0;
                text_width = 0;
                self.0.push(item);
                for column_id in virtual_columns.iter().flatten() {
                    self.0.push(FormatItem::Align(Align {
                        column: Some(*column_id),
                        is_virtual: true,
                    }));
                }
            } else {
                // add item width to current column, remember if bigger than current max.
                text_width += width(&item);
                self.0.push(item);
            }
            // dbg!(&virtual_columns, &columns, &column_index, &text_width);
            // eprintln!("\n-------------------------\n");
        }

        // dbg!("widths calculated, columns assigned", &widths, &self.0);

        text_width = 0;
        for item in self.0.iter_mut() {
            if let FormatItem::Align(align) = item {
                let column_id = align.column.expect("We should have filled this");
                let max_column_width = widths.width(&column_id);
                let pad_width = max_column_width - text_width; // BUG: textwidh sometimes > max_column_width >:-(
                *item = FormatItem::Text(" ".repeat(pad_width));
            } else if is_linebreak(item) {
                text_width = 0;
            } else {
                text_width += width(&item);
            }
        }

        // dbg!("padding added", &self.0);
    }
}

fn is_linebreak(item: &FormatItem) -> bool {
    match item {
        FormatItem::Align { .. } => false,
        FormatItem::AlignmentStart => false,
        FormatItem::AlignmentEnd => false,
        FormatItem::Nothing => false,
        FormatItem::Antispace => false,
        FormatItem::Space => false,
        FormatItem::Newline => true,
        FormatItem::BlankLine => true,
        FormatItem::ExistingWhitespace(it) | FormatItem::Text(it) => it.contains("\n"),
    }
}

fn is_whitespace(item: &FormatItem) -> bool {
    match item {
        FormatItem::Align { .. } => false,
        FormatItem::AlignmentStart => false,
        FormatItem::AlignmentEnd => false,
        FormatItem::Nothing => true,
        FormatItem::Antispace => true,
        FormatItem::Space => true,
        FormatItem::Newline => true,
        FormatItem::BlankLine => true,
        FormatItem::ExistingWhitespace(it) | FormatItem::Text(it) => {
            it.chars().all(char::is_whitespace)
        }
    }
}

fn width(item: &FormatItem) -> usize {
    match item {
        FormatItem::Align { .. } => 0,
        FormatItem::AlignmentStart => 0,
        FormatItem::AlignmentEnd => 0,
        FormatItem::Nothing => 0,
        FormatItem::Antispace => 0,
        FormatItem::Space => 1,
        FormatItem::Newline => 0,
        FormatItem::BlankLine => 0,
        FormatItem::ExistingWhitespace(it) | FormatItem::Text(it) => it.len(), // we pretend byte == char here; hope that doesn't byte us …
    }
}

impl ToString for Formatter {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for output in self.0.iter() {
            result.push_str(output.as_str())
        }
        result
    }
}
