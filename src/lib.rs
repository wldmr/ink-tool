pub mod config;
pub mod edit;
pub mod rules;

use std::{fs::File, process::Command};

use edit::Change;
use tree_sitter::Parser;

use crate::rules::Rules;

pub fn format(config: config::FormatConfig, mut source: String) -> String {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::language())
        .expect("We should be ablet to load an Ink grammar.");

    let mut rules = Rules::new(config);

    let mut tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let mut edits = rules.next_edits(&tree, &source);
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
        eprintln!("{source}");
        for Change { range, text } in edits.iter().rev() {
            edit_count += 1;
            eprintln!(
                "{}:{} '{}' -> '{}'",
                range.start_position.row,
                range.start_position.column,
                &source[range.start_byte..range.old_end_byte],
                text
            );
            source.replace_range(range.start_byte..range.old_end_byte, &text);
            tree.edit(range);
        }

        eprintln!("==== Round {round} ends ====\n");

        tree = parser
            .parse(&source, Some(&tree))
            .expect("There should be a tree here.");
        edits = rules.next_edits(&tree, &source);
    }

    eprintln!("Made {edit_count} edits in {round} rounds.");
    source
}
