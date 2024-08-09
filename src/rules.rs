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

pub struct Rules {
    query: Query,
    cursor: QueryCursor,
    captures: CapIndex,
}

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

impl ToString for Output {
    fn to_string(&self) -> String {
        match self {
            Output::Nothing => "",
            Output::Antispace => "",
            Output::Space => " ",
            Output::Newline => "\n",
            Output::BlankLine => "\n\n",
            Output::ExistingWhitespace(ws) => ws,
            Output::Text(txt) => txt,
        }
        .to_owned()
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
            update(&mut self.align, other.align);
            update(&mut self.before, other.before);
            update(&mut self.after, other.after);
            update(&mut self.replace, other.replace);
        }
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

impl Rules {
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

    pub fn output(&mut self, tree: &tree_sitter::Tree, source: &str) -> String {
        let mut rules = self.rules(tree, source);
        // dbg!(&rules);
        let mut iter = tree.walk();
        let mut out: Vec<Output> = Vec::new();
        self.outputs(&mut out, &mut rules, iter.node(), &mut iter, source);

        // dbg!(&out);

        // This merging can probably be done directly while building the edits.
        let mut merged = Vec::new();
        let mut iter = out.into_iter();
        // .skip_while(|it| !matches!(it, Output::Text(_)));

        if let Some(mut accumulator) = iter.next() {
            while let Some(edit) = iter.next() {
                match accumulator.merge(edit) {
                    Ok(merged) => accumulator = merged,
                    Err((left, right)) => {
                        merged.push(left);
                        accumulator = right;
                    }
                }
            }
            merged.push(accumulator);
        }
        // dbg!(&merged);
        merged
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

        // dbg!(&node.id(), &node, &rule);

        if let Some(output) = rule.before {
            outs.push(output);
        } else if let Some(prev) = node.prev_sibling() {
            let ws = source[prev.end_byte()..node.start_byte()].to_owned();
            if ws.len() != 0 {
                outs.push(Output::ExistingWhitespace(ws))
            }
        } else if let Some(parent) = node.parent() {
            let ws = source[parent.start_byte()..node.start_byte()].to_owned();
            if ws.len() != 0 {
                outs.push(Output::ExistingWhitespace(ws))
            }
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
            if ws.len() != 0 {
                outs.push(Output::ExistingWhitespace(ws))
            }
        } else if let Some(parent) = node.parent() {
            let ws = source[node.end_byte()..parent.end_byte()].to_owned();
            if ws.len() != 0 {
                outs.push(Output::ExistingWhitespace(ws))
            }
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
        rules
    }
}
