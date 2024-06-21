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

macro_rules! def_rule {
    (
        $name:ident($self:ident, $config:ident)
        if $condition:expr;
        match $query:literal where $($querylhs:ident = $queryrhs:expr),*;
        init $($field:ident: $fieldtype:ty = $fieldinit:expr),*;
        edit($node:ident, $source:ident) => $edit:stmt

    ) => {
        struct $name {
            $($field: $fieldtype),*
        }

        impl Rule for $name {
            fn new($config: &FormatConfig) -> Option<Self> {
                $condition.then_some(Self {
                    $($field: $fieldinit),*
                })
            }

            fn rule_name(&$self) -> &'static str {
                stringify!($name)
            }

            fn query(&$self) -> String {
                format!($query, $($querylhs = $queryrhs),*)
            }

            fn edit(&$self, $node: &Node, $source: &str) -> (Range<usize>, String) {
                $edit
            }
        }
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
    let rules: Vec<Box<dyn Rule>> =
        init_rules! {config => AddEndMark, RemoveEndMark, FixEndMark, FixStartMark};

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

trait Rule {
    fn new(config: &FormatConfig) -> Option<Self>
    where
        Self: Sized;
    fn rule_name(&self) -> &'static str;
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

impl FormatConfig {
    fn knot_mark(&self) -> String {
        "=".repeat(self.knot_mark_size)
    }
}

def_rule! { AddEndMark(self, config)
    if config.closing_mark == true;
    match "(knot !function !end_mark) @{node}"
        where node = self.rule_name();
    init
        mark: String = config.knot_mark();
    edit(node, source) => append(&node, &format!(" {}", self.mark))
}

def_rule! { RemoveEndMark(self, config)
    if config.closing_mark == false;
    match "(knot !function end_mark: @{node})"
        where node = self.rule_name();
    init;
    edit(node, source) => replace(&node, "")
}

def_rule! { FixEndMark(self, config)
    if config.closing_mark == true;
    match r#"(
        (knot !function end_mark: _ @{node})
        (#not-eq? @{node} "{mark}")
        )"#
        where node = self.rule_name(), mark = self.mark;
    init
        mark: String = config.knot_mark();
    edit(node, source) => replace(&node, &self.mark)
}

def_rule! { FixStartMark(self, config)
    if true;
    match r#"(
        (knot !function start_mark: _ @{node})
        (#not-eq? @{node} "{mark}")
        )"#
        where node = self.rule_name(), mark = self.mark;
    init
        mark: String = config.knot_mark();
    edit(node, source) => replace(&node, &self.mark)
}

fn append(node: &Node, text: &str) -> (Range<usize>, String) {
    (node.end_byte()..node.end_byte(), text.to_owned())
}
fn replace(node: &Node, text: &str) -> (Range<usize>, String) {
    (node.start_byte()..node.end_byte(), text.to_owned())
}
