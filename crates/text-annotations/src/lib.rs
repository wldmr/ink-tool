use std::{iter::Peekable, str::CharIndices};

/// Defines how Annotations are found in a piece of text
#[derive(Debug, Clone, Copy)]
pub struct AnnotationConfig {
    annotation_line_start: &'static str,
    interval_char: char,
    guide_char: char,
}

impl Default for AnnotationConfig {
    fn default() -> Self {
        SLASH_CARET_PIPE
    }
}

/// Default configuration for most languages.
pub static SLASH_CARET_PIPE: AnnotationConfig = AnnotationConfig {
    annotation_line_start: "//",
    interval_char: '^',
    guide_char: '|',
};

pub fn scan_default_annotations(text: &str) -> Annotations<'_> {
    Annotations::scan_default(text)
}

impl AnnotationConfig {
    /// Construct a default config.
    pub fn new() -> Self {
        Self::default()
    }

    /// The string that introduces an annotation. Default: `//`
    pub fn annotation(self, line_start: impl Into<&'static str>) -> Self {
        Self {
            annotation_line_start: line_start.into(),
            ..self
        }
    }

    /// The character that defines an interval. Default: `^`
    pub fn interval(self, interval_char: char) -> Self {
        Self {
            interval_char,
            ..self
        }
    }

    /// The character that can be used as a visual guide in an annotation. Default: `|`
    pub fn guide(self, guide_char: char) -> Self {
        Self { guide_char, ..self }
    }

    /// Finds all the annotations
    pub fn scan<'a>(&self, text: &'a str) -> Annotations<'a> {
        Annotations::scan(*self, text)
    }
}

/// An annotation on a piece of text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Annotation<'a> {
    /// The full underlying text. Might as well keep the whole thing around,
    /// instead of two individual slices (text + claim).
    pub full_text: &'a str,
    /// The part of `full_text` being annotated
    pub text_location: TextRegion,
    /// The part of `full_text` that annotates
    pub claim_location: TextRegion,
}

impl<'a> Annotation<'a> {
    /// The annotated text
    pub fn text(&self) -> &'a str {
        &self.full_text[self.text_location.byte_range()]
    }

    /// The claim about the annotated text
    pub fn claim(&self) -> &'a str {
        &self.full_text[self.claim_location.byte_range()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TextRegion {
    // (inclusive)
    pub start: TextPos,
    // (exclusive)
    pub end: TextPos,
}

impl TextRegion {
    pub fn new(start: TextPos, end: TextPos) -> Self {
        Self { start, end }
    }

    pub fn byte_range(&self) -> std::ops::Range<usize> {
        self.start.byte..self.end.byte
    }
}

impl From<TextRegion> for lsp_types::Range {
    fn from(value: TextRegion) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TextPos {
    pub byte: usize,
    pub row: u32,
    pub col: u32,
}

impl From<TextPos> for lsp_types::Position {
    fn from(value: TextPos) -> Self {
        Self {
            line: value.row,
            character: value.col,
        }
    }
}
struct TextPosIter<'a> {
    inner: CharIndices<'a>,
    row: u32,
    col: u32,
}

impl<'a> TextPosIter<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            inner: text.char_indices(),
            row: 0,
            col: 0,
        }
    }
}

impl<'a> Iterator for TextPosIter<'a> {
    type Item = (TextPos, char);

    fn next(&mut self) -> Option<Self::Item> {
        let (byte, chr) = self.inner.next()?;
        let pos = TextPos {
            byte,
            row: self.row,
            col: self.col,
        };
        if chr == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some((pos, chr))
    }
}

/// Iterates over some text and extracts annotations on pieces of text.
pub struct Annotations<'a> {
    inner: Peekable<TextPosIter<'a>>,
    comment_starter: &'a str,
    interval_char: char,
    guide_char: char,
    /// Full text that we're iterating over.
    text: &'a str,
    /// Interval of the line we're currently collecting annotions on
    content_line: TextRegion,
}

impl<'a> Annotations<'a> {
    pub fn scan_default(text: &'a str) -> Self {
        Self::scan(AnnotationConfig::new(), text)
    }

    pub fn scan(config: AnnotationConfig, text: &'a str) -> Self {
        Self {
            comment_starter: &config.annotation_line_start,
            interval_char: config.interval_char,
            guide_char: config.guide_char,
            text,
            inner: TextPosIter::new(text).peekable(),
            content_line: Default::default(),
        }
    }
}

impl<'a> Iterator for Annotations<'a> {
    type Item = Annotation<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_parse_annotation()? {
            ParseResult::Annotation(annotation) => Some(annotation),
            ParseResult::ContentLine(content_line) => {
                self.content_line = content_line;
                self.next()
            }
            ParseResult::Ignore => self.next(),
        }
    }
}

enum ParseResult<'a> {
    /// Found a annotation
    Annotation(Annotation<'a>),
    /// Found a new content line
    ContentLine(TextRegion),
    // Found normal comment, do nothing
    Ignore,
}

impl<'a> Annotations<'a> {
    fn try_parse_annotation(&mut self) -> Option<ParseResult<'a>> {
        // Invariant: We are at the beginning of the line here.
        let (line_start, _) = self.inner.peek().cloned()?;
        let whitspace_only = |chr: char| chr.is_whitespace() && chr != '\n';
        self.skip(whitspace_only);

        // Detect comment start.
        // If there's no comment here, then we've found a new content line.
        let (_, mut current_char) = self.inner.peek()?;
        for comment_char in self.comment_starter.chars() {
            if current_char == comment_char {
                (_, current_char) = self.inner.next()?;
            } else {
                let line_end = self.skip_to_next_line()?;
                let interval = TextRegion::new(line_start, line_end);
                return Some(ParseResult::ContentLine(interval));
            }
        }

        // From here on out, we're "inside" a comment.
        // This means we either detect an annotation or ignore the line intirely.
        // It also means we count pipes a whitespace from now on.
        let guide_char = self.guide_char;
        let interval_char = self.interval_char;
        let whitespace_or_guide = |chr: char| whitspace_only(chr) || chr == guide_char;
        self.skip(whitespace_or_guide);

        let Some(target) = self.skip(|chr| chr == interval_char) else {
            // We passed the comment mark, and thus we shouldn't return an interval.
            // (returning an interval means "this is the new content line")
            self.skip_to_next_line();
            return Some(ParseResult::Ignore);
        };
        let text_interval = self.shift_to_content_line(line_start, target);

        self.skip(whitespace_or_guide);

        let Some(mut claim_interval) = self.skip(|chr| chr != '\n') else {
            self.skip_to_next_line();
            return Some(ParseResult::Ignore);
        };

        // correct trailing whitspace/guide chars
        let mut claim_text = &self.text[claim_interval.byte_range()];
        claim_text = claim_text.trim_end_matches(whitespace_or_guide);
        claim_interval.end.byte = claim_interval.start.byte + claim_text.len();
        claim_interval.end.col = claim_interval.start.col + claim_text.chars().count() as u32;

        self.skip_to_next_line();

        Some(ParseResult::Annotation(Annotation {
            full_text: self.text,
            text_location: text_interval,
            claim_location: claim_interval,
        }))
    }

    /// Advance the cursor as long as `predicate` is true. Return the interval advanced over.
    /// Return `None` if the interval would be empty;
    fn skip(&mut self, predicate: impl Fn(char) -> bool) -> Option<TextRegion> {
        let start = self.inner.peek()?.0;
        let mut result = TextRegion::new(start, start);
        while let Some(end) = self.inner.next_if(|(_, chr)| predicate(*chr)) {
            result.end = self.inner.peek().unwrap_or(&end).0;
        }
        if result.start == result.end {
            None
        } else {
            Some(result)
        }
    }

    /// Advance iterator until it has consumed the next newline (if any).
    /// Returns position of the first character on the next line.
    fn skip_to_next_line(&mut self) -> Option<TextPos> {
        // Skip until after newline;
        // If inner finishes, we can just abort. No annotations can follow,
        // so we don't need to keep track of anything.
        loop {
            let (_, chr) = self.inner.next()?;
            if chr == '\n' {
                return Some(self.inner.peek()?.0);
            }
        }
    }

    /// Take an interval and shift it "upwards" that that it points into the current content line
    fn shift_to_content_line(
        &self,
        annotation_line_start: TextPos,
        mut interval: TextRegion,
    ) -> TextRegion {
        let byte_offset = annotation_line_start.byte - self.content_line.start.byte;
        let row_offset = annotation_line_start.row - self.content_line.start.row;
        interval.start.byte -= byte_offset;
        interval.end.byte -= byte_offset;
        interval.start.row -= row_offset;
        interval.end.row -= row_offset;
        interval
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    fn row_cols<'a>(ann: Annotation<'a>) -> (u32, u32, u32) {
        let TextRegion { start, end } = ann.text_location;
        assert_eq!(start.row, end.row);
        (start.row, start.col, end.col)
    }

    #[test]
    fn parsing() {
        let text = indoc! {"\
            fn plus(a: u32, b: u32) -> u32 {
            // |  | ^ param            |||
            // |  |         ^ param    |||
            // |  |    ^^^type         |||
            // |  |            ^^^type |||
            // |  | <- Pipes are treated as whitespace and can be used as visual guides
            // |  |    (oh, and comments don't have to contain annotations)
            // ^^^^ function           |||
            //                         ^^^ type
                a + b
            //    ^ operator
            //  ^^^^^ sum
            }"};
        assert_eq!(
            scan_default_annotations(text)
                .map(|it| (row_cols(it), it.claim(), it.text()))
                .sorted_unstable()
                .collect::<Vec<_>>(),
            vec![
                ((0, 03, 07), "function", "plus"),
                ((0, 08, 09), "param", "a"),
                ((0, 11, 14), "type", "u32"),
                ((0, 16, 17), "param", "b"),
                ((0, 19, 22), "type", "u32"),
                ((0, 27, 30), "type", "u32"),
                ((9, 04, 09), "sum", "a + b"),
                ((9, 06, 07), "operator", "+"),
            ]
        )
    }

    #[test]
    fn customizing() {
        let custom = AnnotationConfig::new()
            .annotation("%")
            .interval('=')
            .guide('.');
        let text = indoc! {"\
            fn plus(a: u32, b: u32) -> u32 {
            %  .  . = param            ...
            %  .  .         = param    ...
            %  ==== function           ...
            %                          === type
                a + b
            %   ===== sum
            }"
        };
        assert_eq!(
            custom
                .scan(text)
                .map(|it| (row_cols(it), it.claim(), it.text()))
                .sorted_unstable()
                .collect::<Vec<_>>(),
            vec![
                ((00, 03, 07), "function", "plus"),
                ((00, 08, 09), "param", "a"),
                ((00, 16, 17), "param", "b"),
                ((00, 27, 30), "type", "u32"),
                ((05, 04, 09), "sum", "a + b"),
            ]
        )
    }
}
