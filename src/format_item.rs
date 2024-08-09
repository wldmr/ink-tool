use std::fmt::{Debug, Write};

use tree_sitter::Point;

use super::PatternIndex;

#[derive(PartialEq)]
pub(crate) enum FormatItem {
    Nothing,
    Antispace,
    Space,
    Newline,
    BlankLine,
    ExistingWhitespace(String),
    Text(String),
}

impl Debug for FormatItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatItem::Nothing => f.write_char('⌧'),
            FormatItem::Antispace => f.write_char('⁀'),
            FormatItem::Space => f.write_char('␣'),
            FormatItem::Newline => f.write_char('⏎'),
            FormatItem::BlankLine => f.write_char('¶'),
            FormatItem::ExistingWhitespace(sp) => {
                f.write_char('∃')?;
                Debug::fmt(&sp, f)
            }
            FormatItem::Text(txt) => f.write_fmt(format_args!("'{txt}'")),
        }
    }
}

impl FormatItem {
    pub fn merge(self, other: FormatItem) -> Result<FormatItem, (FormatItem, FormatItem)> {
        match (&self, &other) {
            (Self::Nothing, _) => Ok(other),
            (_, Self::Nothing) => Ok(self),
            (Self::Antispace, Self::Antispace) => Ok(Self::Antispace),
            (Self::Antispace, Self::Space) => Ok(Self::Antispace),
            (Self::Antispace, Self::Newline) => Ok(Self::Antispace),
            (Self::Antispace, Self::BlankLine) => Ok(Self::Antispace),
            (Self::Antispace, Self::ExistingWhitespace(_)) => Ok(Self::Antispace),
            (Self::Antispace, Self::Text(_)) => Ok(other),
            (Self::Space, Self::Antispace) => Ok(Self::Antispace),
            (Self::Space, Self::Space) => Ok(Self::Space),
            (Self::Space, Self::Newline) => Ok(Self::Newline),
            (Self::Space, Self::BlankLine) => Ok(Self::BlankLine),
            (Self::Space, Self::ExistingWhitespace(_)) => Ok(Self::Space),
            (Self::Space, Self::Text(_)) => Err((self, other)),
            (Self::Newline, Self::Antispace) => Ok(Self::Antispace),
            (Self::Newline, Self::Space) => Ok(Self::Newline),
            (Self::Newline, Self::Newline) => Ok(Self::Newline),
            (Self::Newline, Self::BlankLine) => Ok(Self::BlankLine),
            (Self::Newline, Self::ExistingWhitespace(_)) => Ok(Self::Newline),
            (Self::Newline, Self::Text(_)) => Err((self, other)),
            (Self::BlankLine, Self::Antispace) => Ok(Self::Antispace),
            (Self::BlankLine, Self::Space) => Ok(Self::BlankLine),
            (Self::BlankLine, Self::Newline) => Ok(Self::BlankLine),
            (Self::BlankLine, Self::BlankLine) => Ok(Self::BlankLine),
            (Self::BlankLine, Self::ExistingWhitespace(_)) => Ok(Self::BlankLine),
            (Self::BlankLine, Self::Text(_)) => Err((self, other)),
            (Self::ExistingWhitespace(_), Self::Antispace) => Ok(Self::Antispace),
            (Self::ExistingWhitespace(_), Self::Space) => Ok(Self::Space),
            (Self::ExistingWhitespace(_), Self::Newline) => Ok(Self::Newline),
            (Self::ExistingWhitespace(_), Self::BlankLine) => Ok(Self::BlankLine),
            // Two adjacent nodes reported the same whitespace as after and before respectively, so we can absorb one
            (Self::ExistingWhitespace(a), Self::ExistingWhitespace(b)) if a == b => Ok(self),
            (Self::ExistingWhitespace(_), Self::ExistingWhitespace(_)) => Err((self, other)),
            (Self::ExistingWhitespace(_), Self::Text(_)) => Err((self, other)),
            (Self::Text(_), Self::Antispace) => Err((self, other)),
            (Self::Text(_), Self::Space) => Err((self, other)),
            (Self::Text(_), Self::Newline) => Err((self, other)),
            (Self::Text(_), Self::BlankLine) => Err((self, other)),
            (Self::Text(_), Self::ExistingWhitespace(_)) => Err((self, other)),
            (Self::Text(_), Self::Text(_)) => Err((self, other)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            FormatItem::Nothing => "",
            FormatItem::Antispace => "",
            FormatItem::Space => " ",
            FormatItem::Newline => "\n",
            FormatItem::BlankLine => "\n\n",
            FormatItem::ExistingWhitespace(ws) => ws,
            FormatItem::Text(txt) => txt,
        }
    }
}

impl ToString for FormatItem {
    fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}
#[derive(PartialEq, Eq)]
pub(crate) struct Align {
    pub(crate) pattern: PatternIndex,
    pub(crate) pos: Point,
}

impl Debug for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}|{}:{}",
            self.pattern, self.pos.row, self.pos.column
        ))
    }
}
