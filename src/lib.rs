pub mod config;

mod format_item;
mod formatter;
mod node_rule;
mod scanner;

use config::FormatConfig;
use tree_sitter::{Parser, Query};

pub(crate) type CaptureIndex = u32;
pub(crate) type PatternIndex = usize;
pub(crate) type MatchIndex = u32;
pub(crate) type NodeId = usize;

use crate::scanner::FormatScanner;

static QUERY: &str = include_str!("format.scm");

/// Convenience function for quickly formatting a string.
///
/// For multiple repeated formatting operations, you'll want to construct a [`FormatScanner`] and re-use that.
pub fn format(_config: config::FormatConfig, source: String) -> String {
    let language = tree_sitter_ink::language();

    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .expect("We should be ablet to load a language.");

    let tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let query = Query::new(&language, QUERY).expect("query should be valid");
    let mut scanner = FormatScanner::new(query, FormatConfig::default());

    let mut output = scanner.scan(&tree, &source);

    output.normalize();
    output.to_string()
}
