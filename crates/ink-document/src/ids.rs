use derive_more::{
    derive::{AsRef, Into},
    Debug,
};
use type_sitter::{Node, NodeResult};

// We use the `name(type)` syntax instead of `name: type` because that’s what
// rustfmt can actually format (it gives up on anything more complicated).
// Otherwise we’d have to format it ourselves which is tedious.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Into)]
#[debug("NodeId({_0})")]
pub struct NodeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Into, AsRef)]
#[debug("DefId({})", _0.0)]
pub struct DefId(NodeId);

impl DefId {
    // FIXME: Not happy about this. Breaks the whole point if we can just convert between them nilly-willy.
    pub fn pinkie_promise_from_usage_id(id: UsageId) -> Self {
        Self(id.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRef, Into)]
#[debug("UsageId({})", _0.0)]
pub struct UsageId(NodeId);

impl UsageId {
    // FIXME: Not happy about this. Breaks the whole point if we can just convert between them nilly-willy.
    pub fn pinkie_promise_from_node_id(id: NodeId) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Into, AsRef)]
#[debug("ScopeId({})", _0.0)]
pub struct ScopeId(NodeId);

impl<'a> From<ink_syntax::Knot<'a>> for DefId {
    fn from(value: ink_syntax::Knot<'a>) -> Self {
        Self(NodeId::new(value.name()))
    }
}

impl<'a> From<ink_syntax::Stitch<'a>> for DefId {
    fn from(value: ink_syntax::Stitch<'a>) -> Self {
        Self(NodeId::new(value.name()))
    }
}

impl<'a> From<ink_syntax::TempDef<'a>> for DefId {
    fn from(value: ink_syntax::TempDef<'a>) -> Self {
        Self(NodeId::new(value.name()))
    }
}

impl<'a> From<ink_syntax::List<'a>> for DefId {
    fn from(value: ink_syntax::List<'a>) -> Self {
        Self(NodeId::new(value.name()))
    }
}

impl<'a> From<ink_syntax::ListValueDef<'a>> for DefId {
    fn from(value: ink_syntax::ListValueDef<'a>) -> Self {
        Self(NodeId::new(value.name()))
    }
}

impl<'a> From<ink_syntax::External<'a>> for DefId {
    fn from(value: ink_syntax::External<'a>) -> Self {
        Self(NodeId::new(value.name()))
    }
}

impl<'a> From<ink_syntax::Global<'a>> for DefId {
    fn from(value: ink_syntax::Global<'a>) -> Self {
        Self(NodeId::new(value.name()))
    }
}

impl<'a> From<ink_syntax::Label<'a>> for DefId {
    fn from(value: ink_syntax::Label<'a>) -> Self {
        Self(NodeId::new(value.name()))
    }
}

impl<'a> From<ink_syntax::Param<'a>> for DefId {
    fn from(value: ink_syntax::Param<'a>) -> Self {
        let node = match value.value() {
            Ok(ink_syntax::ParamValue::Divert(it)) => it.target().raw().id(),
            Ok(ink_syntax::ParamValue::Identifier(it)) => it.raw().id(),
            Err(_) => value.value().raw().id(),
        };
        Self(NodeId(node))
    }
}

impl<'tree, N: Node<'tree> + Into<DefId>> From<NodeResult<'tree, N>> for DefId {
    fn from(value: NodeResult<'tree, N>) -> Self {
        Self(NodeId::new(value))
    }
}

impl PartialEq<NodeId> for DefId {
    fn eq(&self, other: &NodeId) -> bool {
        self.0 .0 == other.0
    }
}

impl PartialEq<UsageId> for DefId {
    fn eq(&self, other: &UsageId) -> bool {
        self.0 .0 == other.0 .0
    }
}

impl<'a, N: Node<'a>> PartialEq<N> for DefId {
    fn eq(&self, other: &N) -> bool {
        self.0 .0 == other.raw().id()
    }
}

impl PartialEq<DefId> for UsageId {
    fn eq(&self, other: &DefId) -> bool {
        self.0 .0 == other.0 .0
    }
}

impl From<DefId> for UsageId {
    fn from(value: DefId) -> Self {
        UsageId(value.0)
    }
}

impl<'a, N: Node<'a>> PartialEq<N> for UsageId {
    fn eq(&self, other: &N) -> bool {
        self.0 .0 == other.raw().id()
    }
}

impl<'a> From<ink_syntax::Identifier<'a>> for UsageId {
    fn from(value: ink_syntax::Identifier<'a>) -> Self {
        Self(NodeId::new(value))
    }
}

impl<'a> From<ink_syntax::QualifiedName<'a>> for UsageId {
    fn from(value: ink_syntax::QualifiedName<'a>) -> Self {
        Self(NodeId::new(value))
    }
}

impl<'a, N: Node<'a> + Into<UsageId>> From<NodeResult<'a, N>> for UsageId {
    fn from(value: NodeResult<'a, N>) -> Self {
        Self(NodeId::new(value))
    }
}

impl<'a> From<ink_syntax::Usages<'a>> for UsageId {
    fn from(value: ink_syntax::Usages<'a>) -> Self {
        match value {
            ink_syntax::Usages::Identifier(identifier) => identifier.into(),
            ink_syntax::Usages::QualifiedName(qualified_name) => qualified_name.into(),
        }
    }
}

impl<'a> From<ink_syntax::KnotBlock<'a>> for ScopeId {
    fn from(value: ink_syntax::KnotBlock<'a>) -> Self {
        Self(NodeId::new(value))
    }
}

impl<'a> From<ink_syntax::StitchBlock<'a>> for ScopeId {
    fn from(value: ink_syntax::StitchBlock<'a>) -> Self {
        Self(NodeId::new(value))
    }
}

impl<'a> From<ink_syntax::Ink<'a>> for ScopeId {
    fn from(value: ink_syntax::Ink<'a>) -> Self {
        Self(NodeId::new(value))
    }
}

impl<'a> From<ink_syntax::ScopeBlock<'a>> for ScopeId {
    fn from(value: ink_syntax::ScopeBlock<'a>) -> Self {
        Self(NodeId::new(value))
    }
}

impl<'tree, N: Node<'tree> + Into<ScopeId>> From<NodeResult<'tree, N>> for ScopeId {
    fn from(value: NodeResult<'tree, N>) -> Self {
        Self(NodeId::new(value))
    }
}

impl<'a, N: Node<'a>> PartialEq<N> for ScopeId {
    fn eq(&self, other: &N) -> bool {
        self.0 .0 == other.raw().id()
    }
}

impl NodeId {
    pub fn new<'a, N: type_sitter::Node<'a>>(node: N) -> Self {
        Self(node.raw().id())
    }
}

impl<'a, T: Node<'a>> From<T> for NodeId {
    fn from(value: T) -> Self {
        NodeId::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefinitionInfo {
    /// Usually a Knot, but stitches can be topelevel as well.
    Section {
        stitch: bool,
        params: bool,
    },
    /// Only ever a stitch, but to be consistent with the "scope" naming.
    Subsection {
        parent: NodeId,
        params: bool,
    },
    Function,
    External,
    Var,
    Const,
    List,
    ListItem {
        list: NodeId,
    },
    Temp,
    Param {
        is_ref: bool,
        is_divert: bool,
    },
    Label,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RedirectKind {
    Divert,
    Tunnel,
    NamedTunnelReturn,
    Thread,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsageInfo {
    pub redirect: Option<RedirectKind>,
    pub params: bool,
}
