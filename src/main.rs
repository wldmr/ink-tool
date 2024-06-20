use std::io::Read;

use tree_sitter::{InputEdit, Node, Parser, Point, Query, QueryCursor};
use tree_sitter_ink::language as ink_lang;
use type_sitter::ink::Knot;
use type_sitter_lib::TypedNode;

mod type_sitter;

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

    let mut tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let rules: Vec<Box<dyn Rule>> = vec![Box::new(KnotEndMarkRule::new())];

    for rule in rules {
        for (range, new_text) in rule.edits(&tree.root_node(), &source).into_iter().rev() {
            source.replace_range(range.start_byte..range.old_end_byte, &new_text);
            tree.edit(&range);
        }
        tree = parser
            .parse(&source, Some(&tree))
            .expect("Parsing should work");
    }

    println!("{}", source);
}

trait Rule {
    fn new() -> Self
    where
        Self: Sized;
    fn edits(&self, node: &Node, source: &str) -> Vec<(InputEdit, String)>;
}

struct KnotEndMarkRule {
    query: Query,
    config: KnotRuleConfig,
}

struct KnotRuleConfig {
    mark_size: usize,
    closing_mark: bool,
    mark_separator: String,
}

impl Default for KnotRuleConfig {
    fn default() -> Self {
        Self {
            mark_size: 3,
            closing_mark: true,
            mark_separator: " ".to_string(),
        }
    }
}

impl Rule for KnotEndMarkRule {
    fn new() -> Self {
        Self {
            query: Query::new(&ink_lang(), r#"(knot !function) @knot"#)
                .expect("I wrote this, it must work."),
            config: KnotRuleConfig::default(),
        }
    }

    fn edits(&self, node: &Node, source: &str) -> Vec<(InputEdit, String)> {
        let source = source.as_bytes();
        let kwe_index = self
            .query
            .capture_index_for_name("knot")
            .expect("I mean, just look up!") as usize;
        let mut query_cursor = QueryCursor::new();
        let knots = query_cursor
            .captures(&self.query, *node, source)
            .filter(|&(_, i)| i == kwe_index)
            .flat_map(|(qmatch, _)| qmatch.captures.into_iter())
            .filter_map(|cap| Knot::try_from(cap.node).ok())
            .filter(|knot| !knot.has_error());

        let end_marker = "=".repeat(self.config.mark_size);
        knots
            .filter_map(|knot| match knot.end_mark() {
                None if self.config.closing_mark => {
                    Some(edit(&knot, Edit::Append, vec![&end_marker]))
                }
                Some(Ok(mark))
                    if mark
                        .utf8_text(source)
                        .is_ok_and(|it| it.len() != self.config.mark_size) =>
                {
                    Some(edit(&mark, Edit::Replace, vec![&end_marker]))
                }
                _ => None,
            })
            .collect()
    }
}

enum Edit {
    Append,
    Prepend,
    Replace,
}

fn edit<'a, T: TypedNode<'a>>(node: &T, edit: Edit, lines: Vec<&str>) -> (InputEdit, String) {
    let lines: Vec<String> = lines.into_iter().map(|it| it.into()).collect();
    let text = lines.join("\n");
    let text_len = text.bytes().len();
    let adjust_end_position = |it: &mut Point| {
        it.row += lines.len().checked_sub(1).unwrap_or(0);
        it.column = lines.last().map(String::len).unwrap_or(0);
    };
    let input_edit = match edit {
        Edit::Append => InputEdit {
            start_byte: node.end_byte(),
            old_end_byte: node.end_byte(),
            start_position: node.end_position(),
            old_end_position: node.end_position(),

            new_end_byte: node.end_byte() + text_len,
            new_end_position: node.end_position().modify(adjust_end_position),
        },
        Edit::Prepend => InputEdit {
            start_byte: node.start_byte(),
            old_end_byte: node.start_byte(),
            start_position: node.start_position(),
            old_end_position: node.start_position(),

            new_end_byte: node.start_byte() + text_len,
            new_end_position: node.start_position().modify(adjust_end_position),
        },
        Edit::Replace => InputEdit {
            start_byte: node.start_byte(),
            old_end_byte: node.end_byte(),
            start_position: node.start_position(),
            old_end_position: node.end_position(),

            new_end_byte: node.start_byte() + text_len,
            new_end_position: node.start_position().modify(adjust_end_position),
        },
    };
    (input_edit, text)
}

/// Implements Kotlin-like scope functions.
trait ScopeFunc {
    /// Modify in place
    #[doc(alias = "also")]
    #[doc(alias = "apply")]
    fn modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
        Self: Sized,
    {
        f(&mut self);
        self
    }

    /// Consuming map (can't use self afterwards)
    #[doc(alias = "run")]
    #[doc(alias = "let")]
    #[doc(alias = "with")]
    fn transform<F, T>(self, f: F) -> T
    where
        F: FnOnce(Self) -> T,
        Self: Sized,
    {
        f(self)
    }
}

impl<T> ScopeFunc for T {}
