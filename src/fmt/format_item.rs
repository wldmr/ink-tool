use crate::fmt::constrained_value::Constrained;
use std::fmt::{Debug, Write};

#[derive(PartialEq)]
pub struct Space {
    pub repeats: Constrained,
    pub existing: bool,
}

#[derive(PartialEq, Clone)]
pub enum FormatItem {
    /// Replace something that existist with nothing (i.e. delete)
    Nothing,
    Space(Constrained),
    Line(Constrained),
    // IDEA: Softline?
    Text(String),
}

impl Debug for FormatItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatItem::Nothing => f.write_char('⌧'),
            FormatItem::Space(it) => write!(f, "␣{:?}", it),
            FormatItem::Line(it) => write!(f, "⏎{:?}", it),
            FormatItem::Text(txt) => {
                f.write_char('\'')?;
                f.write_str(txt)?;
                f.write_char('\'')
            }
        }
    }
}
