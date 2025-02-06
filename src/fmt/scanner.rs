use crate::fmt::{
    formatting::Formatting,
    node_rule::{DedentType, IndentType, NodeRules},
    util::constrained_value::Constrained,
};

use tree_sitter::{Node, QueryPredicateArg, TreeCursor};

use std::collections::HashMap;

use super::{format_item::FormatItem, node_rule::NodeRule, CaptureIndex, NodeId, PatternIndex};

use tree_sitter::{Query, QueryCursor};

#[derive(Debug)]
struct CapIndex {
    indent_anchor: Option<CaptureIndex>,
    indent: Option<CaptureIndex>,
    dedent: Option<CaptureIndex>,
    dedent_this: Option<CaptureIndex>,
    take_asis: Option<CaptureIndex>,

    spacing: HashMap<CaptureIndex, RulePositioning>,

    delete: Option<CaptureIndex>,
}

#[derive(Debug)]
enum RulePositioning {
    Before(FormatItem),
    After(FormatItem),
}

pub struct FormatScanner {
    query: Query,
    cursor: QueryCursor,
    captures: CapIndex,
}

impl FormatScanner {
    pub fn new(query: Query) -> Self {
        let spacing = query.capture_names().into_iter().map(|capture| {
            let mut split = capture.splitn(3, '.');
            let kind = split.next();
            let position = split.next();
            let repeats = split.next();

            let item = match kind {
                Some("space") => FormatItem::Space,
                Some("break") => FormatItem::Line,
                _ => return Err(capture),
            };

            let pos = match position {
                Some("before") => RulePositioning::Before,
                Some("after") => RulePositioning::After,
                _ => return Err(capture),
            };

            let constraint = match repeats {
                Some(n) => {
                    let mut split = n.splitn(2, "-");
                    let (a, b) = match (split.next(), split.next()) {
                        (Some(a), Some(b)) => match (a.parse::<u8>(), b.parse::<u8>()) {
                            (Ok(n1), Ok(n2)) => (n1, n2),
                            _ => return Err(capture),
                        },
                        (Some(a), None) => match a.parse::<u8>() {
                            Ok(n1) => (n1, n1),
                            _ => return Err(capture),
                        },

                        _ => return Err(capture),
                    };
                    Constrained::between(a, b)
                }
                None => Constrained::between(1, 1),
            };

            if split.next().is_some() {
                // We want exact matches, so if there happens to be another part, then the capture doesn't match.
                return Err(capture);
            }

            let cap_index = query
                .capture_index_for_name(&capture)
                .expect("we're iterating over existing strings");

            Ok((cap_index, pos(item(constraint))))
        });

        // explicit filter step so it's easy to debug-print if something is off.
        let spacing = spacing.filter_map(Result::ok).collect();
        // log::debug!("{:?}", spacing);

        let captures = CapIndex {
            indent_anchor: query.capture_index_for_name("indent.anchor"),
            indent: query.capture_index_for_name("indent"),
            dedent: query.capture_index_for_name("dedent"),
            dedent_this: query.capture_index_for_name("dedent.this"),
            take_asis: query.capture_index_for_name("take.as-is"),

            spacing,

            delete: query.capture_index_for_name("delete"),
        };
        Self {
            query,
            cursor: QueryCursor::new(),
            captures,
        }
    }

    pub fn scan(
        &mut self,
        tree: &tree_sitter::Tree,
        source: &str,
        formatter: &mut impl Formatting,
    ) {
        let mut rules: HashMap<NodeId, NodeRule> = HashMap::new();

        let mut node_actions: HashMap<(PatternIndex, CaptureIndex, &str), Box<str>> =
            HashMap::new();

        // let capturenames: Vec<_> = self.query.capture_names().iter().enumerate().collect();
        // dbg!(capturenames);

        // This could happen on init, speeds things up when these rules get re-used.
        for pattern_index in 1..self.query.pattern_count() {
            // for prop in self.query.property_settings(pattern_index) {
            //     log::debug!("Found the query property {:?}", prop);
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

                match self.captures.spacing.get(&cap.index) {
                    Some(RulePositioning::Before(item)) => rule.before.push(item.clone()),
                    Some(RulePositioning::After(item)) => rule.after.push(item.clone()),
                    None => {
                        let cap_index = Some(cap.index);
                        if cap_index == self.captures.delete {
                            *rule = NodeRule::default();
                            rule.replace = Some(FormatItem::Nothing);
                            continue;
                        } else if cap_index == self.captures.indent {
                            rule.indent = IndentType::Indent;
                        } else if cap_index == self.captures.indent_anchor {
                            rule.indent = IndentType::Anchor;
                        } else if cap_index == self.captures.dedent_this {
                            rule.dedent = DedentType::DedentThis;
                        } else if cap_index == self.captures.dedent {
                            rule.dedent = DedentType::DedentFollowing;
                        } else if cap_index == self.captures.take_asis {
                            rule.take_asis = true;
                        }
                    }
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
        // doc::debug!("{:#?}", rules);

        let mut iter = tree.walk();
        collect_outputs(formatter, &mut rules, iter.node(), &mut iter, source);
    }
}

fn collect_whitespace(outs: &mut impl Formatting, whitespace: &str) {
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
fn item_to_inkfmt(outs: &mut impl Formatting, item: FormatItem) {
    match item {
        FormatItem::Nothing => {}
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
    outs: &mut impl Formatting,
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

    if matches!(rule.dedent, DedentType::DedentThis) {
        outs.dedent();
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

    for output in rule.after {
        item_to_inkfmt(outs, output);
    }

    if matches!(rule.dedent, DedentType::DedentFollowing) {
        outs.dedent();
    }

    if let Some(next) = node.next_sibling() {
        collect_whitespace(outs, &source[node.end_byte()..next.start_byte()]);
    } else if let Some(parent) = node.parent() {
        collect_whitespace(outs, &source[node.end_byte()..parent.end_byte()]);
    }
}
