use std::fmt::Debug;

use std::collections::HashMap;

use super::format_item::FormatItem;
use super::NodeId;

pub(crate) type NodeRules = HashMap<NodeId, NodeRule>;

#[derive(Default)]
pub(crate) struct NodeRule {
    /// All Nodes with the same index get aliged to te same column
    pub(crate) align_self: bool,
    pub(crate) align_children: bool,
    pub(crate) before: Option<FormatItem>,
    pub(crate) after: Option<FormatItem>,
    pub(crate) replace: Option<FormatItem>,
}

impl Debug for NodeRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Rule")?;
        if self.align_self {
            f.write_str("|")?
        }
        if self.align_children {
            f.write_str("||")?
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
            self.align_self = false;
            self.align_children = false;
            self.before = None;
            self.after = None;
        } else {
            self.align_self |= other.align_self;
            self.align_children |= other.align_children;
            Self::update_option(&mut self.before, other.before);
            Self::update_option(&mut self.after, other.after);
            Self::update_option(&mut self.replace, other.replace);
        }
    }

    pub(crate) fn update_bool(old: &mut bool, new: bool) {
        if *old != new {
            eprintln!("Warning: Updating Old {:?}, New {:?}", old, new);
            *old = new;
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
