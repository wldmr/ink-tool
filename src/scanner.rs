use crate::{
    config,
    formatter::InkFmt,
    node_rule::{IndentType, NodeRules},
    util::constrained_value::Constrained,
};

use tree_sitter::{Node, QueryPredicateArg, TreeCursor};

use std::collections::HashMap;

use super::{format_item::FormatItem, node_rule::NodeRule, CaptureIndex, NodeId, PatternIndex};

use tree_sitter::{Query, QueryCursor};

struct CapIndex {
    indent_anchor: Option<CaptureIndex>,
    indent: Option<CaptureIndex>,
    dedent: Option<CaptureIndex>,
    take_asis: Option<CaptureIndex>,
    no_space_before: Option<CaptureIndex>,
    no_space_after: Option<CaptureIndex>,
    newline_before: Option<CaptureIndex>,
    newline_after: Option<CaptureIndex>,
    space_before: Option<CaptureIndex>,
    space_after: Option<CaptureIndex>,
    space_before_ifpresent: Option<CaptureIndex>,
    space_after_ifpresent: Option<CaptureIndex>,
    blank_line_before: Option<CaptureIndex>,
    blank_line_after: Option<CaptureIndex>,
    no_blank_line_before: Option<CaptureIndex>,
    no_blank_line_after: Option<CaptureIndex>,

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
            take_asis: query.capture_index_for_name("take.as-is"),
            no_space_before: query.capture_index_for_name("no.space.before"),
            no_space_after: query.capture_index_for_name("no.space.after"),
            newline_before: query.capture_index_for_name("newline.before"),
            newline_after: query.capture_index_for_name("newline.after"),
            space_before: query.capture_index_for_name("space.before"),
            space_after: query.capture_index_for_name("space.after"),
            space_before_ifpresent: query.capture_index_for_name("space.before.if-present"),
            space_after_ifpresent: query.capture_index_for_name("space.after.if-present"),
            blank_line_before: query.capture_index_for_name("blankline.before"),
            blank_line_after: query.capture_index_for_name("blankline.after"),
            no_blank_line_before: query.capture_index_for_name("no.blankline.before"),
            no_blank_line_after: query.capture_index_for_name("no.blankline.after"),
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
            for cap in match_.captures {
                let rule = rules.entry(cap.node.id()).or_default();
                // Deleting completely clobers all other intentions related to that node.
                // Let's handle that first;
                if let Some(FormatItem::Nothing) = rule.replace {
                    continue;
                }
                let cap_index = Some(cap.index);
                if cap_index == self.captures.delete {
                    *rule = NodeRule::default();
                    rule.replace = Some(FormatItem::Nothing);
                    continue;
                } else if cap_index == self.captures.indent {
                    rule.indent = IndentType::Indent;
                } else if cap_index == self.captures.indent_anchor {
                    rule.indent = IndentType::Anchor;
                } else if cap_index == self.captures.dedent {
                    rule.dedent = true;
                } else if cap_index == self.captures.take_asis {
                    rule.take_asis = true;
                } else if cap_index == self.captures.no_space_before {
                    rule.before.push(FormatItem::Antispace);
                } else if cap_index == self.captures.no_space_after {
                    rule.after.push(FormatItem::Antispace);
                } else if cap_index == self.captures.space_before {
                    rule.before
                        .push(FormatItem::Space(Constrained::between(1, 1)));
                } else if cap_index == self.captures.space_after {
                    rule.after
                        .push(FormatItem::Space(Constrained::between(1, 1)));
                } else if cap_index == self.captures.space_before_ifpresent {
                    rule.before.push(FormatItem::Space(Constrained::at_most(1)));
                } else if cap_index == self.captures.space_after_ifpresent {
                    rule.after.push(FormatItem::Space(Constrained::at_most(1)));
                } else if cap_index == self.captures.newline_before {
                    rule.before.push(FormatItem::Line(Constrained::at_least(1)));
                } else if cap_index == self.captures.newline_after {
                    rule.after.push(FormatItem::Line(Constrained::at_least(1)));
                } else if cap_index == self.captures.blank_line_before {
                    rule.before.push(FormatItem::Line(Constrained::at_least(2)));
                } else if cap_index == self.captures.blank_line_after {
                    rule.after.push(FormatItem::Line(Constrained::at_least(2)));
                } else if cap_index == self.captures.no_blank_line_before {
                    rule.before.push(FormatItem::Antiblank);
                } else if cap_index == self.captures.no_blank_line_after {
                    rule.after.push(FormatItem::Antiblank);
                }

                if let Some(action) =
                    node_actions.get(&(match_.pattern_index, cap.index, "replace"))
                {
                    if let Some(ref existing) = rule.replace {
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
                    rule.before
                        .push(FormatItem::Text(action.clone().into_string()))
                }
                if let Some(action) = node_actions.get(&(match_.pattern_index, cap.index, "append"))
                {
                    rule.after
                        .push(FormatItem::Text(action.clone().into_string()))
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
    match newlines {
        0 => outs.space(whitespace.len()),
        n => outs.line(n),
    }
}

// IDEA: Get rid of format_item and use something like Vec<FnMut(impl InkFmt)>
fn item_to_inkfmt(outs: &mut impl InkFmt, item: FormatItem) {
    match item {
        FormatItem::Nothing => {}
        FormatItem::Antispace => outs.space(Constrained::at_most(0)),
        FormatItem::Antiblank => outs.line(Constrained::at_most(1)),
        FormatItem::Space(it) => outs.space(it),
        FormatItem::Line(it) => outs.line(it),
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
    if let Some(prev) = node.prev_sibling() {
        collect_whitespace(outs, &source[prev.end_byte()..node.start_byte()]);
    } else if let Some(parent) = node.parent() {
        collect_whitespace(outs, &source[parent.start_byte()..node.start_byte()]);
    }

    for output in rule.before {
        item_to_inkfmt(outs, output);
    }

    match rule.indent {
        IndentType::Indent => outs.indent(),
        IndentType::Anchor => outs.align_indent_to_current_column(),
        IndentType::None => (),
    }

    if let Some(output) = rule.replace {
        item_to_inkfmt(outs, output);
    } else if rule.take_asis || node.child_count() == 0 {
        outs.text(&source[node.byte_range()]);
    } else {
        let children: Vec<_> = node.children(iter).collect();
        for child in children {
            collect_outputs(outs, rules, child, iter, source);
        }
    }

    if rule.dedent {
        outs.dedent();
    }

    for output in rule.after {
        item_to_inkfmt(outs, output);
    }

    if let Some(next) = node.next_sibling() {
        collect_whitespace(outs, &source[node.end_byte()..next.start_byte()]);
    } else if let Some(parent) = node.parent() {
        collect_whitespace(outs, &source[node.end_byte()..parent.end_byte()]);
    }
}
