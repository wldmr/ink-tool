use std::fmt::{Debug, Write};

use std::collections::HashMap;

use super::format_item::FormatItem;
use super::NodeId;

pub(crate) type NodeRules = HashMap<NodeId, NodeRule>;

#[derive(Default)]
pub(crate) enum IndentType {
    #[default]
    None,
    Indent,
    Anchor,
}

#[derive(Default)]
pub(crate) struct NodeRule {
    pub(crate) indent: IndentType,
    pub(crate) dedent: bool,
    pub(crate) is_leaf: bool,
    pub(crate) before: Option<FormatItem>,
    pub(crate) after: Option<FormatItem>,
    pub(crate) replace: Option<FormatItem>,
}

impl Debug for NodeRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Rule")?;
        match self.indent {
            IndentType::Indent => f.write_char('→')?,
            IndentType::Anchor => f.write_str("|>")?,
            IndentType::None => (),
        }
        if self.dedent {
            f.write_str("←")?
        }
        if self.is_leaf {
            f.write_str(".")?
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
        }
        use IndentType::*;
        self.indent = match (&self.indent, &other.indent) {
            (None, None) => None,
            (None, Indent) => Indent,
            (None, Anchor) => Anchor,
            (Indent, None) => Indent,
            (Indent, Indent) => Indent,
            (Indent, Anchor) => Anchor,
            (Anchor, None) => Anchor,
            (Anchor, Indent) => Anchor,
            (Anchor, Anchor) => Anchor,
        };
        self.dedent |= other.dedent;
        self.is_leaf |= other.is_leaf;
        Self::update_option(&mut self.before, other.before);
        Self::update_option(&mut self.after, other.after);
        Self::update_option(&mut self.replace, other.replace);
    }

    pub(crate) fn update_option<T: Debug + PartialEq>(old: &mut Option<T>, new: Option<T>) {
        if new.is_none() {
            return;
        } else if old.as_ref().is_some() && *old != new {
            eprintln!("Warning: Updating Old {:?}, New {:?}", old, new);
        }
        *old = new;
    }
}
