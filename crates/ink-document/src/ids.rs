// We use the `name(type)` syntax instead of `name: type` because that’s what
// rustfmt can actually format (it gives up on anything more complicated).
// Otherwise we’d have to format it ourselves which is tedious.

macro_rules! define_id_tuples {
    ($(
        $name:ident
        (
            $($ty:ty),*
            $(,)?
        )
    ),+$(,)?) => {$(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name($(pub(super) $ty),*);
    )+};

}

define_id_tuples![Definition(NodeId, DefinitionInfo), Usage(NodeId, UsageInfo),];

impl Definition {
    pub fn new<'a, N: type_sitter::Node<'a>>(node: N, info: DefinitionInfo) -> Self {
        Self(NodeId::new(node), info)
    }

    pub fn node(&self) -> NodeId {
        self.0
    }

    pub fn info(&self) -> DefinitionInfo {
        self.1
    }
}

impl Usage {
    pub fn new<'a, N: type_sitter::Node<'a>>(node: N, info: UsageInfo) -> Self {
        Self(NodeId::new(node), info)
    }

    pub fn node(&self) -> NodeId {
        self.0
    }

    pub fn info(&self) -> UsageInfo {
        self.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

impl NodeId {
    pub(crate) fn new<'a, N: type_sitter::Node<'a>>(node: N) -> Self {
        Self(node.raw().id())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefinitionInfo {
    /// Usually a Knot, but stitches can be topelevel as well.
    ToplevelScope {
        stitch: bool,
        params: bool,
    },
    /// Only ever a stitch, but to be consistent with the "scope" naming.
    SubScope {
        parent: Option<NodeId>,
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
