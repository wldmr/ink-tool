use std::{any::Any, borrow::Borrow, collections::HashMap, fmt::Alignment};

use crate::MatchIndex;

use super::format_item::FormatItem;

#[derive(Debug)]
pub struct Formatter(Vec<FormatItem>);

impl Formatter {
    pub(super) fn new(items: Vec<FormatItem>) -> Self {
        Self(items)
    }

    pub fn normalize(&mut self) {
        // This merging could be done directly while building the outputs,
        // but taking it in steps will help with debugging.
        let mut original = std::mem::take(&mut self.0).into_iter();

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

        // Collect the alignment widths
        let mut blockwidths: HashMap<MatchIndex, AlignGroup> = HashMap::new();
        for item in self.0.iter() {
            if is_linebreak(item) {
                for group in blockwidths.values_mut() {
                    group.column_index = 0;
                }
            }
            if let FormatItem::Align(align) = item {
                let group = blockwidths.entry(align.match_id).or_default();
                if let Some(width) = group.bytes_since_newline.get_mut(group.column_index) {
                    if align.pos.column > *width {
                        *width = align.pos.column;
                    }
                } else {
                    group.bytes_since_newline.push(align.pos.column);
                }
            }
        }

        dbg!(&blockwidths);

        // Distribute the alignment widths over all alignments
        for item in self.0.iter_mut() {
            if is_linebreak(item) {
                for group in blockwidths.values_mut() {
                    group.column_index = 0;
                }
            }
            if let FormatItem::Align(align) = item {
                let group = blockwidths
                    .get(&align.match_id)
                    .expect("there should be a block for every match");
                let width = group
                    .bytes_since_newline
                    .get(group.column_index)
                    .expect("We should have calculated the correct number of columns");
                align.pos.column = *width;
            }
        }

        // Replace the alignments with actual spaces
        let mut current_column = 0;
        for item in self.0.iter_mut() {
            if is_linebreak(item) {
                for group in blockwidths.values_mut() {
                    group.column_index = 0;
                }
                current_column = 0;
            } else {
                current_column += width(&item);
            }
            if let FormatItem::Align(align) = item {
                let group = blockwidths
                    .get(&align.match_id)
                    .expect("there should be a block for every match");
                let width = group
                    .bytes_since_newline
                    .get(group.column_index)
                    .expect("We should have calculated the correct number of columns");
                let remaining_width = width.saturating_sub(current_column);
                let pad = " ".repeat(remaining_width);
                *item = FormatItem::Text(pad);
            }
        }
    }
}

fn is_linebreak(item: &FormatItem) -> bool {
    match item {
        FormatItem::Align(_) => false,
        FormatItem::Nothing => false,
        FormatItem::Antispace => false,
        FormatItem::Space => false,
        FormatItem::Newline => true,
        FormatItem::BlankLine => true,
        FormatItem::ExistingWhitespace(it) | FormatItem::Text(it) => it.contains("\n"),
    }
}

fn width(item: &FormatItem) -> usize {
    match item {
        FormatItem::Align(_) => 0,
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

#[derive(Default, Debug)]
struct AlignGroup {
    bytes_since_newline: Vec<usize>,
    /// the index into the vector, not the byte offset
    column_index: usize,
}
