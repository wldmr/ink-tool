pub mod config;
pub mod edit;
pub mod formatter;

use std::{
    fs::File,
    ops::{Deref, DerefMut},
    process::Command,
};

use formatter::{InkFmt, InkFmtError, InkFormatter};
use tree_sitter::Parser;

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

pub fn format(config: config::FormatConfig, source: String) -> Result<String, InkFmtError> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::language())
        .expect("We should be ablet to load an Ink grammar.");

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

    let typed_root: tree_sitter_ink::node_types::Ink = tree
        .root_node()
        .try_into()
        .expect("We should have checked for errors.");

    let mut formatter = InkFormatter::new(&source, tree.walk(), config);
    match typed_root.inkfmt(&mut formatter) {
        Ok(_) => Ok(formatter.into_string()),
        Err(msg) => Err(msg),
    }
}
