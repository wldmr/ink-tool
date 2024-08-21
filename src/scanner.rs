use crate::{
    config,
    format_item::Space,
    formatter::InkFmt,
    node_rule::{IndentType, NodeRules},
};

use tree_sitter::{Node, QueryPredicateArg, TreeCursor};

use std::collections::HashMap;

use super::{format_item::FormatItem, node_rule::NodeRule, CaptureIndex, NodeId, PatternIndex};

use tree_sitter::{Query, QueryCursor};

struct CapIndex {
    indent_anchor: Option<CaptureIndex>,
    indent: Option<CaptureIndex>,
    dedent: Option<CaptureIndex>,
    leaf: Option<CaptureIndex>,
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
            indent_anchor: query.capture_index_for_name("indent.anchor"),
            indent: query.capture_index_for_name("indent"),
            dedent: query.capture_index_for_name("dedent"),
            leaf: query.capture_index_for_name("leaf"),
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

    pub fn scan(&mut self, tree: &tree_sitter::Tree, source: &str, formatter: &mut impl InkFmt) {
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
            let space = || {
                FormatItem::Space(Space {
                    repeats: 1,
                    linebreak: false,
                    existing: false,
                })
            };
            let linebreak = |n| {
                FormatItem::Space(Space {
                    repeats: n,
                    linebreak: true,
                    existing: false,
                })
            };
            for cap in match_.captures {
                let mut rule = NodeRule::default();
                let cap_index = Some(cap.index);
                if cap_index == self.captures.indent {
                    rule.indent = IndentType::Indent;
                } else if cap_index == self.captures.indent_anchor {
                    rule.indent = IndentType::Anchor;
                } else if cap_index == self.captures.dedent {
                    rule.dedent = true;
                } else if cap_index == self.captures.leaf {
                    rule.is_leaf = true;
                } else if cap_index == self.captures.no_space_before {
                    rule.before = Some(FormatItem::Antispace);
                } else if cap_index == self.captures.no_space_after {
                    rule.after = Some(FormatItem::Antispace);
                } else if cap_index == self.captures.space_before {
                    rule.before = Some(space());
                } else if cap_index == self.captures.space_after {
                    rule.after = Some(space());
                } else if cap_index == self.captures.newline_before {
                    rule.before = Some(linebreak(1));
                } else if cap_index == self.captures.newline_after {
                    rule.after = Some(linebreak(1));
                } else if cap_index == self.captures.blank_line_before {
                    rule.before = Some(linebreak(2));
                } else if cap_index == self.captures.blank_line_after {
                    rule.after = Some(linebreak(2));
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

        collect_outputs(formatter, &mut rules, iter.node(), &mut iter, source);
    }
}

fn collect_whitespace(outs: &mut impl InkFmt, whitespace: &str) {
    let newlines = whitespace
        .chars()
        .inspect(|it| assert!(it.is_whitespace()))
        .filter(|it| *it == '\n')
        .count();
    // We make a bunch of decisions about admissable existing whitespace here.
    // These can _only_ be overriden by rules mandating wider spacing.
    match newlines {
        // Existing spaces and linebreaks are kept, but capped at 1.
        0 if whitespace.len() >= 1 => outs.space(1),
        0 => {}
        1 => outs.line(1),
        // At most one blank line allowed
        _ => outs.line(2),
    }
}

// IDEA: Get rid of format_item and use something like Vec<FnMut(impl InkFmt)>
fn item_to_inkfmt(outs: &mut impl InkFmt, item: FormatItem) {
    match item {
        FormatItem::Nothing => {}
        FormatItem::Antispace => outs.antispace(),
        FormatItem::Space(it) => {
            if it.linebreak {
                outs.line(it.repeats)
            } else {
                outs.space(it.repeats)
            }
        }
        FormatItem::Text(it) => outs.text(&it),
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
    outs: &mut impl InkFmt,
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
        item_to_inkfmt(outs, output);
    } else if let Some(prev) = node.prev_sibling() {
        let whitespace = source[prev.end_byte()..node.start_byte()].to_owned();
        collect_whitespace(outs, &whitespace);
    } else if let Some(parent) = node.parent() {
        let whitespace = source[parent.start_byte()..node.start_byte()].to_owned();
        collect_whitespace(outs, &whitespace);
    }

    match rule.indent {
        IndentType::Indent => outs.indent(),
        IndentType::Anchor => outs.align_indent_to_current_column(),
        IndentType::None => (),
    }

    if let Some(output) = rule.replace {
        item_to_inkfmt(outs, output);
    } else if rule.is_leaf || node.child_count() == 0 {
        outs.text(&source[node.byte_range()].to_owned());
    } else {
        let children: Vec<_> = node.children(iter).collect();
        for child in children {
            collect_outputs(outs, rules, child, iter, source);
        }
    }

    if rule.dedent {
        outs.dedent();
    }

    if let Some(output) = rule.after {
        item_to_inkfmt(outs, output);
    } else if let Some(next) = node.next_sibling() {
        let whitespace = source[node.end_byte()..next.start_byte()].to_owned();
        collect_whitespace(outs, &whitespace);
    } else if let Some(parent) = node.parent() {
        let whitespace = source[node.end_byte()..parent.end_byte()].to_owned();
        collect_whitespace(outs, &whitespace);
    }
}
