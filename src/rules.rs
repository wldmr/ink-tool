use std::{
    collections::HashMap,
    fmt::{Debug, Write},
};

use tree_sitter::{Node, Point, Query, QueryCursor, QueryPredicateArg, TreeCursor};

use crate::config;

type CaptureIndex = u32;
type PatternIndex = usize;
type NodeId = usize;

static QUERY: &str = include_str!("format.scm");

#[derive(PartialEq)]
enum Output {
    Nothing,
    Antispace,
    Space,
    Newline,
    BlankLine,
    ExistingWhitespace(String),
    Text(String),
}

impl Debug for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Output::Nothing => f.write_char('⌧'),
            Output::Antispace => f.write_char('⁀'),
            Output::Space => f.write_char('␣'),
            Output::Newline => f.write_char('⏎'),
            Output::BlankLine => f.write_char('¶'),
            Output::ExistingWhitespace(sp) => {
                f.write_char('∃')?;
                Debug::fmt(&sp, f)
            }
            Output::Text(txt) => f.write_fmt(format_args!("'{txt}'")),
        }
    }
}

impl Output {
    pub fn merge(self, other: Output) -> Result<Output, (Output, Output)> {
        match (&self, &other) {
            (Self::Nothing, _) => Ok(other),
            (_, Self::Nothing) => Ok(self),
            (Self::Antispace, Self::Antispace) => Ok(Self::Antispace),
            (Self::Antispace, Self::Space) => Ok(Self::Antispace),
            (Self::Antispace, Self::Newline) => Ok(Self::Antispace),
            (Self::Antispace, Self::BlankLine) => Ok(Self::Antispace),
            (Self::Antispace, Self::ExistingWhitespace(_)) => Ok(Self::Antispace),
            (Self::Antispace, Self::Text(_)) => Ok(other),
            (Self::Space, Self::Antispace) => Ok(Self::Antispace),
            (Self::Space, Self::Space) => Ok(Self::Space),
            (Self::Space, Self::Newline) => Ok(Self::Newline),
            (Self::Space, Self::BlankLine) => Ok(Self::BlankLine),
            (Self::Space, Self::ExistingWhitespace(_)) => Ok(Self::Space),
            (Self::Space, Self::Text(_)) => Err((self, other)),
            (Self::Newline, Self::Antispace) => Ok(Self::Antispace),
            (Self::Newline, Self::Space) => Ok(Self::Newline),
            (Self::Newline, Self::Newline) => Ok(Self::Newline),
            (Self::Newline, Self::BlankLine) => Ok(Self::BlankLine),
            (Self::Newline, Self::ExistingWhitespace(_)) => Ok(Self::Newline),
            (Self::Newline, Self::Text(_)) => Err((self, other)),
            (Self::BlankLine, Self::Antispace) => Ok(Self::Antispace),
            (Self::BlankLine, Self::Space) => Ok(Self::BlankLine),
            (Self::BlankLine, Self::Newline) => Ok(Self::BlankLine),
            (Self::BlankLine, Self::BlankLine) => Ok(Self::BlankLine),
            (Self::BlankLine, Self::ExistingWhitespace(_)) => Ok(Self::BlankLine),
            (Self::BlankLine, Self::Text(_)) => Err((self, other)),
            (Self::ExistingWhitespace(_), Self::Antispace) => Ok(Self::Antispace),
            (Self::ExistingWhitespace(_), Self::Space) => Ok(Self::Space),
            (Self::ExistingWhitespace(_), Self::Newline) => Ok(Self::Newline),
            (Self::ExistingWhitespace(_), Self::BlankLine) => Ok(Self::BlankLine),
            // Two adjacent nodes reported the same whitespace as after and before respectively, so we can absorb one
            (Self::ExistingWhitespace(a), Self::ExistingWhitespace(b)) if a == b => Ok(self),
            (Self::ExistingWhitespace(_), Self::ExistingWhitespace(_)) => Err((self, other)),
            (Self::ExistingWhitespace(_), Self::Text(_)) => Err((self, other)),
            (Self::Text(_), Self::Antispace) => Err((self, other)),
            (Self::Text(_), Self::Space) => Err((self, other)),
            (Self::Text(_), Self::Newline) => Err((self, other)),
            (Self::Text(_), Self::BlankLine) => Err((self, other)),
            (Self::Text(_), Self::ExistingWhitespace(_)) => Err((self, other)),
            (Self::Text(_), Self::Text(_)) => Err((self, other)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Output::Nothing => "",
            Output::Antispace => "",
            Output::Space => " ",
            Output::Newline => "\n",
            Output::BlankLine => "\n\n",
            Output::ExistingWhitespace(ws) => ws,
            Output::Text(txt) => txt,
        }
    }
}

impl ToString for Output {
    fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}

#[derive(PartialEq, Eq)]
struct Align {
    pattern: PatternIndex,
    pos: Point,
}

impl Debug for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}|{}:{}",
            self.pattern, self.pos.row, self.pos.column
        ))
    }
}

#[derive(Default)]
struct Rule {
    /// All Nodes with the same index get aliged to te same column
    align: Option<Align>,
    before: Option<Output>,
    after: Option<Output>,
    replace: Option<Output>,
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Rule")?;
        if let Some(ref field) = self.align {
            f.write_fmt(format_args!("={:?}", field))?
        }
        if let Some(ref field) = self.before {
            f.write_fmt(format_args!("←{:?}", field))?
        }
        if let Some(ref field) = self.replace {
            f.write_fmt(format_args!("•{:?}", field))?
        }
        if let Some(ref field) = self.after {
            f.write_fmt(format_args!("→{:?}", field))?
        }
        Ok(())
    }
}

impl Rule {
    fn merge(&mut self, other: Rule) {
        if self.replace == Some(Output::Nothing) || other.replace == Some(Output::Nothing) {
            self.replace = Some(Output::Nothing);
            self.align = None;
            self.before = None;
            self.after = None;
        } else {
            Self::update_option(&mut self.align, other.align);
            Self::update_option(&mut self.before, other.before);
            Self::update_option(&mut self.after, other.after);
            Self::update_option(&mut self.replace, other.replace);
        }
    }

    fn update_option<T: Debug + PartialEq>(old: &mut Option<T>, new: Option<T>) {
        if new.is_none() {
            return;
        } else if old.as_ref().is_some() {
            eprintln!("Warning: Updating Old {:?}, New {:?}", old, new);
        }
        *old = new;
    }
}

type NodeRules = HashMap<NodeId, Rule>;

#[derive(Debug)]
pub struct FormattableOutput(Vec<Output>);

impl FormattableOutput {
    fn new(tree: &tree_sitter::Tree, source: &str, mut rules: NodeRules) -> Self {
        let mut iter = tree.walk();
        let mut out: Vec<Output> = Vec::new();
        collect_outputs(&mut out, &mut rules, iter.node(), &mut iter, source);
        Self(out)
    }

    pub fn normalize(&mut self) {
        // This merging could be done directly while building the outputs,
        // but taking it in steps will help with debugging.
        let mut original = std::mem::take(&mut self.0).into_iter();

        if let Some(mut accumulator) = original.next() {
            while let Some(output) = original.next() {
                match accumulator.merge(output) {
                    Ok(merged) => accumulator = merged,
                    Err((left, right)) => {
                        self.0.push(left);
                        accumulator = right;
                    }
                }
            }
        }
    }
}

impl ToString for FormattableOutput {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for output in self.0.iter() {
            result.push_str(output.as_str())
        }
        result
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
    outs: &mut Vec<Output>,
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
            outs.push(Output::ExistingWhitespace(whitespace))
        }
    } else if let Some(parent) = node.parent() {
        let whitespace = source[parent.start_byte()..node.start_byte()].to_owned();
        if whitespace.len() != 0 {
            outs.push(Output::ExistingWhitespace(whitespace))
        }
    }

    if let Some(output) = rule.replace {
        outs.push(output);
    } else if node.child_count() == 0 {
        outs.push(Output::Text(source[node.byte_range()].to_owned()))
    } else {
        let children: Vec<_> = node.children(iter).collect();
        for child in children {
            collect_outputs(outs, rules, child, iter, source);
        }
    }

    if let Some(output) = rule.after {
        outs.push(output);
    } else if let Some(next) = node.next_sibling() {
        let whitespace = source[node.end_byte()..next.start_byte()].to_owned();
        if whitespace.len() != 0 {
            outs.push(Output::ExistingWhitespace(whitespace))
        }
    } else if let Some(parent) = node.parent() {
        let whitespace = source[node.end_byte()..parent.end_byte()].to_owned();
        if whitespace.len() != 0 {
            outs.push(Output::ExistingWhitespace(whitespace))
        }
    }
}

struct CapIndex {
    align: Option<CaptureIndex>,
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
    pub fn new(_config: config::FormatConfig) -> Self {
        let query = Query::new(&tree_sitter_ink::language(), QUERY).expect("query should be valid");
        let captures = CapIndex {
            align: query.capture_index_for_name("align"),
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

    pub fn scan(&mut self, tree: &tree_sitter::Tree, source: &str) -> FormattableOutput {
        let mut rules: HashMap<NodeId, Rule> = HashMap::new();

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
                let mut rule = Rule::default();
                let cap_index = Some(cap.index);
                if cap_index == self.captures.align {
                    rule.align = Some(Align {
                        pattern: match_.pattern_index,
                        pos: cap.node.start_position(),
                    });
                } else if cap_index == self.captures.no_space_before {
                    rule.before = Some(Output::Antispace);
                } else if cap_index == self.captures.no_space_after {
                    rule.after = Some(Output::Antispace);
                } else if cap_index == self.captures.space_before {
                    rule.before = Some(Output::Space);
                } else if cap_index == self.captures.space_after {
                    rule.after = Some(Output::Space);
                } else if cap_index == self.captures.newline_before {
                    rule.before = Some(Output::Newline);
                } else if cap_index == self.captures.newline_after {
                    rule.after = Some(Output::Newline);
                } else if cap_index == self.captures.blank_line_before {
                    rule.before = Some(Output::BlankLine);
                } else if cap_index == self.captures.blank_line_after {
                    rule.after = Some(Output::BlankLine);
                } else if cap_index == self.captures.delete {
                    // eprintln!(
                    //     "Delete requested for {:?} (id {}), overrides all other rules",
                    //     cap,
                    //     cap.node.id()
                    // );
                    rule.replace = Some(Output::Nothing);
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
                    rule.replace = Some(Output::Text(action.clone().into_string()))
                }

                if let Some(action) =
                    node_actions.get(&(match_.pattern_index, cap.index, "prepend"))
                {
                    rule.before = Some(Output::Text(action.clone().into_string()))
                }
                if let Some(action) = node_actions.get(&(match_.pattern_index, cap.index, "append"))
                {
                    rule.after = Some(Output::Text(action.clone().into_string()))
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

        FormattableOutput::new(tree, source, rules)
    }
}
