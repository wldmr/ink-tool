pub mod config;
pub mod rules;

use tree_sitter::Parser;

use crate::rules::FormatScanner;

pub fn format(config: config::FormatConfig, source: String) -> String {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::language())
        .expect("We should be ablet to load an Ink grammar.");

    let mut scanner = FormatScanner::new(config);

    let tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let mut output = scanner.scan(&tree, &source);

    output.normalize();
    output.to_string()
}
