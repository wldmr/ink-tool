use std::io::Read;

use tree_sitter::{Parser, Query, QueryCursor};

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::language())
        .expect("Error loading Ink grammar");

    let mut stdin = std::io::stdin().lock();
    let mut source = String::new();
    stdin
        .read_to_string(&mut source)
        .expect("Why can't we read from stdin?");
    let source = source;

    let tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let query = Query::new(
        &tree_sitter_ink::language(),
        "(knot name: (identifier) @knot_name)",
    )
    .expect("I wrote this, it must work.");

    println!("Knots:");
    let mut query_cursor = QueryCursor::new();
    let captures = query_cursor.captures(&query, tree.root_node(), source.as_bytes());
    for (qmatch, _size) in captures {
        for cap in qmatch.captures {
            let range = cap.node.range();
            let text = &source[range.start_byte..range.end_byte];
            println!("    {}", text);
        }
    }
}
