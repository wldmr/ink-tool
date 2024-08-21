use std::fmt::{Debug, Formatter, Write};

#[derive(PartialEq)]
pub struct Space {
    pub repeats: usize,
    pub linebreak: bool,
    pub existing: bool,
}

#[derive(PartialEq)]
pub enum FormatItem {
    Nothing,
    Antispace,
    Space(Space),
    Text(String),
}

fn repeat_char(f: &mut Formatter<'_>, c: char, repeats: usize) -> std::fmt::Result {
    for _ in 0..repeats {
        f.write_char(c)?;
    }
    Ok(())
}
impl Debug for FormatItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatItem::Nothing => f.write_char('⌧'),
            FormatItem::Antispace => f.write_char('⁀'),
            FormatItem::Space(Space {
                repeats,
                existing,
                linebreak,
            }) => {
                if *existing {
                    f.write_char('∃')?;
                }
                let c = if *linebreak { '⏎' } else { '␣' };
                repeat_char(f, c, *repeats)
            }
            FormatItem::Text(txt) => {
                f.write_char('\'')?;
                f.write_str(txt)?;
                f.write_char('\'')
            }
        }
    }
}
