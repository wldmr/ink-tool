use std::{collections::HashMap, io::Read, ops::Range};

use tree_sitter::{Node, Parser, Query, QueryCursor};
use tree_sitter_ink::language as ink_lang;

/// Wrapping a rule in a box is a bit ugly, so we macro it away.
/// Seems to me that this should be easier,
macro_rules! init_rules {
    ($config:ident => $($rule:ident),+) => {
        vec![$($rule::new(&$config).map(|rule| Box::new(rule) as Box<dyn Rule>)),+]
        .into_iter()
        .filter_map(|maybe_rule| maybe_rule)
        .collect()
    };
}

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

    let config = FormatConfig::default();
    let rules: Vec<Box<dyn Rule>> = init_rules! {config => AddEndMark, FixEndMark};

    let query = rules
        .iter()
        .map(|rule| rule.query())
        .collect::<Vec<String>>()
        .join("\n");
    dbg!(&query);
    let query = Query::new(&ink_lang(), &query).expect("valid query");

    let rules: HashMap<u32, Box<dyn Rule>> = rules
        .into_iter()
        .map(|rule| {
            let index = query
                .capture_index_for_name(rule.rule_name())
                .expect("Query is built with type ids");
            (index, rule)
        })
        .collect();

    let mut query_cursor = QueryCursor::new();
    let edits: Vec<_> = query_cursor
        .captures(&query, tree.root_node(), source.as_bytes())
        .flat_map(|(qmatch, _)| qmatch.captures.into_iter())
        .filter_map(|capture| {
            rules
                .get(&capture.index)
                .map(|rule| rule.edit(&capture.node, &source))
        })
        .collect();

    dbg!(&edits);

    for (range, new_text) in edits.into_iter().rev() {
        source.replace_range(range, &new_text);
    }

    println!("{}", source);
}

macro_rules! impl_rule_name {
    ($ty:ty) => {
        impl RuleName for $ty {
            fn rule_name(&self) -> &'static str {
                stringify!($ty)
            }
        }
    };
}

trait RuleName {
    fn rule_name(&self) -> &'static str;
}

trait Rule: RuleName {
    fn new(config: &FormatConfig) -> Option<Self>
    where
        Self: Sized;
    fn query(&self) -> String;
    fn edit(&self, node: &Node, source: &str) -> (Range<usize>, String);
}

struct FormatConfig {
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

impl_rule_name!(AddEndMark);
struct AddEndMark {
    mark: String,
}

impl Rule for AddEndMark {
    fn new(config: &FormatConfig) -> Option<Self> {
        config.closing_mark.then_some(Self {
            mark: "=".repeat(config.knot_mark_size),
        })
    }

    fn query(&self) -> String {
        format!(r#"(knot !function !end_mark) @{}"#, self.rule_name())
    }

    fn edit(&self, node: &Node, _source: &str) -> (Range<usize>, String) {
        edit(&node, Edit::Append, &self.mark)
    }
}

impl_rule_name!(FixEndMark);
struct FixEndMark {
    mark: String,
}

impl Rule for FixEndMark {
    fn new(config: &FormatConfig) -> Option<Self> {
        config.closing_mark.then_some(Self {
            mark: "=".repeat(config.knot_mark_size),
        })
    }

    fn query(&self) -> String {
        format!(
            r#"((knot !function end_mark: _ @{}) (#not-eq? @{} "{}"))"#,
            self.rule_name(),
            self.rule_name(),
            self.mark,
        )
    }

    fn edit(&self, node: &Node, _source: &str) -> (Range<usize>, String) {
        edit(&node, Edit::Replace, &self.mark)
    }
}

enum Edit {
    Append,
    Prepend,
    Replace,
}

fn edit(node: &Node, edit: Edit, text: &str) -> (Range<usize>, String) {
    let input_edit = match edit {
        Edit::Append => node.end_byte()..node.end_byte(),
        Edit::Prepend => node.start_byte()..node.start_byte(),
        Edit::Replace => node.start_byte()..node.end_byte(),
    };
    (input_edit, text.to_owned())
}
