pub mod config;

mod format_item;
mod formatter;
mod node_rule;
mod scanner;
mod util;

use tree_sitter::{Parser, Query};

pub(crate) type CaptureIndex = u32;
pub(crate) type PatternIndex = usize;
pub(crate) type NodeId = usize;

use crate::{
    formatter::{InkFormatter, Tracing},
    scanner::FormatScanner,
};

static QUERY: &str = include_str!("format.scm");

/// Convenience function for quickly formatting a string.
///
/// For multiple repeated formatting operations, you'll want to construct a [`FormatScanner`] and re-use that.
pub fn format(config: config::FormatConfig, source: String) -> String {
    let language = tree_sitter_ink::language();

    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .expect("We should be able to load a language.");

    let tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let query = Query::new(&language, QUERY).expect("query should be valid");
    let mut scanner = FormatScanner::new(query, config);
    let mut result = String::new();
    let mut formatter = Tracing::new(InkFormatter::new(Tracing::new(&mut result)));

    scanner.scan(&tree, &source, &mut formatter);
    // eprintln!("outer:\n{}", formatter.trace);
    // eprintln!("inner:\n{}\n", formatter.downstream.downstream.trace);
    result
}
