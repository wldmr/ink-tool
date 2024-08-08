use std::{collections::HashMap, fmt::Debug};

use tree_sitter::{Node, Point, Query, QueryCursor, QueryPredicateArg, TreeCursor};

use crate::config;

type CaptureIndex = u32;
type PatternIndex = usize;
type NodeId = usize;

static QUERY: &str = include_str!("format.scm");

pub struct Rules {
    query: Query,
    cursor: QueryCursor,
    captures: CapIndex,
}

#[derive(Debug, PartialEq)]
enum Output {
    Nothing,
    Space,
    Newline,
    BlankLine,
    ExistingWhitespace(String),
    Text(String),
}

impl Output {
    pub fn maybe_merge(&mut self, other: Output) -> Option<Output> {
        match (&self, other) {
            (Self::Nothing, other @ _) => {
                *self = other;
                None
            }
            (_, Self::Nothing) => None,

            (Self::Space, Self::Space) => {
                *self = Self::Space;
                None
            }
            (Self::Space, Self::Newline)
            | (Self::Newline, Self::Space)
            | (Self::Newline, Self::Newline) => {
                *self = Self::Newline;
                None
            }
            (Self::Space, Self::BlankLine)
            | (Self::Newline, Self::BlankLine)
            | (Self::BlankLine, Self::Space)
            | (Self::BlankLine, Self::Newline)
            | (Self::BlankLine, Self::BlankLine) => {
                *self = Self::BlankLine;
                None
            }

            (Self::ExistingWhitespace(a), Self::ExistingWhitespace(b)) => {
                if *a == b {
                    // Two adjacent nodes reported the same whitespace as after and before respectively,
                    // so we can absorb one;
                    None
                } else {
                    // Not really sure how that could happen. Would this be a bug?
                    Some(Self::ExistingWhitespace(b))
                }
            }

            (Self::ExistingWhitespace(_), other @ Self::Text(_)) => Some(other),
            (Self::Text(_), other @ Self::ExistingWhitespace(_)) => Some(other),

            // all other rules clober existing whitespace
            (Self::ExistingWhitespace(_), other @ _) => Some(other),
            (_, Self::ExistingWhitespace(_)) => None,

            (_, other @ Self::Text(_)) | (Self::Text(_), other @ _) => Some(other),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Align {
    pattern: PatternIndex,
    pos: Point,
}

impl ToString for Output {
    fn to_string(&self) -> String {
        match self {
            Output::Nothing => "",
            Output::Space => " ",
            Output::Newline => "\n",
            Output::BlankLine => "\n\n",
            Output::ExistingWhitespace(ws) => ws,
            Output::Text(txt) => txt,
        }
        .to_owned()
    }
}

#[derive(Default, Debug)]
struct Rule {
    /// All Nodes with the same index get aliged to the same column
    align: Option<Align>,
    before: Option<Output>,
    after: Option<Output>,
    replace: Option<Output>,
}

impl Rule {
    fn merge(&mut self, other: Rule) {
        update(&mut self.align, other.align);
        update(&mut self.before, other.before);
        update(&mut self.after, other.after);
    }

    fn is_empty(&self) -> bool {
        self.align.is_none()
            && self.before.is_none()
            && self.after.is_none()
            && self.replace.is_none()
    }
}

fn update<T: Debug + PartialEq>(old: &mut Option<T>, new: Option<T>) {
    if new.is_none() {
        return;
    } else if old.as_ref().is_some() {
        eprintln!("Warning: Updating Old {:?}, New {:?}", old, new);
    }
    *old = new;
}

struct CapIndex {
    align: Option<CaptureIndex>,
    nothing_before: Option<CaptureIndex>,
    nothing_after: Option<CaptureIndex>,
    newline_before: Option<CaptureIndex>,
    newline_after: Option<CaptureIndex>,
    space_before: Option<CaptureIndex>,
    space_after: Option<CaptureIndex>,
    blank_line_before: Option<CaptureIndex>,
    blank_line_after: Option<CaptureIndex>,
    delete: Option<CaptureIndex>,
}

impl Rules {
    pub fn new(_config: config::FormatConfig) -> Self {
        let query = Query::new(&tree_sitter_ink::language(), QUERY).expect("query should be valid");
        let captures = CapIndex {
            align: query.capture_index_for_name("align"),
            nothing_before: query.capture_index_for_name("nothing.before"),
            nothing_after: query.capture_index_for_name("nothing.before"),
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

    pub fn output(&mut self, tree: &tree_sitter::Tree, source: &str) -> String {
        let mut rules = self.rules(tree, source);
        // dbg!(&rules);
        let mut iter = tree.walk();
        let mut edits: Vec<Output> = Vec::new();
        self.outputs(&mut edits, &mut rules, iter.node(), &mut iter, source);

        // This merging can probably be done directly while building the edits.
        let mut merged_edits = Vec::new();
        let mut iter = edits.into_iter();

        if let Some(mut accumulator) = iter.next() {
            while let Some(edit) = iter.next() {
                if let Some(unmerged) = accumulator.maybe_merge(edit) {
                    merged_edits.push(accumulator);
                    accumulator = unmerged;
                }
            }
            merged_edits.push(accumulator);
        }
        merged_edits
            .into_iter()
            .map(|it| it.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn outputs<'t>(
        &mut self,
        outs: &mut Vec<Output>,
        rules: &mut HashMap<NodeId, Rule>,
        node: Node<'t>,
        iter: &mut TreeCursor<'t>,
        source: &str,
    ) {
        let rule = rules.remove(&node.id()).unwrap_or_default();
        if let Some(output) = rule.before {
            outs.push(output);
        } else if let Some(prev) = node.prev_sibling() {
            let ws = source[prev.end_byte()..node.start_byte()].to_owned();
            outs.push(Output::ExistingWhitespace(ws))
        } else if let Some(parent) = node.parent() {
            let ws = source[parent.start_byte()..node.start_byte()].to_owned();
            outs.push(Output::ExistingWhitespace(ws))
        }
        if let Some(output) = rule.replace {
            outs.push(output);
        } else if node.child_count() == 0 {
            outs.push(Output::Text(source[node.byte_range()].to_owned()))
        } else {
            let collect: Vec<_> = node.children(iter).collect();
            for child in collect {
                self.outputs(outs, rules, child, iter, source);
            }
        }
        if let Some(output) = rule.after {
            outs.push(output);
        } else if let Some(next) = node.next_sibling() {
            let ws = source[node.end_byte()..next.start_byte()].to_owned();
            outs.push(Output::ExistingWhitespace(ws))
        } else if let Some(parent) = node.parent() {
            let ws = source[node.end_byte()..parent.end_byte()].to_owned();
            outs.push(Output::ExistingWhitespace(ws))
        }
    }

    fn rules<'cur, 'tree>(
        &mut self,
        tree: &tree_sitter::Tree,
        source: &'tree str,
    ) -> HashMap<NodeId, Rule> {
        let mut rules: HashMap<NodeId, Rule> = HashMap::new();

        let mut node_actions: HashMap<(PatternIndex, CaptureIndex, &str), Box<str>> =
            HashMap::new();

        // let capturenames: Vec<_> = self.query.capture_names().iter().enumerate().collect();
        // dbg!(capturenames);

        // This could happen on init, speeds things up when these rules get re-used.
        for pattern_index in 1..self.query.pattern_count() {
            for prop in self.query.property_settings(pattern_index) {
                eprintln!("Found the query property {:?}", prop);
            }
            for prop in self.query.general_predicates(pattern_index) {
                let op = &*prop.operator;
                let args = &*prop.args;
                match (op, args) {
                    (op, [QueryPredicateArg::Capture(index), QueryPredicateArg::String(value)])
                        if op == "before" || op == "after" || op == "replace" =>
                    {
                        let key = (pattern_index, *index, op);
                        if let Some(other) = node_actions.insert(key, value.clone()) {
                            panic!(
                            "Pattern {pattern_index}: Duplicate node action for @{op}: Previous '{other}', replaced by '{value}'",
                        )
                        }
                    }
                    (op, args) => {
                        panic!(
                            "Pattern {pattern_index}: Unknown query predicate @{op}({:?})",
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
                } else if cap_index == self.captures.nothing_before {
                    rule.before = Some(Output::Nothing);
                } else if cap_index == self.captures.nothing_after {
                    rule.after = Some(Output::Nothing);
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
                    rule.replace = Some(Output::Nothing);
                }

                if let Some(existing) = rules.get_mut(&cap.node.id()) {
                    existing.merge(rule);
                } else {
                    rules.insert(cap.node.id(), rule);
                }
            }
        }
        rules
    }
}
