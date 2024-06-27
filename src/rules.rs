use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

use tree_sitter::{InputEdit, Node, Point, Query, QueryMatch, QueryPredicateArg, QueryProperty};

use super::FormatConfig;

pub(crate) type CaptureIndex = u32;

pub trait Rule {
    fn new(query: &Rc<Query>, _config: &Rc<FormatConfig>) -> Option<Self>
    where
        Self: Sized;
    fn captures(&self) -> Vec<&'static str>;
    fn visit(
        &mut self,
        m: &QueryMatch,
        props: &[QueryProperty],
        source: &str,
    ) -> Option<EditResult>;
}

pub type EditResult = (InputEdit, String);

/// Wrapping a rule in a box is a bit ugly, so we macro it away.
/// Seems to me that this should be easier,
macro_rules! init_rules {
    ($query:ident, $config:ident => $($rule:ident),+) => {
        vec![$($rule::new(&$query, &$config).map(|rule| Box::new(rule) as Box<dyn crate::rules::Rule>)),+]
        .into_iter()
        .filter_map(|maybe_rule| maybe_rule)
        .collect()
    };
}

pub(super) fn init_rules(config: Rc<FormatConfig>, query: &Rc<Query>) -> Vec<Box<dyn Rule>> {
    init_rules![query, config => ReplaceThis, ReplaceBetween, IndentAnchored, Rewrite]
}

pub(super) struct ReplaceThis {
    capture: CaptureIndex,
}

impl Rule for ReplaceThis {
    fn new(query: &Rc<Query>, _config: &Rc<FormatConfig>) -> Option<Self> {
        query
            .capture_index_for_name("replace.this")
            .map(|capture| Self { capture })
    }

    fn captures(&self) -> Vec<&'static str> {
        vec!["replace.this"]
    }

    fn visit(
        &mut self,
        m: &QueryMatch,
        props: &[QueryProperty],
        source: &str,
    ) -> Option<EditResult> {
        let replacement = props
            .iter()
            .find_map(|p| (p.key.deref() == "replacement").then(|| p.value.clone()))
            .flatten()?;
        let replacement = replacement.deref();
        m.captures.iter().find_map(|cap| {
            if cap.index != self.capture {
                return None;
            }
            let existing = &source[cap.node.start_byte()..cap.node.end_byte()];
            (replacement != existing).then(|| replace(&cap.node, replacement))
        })
    }
}

pub(crate) struct ReplaceBetween {
    start: CaptureIndex,
    end: CaptureIndex,
}

impl Rule for ReplaceBetween {
    fn new(query: &Rc<Query>, _config: &Rc<FormatConfig>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            start: query.capture_index_for_name("replace.start")?,
            end: query.capture_index_for_name("replace.end")?,
        })
    }

    fn captures(&self) -> Vec<&'static str> {
        vec!["replace.before"]
    }

    fn visit(
        &mut self,
        m: &QueryMatch,
        props: &[QueryProperty],
        source: &str,
    ) -> Option<EditResult> {
        let replacement_text = props
            .iter()
            .find_map(|p| (p.key.deref() == "replacement").then(|| p.value.clone()))
            .flatten()?;
        let replacement = replacement_text.deref();
        let start_byte = m
            .captures
            .iter()
            .find_map(|c| (c.index == self.start).then_some(c.node))?
            .end_byte();
        let end_byte = m
            .captures
            .iter()
            .find_map(|c| (c.index == self.end).then_some(c.node))?
            .start_byte();
        let extent = (start_byte, end_byte);
        let preceding_text = &source[extent.0..extent.1];
        (replacement != preceding_text).then(|| replace_range(extent.0, extent.1, replacement))
    }
}

#[derive(Default)]
pub(super) struct IndentAnchored {
    anchor: CaptureIndex,
    anchored: CaptureIndex,
}

impl Rule for IndentAnchored {
    fn new(query: &Rc<Query>, _config: &Rc<FormatConfig>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            anchor: query.capture_index_for_name("indent.anchor")?,
            anchored: query.capture_index_for_name("indent.to.anchor")?,
        })
    }

    fn captures(&self) -> Vec<&'static str> {
        vec!["indent.anhor", "indent.to.anchor"]
    }

    fn visit(
        &mut self,
        m: &QueryMatch,
        _props: &[QueryProperty],
        source: &str,
    ) -> Option<EditResult> {
        let anchor = m.captures.iter().find(|it| it.index == self.anchor)?.node;
        let anchored = m.captures.iter().find(|it| it.index == self.anchored)?.node;

        let target_whitespace_width = anchor.start_position().column;
        let existing_whitespace_width = anchored.start_position().column;

        let start = anchored.start_byte() - existing_whitespace_width;
        let end = anchored.start_byte();
        let existing = &source[start..end];

        let replacement = &" ".repeat(target_whitespace_width);

        (existing != replacement).then(|| replace_range(start, end, replacement))
    }
}

pub(super) struct Rewrite {
    query: Rc<Query>,
    capture: CaptureIndex,
}

impl Rule for Rewrite {
    fn new(query: &Rc<Query>, _config: &Rc<FormatConfig>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            query: query.clone(),
            capture: query.capture_index_for_name("rewrite")?,
        })
    }

    fn captures(&self) -> Vec<&'static str> {
        vec!["rewrite"]
    }

    fn visit(
        &mut self,
        m: &QueryMatch,
        _props: &[QueryProperty],
        source: &str,
    ) -> Option<EditResult> {
        let to_replace = m.captures.iter().find(|it| it.index == self.capture)?.node;
        let new_order = self
            .query
            .general_predicates(m.pattern_index)
            .iter()
            .find(|pred| pred.operator.deref() == "rewrite-to")
            .map(|it| it.args.deref())?;
        let captures: HashSet<CaptureIndex> = new_order
            .iter()
            .filter_map(|it| {
                if let QueryPredicateArg::Capture(n) = *it {
                    Some(n)
                } else {
                    None
                }
            })
            .collect();
        let original_nodes: HashMap<CaptureIndex, &str> = m
            .captures
            .iter()
            .filter(|it| captures.contains(&it.index))
            .map(|it| (it.index, &source[it.node.start_byte()..it.node.end_byte()]))
            .collect();
        let output: Vec<&str> = new_order
            .iter()
            .map(|item| match item {
                QueryPredicateArg::Capture(n) => original_nodes.get(n).unwrap(),
                QueryPredicateArg::String(s) => s.deref(),
            })
            .collect();
        let output = output.join("");
        Some(replace_range(
            to_replace.start_byte(),
            to_replace.end_byte(),
            &output,
        ))
    }
}

fn replace(node: &Node, text: &str) -> EditResult {
    (
        InputEdit {
            start_byte: node.start_byte(),
            old_end_byte: node.end_byte(),
            new_end_byte: node.start_byte() + text.len(),
            // Since we're not going the reuse the tree, we won't spend any time getting these right:
            start_position: node.start_position(),
            old_end_position: node.start_position(),
            new_end_position: node.start_position(),
        },
        text.to_owned(),
    )
}
fn replace_range(start: usize, end: usize, text: &str) -> EditResult {
    (
        InputEdit {
            start_byte: start,
            old_end_byte: end,
            new_end_byte: start + text.len(),
            // Since we're not going the reuse the tree, we won't spend any time getting these right:
            start_position: Point::new(0, 0),
            old_end_position: Point::new(0, 0),
            new_end_position: Point::new(0, 0),
        },
        text.to_owned(),
    )
}
