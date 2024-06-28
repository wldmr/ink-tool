use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

use tree_sitter::{InputEdit, Node, Point, Query, QueryMatch, QueryPredicateArg};

use super::FormatConfig;

pub(crate) type CaptureIndex = u32;

pub trait Rule {
    fn new(query: &Query, _config: &Rc<FormatConfig>) -> Option<Self>
    where
        Self: Sized;
    fn captures(&self) -> Vec<&'static str>;

    fn edit(&mut self, query: &Query, match_: &QueryMatch, source: &str) -> Option<EditResult>;

    fn edit_if_needed(
        &mut self,
        query: &Query,
        match_: &QueryMatch,
        source: &str,
    ) -> Option<EditResult> {
        self.edit(query, match_, source).filter(|edit| {
            let existing_text = &source[edit.0.start_byte..edit.0.old_end_byte];
            existing_text != edit.1
        })
    }
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

pub(super) fn init_rules(config: Rc<FormatConfig>, query: &Query) -> Vec<Box<dyn Rule>> {
    init_rules![query, config => ReplaceThis, ReplaceBetween, IndentAnchored, Rewrite]
}

pub(super) struct ReplaceThis {
    capture: CaptureIndex,
}

impl Rule for ReplaceThis {
    fn new(query: &Query, _config: &Rc<FormatConfig>) -> Option<Self> {
        Some(Self {
            capture: query.capture_index_for_name("replace.this")?,
        })
    }

    fn captures(&self) -> Vec<&'static str> {
        vec!["replace.this"]
    }

    fn edit(&mut self, query: &Query, match_: &QueryMatch, _source: &str) -> Option<EditResult> {
        let replacement = query
            .property_settings(match_.pattern_index)
            .iter()
            .find_map(|p| (p.key.deref() == "replacement").then(|| p.value.clone()))
            .flatten()?;
        let replacement = replacement.deref();
        match_
            .captures
            .iter()
            .find(|cap| cap.index == self.capture)
            .map(|cap| replace(&cap.node, replacement))
    }
}

pub(crate) struct ReplaceBetween {
    start: CaptureIndex,
    end: CaptureIndex,
}

impl Rule for ReplaceBetween {
    fn new(query: &Query, _config: &Rc<FormatConfig>) -> Option<Self>
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

    fn edit(&mut self, query: &Query, match_: &QueryMatch, _source: &str) -> Option<EditResult> {
        let replacement_text = query
            .property_settings(match_.pattern_index)
            .iter()
            .find_map(|p| (p.key.deref() == "replacement").then(|| p.value.clone()))
            .flatten()?;
        let replacement = replacement_text.deref();
        let start_byte = match_
            .captures
            .iter()
            .find_map(|c| (c.index == self.start).then_some(c.node))?
            .end_byte();
        let end_byte = match_
            .captures
            .iter()
            .find_map(|c| (c.index == self.end).then_some(c.node))?
            .start_byte();
        Some(replace_range(start_byte, end_byte, replacement))
    }
}

pub(super) struct IndentAnchored {
    anchor: CaptureIndex,
    anchored: CaptureIndex,
}

impl Rule for IndentAnchored {
    fn new(query: &Query, _config: &Rc<FormatConfig>) -> Option<Self>
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

    fn edit(&mut self, _query: &Query, match_: &QueryMatch, _source: &str) -> Option<EditResult> {
        let anchor = match_
            .captures
            .iter()
            .find(|it| it.index == self.anchor)?
            .node;
        let anchored = match_
            .captures
            .iter()
            .find(|it| it.index == self.anchored)?
            .node;

        let target_whitespace_width = anchor.start_position().column;
        let existing_whitespace_width = anchored.start_position().column;

        let start = anchored.start_byte() - existing_whitespace_width;
        let end = anchored.start_byte();

        let replacement = &" ".repeat(target_whitespace_width);

        Some(replace_range(start, end, replacement))
    }
}

pub(super) struct Rewrite {
    capture: CaptureIndex,
}

impl Rule for Rewrite {
    fn new(query: &Query, _config: &Rc<FormatConfig>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            capture: query.capture_index_for_name("rewrite")?,
        })
    }

    fn captures(&self) -> Vec<&'static str> {
        vec!["rewrite"]
    }

    fn edit(&mut self, query: &Query, match_: &QueryMatch, source: &str) -> Option<EditResult> {
        let to_replace = match_
            .captures
            .iter()
            .find(|it| it.index == self.capture)?
            .node;
        let new_order = query
            .general_predicates(match_.pattern_index)
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
        let original_nodes: HashMap<CaptureIndex, &str> = match_
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
