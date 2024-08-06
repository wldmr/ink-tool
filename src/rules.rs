use std::{collections::HashMap, fmt::Debug};

use tree_sitter::{Node, Query, QueryCursor, QueryPredicateArg};

use crate::{config, edit::Change};

type CaptureIndex = u32;
type PatternIndex = usize;
type NodeId = usize;

static QUERY: &str = include_str!("format.scm");

pub struct Rules {
    query: Query,
    cursor: QueryCursor,
    captures: CapIndex,
}

#[derive(Default, Debug)]
struct Rule {
    /// All Nodes with the same index get aliged to the same column
    align: Option<PatternIndex>,
    before: Option<Box<str>>,
    after: Option<Box<str>>,
    replace: Option<Box<str>>,
}

impl Rule {
    fn apply<'t, 's>(&self, node: &Node<'t>, source: &'s str) -> Vec<Change> {
        Vec::new()
    }

    fn merge(&mut self, other: Rule) -> Result<(), String> {
        update(&mut self.align, other.align)?;
        update(&mut self.before, other.before)?;
        update(&mut self.after, other.after)?;
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.align.is_none()
            && self.before.is_none()
            && self.after.is_none()
            && self.replace.is_none()
    }
}

fn update<T: Debug + PartialEq>(old: &mut Option<T>, new: Option<T>) -> Result<(), String> {
    if new.is_none() {
        return Ok(());
    } else if old.as_ref().is_none() {
        *old = new;
    } else if old
        .as_ref()
        .is_some_and(|this| new.is_none() || new.as_ref().is_some_and(|that| this != that))
    {
        return Err(format!("Disagreement: Old {:?}, New {:?}", old, new).to_owned());
    }
    Ok(())
}

struct CapIndex {
    align: Option<CaptureIndex>,
}

fn eq(idx: CaptureIndex, other: Option<CaptureIndex>) -> bool {
    other.is_some_and(|it| it == idx)
}

impl Rules {
    pub fn new(_config: config::FormatConfig) -> Self {
        let query = Query::new(&tree_sitter_ink::language(), QUERY).expect("query should be valid");
        let captures = CapIndex {
            align: query.capture_index_for_name("align"),
        };
        Self {
            query,
            cursor: QueryCursor::new(),
            captures,
        }
    }

    pub fn next_edits(&mut self, tree: &tree_sitter::Tree, source: &str) -> Vec<Change> {
        let rules = self.rules(tree, source);
        dbg!(&rules);
        let mut edits = Vec::new();
        edits
    }

    fn rules<'cur, 'tree>(
        &mut self,
        tree: &tree_sitter::Tree,
        source: &'tree str,
    ) -> HashMap<NodeId, Rule> {
        let mut rules: HashMap<NodeId, Rule> = HashMap::new();

        let mut node_actions: HashMap<(PatternIndex, CaptureIndex, &str), Box<str>> =
            HashMap::new();

        // This could happen on init, speeds things up when these rules get re-used.
        for pattern_index in 1..self.query.pattern_count() {
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

        dbg!(&node_actions);

        for match_ in self
            .cursor
            .matches(&self.query, tree.root_node(), source.as_bytes())
        {
            dbg!(&match_);
            for cap in match_.captures {
                let mut rule = Rule::default();
                if eq(cap.index, self.captures.align) {
                    rule.align = Some(match_.pattern_index);
                }
                if let Some(string) = node_actions.get(&(match_.pattern_index, cap.index, "before"))
                {
                    rule.before = Some(string.clone());
                }
                if let Some(string) = node_actions.get(&(match_.pattern_index, cap.index, "after"))
                {
                    rule.after = Some(string.clone());
                }
                if let Some(string) =
                    node_actions.get(&(match_.pattern_index, cap.index, "replace"))
                {
                    rule.replace = Some(string.clone());
                }

                if rule.is_empty() {
                    continue;
                }
                if let Some(existing) = rules.get_mut(&cap.node.id()) {
                    existing.merge(rule).expect("Rules shouldn't contradict");
                } else {
                    rules.insert(cap.node.id(), rule);
                }
            }
        }
        rules
    }
}
