use std::{io::Read, ops::Range};

use tree_sitter::{Node, Parser, Query, QueryCursor, Tree};

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::language())
        .expect("We should be ablet to load an Ink grammar.");

    let mut source = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut source)
        .expect("Why can't we read from stdin?");

    let tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    for (range, new_text) in edits(&tree, &source).into_iter().rev() {
        source.replace_range(range, &new_text);
    }

    println!("{}", source);
}

fn edits(tree: &Tree, source: &str) -> Vec<(Range<usize>, String)> {
    let query = Query::new(
        &tree_sitter_ink::language(),
        r#"
        (knot name: (identifier) !end_mark) @knot_without_end
        "#,
    )
    .expect("I wrote this, it must work.");

    let mut query_cursor = QueryCursor::new();
    let kwe_index = query
        .capture_index_for_name("knot_without_end")
        .expect("I mean, just look up!") as usize;
    let matches: Vec<Vec<Node>> = query_cursor
        .captures(&query, tree.root_node(), source.as_bytes())
        .filter(|&(_, i)| i == kwe_index)
        .map(|it| it.0.captures.iter().map(|cap| cap.node).collect::<Vec<_>>())
        .collect();
    let mut result = Vec::new();
    for nodes in matches {
        for node in nodes {
            let old_text = &source[node.byte_range()];
            let new_text = old_text.to_string() + " ===";
            let range = node.start_byte()..node.end_byte();
            result.push((range, new_text));
        }
    }
    result
}
