use std::fmt::{Debug, Formatter, Write};

use crate::util::constrained_value::Constrained;

#[derive(PartialEq)]
pub struct Space {
    pub repeats: Constrained,
    pub existing: bool,
}

#[derive(PartialEq)]
pub enum FormatItem {
    /// Replace something that existist with nothing (i.e. delete)
    Nothing,
    Space(Constrained),
    Line(Constrained),
    // IDEA: Softline?
    Text(String),
}

fn repeat_char(f: &mut Formatter<'_>, c: char, repeats: u8) -> std::fmt::Result {
    for _ in 0..repeats {
        f.write_char(c)?;
    }
    Ok(())
}
impl Debug for FormatItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatItem::Nothing => f.write_char('⌧'),
            FormatItem::Space(it) => repeat_char(f, '␣', it.value()),
            FormatItem::Line(it) => repeat_char(f, '⏎', it.value()),
            FormatItem::Text(txt) => {
                f.write_char('\'')?;
                f.write_str(txt)?;
                f.write_char('\'')
            }
        }
    }
}
