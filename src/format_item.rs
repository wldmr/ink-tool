use std::fmt::{Debug, Write};

use crate::formatter::columns::ColumnId;

#[derive(PartialEq)]
pub(crate) enum FormatItem {
    Align(Align),
    AlignmentStart,
    AlignmentEnd,
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
            FormatItem::Align(align) => Debug::fmt(&align, f),
            FormatItem::AlignmentStart => f.write_str("{|"),
            FormatItem::AlignmentEnd => f.write_str("|}"),
            FormatItem::Nothing => f.write_char('⌧'),
            FormatItem::Antispace => f.write_char('⁀'),
            FormatItem::Space => f.write_char('␣'),
            FormatItem::Newline => f.write_char('⏎'),
            FormatItem::BlankLine => f.write_char('¶'),
            FormatItem::ExistingWhitespace(sp) => {
                f.write_char('∃')?;
                for c in sp.chars() {
                    match c {
                        '\n' => f.write_char('⏎')?,
                        '\t' => f.write_char('»')?,
                        ' ' => f.write_char('␣')?,
                        it if it.is_control() => f.write_char('␦')?,
                        _ => f.write_char(c)?,
                    };
                }
                Ok(())
            }
            FormatItem::Text(txt) => {
                f.write_char('\'')?;
                f.write_str(txt)?;
                f.write_char('\'')
            }
        }
    }
}

impl FormatItem {
    pub fn merge(self, other: FormatItem) -> Result<FormatItem, (FormatItem, FormatItem)> {
        match (&self, &other) {
            // Alignments never get collapsed
            (Self::Align { .. }, _) | (_, Self::Align { .. }) => Err((self, other)),

            // Alignment Starts distribute to the right through spaces (including linebreaks)
            (
                Self::AlignmentStart,
                Self::Antispace | Self::Space | Self::Newline | Self::BlankLine,
            ) => Err((other, Self::AlignmentStart)),
            (
                Self::Antispace | Self::Space | Self::Newline | Self::BlankLine,
                Self::AlignmentStart,
            ) => Err((self, Self::AlignmentStart)),
            (Self::AlignmentStart, _) | (_, Self::AlignmentStart) => Err((self, other)),

            // Alignment Ends distribute to the left through spaces (incl. linebreaks)
            (
                Self::AlignmentEnd,
                Self::Antispace | Self::Space | Self::Newline | Self::BlankLine,
            ) => Err((Self::AlignmentEnd, other)),
            (
                Self::Antispace | Self::Space | Self::Newline | Self::BlankLine,
                Self::AlignmentEnd,
            ) => Err((Self::AlignmentEnd, self)),
            (Self::AlignmentEnd, _) | (_, Self::AlignmentEnd) => Err((self, other)),

            // Nothings always gets collapsed
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
            FormatItem::Align { .. } => "",   // Should they print themselves?
            FormatItem::AlignmentStart => "", // Should they print themselves?
            FormatItem::AlignmentEnd => "",   // Should they print themselves?
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

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Align {
    pub column: Option<ColumnId>,
    pub is_virtual: bool,
}

impl Align {
    pub fn new() -> Self {
        Self {
            column: None,
            is_virtual: false,
        }
    }
}

impl Debug for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_virtual {
            f.write_char('¦')?;
        } else {
            f.write_char('|')?;
        }
        if let Some(id) = self.column {
            Debug::fmt(&id, f)
        } else {
            f.write_char('?')
        }
    }
}
