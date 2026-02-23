use crate::lsp::salsa::{node_infos, InkGetters as _, Ops};
use ink_document::InkDocument;
use lsp_types::{Range, Uri};
use mini_milc::{Db, Old, Subquery, Updated};
use std::{borrow::Cow, collections::HashMap};
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

#[derive(Debug, Default, PartialEq, Eq)]
pub struct NodeInfos {
    /// Which scope node the node resides in
    parent_scope: HashMap<Range, Range>,

    usage_to_def: HashMap<Range, Vec1<Range>>,
    def_to_usage: HashMap<Range, Vec1<Range>>,

    /// Which names could not be resolved and the corresponding *identifier* ranges.
    ///
    /// So if the following couldn’t be resolved because `Stitch` does not contain `Label`:
    ///
    /// ``` ink
    /// -> Stitch.Label
    /// // ^^^^^^^^^^^^ this is the key (String)
    /// //        ^^^^^ this is the value (the Range of the `Label` identifier)
    /// ```
    unresolved: HashMap<String, Vec<Range>>,

    node_kind: HashMap<Range, NodeKind>,
}

impl NodeInfos {
    fn add_unresolved<T: AsRef<str> + ToString>(&mut self, usage: Range, text: T) {
        // Assume the unresolved usages happen several times (referring to existing globals),
        // so we only create the owned string as late as possible (but with 2 accesses :-/)
        if let Some(unresolved_ids) = self.unresolved.get_mut(text.as_ref()) {
            unresolved_ids.push(usage);
        } else {
            self.unresolved.insert(text.to_string(), vec![usage]);
        }
    }

    fn resolve_from<'a>(&mut self, defs: &mut Definitions, usage: Range, text: &str) -> bool {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    KnotOrFunctionBlock,
    StitchBlock,
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
    Usage,
}

#[derive(Debug)]
struct Scope<'a> {
    range: Range,
    name: Option<&'a str>,
    locals: Definitions<'a>,
    /// temps don't transfer to subscopes, so we keep track of them separately
    temps: Definitions<'a>,
    usages: Usages<'a>,
}

impl<'a> Scope<'a> {
    fn new(block: impl Into<Range>) -> Self {
        Self {
            range: block.into(),
            name: None,
            locals: Default::default(),
            temps: Default::default(),
            usages: Default::default(),
        }
    }
}
type Definitions<'a> = HashMap<Cow<'a, str>, Vec1<Range>>;
type Usages<'a> = HashMap<Range, &'a str>;

#[derive(Debug)]
struct Vstr<'a> {
    uri: &'a Uri,
    doc: &'a InkDocument,
    ink: Scope<'a>,
    knot: Option<Scope<'a>>,
    stitch: Option<Scope<'a>>,
    qname: Option<ink_syntax::QualifiedName<'a>>,
    list: Option<(Range, &'a str)>,
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
            ink: Scope::new(doc.lsp_range(doc.root().range())),
            knot: None,
            stitch: None,
            qname: None,
            list: Default::default(),
        }
    }

    fn range(&self, node: impl Node<'a>) -> Range {
        self.doc.lsp_range(node.range())
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
        // dbg!(&state);
        // dbg!(&self);
        match node {
            /*** Scopes ***/
            KnotBlock(knot_block) => {
                let range = self.range(knot_block);
                state.node_kind.insert(range, NodeKind::KnotOrFunctionBlock);
                state.parent_scope.insert(range, self.current_scope().range);
                self.knot = Some(Scope::new(range));
                eprintln!("Entering Knot scope");
                Descend
            }
            StitchBlock(stitch_block) => {
                let range = self.range(stitch_block);
                state.node_kind.insert(range, NodeKind::StitchBlock);
                state.parent_scope.insert(range, self.current_scope().range);
                self.stitch = Some(Scope::new(range));
                Descend
            }

            /*** Names ***/
            Knot(knot) => {
                let range = self.range(knot.name());
                let is_func = knot.function().is_some_and(|it| it.is_ok());
                let kind = if is_func {
                    NodeKind::Function
                } else {
                    NodeKind::Knot
                };
                state.node_kind.insert(range, kind);
                state.parent_scope.insert(range, self.current_scope().range);

                let name = self.doc.node_text(knot.name());
                self.current_scope_mut().name = Some(name);
                Descend
            }

            Stitch(stitch) => {
                let range = self.range(stitch.name());
                state.node_kind.insert(range, NodeKind::Stitch);
                state.parent_scope.insert(range, self.current_scope().range);

                let name = self.doc.node_text(stitch.name());
                self.current_scope_mut().name = Some(name);

                if let Some(knot_scope) = self.knot.as_mut() {
                    // If we are inside a knot block, add our name to its locals …
                    knot_scope
                        .locals
                        .entry(name.into())
                        .and_modify(|defs| defs.push(range))
                        .or_insert_with(|| Vec1::new(range));
                }
                Descend
            }

            Label(label) => {
                let range = self.range(label.name());
                state.node_kind.insert(range, NodeKind::Label);
                state.parent_scope.insert(range, self.current_scope().range);

                let label_name = self.doc.node_text(label.name());

                if let Some(knot_scope) = self.knot.as_mut() {
                    knot_scope
                        .locals
                        .entry(label_name.into())
                        .and_modify(|defs| defs.push(range))
                        .or_insert_with(|| Vec1::new(range));
                    if let Some(stitch_name) = self.stitch.as_ref().and_then(|it| it.name) {
                        // if we're inside a stitch, the stitch name can also be given
                        knot_scope
                            .locals
                            .entry(format!("{stitch_name}.{label_name}").into())
                            .and_modify(|defs| defs.push(range))
                            .or_insert_with(|| Vec1::new(range));
                    }
                } else if let Some(stitch_scope) = self.stitch.as_mut() {
                    // We're in a global stitch
                    stitch_scope
                        .locals
                        .entry(label_name.into())
                        .and_modify(|defs| defs.push(range))
                        .or_insert_with(|| Vec1::new(range));
                }
                /* Else: global label, not out concern. */
                Ignore
            }

            Param(param) => {
                let name_node = param.value().map(|val| match val {
                    ink_syntax::ParamValue::Divert(divert) => divert.target().upcast(),
                    ink_syntax::ParamValue::Identifier(identifier) => identifier.upcast(),
                });
                let range = self.range(name_node);
                state.node_kind.insert(range, NodeKind::Param);
                state.parent_scope.insert(range, self.current_scope().range);
                let param_name = self.doc.node_text(name_node);
                self.current_scope_mut()
                    .locals
                    .entry(param_name.into())
                    .and_modify(|vec1| vec1.push(range))
                    .or_insert_with(|| Vec1::new(range));
                Descend
            }

            TempDef(temp) => {
                let range = self.range(temp.name());
                state.node_kind.insert(range, NodeKind::Temp);
                state.parent_scope.insert(range, self.current_scope().range);
                let temp_name = self.doc.node_text(temp.name());
                self.current_scope_mut()
                    .temps
                    .entry(temp_name.into())
                    .and_modify(|vec1| vec1.push(range))
                    .or_insert_with(|| Vec1::new(range));
                Descend
            }

            /*** Usages ***/
            // XXX: There is a bug(?) somewhere that causes qualified names and identifiers to be wrapped in an expr.
            // Not sure why, but we work around this here:
            QualifiedName(qname) | Expr(ink_syntax::Expr::QualifiedName(qname)) => {
                self.qname = Some(qname);
                let range = self.range(qname);
                state.node_kind.insert(range, NodeKind::Usage);
                state.parent_scope.insert(range, self.current_scope().range);
                Descend
            }

            Identifier(identifier) | Expr(ink_syntax::Expr::Identifier(identifier)) => {
                let range = self.range(identifier);
                state.node_kind.insert(range, NodeKind::Usage);
                let byte_range = self
                    .qname
                    .map(|qname| dbg!(qname.start_byte()..identifier.end_byte()))
                    .unwrap_or_else(|| identifier.byte_range());
                let text = self.doc.text(byte_range);
                self.current_scope_mut().usages.insert(range, text);
                Ignore
            }

            /*** Unused ***/
            AltArm(_) => Descend,
            Alternatives(_) => Descend,
            Args(_) => Descend,
            Assignment(_) => Descend,
            Binary(_) => Descend,
            BlockComment(_) => Ignore,
            Boolean(_) => Ignore,
            Call(_) => Descend,
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
            Divert(_) => Descend,
            Else(_) => Ignore,
            Eol(_) => Ignore,
            Eval(_) => Descend,
            Expr(_) => Descend,
            External(_) => Ignore, // Doesn't have any internal structure.
            Gather(_) => Descend,
            GatherBlock(_) => Descend,
            GatherMark(_) => Ignore,
            GatherMarks(_) => Ignore,
            Global(_) => Ignore, // VAR / CONST can't contain any usages
            Glue(_) => Ignore,
            Include(_) => Ignore,
            Ink(_) => Descend,
            LineComment(_) => Ignore,
            List(_) => Ignore,
            ListValueDef(_) => Ignore,
            ListValueDefs(_) => Ignore,
            ListValues(_) => Descend,
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
            Thread(_) => Descend,
            TodoComment(_) => Ignore,
            Tunnel(_) => Descend,
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
                for (usage_id, text) in self.ink.usages.drain() {
                    let resolved = state.resolve_from(&mut self.ink.temps, usage_id, text);
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

                for (usage_id, text) in scope.usages.drain() {
                    let resolved_temps = state.resolve_from(&mut scope.temps, usage_id, text);
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

                for (usage_id, text) in scope.usages.drain() {
                    let resolved_temps = state.resolve_from(&mut scope.temps, usage_id, text);
                    let resolved_locals = state.resolve_from(&mut scope.locals, usage_id, text);
                    let resolved_locally = resolved_temps || resolved_locals;

                    // If we've not yet found anything, and the stitch is part of a knot, we look at its locals:
                    let resolved = resolved_locally
                        || self // short-circuit: Only execute if we haven't resolved anything yet.
                            .knot
                            .as_mut()
                            // NOTE: Not looking at parent's temps here, we can't see those.
                            .map(|knot| state.resolve_from(&mut knot.locals, usage_id, text))
                            .unwrap_or(false);

                    if !resolved {
                        state.add_unresolved(usage_id, text);
                    }
                }
            }
            List(_) => self.list = None,
            QualifiedName(_) | Expr(ink_syntax::Expr::QualifiedName(_)) => self.qname = None,
            _ => {}
        }
    }

    fn combine(_: &mut Self::State, _: Self::State) {
        unreachable!("We don't have substates")
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
        let loc: HashMap<&str, Range> = text_annotations::scan_default_annotations(text)
            .map(|it| (it.claim(), Range::from(it.text_location)))
            .collect();

        check!(infos.usage_to_def.get(&loc["top-ref1"]) == Some(&vec1![loc["top-def"]]));
        check!(infos.usage_to_def.get(&loc["top-ref2"]) == None);
        check!(infos.usage_to_def.get(&loc["top-ref3"]) == None);

        check!(infos.usage_to_def.get(&loc["knot-ref1"]) == None);
        check!(infos.usage_to_def.get(&loc["knot-ref2"]) == Some(&vec1![loc["knot-def"]]));
        check!(infos.usage_to_def.get(&loc["knot-ref3"]) == None);

        check!(infos.usage_to_def.get(&loc["stitch-ref1"]) == None);
        check!(infos.usage_to_def.get(&loc["stitch-ref2"]) == None);
        check!(infos.usage_to_def.get(&loc["stitch-ref3"]) == Some(&vec1![loc["stitch-def"]]));
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
        let loc: HashMap<&str, Range> = text_annotations::scan_default_annotations(text)
            .map(|it| (it.claim(), Range::from(it.text_location)))
            .collect();

        check!(infos.usage_to_def.get(&loc["usage-knot-p1"]) == Some(&vec1![loc["def-knot-p1"]]));
        check!(infos.usage_to_def.get(&loc["usage-knot-p2"]) == Some(&vec1![loc["def-knot-p2"]]));

        check!(
            infos.usage_to_def.get(&loc["usage-stitch-p1"]) == Some(&vec1![loc["def-stitch-p1"]]),
            "p1 refers to the inner stitch scope"
        );
        check!(
            infos.usage_to_def.get(&loc["usage-stitch-p2"]) == Some(&vec1![loc["def-knot-p2"]]),
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
        let loc: HashMap<&str, Range> = text_annotations::scan_default_annotations(text)
            .map(|it| (it.claim(), Range::from(it.text_location)))
            .collect();

        dbg!(&infos);
        check!(infos.usage_to_def.get(&loc["usage:knot-1"]) == Some(&vec1![loc["outer"]]));
        check!(infos.usage_to_def.get(&loc["usage:knot-2"]) == Some(&vec1![loc["inner"]]));
        check!(infos.usage_to_def.get(&loc["usage:knot-3"]) == Some(&vec1![loc["Stitch"]]));

        check!(infos.usage_to_def.get(&loc["usage:stitch-1"]) == Some(&vec1![loc["inner"]]));
        check!(
            infos.usage_to_def.get(&loc["usage:stitch-2"]) == None,
            r"because `outer` is not namespaced by Stitch "
        );
        check!(infos.usage_to_def.get(&loc["usage:stitch-3"]) == Some(&vec1![loc["Stitch"]]));

        check!(
            infos.usage_to_def.get(&loc["usage:stitch-4"]) == Some(&vec1![loc["outer"]]),
            "plain outer works"
        );
    }
}
