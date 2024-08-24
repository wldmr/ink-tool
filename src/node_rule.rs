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
    pub(crate) take_asis: bool,
    pub(crate) before: Vec<FormatItem>,
    pub(crate) after: Vec<FormatItem>,
    pub(crate) replace: Option<FormatItem>,
}

impl Debug for NodeRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.take_asis {
            f.write_str("Leaf")?
        }
        f.write_str("Rule")?;
        match self.indent {
            IndentType::Indent => f.write_char('→')?,
            IndentType::Anchor => f.write_str("|>")?,
            IndentType::None => (),
        }
        if self.dedent {
            f.write_str("←")?
        }
        if !self.before.is_empty() {
            f.write_fmt(format_args!("←{:?}", self.before))?
        }
        if let Some(ref it) = self.replace {
            f.write_fmt(format_args!("•{:?}", it))?
        }
        if !self.after.is_empty() {
            f.write_fmt(format_args!("→{:?}", self.after))?
        }
        Ok(())
    }
}
