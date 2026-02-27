use crate::lsp::{
    location::TextRange,
    salsa::{node_infos, InkGetters as _, Ops},
};
use enumflags2::{bitflags, BitFlags};
use ink_document::InkDocument;
use lsp_types::Uri;
use mini_milc::{Db, Old, Subquery, Updated};
use std::{borrow::Cow, collections::HashMap, hint::unreachable_unchecked};
use tree_traversal::Visitor;
use type_sitter::Node;
use util::{nonempty::Vec1, vec1};

impl Subquery<Ops, NodeInfos> for node_infos {
    fn value(&self, db: &impl Db<Ops>, old: Old<NodeInfos>) -> Updated<NodeInfos> {
        let doc_ids = db.doc_ids();
        let uri = doc_ids
            .get(self.docid)
            .expect("We docid should only exist when URI exsists.");
        let doc = db.document(self.docid);
        let new = Vstr::new(uri, &doc).traverse(doc.root());
        old.update(new)
    }
}

// Public interface
impl NodeInfos {
    /// NOTE: `range` must be the actual definition site.
    pub fn local_usages(&self, range: TextRange) -> Option<&Vec1<TextRange>> {
        self.def_to_usage.get(&range)
    }
    pub fn local_definitions(&self, range: TextRange) -> Option<&Vec1<TextRange>> {
        self.usage_to_def.get(&range)
    }

    pub fn global_names(&self, range: TextRange) -> Option<&Vec1<String>> {
        self.global_names_by_range.get(&range)
    }
    pub fn global_ranges(&self, text: &str) -> Option<&Vec1<TextRange>> {
        self.global_ranges_by_name.get(text)
    }

    pub fn unresolved_names(&self, range: TextRange) -> Option<&Vec1<String>> {
        self.unresolved_name_by_range.get(&range)
    }
    pub fn unresolved_ranges(&self, text: &str) -> Option<&Vec1<TextRange>> {
        self.unresolved_range_by_name.get(text)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct NodeInfos {
    /// Which scope node the node resides in
    parent_scope: HashMap<TextRange, TextRange>,

    /// The global names defined in this file.
    global_names_by_range: HashMap<TextRange, Vec1<String>>,
    global_ranges_by_name: HashMap<String, Vec1<TextRange>>,

    usage_to_def: HashMap<TextRange, Vec1<TextRange>>,
    def_to_usage: HashMap<TextRange, Vec1<TextRange>>,

    /// Which names could not be resolved and the corresponding *identifier* ranges.
    ///
    /// So if the following couldn’t be resolved because `Stitch` does not contain `Label`:
    ///
    /// ``` ink
    /// -> Stitch.Label
    /// // ^^^^^^^^^^^^ this is the key (String)
    /// //        ^^^^^ this is the value (the TextRange of the `Label` identifier)
    /// ```
    unresolved_range_by_name: HashMap<String, Vec1<TextRange>>,
    unresolved_name_by_range: HashMap<TextRange, Vec1<String>>,

    node_kind: HashMap<TextRange, BitFlags<NodeKind>>,
}

// Private helpers
impl NodeInfos {
    fn add_global<T: ToString>(&mut self, text: T, range: TextRange) {
        self.global_names_by_range
            .entry(range)
            .and_modify(|vec1| vec1.push(text.to_string()))
            .or_insert_with(|| vec1![text.to_string()]);
        self.global_ranges_by_name
            .entry(text.to_string())
            .and_modify(|vec1| vec1.push(range))
            .or_insert_with(|| vec1![range]);
    }

    fn add_unresolved<T: AsRef<str> + ToString>(&mut self, usage: TextRange, text: T) {
        // Assume the unresolved usages happen several times (referring to existing globals),
        // so we only create the owned string as late as possible (but with 2 accesses :-/)
        if let Some(unresolved_ids) = self.unresolved_range_by_name.get_mut(text.as_ref()) {
            unresolved_ids.push(usage);
        } else {
            self.unresolved_range_by_name
                .insert(text.to_string(), vec1![usage]);
        }
        self.unresolved_name_by_range
            .entry(usage)
            .or_default()
            .push(text.to_string());
    }

    fn resolve_from<'a>(&mut self, defs: &mut Definitions, usage: TextRange, text: &str) -> bool {
        if let Some(def) = defs.get(text).cloned() {
            for def in def {
                self.usage_to_def
                    .entry(usage)
                    .and_modify(|vec1| vec1.push(def))
                    .or_insert_with(|| vec1![def]);
                self.def_to_usage
                    .entry(def)
                    .and_modify(|vec| vec.push(usage))
                    .or_insert_with(|| vec1![usage]);
            }
            true
        } else {
            false
        }
    }

    fn add_node_kind(&mut self, range: TextRange, kind: impl Into<BitFlags<NodeKind>>) {
        self.node_kind.entry(range).or_default().insert(kind);
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
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
}

#[derive(Debug)]
struct Scope<'a> {
    range: TextRange,
    name: &'a str,
    locals: Definitions<'a>,
    /// temps don't transfer to subscopes, so we keep track of them separately
    temps: Definitions<'a>,
    usages: Usages<'a>,
}

impl<'a> Scope<'a> {
    fn new(block: impl Into<TextRange>) -> Self {
        Self {
            range: block.into(),
            name: "",
            locals: Default::default(),
            temps: Default::default(),
            usages: Default::default(),
        }
    }

    fn add_local(&mut self, text: impl Into<Cow<'a, str>>, range: TextRange) {
        self.locals
            .entry(text.into())
            .and_modify(|defs| defs.push(range))
            .or_insert_with(|| Vec1::new(range));
    }

    fn add_temp(&mut self, text: impl Into<Cow<'a, str>>, range: TextRange) {
        self.temps
            .entry(text.into())
            .and_modify(|defs| defs.push(range))
            .or_insert_with(|| Vec1::new(range));
    }

    /// This is a "normal" usage that might resolve the a temp variable
    fn add_usage(&mut self, usage_id: TextRange, text: &'a str) {
        self.usages.insert(usage_id, (text, true));
    }

    /// This is a usage from "outside" this scope, which means it can't  see our temps.
    fn add_non_temp_usage(&mut self, usage_id: TextRange, text: &'a str) {
        self.usages.insert(usage_id, (text, false));
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

type Definitions<'a> = HashMap<Cow<'a, str>, Vec1<TextRange>>;
type Usages<'a> = HashMap<TextRange, (&'a str, bool)>;

#[derive(Debug)]
struct Vstr<'a> {
    uri: &'a Uri,
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
    fn new(uri: &'a Uri, doc: &'a InkDocument) -> Self {
        Self {
            uri,
            doc,
            ink: Scope::new(TextRange::from(doc.lsp_range(doc.root().range()))),
            knot: None,
            stitch: None,
            qname: None,
            list: Default::default(),
            call: false,
            redirect: false,
            listvalues: false,
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

        match node {
            /*** Scopes ***/
            KnotBlock(knot_block) => {
                let range = self.range(knot_block);
                state.add_node_kind(range, NodeKind::Knot | NodeKind::Block | NodeKind::Global);
                state.parent_scope.insert(range, self.current_scope().range);
                self.knot = Some(Scope::new(range));
                Descend
            }
            StitchBlock(stitch_block) => {
                let range = self.range(stitch_block);
                let visibility = if self.knot.is_some() {
                    NodeKind::Local
                } else {
                    NodeKind::Global
                };
                state.add_node_kind(range, NodeKind::Stitch | NodeKind::Block | visibility);
                state.parent_scope.insert(range, self.current_scope().range);
                self.stitch = Some(Scope::new(range));
                Descend
            }

            /*** Names ***/
            Knot(knot) => {
                let range = self.range(knot.name());
                let kind = if knot.function().is_some() {
                    NodeKind::Function
                } else {
                    NodeKind::Knot
                };
                state.add_node_kind(range, NodeKind::Definition | NodeKind::Global | kind);
                state.parent_scope.insert(range, self.current_scope().range);

                let name = self.doc.node_text(knot.name());
                self.current_scope_mut().name = name;
                state.add_global(name, range);
                Descend
            }

            Stitch(stitch) => {
                let range = self.range(stitch.name());
                state.parent_scope.insert(range, self.current_scope().range);

                let name = self.doc.node_text(stitch.name());
                self.current_scope_mut().name = name;

                let mut kind = NodeKind::Definition | NodeKind::Global | NodeKind::Stitch;

                if let Some(knot) = self.knot.as_mut() {
                    // If we are inside a knot block, add our name to its locals …
                    knot.add_local(name, range);
                    kind |= NodeKind::Local;
                    state.add_global(format!("{knot}.{name}"), range);
                } else {
                    state.add_global(name, range);
                }
                state.add_node_kind(range, kind);
                Descend
            }

            Label(label) => {
                let range = self.range(label.name());
                let mut kind = NodeKind::Definition | NodeKind::Global | NodeKind::Label;
                state.parent_scope.insert(range, self.current_scope().range);

                let name = self.doc.node_text(label.name());

                match (self.knot.as_mut(), self.stitch.as_mut()) {
                    (None, None) => {
                        state.add_global(name, range);
                    }
                    (None, Some(scope)) | (Some(scope), None) => {
                        kind |= NodeKind::Local;
                        scope.add_local(name, range);
                        state.add_global(format!("{scope}.{name}"), range);
                    }
                    (Some(knot), Some(stitch)) => {
                        kind |= NodeKind::Local;
                        // Basically, the stitch name is optional both inside the knot and globally.
                        knot.add_local(name, range);
                        knot.add_local(format!("{stitch}.{name}"), range);
                        state.add_global(format!("{knot}.{name}"), range);
                        state.add_global(format!("{knot}.{stitch}.{name}"), range);
                    }
                }
                state.add_node_kind(range, kind);
                Descend
            }

            Param(param) => {
                let name_node = param.value().map(|val| match val {
                    ink_syntax::ParamValue::Divert(divert) => divert.target().upcast(),
                    ink_syntax::ParamValue::Identifier(identifier) => identifier.upcast(),
                });
                let range = self.range(name_node);
                let kind = NodeKind::Definition | NodeKind::Local | NodeKind::Param;
                state.add_node_kind(range, kind);
                state.parent_scope.insert(range, self.current_scope().range);
                let param_name = self.doc.node_text(name_node);
                self.current_scope_mut().add_local(param_name, range);
                Descend
            }

            TempDef(temp) => {
                let range = self.range(temp.name());
                let kind = NodeKind::Definition | NodeKind::Local | NodeKind::Temp;
                state.add_node_kind(range, kind);
                state.parent_scope.insert(range, self.current_scope().range);
                let temp_name = self.doc.node_text(temp.name());
                self.current_scope_mut().add_temp(temp_name, range);
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

            // XXX: There is a bug(?) somewhere that causes qualified names and identifiers to be wrapped in an expr.
            // Not sure why, but we work around this here:
            QualifiedName(qname) | Expr(ink_syntax::Expr::QualifiedName(qname)) => {
                self.qname = Some(qname);
                Descend
            }

            Identifier(identifier) | Expr(ink_syntax::Expr::Identifier(identifier)) => {
                let range = self.range(identifier);
                let mut kind = BitFlags::from(NodeKind::Usage);
                kind.set(NodeKind::Call, self.call);
                kind.set(NodeKind::Redirect, self.redirect);
                kind.set(NodeKind::ListItem, self.listvalues);
                state.add_node_kind(range, kind);
                let byte_range = self
                    .qname
                    .map(|qname| qname.start_byte()..identifier.end_byte())
                    .unwrap_or_else(|| identifier.byte_range());
                let text = self.doc.text(byte_range);
                self.current_scope_mut().add_usage(range, text);
                Ignore
            }

            /*** Globals ***/
            External(ext) => {
                let range = self.range(ext.name());
                let kind = NodeKind::Definition | NodeKind::Function | NodeKind::External;
                state.add_node_kind(range, kind);
                state.add_global(self.doc.node_text(ext.name()), range);
                Descend
            }

            Global(global) => {
                let range = self.range(global.name());
                let keyword = if global.keyword().is_ok_and(|it| it.as_const().is_some()) {
                    NodeKind::Const
                } else {
                    NodeKind::Var
                };
                state.add_node_kind(range, NodeKind::Definition | keyword);
                state.add_global(self.doc.node_text(global.name()), range);
                Descend
            }

            List(list) => {
                let list_name = self.doc.node_text(list.name());
                let range = self.range(list.name());
                state.add_node_kind(range, NodeKind::Definition | NodeKind::List);
                self.list = Some((range, list_name));
                state.add_global(list_name, range);
                Descend
            }
            ListValueDefs(_) => Descend,

            ListValueDef(def) => {
                let item_name = self.doc.node_text(def.name());
                let range = self.range(def.name());
                state.add_node_kind(range, NodeKind::Definition | NodeKind::ListItem);
                state.add_global(item_name, range);
                if let Some((_range, list_name)) = self.list {
                    state.add_global(format!("{list_name}.{item_name}"), range);
                }
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
            Args(_) => Descend,
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
            Return(_) => Ignore,
            String(_) => Ignore,
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
                for (usage_id, (text, can_be_temp)) in self.ink.usages.drain() {
                    let resolved =
                        can_be_temp && state.resolve_from(&mut self.ink.temps, usage_id, text);
                    if !resolved {
                        state.add_unresolved(usage_id, text);
                    }
                }
            }

            KnotBlock(_) => {
                let mut scope = self
                    .knot
                    .take()
                    .expect("scope should have been set on entry");

                for (usage_id, (text, can_be_temp)) in scope.usages.drain() {
                    let resolved_temps =
                        can_be_temp && state.resolve_from(&mut scope.temps, usage_id, text);
                    let resolved_locals = state.resolve_from(&mut scope.locals, usage_id, text);
                    let resolved_locally = resolved_temps || resolved_locals;

                    if !resolved_locally {
                        state.add_unresolved(usage_id, text);
                    }
                }
            }

            StitchBlock(_) => {
                let mut scope = self
                    .stitch
                    .take()
                    .expect("scope should have been set on entry");

                for (usage_id, (text, can_be_temp)) in scope.usages.drain() {
                    let resolved_temps =
                        can_be_temp && state.resolve_from(&mut scope.temps, usage_id, text);
                    let resolved_locals = state.resolve_from(&mut scope.locals, usage_id, text);
                    let resolved_locally = resolved_temps || resolved_locals;

                    // If we've not yet found anything, and the stitch is part of a knot, we look at its locals:
                    if !resolved_locally {
                        if let Some(knot) = self.knot.as_mut() {
                            // NOTE: Not looking at parent's temps here, we can't see those.
                            knot.add_non_temp_usage(usage_id, text);
                        } else {
                            state.add_unresolved(usage_id, text);
                        }
                    }
                }
            }

            List(_) => self.list = None,
            QualifiedName(_) | Expr(ink_syntax::Expr::QualifiedName(_)) => self.qname = None,
            Divert(_) | Tunnel(_) | Thread(_) => self.redirect = false,
            Call(_) => self.call = false,
            ListValues(_) => self.listvalues = false,

            _ => {}
        }
    }

    fn combine(_: &mut Self::State, _: Self::State) {
        // SAFETY: We never DescendWith, therefore we never combine.
        unsafe { unreachable_unchecked() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use indoc::indoc;
    use std::str::FromStr as _;

    #[test]
    fn temps_are_only_visible_in_their_defining_scope() {
        let text = indoc! {"
            ~ temp toplevel_temp = 1
            //     ^^^^^^^^^^^^^ top-def
            - {toplevel_temp}
            // ^^^^^^^^^^^^^ top-ref1
            - {knot_temp}
            // ^^^^^^^^^ top-ref2
            - {stitch_temp}
            // ^^^^^^^^^^^ top-ref3

            === Knot ===
            ~ temp knot_temp = 1
            //     ^^^^^^^^^ knot-def

            - {toplevel_temp}
            // ^^^^^^^^^^^^^ knot-ref1
            - {knot_temp}
            // ^^^^^^^^^ knot-ref2
            - {stitch_temp}
            // ^^^^^^^^^^^ knot-ref3

            = Stitch
            ~ temp stitch_temp = 1
            //     ^^^^^^^^^^^ stitch-def

            - {toplevel_temp}
            // ^^^^^^^^^^^^^ stitch-ref1
            - {knot_temp}
            // ^^^^^^^^^ stitch-ref2
            - {stitch_temp}
            // ^^^^^^^^^^^ stitch-ref3
        "};
        let doc = InkDocument::new(text.to_string(), None);

        let uri = Uri::from_str("file:///main.ink").unwrap();
        let mut vstr = Vstr::new(&uri, &doc);
        let infos = vstr.traverse(doc.root());
        let loc: HashMap<&str, TextRange> = text_annotations::scan_default_annotations(text)
            .map(|it| (it.claim(), lsp_types::Range::from(it.text_location).into()))
            .collect();

        check!(infos.local_definitions(loc["top-ref1"]) == Some(&vec1![loc["top-def"]]));
        check!(infos.local_definitions(loc["top-ref2"]) == None);
        check!(infos.local_definitions(loc["top-ref3"]) == None);

        check!(infos.local_definitions(loc["knot-ref1"]) == None);
        check!(infos.local_definitions(loc["knot-ref2"]) == Some(&vec1![loc["knot-def"]]));
        check!(infos.local_definitions(loc["knot-ref3"]) == None);

        check!(infos.local_definitions(loc["stitch-ref1"]) == None);
        check!(infos.local_definitions(loc["stitch-ref2"]) == None);
        check!(infos.local_definitions(loc["stitch-ref3"]) == Some(&vec1![loc["stitch-def"]]));
    }

    #[test]
    fn params_are_visible_in_subscopes() {
        let text = indoc! {"
            === Knot(p1, p2) ===
            //       ^^ def-knot-p1
            //           ^^ def-knot-p2

            I see {p1} & {p2}.
            //     ^^ usage-knot-p1
            //            ^^ usage-knot-p2

            = Stitch(p1)
            //       ^^ def-stitch-p1
            I see {p1} & {p2}.
            //     ^^ usage-stitch-p1
            //            ^^ usage-stitch-p2
        "};
        let doc = InkDocument::new(text.to_string(), None);

        let uri = Uri::from_str("file:///main.ink").unwrap();
        let mut vstr = Vstr::new(&uri, &doc);
        let infos = vstr.traverse(doc.root());
        let loc: HashMap<&str, TextRange> = text_annotations::scan_default_annotations(text)
            .map(|it| (it.claim(), lsp_types::Range::from(it.text_location).into()))
            .collect();

        check!(infos.local_definitions(loc["usage-knot-p1"]) == Some(&vec1![loc["def-knot-p1"]]));
        check!(infos.local_definitions(loc["usage-knot-p2"]) == Some(&vec1![loc["def-knot-p2"]]));

        check!(
            infos.local_definitions(loc["usage-stitch-p1"]) == Some(&vec1![loc["def-stitch-p1"]]),
            "p1 refers to the inner stitch scope"
        );
        check!(
            infos.local_definitions(loc["usage-stitch-p2"]) == Some(&vec1![loc["def-knot-p2"]]),
            "p2 refers to the outer knot scope"
        );
    }

    #[test]
    fn locals_are_visible_in_subscopes() {
        let text = indoc! {"
            === Knot ===

            -> outer
            // ^^^^^ usage:knot-1
            -> Stitch.inner
            //        ^^^^^ usage:knot-2
            // ^^^^^^ usage:knot-3

            - (outer) Yea!
            // ^^^^^ outer

            = Stitch(p1)
            //^^^^^^ Stitch

            - (inner) Yea!
            // ^^^^^ inner

            -> inner
            // ^^^^^ usage:stitch-1
            -> Stitch.outer
            //        ^^^^^ usage:stitch-2
            // ^^^^^^ usage:stitch-3
            -> outer
            // ^^^^^ usage:stitch-4
             
        "};
        let doc = InkDocument::new(text.to_string(), None);

        let uri = Uri::from_str("file:///main.ink").unwrap();
        let mut vstr = Vstr::new(&uri, &doc);
        let infos = vstr.traverse(doc.root());
        let loc: HashMap<&str, TextRange> = text_annotations::scan_default_annotations(text)
            .map(|it| (it.claim(), lsp_types::Range::from(it.text_location).into()))
            .collect();

        dbg!(&infos);
        check!(infos.local_definitions(loc["usage:knot-1"]) == Some(&vec1![loc["outer"]]));
        check!(infos.local_definitions(loc["usage:knot-2"]) == Some(&vec1![loc["inner"]]));
        check!(infos.local_definitions(loc["usage:knot-3"]) == Some(&vec1![loc["Stitch"]]));

        check!(infos.local_definitions(loc["usage:stitch-1"]) == Some(&vec1![loc["inner"]]));
        check!(
            infos.local_definitions(loc["usage:stitch-2"]) == None,
            r"because `outer` is not namespaced by Stitch "
        );
        check!(infos.local_definitions(loc["usage:stitch-3"]) == Some(&vec1![loc["Stitch"]]));

        check!(
            infos.local_definitions(loc["usage:stitch-4"]) == Some(&vec1![loc["outer"]]),
            "plain outer works"
        );
    }

    #[test]
    fn globals() {
        let text = indoc! {"
            -> Knot.outer
            -> should_be_true(false)
            LIST list = item1, (item2) = 5
            - (global)
            === Knot ===
            - (outer) Yea!
            VAR var = true
            = Stitch
            - (inner) Yea!

            == function should_be_true(param)
                CONST const = 1
                ~ return true
        "};
        let doc = InkDocument::new(text.to_string(), None);

        let uri = Uri::from_str("file:///main.ink").unwrap();
        let mut vstr = Vstr::new(&uri, &doc);
        let infos = vstr.traverse(doc.root());

        // Just a quick glance at whether they are there. Let's trust that the locations, node types etc. are correct.
        let global_names: std::collections::BTreeSet<_> = infos
            .global_names_by_range
            .values()
            .flat_map(|it| it.iter().map(|it| it.as_str()))
            .collect();
        check!(
            global_names
                == [
                    "list",
                    "list.item1",
                    "list.item2",
                    "item1",
                    "item2",
                    "global",
                    "Knot",
                    "Knot.outer",
                    "var",
                    "Knot.Stitch",
                    "Knot.inner",
                    "Knot.Stitch.inner",
                    "should_be_true",
                    "const"
                ]
                .into()
        );
    }
}
