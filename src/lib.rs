mod rules;

use std::rc::Rc;

use rules::{init_rules, EditResult, Rule};
use tree_sitter::{Parser, Query, QueryCursor};

static QUERY: &str = include_str!("format.scm");
pub struct FormatConfig {
    knot_mark_size: usize,
    closing_mark: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            knot_mark_size: 3,
            closing_mark: true,
        }
    }
}

pub fn format(config: FormatConfig, mut source: String) -> String {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::language())
        .expect("We should be ablet to load an Ink grammar.");

    let mut tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let config = Rc::new(config);
    let query =
        Rc::new(Query::new(&tree_sitter_ink::language(), QUERY).expect("query should be valid"));

    let mut rules = init_rules(config, &query);

    let mut query_cursor = QueryCursor::new();

    while let Some((range, new_text)) =
        next_edit(&mut query_cursor, &query, &tree, &source, &mut rules)
    {
        source.replace_range(range.start_byte..range.old_end_byte, &new_text);
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
) -> Option<EditResult> {
    for m in query_cursor.matches(&query, tree.root_node(), source.as_bytes()) {
        let props = query.property_settings(m.pattern_index);
        for rule in rules.iter_mut() {
            if let edit @ Some(_) = rule.visit(&m, &props, source) {
                return edit;
            }
        }
    }
    None
}
