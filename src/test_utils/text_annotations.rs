use std::{iter::Peekable, str::CharIndices};

/// Finds annotations in a piece of text
#[derive(Debug, Clone)]
pub struct AnnotationScanner {
    annotation_line_start: String,
    interval_char: char,
    guide_char: char,
}

impl AnnotationScanner {
    /// Construct a scanner that treats lines starting with `line_comment` as comments
    pub fn new() -> Self {
        Self {
            annotation_line_start: String::from("//"),
            interval_char: '^',
            guide_char: '|',
        }
    }

    /// The string that introduces an annotation. Default: `//`
    pub fn annotation(self, line_start: impl Into<String>) -> Self {
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
    pub fn scan<'a>(&'a self, text: &'a str) -> impl Iterator<Item = Annotation<'a>> {
        Annonations::new(self, text)
    }
}

/// An annotation on a piece of text
///
/// Corresponds to a byte range in the input text.
/// Use this to construct assertions about the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Annotation<'a> {
    pub bytes: (usize, usize),
    /// The claim associated with the byte range (i.e. "is function", "defines variable x", etc.)
    pub claim: &'a str,
}

impl Annotation<'_> {
    /// Convenience for converting `self.bytes` into a range
    pub fn byte_range(&self) -> std::ops::Range<usize> {
        self.bytes.0..self.bytes.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct PositionInterval(usize, usize);

/// Iterates over some text and extracts annotations on pieces of text.
struct Annonations<'a> {
    inner: Peekable<CharIndices<'a>>,
    comment_starter: &'a str,
    interval_char: char,
    guide_char: char,
    /// Full text that we're iterating over.
    text: &'a str,
    /// Interval of the line we're currently collecting annotions on
    content_line: PositionInterval,
}

impl<'a> Annonations<'a> {
    fn new(scanner: &'a AnnotationScanner, text: &'a str) -> Self {
        Self {
            comment_starter: &scanner.annotation_line_start,
            interval_char: scanner.interval_char,
            guide_char: scanner.guide_char,
            text,
            inner: text.char_indices().peekable(),
            content_line: Default::default(),
        }
    }
}

impl<'a> Iterator for Annonations<'a> {
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
    ContentLine(PositionInterval),
    // Found normal comment, do nothing
    Ignore,
}

impl<'a> Annonations<'a> {
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
                let interval = PositionInterval(line_start, line_end);
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
        let target = self.shift_to_content_line(line_start, target);

        self.skip(whitespace_or_guide);

        let Some(claim) = self.skip(|chr| chr != '\n') else {
            self.skip_to_next_line();
            return Some(ParseResult::Ignore);
        };

        self.skip_to_next_line();

        Some(ParseResult::Annotation(Annotation {
            bytes: (target.0, target.1),
            claim: &self.text[claim.0..claim.1].trim_matches(whitespace_or_guide),
        }))
    }

    /// Advance the cursor as long as `predicate` is true. Return the interval advanced over.
    /// Return `None` if the interval would be empty;
    fn skip(&mut self, predicate: impl Fn(char) -> bool) -> Option<PositionInterval> {
        let start = self.inner.peek()?.0;
        let mut result = PositionInterval(start, start);
        while let Some((end, _)) = self.inner.next_if(|(_, chr)| predicate(*chr)) {
            result.1 = end + 1;
        }
        if result.0 == result.1 {
            None
        } else {
            Some(result)
        }
    }

    /// Advance iterator until it has consumed the next newline (if any).
    /// Returns index of the zero'th character on the new line.
    fn skip_to_next_line(&mut self) -> Option<usize> {
        // Skip until after newline;
        // If inner finishes, we can just abort. No annotations can follow,
        // so we don't need to keep track of anything.
        loop {
            let (pos, chr) = self.inner.next()?;
            if chr == '\n' {
                return Some(pos + 1);
            }
        }
    }

    /// Take an interval and shift it "upwards" that that it points into the current content line
    fn shift_to_content_line(
        &self,
        annotation_line_start: usize,
        mut interval: PositionInterval,
    ) -> PositionInterval {
        let offset = annotation_line_start - self.content_line.0;
        interval.0 -= offset;
        interval.1 -= offset;
        interval
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    #[test]
    fn parsing() {
        let scanner = AnnotationScanner::new();
        let text = "
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
            }";
        assert_eq!(
            scanner
                .scan(text)
                .map(|it| (&text[it.byte_range()], it.claim))
                .unique()
                .sorted_unstable()
                .collect::<Vec<_>>(),
            vec![
                ("+", "operator"),
                ("a", "param"),
                ("a + b", "sum"),
                ("b", "param"),
                ("plus", "function"),
                ("u32", "type"),
            ]
        )
    }

    #[test]
    fn customizing() {
        let scanner = AnnotationScanner::new()
            .annotation('%')
            .interval('=')
            .guide('.');
        let text = "
            fn plus(a: u32, b: u32) -> u32 {
            %  .  . = param            ...
            %  .  .         = param    ...
            %  ==== function           ...
            %                          === type
                a + b
            %   ===== sum
            }";
        assert_eq!(
            scanner
                .scan(text)
                .map(|it| (&text[it.byte_range()], it.claim))
                .unique()
                .sorted_unstable()
                .collect::<Vec<_>>(),
            vec![
                ("a", "param"),
                ("a + b", "sum"),
                ("b", "param"),
                ("plus", "function"),
                ("u32", "type"),
            ]
        )
    }
}
