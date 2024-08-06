use std::{collections::HashMap, fmt::Debug};

use tree_sitter::{Node, Query, QueryCursor, QueryPredicateArg};

use crate::{
    config,
    edit::{replace, replace_between, Change},
};

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
        let mut edits = Vec::new();
        let mut iter = TSNodeIterator::new(tree.root_node());
        while let Some(item) = iter.next() {
            if let Some(rule) = rules.get(&item.node.id()) {
                if let Some(ref text) = rule.before {
                    if let Some(prev_sibling) = item.prev_sibling {
                        let var_name = &source[prev_sibling.end_byte()..item.node.start_byte()];
                        if var_name != &**text {
                            edits.push(replace_between(&prev_sibling, &item.node, &text));
                        }
                    }
                }
                if let Some(ref replacement) = rule.replace {
                    if source[item.node.byte_range()] != **replacement {
                        edits.push(replace(&item.node, &replacement));
                    }
                    iter.skip_children();
                }
                if let Some(ref text) = rule.after {
                    if let Some(next_sibling) = item.next_sibling {
                        let var_name = &source[item.node.end_byte()..next_sibling.start_byte()];
                        if var_name != &**text {
                            edits.push(replace_between(&item.node, &next_sibling, &text));
                        }
                    }
                }
            }
        }
        edits.sort_unstable_by_key(|it| it.range.start_byte);
        dbg!(&edits);
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

        for match_ in self
            .cursor
            .matches(&self.query, tree.root_node(), source.as_bytes())
        {
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

struct TSNodeIterator<'a> {
    cursor: tree_sitter::TreeCursor<'a>,
    skip_children: bool,
}

impl<'a> TSNodeIterator<'a> {
    fn new(node: tree_sitter::Node<'a>) -> Self {
        Self {
            cursor: node.walk(),
            skip_children: false,
        }
    }

    /// Skip the children of the current node,
    /// and the current node only. Will continue to the next sibling (or cousin)
    /// and walk its children normally.
    fn skip_children(&mut self) {
        self.skip_children = true;
    }
}

impl<'a> TSNodeIterator<'a> {
    fn backtrack(&mut self) -> bool {
        if !self.cursor.goto_parent() {
            // no more parents -> we're done
            return false;
        } else if self.cursor.goto_next_sibling() {
            return true;
        } else {
            self.backtrack()
        }
    }
}

#[derive(Debug)]
struct NodeItem<'t> {
    node: Node<'t>,
    prev_sibling: Option<Node<'t>>,
    next_sibling: Option<Node<'t>>,
}

impl<'a> Iterator for TSNodeIterator<'a> {
    type Item = NodeItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if (!self.skip_children && self.cursor.goto_first_child())
            || self.cursor.goto_next_sibling()
            || self.backtrack()
        {
            self.skip_children = false;
            let node = self.cursor.node();
            Some(NodeItem {
                node,
                // these might be slow and we should perhaps determine them during the iteration (peek & cache)
                prev_sibling: node.prev_sibling(),
                next_sibling: node.next_sibling(),
            })
        } else {
            None
        }
    }
}
