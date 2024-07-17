use std::{collections::HashMap, usize};

use tree_sitter::{Node, Query};
use tree_sitter_ink::NODE_TYPES;

use crate::{config, FormatToken};

use config::FormatConfig;

type CaptureIndex = u32;

pub trait Rule {
    fn new(query: &Query, config: &FormatConfig) -> Option<Self>
    where
        Self: Sized;
    fn captures(&self) -> Vec<CaptureIndex>;

    fn apply<'s, 'src: 'tok, 'cap, 'tok>(
        &'s mut self,
        capture: &'cap CaptureIndex,
        node: &'cap Node,
        source: &'src str,
    ) -> Vec<FormatToken<&'tok str>>;
}

pub struct Rules(HashMap<CaptureIndex, usize>, Vec<Box<dyn Rule>>);
impl Rules {
    pub fn apply<'s, 'src: 't, 'n, 't>(
        &'s mut self,
        capture: &CaptureIndex,
        node: &'n Node,
        source: &'src str,
    ) -> Vec<FormatToken<&'t str>> {
        if let Some(rule_idx) = self.0.get(capture) {
            let rule = self
                .1
                .get_mut(*rule_idx)
                .expect("should only contain valid rule indices");
            rule.apply(capture, node, source)
        } else {
            Vec::new()
        }
    }
}

/// Wrapping a rule in a box is a bit ugly, so we macro it away.
/// Seems to me that this should be easier,
macro_rules! rules_to_vec {
    ($query:ident, $config:ident => $($rule:ident),+) => {
        vec![$($rule::new(&$query, &$config).map(|rule| Box::new(rule) as Box<dyn crate::rules::Rule>)),+]
        .into_iter()
        .filter_map(|maybe_rule| maybe_rule)
        .collect::<Vec<_>>()
    };
}

pub(super) fn init_rules(config: config::FormatConfig, query: &Query) -> Rules {
    let rules = rules_to_vec![query, config => AsIs];
    let mut rulemap: HashMap<CaptureIndex, usize> = HashMap::new();
    for (rule_idx, rule) in rules.iter().enumerate() {
        for capture_idx in rule.captures() {
            if let Some(_duplicate) = rulemap.insert(capture_idx, rule_idx) {
                panic!("Bug! Duplicate rule for capture name '{capture_idx}'");
            }
        }
    }
    Rules(rulemap, rules)
}

pub(super) struct AsIs {
    as_is: CaptureIndex,
}
impl Rule for AsIs {
    fn new(query: &Query, _config: &FormatConfig) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            as_is: query.capture_index_for_name("as-is")?,
        })
    }

    fn captures(&self) -> Vec<CaptureIndex> {
        vec![self.as_is]
    }

    fn apply<'s, 'src: 'tok, 'cap, 'tok>(
        &'s mut self,
        _capture: &'cap CaptureIndex,
        node: &'cap Node,
        source: &'src str,
    ) -> Vec<FormatToken<&'tok str>> {
        vec![FormatToken::Node(
            &source[node.start_byte()..node.end_byte()],
        )]
    }
}

#[derive(Default)]
pub(super) struct Newline {
    before: CaptureIndex,
    after: CaptureIndex,
}
impl Rule for Newline {
    fn new(query: &Query, _config: &FormatConfig) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            before: query.capture_index_for_name("newline.before")?,
            after: query.capture_index_for_name("newline.after")?,
        })
    }

    fn captures(&self) -> Vec<CaptureIndex> {
        vec![self.before, self.after]
    }

    fn apply<'s, 'src: 'tok, 'cap, 'tok>(
        &'s mut self,
        capture: &'cap CaptureIndex,
        node: &'cap Node,
        source: &'src str,
    ) -> Vec<FormatToken<&'tok str>> {
        vec![]
    }
}
