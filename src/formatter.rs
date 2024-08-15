use std::mem;

use super::format_item::FormatItem;

#[derive(Debug)]
pub struct Formatter(Vec<FormatItem>);

/// Abstract ID of a column
pub mod columns;
pub mod nested_columns;

impl Formatter {
    pub(super) fn new_from_items(items: Vec<FormatItem>) -> Self {
        Self(items)
    }

    pub fn normalize(&mut self) {
        dbg!("before normalization", &self.0);

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

        dbg!("spaces collapsed", &self.0);

        // columns in consecutive lines share the same width
        let mut original = mem::take(&mut self.0).into_iter();

        let mut indent_widths: Vec<usize> = vec![0];
        let mut current_column = 0;
        while let Some(item) = original.next() {
            eprintln!(
                "Item: {:?}, Column: {:?}. Indents = {:?}",
                &item, &current_column, &indent_widths
            );
            if let FormatItem::Indent { is_anchor } = item {
                if is_anchor {
                    // an anchor supercedes the previous indent
                    eprintln!("replace {current_column}");
                    let top_indent = indent_widths.last_mut().expect("this mustn't be empty");
                    *top_indent = current_column;
                } else {
                    eprintln!("add {current_column} + 2");
                    // just a bog-standard indent. IDEA: Make the indent configurable? Not sure Ink needs this, but it seems like the thing to do.
                    indent_widths.push(current_column + 2);
                }
            } else if let FormatItem::Dedent = item {
                current_column = indent_widths.pop().expect("we don't make mistakes");
            } else if is_linebreak(&item) {
                current_column = *indent_widths.last().expect("we _don't_ make mistakes");
                let pad = " ".repeat(current_column);
                let pad = FormatItem::Text(pad);
                self.0.push(item);
                self.0.push(pad);
            } else {
                current_column += width(&item);
                self.0.push(item);
            }
        }
        dbg!(&current_column, &indent_widths);

        // dbg!("padding added", &self.0);
    }
}

fn is_linebreak(item: &FormatItem) -> bool {
    match item {
        FormatItem::Indent { .. } => false,
        FormatItem::Dedent => false,
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
        FormatItem::Indent { .. } => 0,
        FormatItem::Dedent => 0,
        FormatItem::Nothing => 0,
        FormatItem::Antispace => 0,
        FormatItem::Space => 1,
        FormatItem::Newline => 0,
        FormatItem::BlankLine => 0,
        FormatItem::ExistingWhitespace(it) => it
            .split('\n')
            .rev()
            .find_map(|it| Some(it.len()))
            .unwrap_or_default(),
        FormatItem::Text(it) => it.len(),
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
