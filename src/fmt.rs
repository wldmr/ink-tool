mod format_item;
mod formatting;
mod node_rule;
mod scanner;
mod util;

pub(crate) type CaptureIndex = u32;
pub(crate) type PatternIndex = usize;
pub(crate) type NodeId = usize;

use tree_sitter::{Language, Parser, Query};

use crate::fmt::scanner::FormatScanner;

use self::formatting::{Layout, Tracing};

static QUERY: &str = include_str!("fmt/format.scm");

/// Convenience function for quickly formatting a string.
///
/// For multiple repeated formatting operations, you'll want to construct a [`FormatScanner`] and re-use that.
pub fn format(source: String) -> String {
    let language: Language = tree_sitter_ink::LANGUAGE.into();

    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .expect("We should be able to load a language.");

    let tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let query = Query::new(&language, QUERY).expect("query should be valid");
    let mut scanner = FormatScanner::new(query);
    let mut result = String::new();
    let mut formatter = Tracing::new(Layout::new(Tracing::new(&mut result)));

    scanner.scan(&tree, &source, &mut formatter);
    // eprintln!("outer:\n{}", formatter.trace);
    // eprintln!("inner:\n{}\n", formatter.downstream.downstream.trace);
    result
}
