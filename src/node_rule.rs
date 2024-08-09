use std::fmt::Debug;

use std::collections::HashMap;

use super::format_item::{Align, FormatItem};
use super::NodeId;

pub(crate) type NodeRules = HashMap<NodeId, NodeRule>;

#[derive(Default)]
pub(crate) struct NodeRule {
    /// All Nodes with the same index get aliged to te same column
    pub(crate) align: Option<Align>,
    pub(crate) before: Option<FormatItem>,
    pub(crate) after: Option<FormatItem>,
    pub(crate) replace: Option<FormatItem>,
}

impl Debug for NodeRule {
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

impl NodeRule {
    pub(crate) fn merge(&mut self, other: NodeRule) {
        if self.replace == Some(FormatItem::Nothing) || other.replace == Some(FormatItem::Nothing) {
            self.replace = Some(FormatItem::Nothing);
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

    pub(crate) fn update_option<T: Debug + PartialEq>(old: &mut Option<T>, new: Option<T>) {
        if new.is_none() {
            return;
        } else if old.as_ref().is_some() {
            eprintln!("Warning: Updating Old {:?}, New {:?}", old, new);
        }
        *old = new;
    }
}
