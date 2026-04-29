use crate::lsp::{
    location::TextRange,
    salsa::{node_infos, InkGetters as _, Ops},
};
use bimap::BiHashMap;
use enumflags2::{bitflags, BitFlags};
use ink_document::{
    ids::{DefId, NodeId, UsageId},
    InkDocument,
};
use mini_milc::{Db, Old, Subquery, Updated};
use std::{collections::HashMap, hint::unreachable_unchecked};
use tree_traversal::Visitor;
use type_sitter::Node;

impl Subquery<Ops, NodeInfos> for node_infos {
    fn value(&self, db: &impl Db<Ops>, old: Old<NodeInfos>) -> Updated<NodeInfos> {
        let doc = db.document(self.docid);
        let new = Vstr::new(&doc).traverse(doc.root());
        old.update(new)
    }
}

// Public interface
impl NodeInfos {
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
    pub locs: BiHashMap<NodeId, lsp_types::Range>,
    pub flags: HashMap<NodeId, BitFlags<NodeFlag>>,
}

// Private helpers
impl NodeInfos {
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
struct Vstr<'a> {
    doc: &'a InkDocument,
    knot: Option<ink_syntax::KnotBlock<'a>>,
    stitch: Option<ink_syntax::StitchBlock<'a>>,
    qname: Option<ink_syntax::QualifiedName<'a>>,
    list: Option<(TextRange, &'a str)>,
    call: bool,
    redirect: bool,
    /// is the current usage a listvalues query (`list_name ? (item.name)`)
    listvalues: bool,
    external: bool,
}

impl<'a> Vstr<'a> {
    fn new(doc: &'a InkDocument) -> Self {
        Self {
            doc,
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
                self.knot = Some(block);
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
                self.stitch = Some(block);
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

                Descend
            }

            Stitch(stitch) => {
                let defid = DefId::from(stitch);

                let mut kind = NodeFlag::Definition | NodeFlag::Global | NodeFlag::Stitch;
                if stitch.params().is_some() {
                    kind |= NodeFlag::HasParams;
                }

                if self.knot.is_some() {
                    kind |= NodeFlag::Local;
                }
                state.add_node_kind(defid.into(), kind);
                Descend
            }

            Label(label) => {
                let mut kind = NodeFlag::Definition | NodeFlag::Global | NodeFlag::Label;
                let defid = DefId::from(label);

                if self.knot.is_some() || self.stitch.is_some() {
                    kind |= NodeFlag::Local;
                }
                state.add_node_kind(defid.into(), kind);
                Descend
            }

            Param(param) => {
                let defid = DefId::from(param);
                let kind =
                    NodeFlag::Definition | NodeFlag::Usage | NodeFlag::Local | NodeFlag::Param;
                if self.external {
                    state.add_node_kind(defid.into(), kind | NodeFlag::External);
                } else {
                    state.add_node_kind(defid.into(), kind);
                }
                Ignore // No need to descend, we've already recorded the usage.
            }

            TempDef(temp) => {
                let kind = NodeFlag::Definition | NodeFlag::Local | NodeFlag::Temp;
                let defid = DefId::from(temp);
                state.add_node_kind(defid.into(), kind);
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
                Descend
            }

            List(list) => {
                let defid = DefId::from(list);
                let list_name = self.doc.node_text(list.name());
                let range = self.range(list.name());
                state.add_node_kind(defid.into(), NodeFlag::Definition | NodeFlag::List);
                self.list = Some((range, list_name));
                Descend
            }
            ListValueDefs(_) => Descend,

            ListValueDef(def) => {
                let defid = DefId::from(def);
                state.add_node_kind(defid.into(), NodeFlag::Definition | NodeFlag::ListItem);
                // if no list was set, we got here via an ERROR node … oh well …
                Descend
            }

            ListValues(_) => {
                self.listvalues = true;
                Descend
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
            Include(_) => Ignore,
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
    fn leave(&mut self, node: ink_syntax::AllNamed<'a>, _: &mut Self::State) {
        use ink_syntax::AllNamed::*;
        match node {
            KnotBlock(_) => self.knot = None,
            StitchBlock(_) => self.stitch = None,
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
