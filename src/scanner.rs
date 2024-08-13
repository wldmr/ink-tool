use crate::{config, format_item::Align, node_rule::NodeRules};

use tree_sitter::{Node, QueryPredicateArg, TreeCursor};

use std::collections::HashMap;

use super::{
    format_item::FormatItem, formatter::Formatter, node_rule::NodeRule, CaptureIndex, NodeId,
    PatternIndex,
};

use tree_sitter::{Query, QueryCursor};

struct CapIndex {
    align: Option<CaptureIndex>,
    aligned: Option<CaptureIndex>,
    no_space_before: Option<CaptureIndex>,
    no_space_after: Option<CaptureIndex>,
    newline_before: Option<CaptureIndex>,
    newline_after: Option<CaptureIndex>,
    space_before: Option<CaptureIndex>,
    space_after: Option<CaptureIndex>,
    blank_line_before: Option<CaptureIndex>,
    blank_line_after: Option<CaptureIndex>,
    delete: Option<CaptureIndex>,
}

pub struct FormatScanner {
    query: Query,
    cursor: QueryCursor,
    captures: CapIndex,
}

impl FormatScanner {
    pub fn new(query: Query, _config: config::FormatConfig) -> Self {
        let captures = CapIndex {
            align: query.capture_index_for_name("align"),
            aligned: query.capture_index_for_name("aligned"),
            no_space_before: query.capture_index_for_name("no.space.before"),
            no_space_after: query.capture_index_for_name("no.space.after"),
            newline_before: query.capture_index_for_name("newline.before"),
            newline_after: query.capture_index_for_name("newline.after"),
            space_before: query.capture_index_for_name("space.before"),
            space_after: query.capture_index_for_name("space.after"),
            blank_line_before: query.capture_index_for_name("blankline.before"),
            blank_line_after: query.capture_index_for_name("blankline.after"),
            delete: query.capture_index_for_name("delete"),
        };
        Self {
            query,
            cursor: QueryCursor::new(),
            captures,
        }
    }

    pub fn scan(&mut self, tree: &tree_sitter::Tree, source: &str) -> Formatter {
        let mut rules: HashMap<NodeId, NodeRule> = HashMap::new();

        let mut node_actions: HashMap<(PatternIndex, CaptureIndex, &str), Box<str>> =
            HashMap::new();

        // let capturenames: Vec<_> = self.query.capture_names().iter().enumerate().collect();
        // dbg!(capturenames);

        // This could happen on init, speeds things up when these rules get re-used.
        for pattern_index in 1..self.query.pattern_count() {
            // for prop in self.query.property_settings(pattern_index) {
            //     eprintln!("Found the query property {:?}", prop);
            // }
            for prop in self.query.general_predicates(pattern_index) {
                let op = &*prop.operator;
                let args = &*prop.args;
                match (op, args) {
                    (op, [QueryPredicateArg::Capture(index), QueryPredicateArg::String(value)])
                        if op == "prepend" || op == "append" || op == "replace" =>
                    {
                        let key = (pattern_index, *index, op);
                        if let Some(other) = node_actions.insert(key, value.clone()) {
                            panic!(
                            "Pattern {pattern_index}: Duplicate node action for #{op}: Previous '{other}', replaced by '{value}'",
                        )
                        }
                    }
                    (op, args) => {
                        panic!(
                            "Pattern {pattern_index}: Unknown query predicate #{op}({:?})",
                            args
                        )
                    }
                }
            }
        }

        for match_ in self
            .cursor
            .matches(&self.query, tree.root_node(), source.as_bytes())
        {
            for cap in match_.captures {
                let mut rule = NodeRule::default();
                let cap_index = Some(cap.index);
                if cap_index == self.captures.align {
                    // dbg!(&match_.id(), &match_.captures);
                    rule.align_self = true;
                } else if cap_index == self.captures.aligned {
                    rule.align_children = true;
                } else if cap_index == self.captures.no_space_before {
                    rule.before = Some(FormatItem::Antispace);
                } else if cap_index == self.captures.no_space_after {
                    rule.after = Some(FormatItem::Antispace);
                } else if cap_index == self.captures.space_before {
                    rule.before = Some(FormatItem::Space);
                } else if cap_index == self.captures.space_after {
                    rule.after = Some(FormatItem::Space);
                } else if cap_index == self.captures.newline_before {
                    rule.before = Some(FormatItem::Newline);
                } else if cap_index == self.captures.newline_after {
                    rule.after = Some(FormatItem::Newline);
                } else if cap_index == self.captures.blank_line_before {
                    rule.before = Some(FormatItem::BlankLine);
                } else if cap_index == self.captures.blank_line_after {
                    rule.after = Some(FormatItem::BlankLine);
                } else if cap_index == self.captures.delete {
                    // eprintln!(
                    //     "Delete requested for {:?} (id {}), overrides all other rules",
                    //     cap,
                    //     cap.node.id()
                    // );
                    rule.replace = Some(FormatItem::Nothing);
                    let _ = rules.insert(cap.node.id(), rule);
                    continue;
                }

                if let Some(action) =
                    node_actions.get(&(match_.pattern_index, cap.index, "replace"))
                {
                    if let Some(existing) = rule.replace {
                        panic!(
                            "Conflicting directives for replacement of {:?}: {:?} vs {:?}",
                            cap, existing, action
                        );
                    }
                    rule.replace = Some(FormatItem::Text(action.clone().into_string()))
                }

                if let Some(action) =
                    node_actions.get(&(match_.pattern_index, cap.index, "prepend"))
                {
                    rule.before = Some(FormatItem::Text(action.clone().into_string()))
                }
                if let Some(action) = node_actions.get(&(match_.pattern_index, cap.index, "append"))
                {
                    rule.after = Some(FormatItem::Text(action.clone().into_string()))
                }

                if let Some(existing) = rules.get_mut(&cap.node.id()) {
                    // eprintln!(
                    //     "Merging {:?} and {:?} for {:?} for node {}",
                    //     existing,
                    //     rule,
                    //     cap,
                    //     cap.node.id()
                    // );
                    existing.merge(rule);
                } else {
                    rules.insert(cap.node.id(), rule);
                }
            }
        }

        let mut iter = tree.walk();
        let mut out: Vec<FormatItem> = Vec::new();
        collect_outputs(&mut out, &mut rules, iter.node(), &mut iter, source);

        Formatter::new_from_items(out)
    }
}

/// Applies the appropriate rule from `rules` (if any) to the current node.
/// If no rule applies, simply copies the input to the output for leaf nodes,
/// including leading and trailing whitspace.
///
/// Recursively walk the children of this node.
///
/// `outs` will contain the collected output items, in the order that it should appear in the output.
fn collect_outputs<'t>(
    outs: &mut Vec<FormatItem>,
    rules: &mut NodeRules,
    node: Node<'t>,
    iter: &mut TreeCursor<'t>,
    source: &str,
) {
    let rule = rules.remove(&node.id()).unwrap_or_default();

    // dbg!(&node.id(), &node, &rule);

    // TODO: We double up existing whitespace by adding it before and after. It probably makes sense to not do that.
    // (unlike spaces added by rules, there's not much debugging value in duplicate existing whitespace).

    if let Some(output) = rule.before {
        outs.push(output);
    } else if let Some(prev) = node.prev_sibling() {
        let whitespace = source[prev.end_byte()..node.start_byte()].to_owned();
        if whitespace.len() != 0 {
            outs.push(FormatItem::ExistingWhitespace(whitespace))
        }
    } else if let Some(parent) = node.parent() {
        let whitespace = source[parent.start_byte()..node.start_byte()].to_owned();
        if whitespace.len() != 0 {
            outs.push(FormatItem::ExistingWhitespace(whitespace))
        }
    }

    if rule.align_self {
        outs.push(FormatItem::Align(Align::new()));
    }

    if rule.align_children {
        outs.push(FormatItem::AlignmentStart);
    }

    if let Some(output) = rule.replace {
        outs.push(output);
    } else if node.child_count() == 0 {
        outs.push(FormatItem::Text(source[node.byte_range()].to_owned()))
    } else {
        let children: Vec<_> = node.children(iter).collect();
        for child in children {
            collect_outputs(outs, rules, child, iter, source);
        }
    }

    if rule.align_children {
        outs.push(FormatItem::AlignmentEnd);
    }

    if let Some(output) = rule.after {
        outs.push(output);
    } else if let Some(next) = node.next_sibling() {
        let whitespace = source[node.end_byte()..next.start_byte()].to_owned();
        if whitespace.len() != 0 {
            outs.push(FormatItem::ExistingWhitespace(whitespace))
        }
    } else if let Some(parent) = node.parent() {
        let whitespace = source[node.end_byte()..parent.end_byte()].to_owned();
        if whitespace.len() != 0 {
            outs.push(FormatItem::ExistingWhitespace(whitespace))
        }
    }
}
