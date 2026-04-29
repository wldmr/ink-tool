use crate::lsp::{
    location::TextRange,
    salsa::{node_infos, InkGetters as _, Name, Ops},
};
use bimap::BiHashMap;
use enumflags2::{bitflags, BitFlags};
use ink_document::{
    ids::{DefId, NodeId, ScopeId, UsageId},
    InkDocument,
};
use mini_milc::{Db, Old, Subquery, Updated};
use std::{
    collections::HashMap,
    hint::unreachable_unchecked,
    path::{Path, PathBuf},
};
use tree_traversal::Visitor;
use type_sitter::Node;
use util::nonempty::{MapOfNonEmpty, Vec1};

impl Subquery<Ops, NodeInfos> for node_infos {
    fn value(&self, db: &impl Db<Ops>, old: Old<NodeInfos>) -> Updated<NodeInfos> {
        let doc = db.document(self.docid);
        let new = Vstr::new(&doc).traverse(doc.root());
        old.update(new)
    }
}

// Public interface
impl NodeInfos {
    /// NOTE: `range` must be the actual definition site.
    pub fn local_usages(&self, range: DefId) -> Option<&Vec1<UsageId>> {
        self.def_to_usage.get(&range)
    }
    pub fn local_definitions(&self, range: UsageId) -> Option<&Vec1<DefId>> {
        self.usage_to_def.get(&range)
    }

    pub fn global_names(&self, range: DefId) -> Option<&Vec1<Name>> {
        self.global_names_by_range.get(&range)
    }
    pub fn global_ranges(&self, text: impl Into<Name>) -> Option<&Vec1<DefId>> {
        self.global_ranges_by_name.get(&text.into())
    }
    pub fn iter_globals(&self) -> impl Iterator<Item = (&Name, DefId)> {
        self.global_ranges_by_name
            .iter()
            .flat_map(|(s, ranges)| ranges.iter().map(move |r| (s, *r)))
    }

    pub fn unresolved_names(&self, range: UsageId) -> Option<&Vec1<Name>> {
        self.unresolved_name_by_range.get(&range)
    }
    pub fn unresolved_ranges(&self, text: impl Into<Name>) -> Option<&Vec1<UsageId>> {
        self.unresolved_range_by_name.get(&text.into())
    }

    pub fn parent_scope(&self, range: NodeId) -> Option<ScopeId> {
        self.parent_scope.get(&range).copied()
    }

    pub fn addresses_in_scope<T: Into<ScopeId>>(
        &self,
        range: T,
    ) -> impl Iterator<Item = (&Name, DefId)> + use<'_, T> {
        let iter: Box<dyn Iterator<Item = (&Name, DefId)>> =
            match self.addresses_in_scope.get(&range.into()) {
                Some(vec1) => Box::new(
                    vec1.iter()
                        .flat_map(|(s, ranges)| ranges.iter().map(move |r| (s, *r))),
                ),
                None => Box::new(std::iter::empty()),
            };
        iter
    }

    pub fn locals_in_scope<T: Into<ScopeId>>(
        &self,
        range: T,
    ) -> impl Iterator<Item = (&Name, DefId)> + use<'_, T> {
        let iter: Box<dyn Iterator<Item = (&Name, DefId)>> =
            match self.locals_in_scope.get(&range.into()) {
                Some(vec1) => Box::new(
                    vec1.iter()
                        .flat_map(|(s, ranges)| ranges.iter().map(move |r| (s, *r))),
                ),
                None => Box::new(std::iter::empty()),
            };
        iter
    }

    pub fn flags(&self, id: impl Into<NodeId>) -> BitFlags<NodeFlag> {
        self.flags.get(&id.into()).copied().unwrap_or_default()
    }

    pub fn flags_by_range(&self, id: impl Into<lsp_types::Range>) -> BitFlags<NodeFlag> {
        let id = self.locs.get_by_right(&id.into()).copied().unwrap();
        self.flags(id)
    }

    pub fn iter_flags(&self) -> impl Iterator<Item = (UsageId, BitFlags<NodeFlag>)> + use<'_> {
        self.flags
            .iter()
            .map(|(a, b)| (UsageId::pinkie_promise_from_node_id(*a), *b))
    }

    pub fn iter_definitions(&self) -> impl Iterator<Item = (DefId, BitFlags<NodeFlag>)> + use<'_> {
        self.iter_flags()
            .filter(|(_, flags)| flags.contains(NodeFlag::Definition))
            .map(|it| (DefId::pinkie_promise_from_usage_id(it.0), it.1))
    }

    pub fn imported_files(&self) -> impl ExactSizeIterator<Item = (&Path, TextRange)> + use<'_> {
        self.imported_files.iter().map(|it| (it.0.as_path(), it.1))
    }
}

/// Poor man’s match statement for bitflags. I know a macro may a bit silly, but the
/// if-else-if is quite noisy and obscures the mapping.
///
/// Has two versions, depending on whether the last branch is a catch-all branch
/// (`_ => [expr]`):
///
/// 1.  If there is no catch-all branch, then return an `Option<…>`,
/// 2.  if there is one, then return the plain type of the expressions.
macro_rules! match_flags {
    ( match ($flags:expr) { $($a:expr => $b:expr$(,)?)+ } ) => {{
        let it = $flags;
        $( if it.contains($a) { Some($b) } else )* { None }
    }};

    ( match ($flags:expr) { $($a:expr => $b:expr$(,)?)+, _ => $fallback:expr $(,)? } ) => {{
        let it = $flags;
        $( if it.contains($a) { $b } else )* { $fallback }
    }};
}
pub(crate) use match_flags;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct NodeInfos {
    /// Which scope node the node resides in
    parent_scope: HashMap<NodeId, ScopeId>,
    addresses_in_scope: HashMap<ScopeId, Vec1<(Name, Vec1<DefId>)>>,
    locals_in_scope: HashMap<ScopeId, Vec1<(Name, Vec1<DefId>)>>,

    /// The global names defined in this file.
    global_names_by_range: HashMap<DefId, Vec1<Name>>,
    global_ranges_by_name: HashMap<Name, Vec1<DefId>>,

    usage_to_def: HashMap<UsageId, Vec1<DefId>>,
    def_to_usage: HashMap<DefId, Vec1<UsageId>>,

    /// Which names could not be resolved and the corresponding *identifier* ranges.
    ///
    /// So if the following couldn’t be resolved because `Stitch` does not contain `Label`:
    ///
    /// ``` ink
    /// -> Stitch.Label
    /// // ^^^^^^^^^^^^ this is the key (String)
    /// //        ^^^^^ this is the value (the TextRange of the `Label` identifier)
    /// ```
    unresolved_range_by_name: HashMap<Name, Vec1<UsageId>>,
    unresolved_name_by_range: HashMap<UsageId, Vec1<Name>>,

    imported_files: Vec<(PathBuf, TextRange)>,

    pub locs: BiHashMap<NodeId, lsp_types::Range>,
    flags: HashMap<NodeId, BitFlags<NodeFlag>>,
}

// Private helpers
impl NodeInfos {
    fn add_global<T: Into<Name>>(&mut self, text: T, def: DefId) {
        let name = text.into();
        self.global_names_by_range.register(def, name);
        self.global_ranges_by_name.register(name, def);
    }

    fn add_unresolved(&mut self, usage: UsageId, name: impl Into<Name>) {
        let name = name.into();
        self.unresolved_range_by_name.register(name, usage);
        self.unresolved_name_by_range.register(usage, name);
    }

    fn resolve_from<'a>(
        &mut self,
        defs: &mut Definitions,
        usage: UsageId,
        text: impl Into<Name>,
    ) -> bool {
        if let Some(def) = defs.get(&text.into()).cloned() {
            for def in def {
                self.usage_to_def.register(usage, def);
                self.def_to_usage.register(def, usage);
            }
            true
        } else {
            false
        }
    }

    fn add_node_kind(&mut self, id: NodeId, kind: impl Into<BitFlags<NodeFlag>>) {
        self.flags.entry(id).or_default().insert(kind);
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeFlag {
    /// Blocks
    Block,
    Global,
    Local,
    Definition,
    /// Ink Syntax Items
    List,
    ListItem,
    Var,
    Const,
    Temp,
    Label,
    Knot,
    Function,
    External,
    Stitch,
    Param,
    /// Usages
    Usage,
    Redirect,
    Call,
    /// Other information
    HasParams,
    Builtin,
}

#[derive(Debug)]
struct Scope<'a> {
    id: ScopeId,
    name: &'a str,
    addresses: Definitions,
    /// temps don't transfer to subscopes, so we keep track of them separately
    locals: Definitions,
    usages: Usages,
}

impl<'a> Scope<'a> {
    fn new(block: impl Into<ScopeId>) -> Self {
        Self {
            id: block.into(),
            name: "",
            addresses: Default::default(),
            locals: Default::default(),
            usages: Default::default(),
        }
    }

    fn add_address(&mut self, text: impl Into<Name>, def: DefId) {
        self.addresses.register(text, def);
    }

    fn add_local(&mut self, text: impl Into<Name>, def: DefId) {
        self.locals.register(text, def);
    }

    /// This is a "normal" usage that might resolve the a local variable
    fn add_usage(&mut self, usage_id: UsageId, text: &'a str) {
        self.usages.insert(usage_id.into(), (text.into(), true));
    }

    /// This is a usage from "outside" this scope, which means it can't see our temps and params.
    fn add_outside_usage(&mut self, usage_id: UsageId, text: &'a str) {
        self.usages.insert(usage_id.into(), (text.into(), false));
    }
}

/// We do this so we can just use the scope itself for label formatting.
///
/// That is, being able to say `format!("{scope}.{local_name}")` vs
/// `format!("{}.{local_name}", scope.name)` or using a let binding.
impl<'a> std::fmt::Display for Scope<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

type Definitions = HashMap<Name, Vec1<DefId>>;
type Usages = HashMap<UsageId, (Name, bool)>;

#[derive(Debug)]
struct Vstr<'a> {
    doc: &'a InkDocument,
    ink: Scope<'a>,
    knot: Option<Scope<'a>>,
    stitch: Option<Scope<'a>>,
    qname: Option<ink_syntax::QualifiedName<'a>>,
    list: Option<(TextRange, &'a str)>,
    call: bool,
    redirect: bool,
    /// is the current usage a listvalues query (`list_name ? (item.name)`)
    listvalues: bool,
    external: bool,
}

impl<'a> Vstr<'a> {
    fn current_scope(&self) -> &Scope<'a> {
        self.stitch
            .as_ref()
            .or(self.knot.as_ref())
            .unwrap_or(&self.ink)
    }
    fn current_scope_mut(&mut self) -> &mut Scope<'a> {
        self.stitch
            .as_mut()
            .or(self.knot.as_mut())
            .unwrap_or(&mut self.ink)
    }
}

impl<'a> Vstr<'a> {
    fn new(doc: &'a InkDocument) -> Self {
        Self {
            doc,
            ink: Scope::new(doc.root()),
            knot: None,
            stitch: None,
            qname: None,
            list: Default::default(),
            call: false,
            redirect: false,
            listvalues: false,
            external: false,
        }
    }

    fn range(&self, node: impl Node<'a>) -> TextRange {
        self.doc.lsp_range(node.range()).into()
    }
}

impl<'a> Visitor<'a, ink_syntax::AllNamed<'a>> for Vstr<'a> {
    type State = NodeInfos;

    fn visit(
        &mut self,
        node: ink_syntax::AllNamed<'a>,
        state: &mut Self::State,
    ) -> tree_traversal::VisitInstruction<Self::State> {
        use ink_syntax::AllNamed::*;
        use tree_traversal::VisitInstruction::{Descend, Ignore};

        state
            .locs
            .insert(node.into(), self.doc.lsp_range(node.range()).into());

        match node {
            /*** Scopes ***/
            KnotBlock(block) => {
                let nodeid = NodeId::from(block);
                state.add_node_kind(nodeid, NodeFlag::Knot | NodeFlag::Block | NodeFlag::Global);
                state.parent_scope.insert(nodeid, self.current_scope().id);
                self.knot = Some(Scope::new(block));
                Descend
            }
            StitchBlock(block) => {
                let visibility = if self.knot.is_some() {
                    NodeFlag::Local
                } else {
                    NodeFlag::Global
                };
                let nodeid = NodeId::from(block);
                state.add_node_kind(nodeid, NodeFlag::Stitch | NodeFlag::Block | visibility);
                state
                    .parent_scope
                    .insert(block.into(), self.current_scope().id);
                self.stitch = Some(Scope::new(block));
                Descend
            }

            /*** Names ***/
            Knot(knot) => {
                let mut kind = NodeFlag::Definition | NodeFlag::Global;
                let defid = DefId::from(knot);
                if knot.function().is_some() {
                    kind |= NodeFlag::Function
                } else {
                    kind |= NodeFlag::Knot
                };
                if knot.params().is_some() {
                    kind |= NodeFlag::HasParams;
                }
                state.add_node_kind(defid.into(), kind);
                state
                    .parent_scope
                    .insert(defid.into(), self.current_scope().id);

                let name = self.doc.node_text(knot.name());
                self.current_scope_mut().name = name;
                state.add_global(name, defid);
                Descend
            }

            Stitch(stitch) => {
                let defid = DefId::from(stitch);
                state
                    .parent_scope
                    .insert(defid.into(), self.current_scope().id);

                let name = self.doc.node_text(stitch.name());
                self.current_scope_mut().name = name;

                let mut kind = NodeFlag::Definition | NodeFlag::Global | NodeFlag::Stitch;
                if stitch.params().is_some() {
                    kind |= NodeFlag::HasParams;
                }

                if let Some(knot) = self.knot.as_mut() {
                    // If we are inside a knot block, add our name to its locals …
                    knot.add_address(name, defid);
                    kind |= NodeFlag::Local;
                    state.add_global(format!("{knot}.{name}"), defid);
                } else {
                    state.add_global(name, defid);
                }
                state.add_node_kind(defid.into(), kind);
                Descend
            }

            Label(label) => {
                let mut kind = NodeFlag::Definition | NodeFlag::Global | NodeFlag::Label;
                let defid = DefId::from(label);
                state
                    .parent_scope
                    .insert(defid.into(), self.current_scope().id);

                let name = self.doc.node_text(label.name());

                match (self.knot.as_mut(), self.stitch.as_mut()) {
                    (None, None) => {
                        state.add_global(name, defid);
                    }
                    (None, Some(scope)) | (Some(scope), None) => {
                        kind |= NodeFlag::Local;
                        scope.add_address(name, defid);
                        state.add_global(format!("{scope}.{name}"), defid);
                    }
                    (Some(knot), Some(stitch)) => {
                        kind |= NodeFlag::Local;
                        // Basically, the stitch name is optional both inside the knot and globally.
                        knot.add_address(name, defid);
                        knot.add_address(format!("{stitch}.{name}"), defid);
                        state.add_global(format!("{knot}.{name}"), defid);
                        state.add_global(format!("{knot}.{stitch}.{name}"), defid);
                    }
                }
                state.add_node_kind(defid.into(), kind);
                Descend
            }

            Param(param) => {
                let name_node = param.value().map(|val| match val {
                    ink_syntax::ParamValue::Divert(divert) => divert.target().upcast(),
                    ink_syntax::ParamValue::Identifier(identifier) => identifier.upcast(),
                });
                let defid = DefId::from(param);
                let kind =
                    NodeFlag::Definition | NodeFlag::Usage | NodeFlag::Local | NodeFlag::Param;
                if self.external {
                    state.add_node_kind(defid.into(), kind | NodeFlag::External);
                } else {
                    state.add_node_kind(defid.into(), kind);
                    let param_name = self.doc.node_text(name_node);
                    let scope = self.current_scope_mut();
                    // Externals don't define a scope so we only add params to the scope if we're not in an EXTERNAL.
                    state.parent_scope.insert(defid.into(), scope.id);
                    scope.add_local(param_name, defid);
                    scope.add_usage(defid.into(), param_name);
                }
                Ignore // No need to descend, we've already recorded the usage.
            }

            TempDef(temp) => {
                let kind = NodeFlag::Definition | NodeFlag::Local | NodeFlag::Temp;
                let defid = DefId::from(temp);
                state.add_node_kind(defid.into(), kind);
                state
                    .parent_scope
                    .insert(defid.into(), self.current_scope().id);
                let temp_name = self.doc.node_text(temp.name());
                self.current_scope_mut().add_local(temp_name, defid);
                Descend
            }

            /*** Usages ***/
            Divert(_) | Tunnel(_) | Thread(_) => {
                self.redirect = true;
                Descend
            }
            Call(_) => {
                self.call = true;
                Descend
            }

            Args(_) => {
                // Arguments don't inherit redirect or call flags.
                self.redirect = false;
                self.call = false;
                Descend
            }

            // XXX: There is a bug(?) somewhere that causes qualified names and identifiers to be wrapped in an expr.
            // Not sure why, but we work around this here:
            QualifiedName(qname) | Expr(ink_syntax::Expr::QualifiedName(qname)) => {
                self.qname = Some(qname);
                Descend
            }

            Identifier(identifier) | Expr(ink_syntax::Expr::Identifier(identifier)) => {
                let byte_range = self
                    .qname
                    .map(|qname| qname.start_byte()..identifier.end_byte())
                    .unwrap_or_else(|| identifier.byte_range());
                let usgid = UsageId::from(identifier);
                let text = self.doc.text(byte_range);
                let builtin =
                    (self.redirect && builtin_addr(text)) || (self.call && builtin_func(text));

                let mut kind = BitFlags::from(NodeFlag::Usage);
                kind.set(NodeFlag::Call, self.call);
                kind.set(NodeFlag::Redirect, self.redirect);
                kind.set(NodeFlag::ListItem, self.listvalues);
                kind.set(NodeFlag::Builtin, builtin);
                state.add_node_kind(usgid.into(), kind);

                if !builtin {
                    self.current_scope_mut().add_usage(usgid, text);
                }
                Ignore
            }

            /*** Globals ***/
            External(ext) => {
                let mut kind = NodeFlag::Definition | NodeFlag::Function | NodeFlag::External;
                let defid = DefId::from(ext);
                if ext.params().is_ok() {
                    kind |= NodeFlag::HasParams;
                }
                self.external = true;

                state.add_node_kind(defid.into(), kind);
                state.add_global(self.doc.node_text(ext.name()), defid);
                Descend
            }

            Global(global) => {
                let defid = DefId::from(global);
                let keyword = if global.keyword().is_ok_and(|it| it.as_const().is_some()) {
                    NodeFlag::Const
                } else {
                    NodeFlag::Var
                };
                state.add_node_kind(defid.into(), NodeFlag::Definition | keyword);
                state.add_global(self.doc.node_text(global.name()), defid);
                Descend
            }

            List(list) => {
                let defid = DefId::from(list);
                let list_name = self.doc.node_text(list.name());
                let range = self.range(list.name());
                state.add_node_kind(defid.into(), NodeFlag::Definition | NodeFlag::List);
                self.list = Some((range, list_name));
                state.add_global(list_name, defid);
                Descend
            }
            ListValueDefs(_) => Descend,

            ListValueDef(def) => {
                let defid = DefId::from(def);
                let item_name = self.doc.node_text(def.name());
                state.add_node_kind(defid.into(), NodeFlag::Definition | NodeFlag::ListItem);
                state.add_global(item_name, defid);
                if let Some((_range, list_name)) = self.list {
                    state.add_global(format!("{list_name}.{item_name}"), defid);
                }
                // if no list was set, we got here via an ERROR node … oh well …
                Descend
            }

            ListValues(_) => {
                self.listvalues = true;
                Descend
            }

            Include(include) => {
                let node = include.path();
                let path = self.doc.text(node.byte_range());
                let range = self.doc.lsp_range(node.range());
                state.imported_files.push((path.into(), range.into()));
                Ignore
            }

            /*** Unused ***/
            AltArm(_) => Descend,
            Alternatives(_) => Descend,
            Assignment(_) => Descend,
            Binary(_) => Descend,
            BlockComment(_) => Ignore,
            Boolean(_) => Ignore,
            Choice(_) => Descend,
            ChoiceBlock(_) => Descend,
            ChoiceMark(_) => Ignore,
            ChoiceMarks(_) => Ignore,
            ChoiceOnly(_) => Descend,
            Code(_) => Descend,
            CondArm(_) => Descend,
            CondBlock(_) => Descend,
            Condition(_) => Descend,
            ConditionalText(_) => Descend,
            Content(_) => Descend,
            Else(_) => Ignore,
            Eol(_) => Ignore,
            Eval(_) => Descend,
            Expr(_) => Descend,
            Gather(_) => Descend,
            GatherBlock(_) => Descend,
            GatherMark(_) => Ignore,
            GatherMarks(_) => Ignore,
            Glue(_) => Ignore,
            Ink(_) => Descend,
            LineComment(_) => Ignore,
            MultilineAlternatives(_) => Descend,
            Number(_) => Ignore,
            Paragraph(_) => Descend,
            Params(_) => Descend,
            Paren(_) => Descend,
            Path(_) => Ignore,
            Postfix(_) => Descend,
            Return(_) => Descend,
            String(_) => Descend, // because String interpolation/evaluation
            Tag(_) => Descend,
            Text(_) => Ignore,
            TodoComment(_) => Ignore,
            Unary(_) => Descend,
        }
    }

    /// We resolve variables only upon leaving a scope.
    ///
    /// This is because, all names are already visible in their scope, even before they are declared.
    /// Therefore we treat
    fn leave(&mut self, node: ink_syntax::AllNamed<'a>, state: &mut Self::State) {
        use ink_syntax::AllNamed::*;
        match node {
            Ink(_) => {
                for (usage_id, (text, maybe_local)) in self.ink.usages.drain() {
                    let resolved =
                        maybe_local && state.resolve_from(&mut self.ink.locals, usage_id, text);
                    if !resolved {
                        state.add_unresolved(usage_id, text);
                    }
                }
                if let Some(locals) = Vec1::from_iter(self.ink.locals.drain()) {
                    state.locals_in_scope.insert(self.ink.id, locals);
                }
            }

            KnotBlock(_) => {
                let mut scope = self
                    .knot
                    .take()
                    .expect("scope should have been set on entry");

                for (usage_id, (text, maybe_local)) in scope.usages.drain() {
                    let resolved_locals =
                        maybe_local && state.resolve_from(&mut scope.locals, usage_id, text);
                    let resolved_addresses =
                        state.resolve_from(&mut scope.addresses, usage_id, text);
                    let resolved_locally = resolved_locals || resolved_addresses;

                    if !resolved_locally {
                        state.add_unresolved(usage_id, text);
                    }
                }
                if let Some(locals) = Vec1::from_iter(scope.locals.drain()) {
                    state.locals_in_scope.insert(scope.id, locals);
                }
                if let Some(addresses) = Vec1::from_iter(scope.addresses.drain()) {
                    state.addresses_in_scope.insert(scope.id, addresses);
                }
            }

            StitchBlock(_) => {
                let mut scope = self
                    .stitch
                    .take()
                    .expect("scope should have been set on entry");

                for (usage_id, (text, maybe_local)) in scope.usages.drain() {
                    let resolved_locals =
                        maybe_local && state.resolve_from(&mut scope.locals, usage_id, text);
                    let resolved_addresses =
                        state.resolve_from(&mut scope.addresses, usage_id, text);
                    let resolved_locally = resolved_locals || resolved_addresses;

                    // If we've not yet found anything, and the stitch is part of a knot, we look at its locals:
                    // FIXME: This is wrong! Ink doesn't even resolve parent params!
                    if !resolved_locally {
                        if let Some(knot) = self.knot.as_mut() {
                            knot.add_outside_usage(usage_id, text.into());
                        } else {
                            state.add_unresolved(usage_id, text);
                        }
                    }
                }
                if let Some(locals) = Vec1::from_iter(scope.locals.drain()) {
                    state.locals_in_scope.insert(scope.id, locals);
                }
                if let Some(addresses) = Vec1::from_iter(scope.addresses.drain()) {
                    state.addresses_in_scope.insert(scope.id, addresses);
                }
            }

            List(_) => self.list = None,
            QualifiedName(_) | Expr(ink_syntax::Expr::QualifiedName(_)) => self.qname = None,
            Divert(_) | Tunnel(_) | Thread(_) => self.redirect = false,
            Call(_) => self.call = false,
            ListValues(_) => self.listvalues = false,
            External(_) => self.external = false,

            _ => {}
        }
    }

    fn combine(_: &mut Self::State, _: Self::State) {
        // SAFETY: We never DescendWith, therefore we never combine.
        unsafe { unreachable_unchecked() }
    }
}

fn builtin_addr(s: &str) -> bool {
    match s {
        "DONE" | "END" => true,
        _ => false,
    }
}

fn builtin_func(s: &str) -> bool {
    match s {
        "CHOICE_COUNT" | "FLOAT" | "FLOOR" | "INT" | "LIST_ALL" | "LIST_COUNT" | "LIST_INVERT"
        | "LIST_MAX" | "LIST_MIN" | "LIST_RANDOM" | "LIST_RANGE" | "LIST_VALUE" | "POW"
        | "RANDOM" | "SEED_RANDOM" | "TURNS" | "TURNS_SINCE" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    pub mod assertions;

    use super::*;
    use indoc::indoc;
    use rassert::prelude::*;

    mod flags {
        use super::*;
        use assertions::BitFlagsExpectation as _;
        use util::softly;
        use NodeFlag::*;

        #[test]
        fn builtins() {
            let text = indoc! {"
            {
                - TURNS_SINCE(-> somewhere) > 3: -> END
                //|         |                       ^^^@
                //|         |    ^^^^^^^^^@
                //^^^^^^^^^^^@
                - RANDOM(0, 100) > 50: -> DONE
                //|    |                  ^^^^@
                //^^^^^^@
                - OTHER(): -> somewhere_else
                //|   |       ^^^^^^^^^^^^^^@
                //^^^^^@
            }"};

            let doc = InkDocument::new(text.to_string(), None);
            let infos = Vstr::new(&doc).traverse(doc.root());
            let flags = scan_flags(text, infos);

            softly!(
                expect!(&flags["TURNS_SINCE"]).to_contain(Builtin | Call),
                expect!(&flags["RANDOM"]).to_contain(Builtin | Call),
                expect!(&flags["END"]).to_contain(Builtin | Redirect),
                expect!(&flags["DONE"]).to_contain(Builtin | Redirect),
                expect!(&flags["OTHER"])
                    .soft()
                    .to_contain(Call)
                    .and()
                    .not()
                    .to_contain(Builtin),
                expect!(&flags["OTHER"])
                    .to_contain(Call)
                    .and()
                    .not()
                    .to_contain(Builtin),
                expect!(&flags["somewhere"])
                    .to_contain(Redirect)
                    .and()
                    .not()
                    .to_contain(Builtin),
                expect!(&flags["somewhere_else"])
                    .to_contain(Redirect)
                    .and()
                    .not()
                    .to_contain(Builtin)
            );
        }

        #[test]
        fn only_function_name_has_call_flag() {
            let text = indoc! {"
                Nested calls: {func1(a, func2(b, c)) + other} are possible
                //             |   | |  |   | |  |     ^^^^^@
                //             |   | |  |   | |  ^@
                //             |   | |  |   | ^@
                //             |   | |  ^^^^^@
                //             |   | ^@
                //             ^^^^^@
            "};

            let doc = InkDocument::new(text.to_string(), None);
            let infos = Vstr::new(&doc).traverse(doc.root());
            let flags = scan_flags(text, infos);

            softly!(
                expect!(&flags["func1"]).to_contain(Call),
                expect!(&flags["func2"]).to_contain(Call),
                expect!(&flags["a"]).not().to_contain(Call),
                expect!(&flags["b"]).not().to_contain(Call),
                expect!(&flags["c"]).not().to_contain(Call),
                expect!(&flags["other"]).not().to_contain(Call)
            );
        }

        #[test]
        fn only_redirect_target_contains_redirect_flag() {
            let text = indoc! {r"
                Redirects -> knot(param, -> next_knot) can be nested
                //           |  | |   |     ^^^^^^^^^@
                //           |  | ^^^^^@
                //           ^^^^@
            "};

            let doc = InkDocument::new(text.to_string(), None);
            let infos = Vstr::new(&doc).traverse(doc.root());
            let flags = scan_flags(text, infos);

            softly!(
                expect!(&flags["knot"])
                    .to_contain(Redirect)
                    .conclude_panic(),
                expect!(&flags["next_knot"])
                    .to_contain(Redirect)
                    .conclude_panic(),
                expect!(&flags["param"])
                    .not()
                    .to_contain(Redirect)
                    .conclude_panic()
            );
        }

        fn scan_flags<'a>(
            text: &'a str,
            infos: NodeInfos,
        ) -> HashMap<&'a str, BitFlags<NodeFlag, u32>> {
            text_annotations::scan_default_annotations(text)
                .map(|ann| {
                    // convention to use "the text itself"
                    let name = if ann.claim() == "@" {
                        ann.text()
                    } else {
                        ann.claim()
                    };
                    let flags = infos.flags_by_range(ann.text_location);
                    (name, flags)
                })
                .collect()
        }
    }
}
