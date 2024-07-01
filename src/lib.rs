pub mod config;
pub mod edit;
pub mod rules;

use std::{fs::File, process::Command};

use edit::Change;
use rules::{init_rules, Rule};
use tree_sitter::{Parser, Query, QueryCursor};

static QUERY: &str = include_str!("format.scm");

pub fn format(config: config::FormatConfig, mut source: String) -> String {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::language())
        .expect("We should be ablet to load an Ink grammar.");

    let query = Query::new(&tree_sitter_ink::language(), QUERY).expect("query should be valid");

    let mut rules = init_rules(config, &query);

    let mut query_cursor = QueryCursor::new();

    let mut tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let mut edits = next_edits(&mut query_cursor, &query, &tree, &source, &mut rules);
    let mut round = 0;
    let mut edit_count = 0;
    while !edits.is_empty() {
        round += 1;
        if tree.root_node().has_error() {
            let graph = File::create("log.dot").expect("I should be able to create that file");
            tree.print_dot_graph(&graph);
            Command::new("dot")
                .args(["-Tsvg", "-O", "log.dot"])
                .output()
                .expect("I should be able to call dot");

            std::fs::write("error.fmt", &source).expect("I should be able to write that file");
            panic!("Source can't be parsed. See log.dot.svg.\n{source}");
        }
        for Change { range, text } in edits.iter_mut().rev() {
            edit_count += 1;
            source.replace_range(range.start_byte..range.old_end_byte, &text);
            tree.edit(range);
        }

        // eprintln!("After round {round}:\n{source}");

        tree = parser
            .parse(&source, Some(&tree))
            .expect("There should be a tree here.");
        edits = next_edits(&mut query_cursor, &query, &tree, &source, &mut rules);
    }

    eprintln!("Made {edit_count} edits in {round} rounds.");
    source
}

fn next_edits(
    query_cursor: &mut QueryCursor,
    query: &Query,
    tree: &tree_sitter::Tree,
    source: &str,
    rules: &mut Vec<Box<dyn Rule>>,
) -> Vec<Change> {
    let mut edits = Vec::new();
    for match_ in query_cursor.matches(&query, tree.root_node(), source.as_bytes()) {
        for rule in rules.iter_mut() {
            let edit = rule.edit_if_needed(&query, &match_, source);
            if let Some(edit) = edit {
                let prev = edits.last();
                if prev.is_none()
                    || prev.is_some_and(|prev: &Change| {
                        edit.range.start_byte >= prev.range.old_end_byte
                    })
                {
                    edits.push(edit);
                }
            }
        }
    }
    edits
}
