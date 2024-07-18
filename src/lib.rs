pub mod config;
pub mod edit;
pub mod rules;

use std::{
    fs::File,
    ops::{Deref, DerefMut},
    process::Command,
};

use rules::init_rules;
use tree_sitter::{Parser, Query, QueryCursor};

static QUERY: &str = include_str!("format.scm");

#[derive(Debug)]
pub enum FormatToken<T> {
    Node(T),
}

#[derive(Debug, Default)]
struct FormatTokens<T>(Vec<FormatToken<T>>);

impl<T> Deref for FormatTokens<T> {
    type Target = Vec<FormatToken<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for FormatTokens<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ToString> ToString for FormatTokens<T> {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for token in self.0.iter() {
            match token {
                FormatToken::Node(txt) => result.push_str(&txt.to_string()),
            }
        }
        result
    }
}

pub fn format(config: config::FormatConfig, source: String) -> String {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::language())
        .expect("We should be ablet to load an Ink grammar.");

    let query = Query::new(&tree_sitter_ink::language(), QUERY).expect("query should be valid");

    let mut rules = init_rules(config, &query);

    let mut query_cursor = QueryCursor::new();

    let tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

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

    let mut tokens: FormatTokens<&str> = FormatTokens::default();
    // for (match_, capture_index) in
    //     query_cursor.captures(&query, tree.root_node(), source.as_bytes())
    // {
    //     let cap = match_.captures[capture_index];
    //     for token in rules.apply(&cap.index, &cap.node, &source) {
    //         tokens.push(token);
    //     }
    // }
    let mut previous_end_byte = 0;
    for event in DepthFirstIterator::new(tree.root_node()) {
        // eprintln!("{}{:?}", "   ".repeat(depth), event);
        if let NodeEvent::Enter(node) = event {
            if node.child_count() == 0 {
                // copy whitespace;
                tokens.push(FormatToken::Node(
                    &source[previous_end_byte..node.start_byte()],
                ));
                // copy content;
                tokens.push(FormatToken::Node(
                    &source[node.start_byte()..node.end_byte()],
                ));
                previous_end_byte = node.end_byte();
            }
        }
    }

    tokens.to_string()
}

type NodeId = usize;

#[derive(Debug)]
enum NodeEvent<'a> {
    Enter(tree_sitter::Node<'a>),
    Exit(tree_sitter::Node<'a>),
}

struct DepthFirstIterator<'a> {
    cursor: tree_sitter::TreeCursor<'a>,
    buffer: Option<NodeEvent<'a>>,
    descending: bool,
    done: bool,
}

impl<'a> DepthFirstIterator<'a> {
    fn new(node: tree_sitter::Node<'a>) -> Self {
        Self {
            cursor: node.walk(),
            buffer: Some(NodeEvent::Enter(node)),
            descending: true,
            done: false,
        }
    }
}

impl<'a> Iterator for DepthFirstIterator<'a> {
    type Item = NodeEvent<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else if self.buffer.is_some() {
            self.buffer.take()
        } else {
            let current_node = self.cursor.node();
            if self.descending && self.cursor.goto_first_child() {
                Some(NodeEvent::Enter(self.cursor.node()))
            } else if self.cursor.goto_next_sibling() {
                self.descending = true;
                let next = NodeEvent::Exit(current_node);
                self.buffer = Some(NodeEvent::Enter(self.cursor.node()));
                Some(next)
            } else if self.cursor.goto_parent() {
                self.descending = false;
                self.buffer = Some(NodeEvent::Exit(current_node));
                self.next()
            } else {
                // No more siblings or parents -> we're done.
                self.done = true;
                Some(NodeEvent::Exit(current_node)) // curren_node == root node
            }
        }
    }
}
