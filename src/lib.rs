pub mod config;
pub mod edit;
pub mod rules;

use edit::Change;
use rules::{init_rules, Rule};
use tree_sitter::{Parser, Query, QueryCursor};

static QUERY: &str = include_str!("format.scm");

pub fn format(config: config::FormatConfig, mut source: String) -> String {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::language())
        .expect("We should be ablet to load an Ink grammar.");

    let mut tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let query = Query::new(&tree_sitter_ink::language(), QUERY).expect("query should be valid");

    let mut rules = init_rules(config, &query);

    let mut query_cursor = QueryCursor::new();

    while let Some(Change { range, text }) =
        next_edit(&mut query_cursor, &query, &tree, &source, &mut rules)
    {
        source.replace_range(range.start_byte..range.old_end_byte, &text);
        tree.edit(&range);
        tree = parser
            .parse(&source, Some(&tree))
            .expect("Parsing should work, even if repeated.");
    }

    source
}

fn next_edit(
    query_cursor: &mut QueryCursor,
    query: &Query,
    tree: &tree_sitter::Tree,
    source: &str,
    rules: &mut Vec<Box<dyn Rule>>,
) -> Option<Change> {
    for match_ in query_cursor.matches(&query, tree.root_node(), source.as_bytes()) {
        for rule in rules.iter_mut() {
            let edit = rule.edit_if_needed(&query, &match_, source);
            if edit.is_some() {
                return edit;
            }
        }
    }
    None
}
