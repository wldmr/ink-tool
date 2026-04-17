use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasherDefault,
};

use derive_more::derive::{AsRef, Debug, Display, Into};
use ink_document::{
    ids::{DefId, ScopeId, UsageId},
    InkDocument,
};
use ink_syntax::AllNamed;
use mini_milc::subquery;
use tap::Tap;
use tree_traversal::{VisitInstruction, Visitor};
use type_sitter::Node;
use ustr::{ustr, IdentityHasher, Ustr};
use util::nonempty::{MapOfNonEmpty as _, Vec1};

use crate::lsp::{salsa::ink_inventory, InkGetters, Ops};

subquery!(Ops, ink_inventory, InkInventory, |self, db| {
    let doc = db.document(self.docid);
    traverse(&doc)
});

fn traverse(doc: &InkDocument) -> InkInventory {
    Vstr::new(doc).traverse_with_state(doc.root(), InkInventory::new(ScopeId::from(doc.root())))
}

#[derive(
    Default, Display, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Into,
)]
#[debug("Name({_0})")]
#[display("{_0}")]
pub struct Name(Ustr);
impl<T: AsRef<str>> From<T> for Name {
    fn from(value: T) -> Self {
        Name(ustr(value.as_ref()))
    }
}

pub type NameSet = HashSet<Name, BuildHasherDefault<IdentityHasher>>;
pub type NameMap<T> = HashMap<Name, T, BuildHasherDefault<IdentityHasher>>;
pub type LocMap<T> = HashMap<UsageId, T, BuildHasherDefault<IdentityHasher>>;

/// Describes the names declared and used in a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InkInventory {
    pub scope_id: ScopeId,
    pub lists: Vec<List>,
    pub vars: NameMap<Vec1<DefId>>,
    pub consts: NameMap<Vec1<DefId>>,
    pub sections: Vec<Section>,
    pub externals: NameMap<Vec1<DefId>>,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    pub name: Name,
    pub id: DefId,
    // We keep a Vec1 for the for the unlikely event that the user types the same name twice.
    pub items: NameMap<Vec1<DefId>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Body {
    pub temps: NameMap<Vec1<DefId>>,
    pub labels: NameMap<Vec1<DefId>>,
    pub usages: NameMap<Vec1<UsageId>>,
    pub names: LocMap<Name>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub name: Name,
    pub name_id: DefId,
    pub scope_id: ScopeId,
    pub section_kind: SectionKind,
    pub params: NameMap<Vec1<DefId>>,
    pub body: Body,
    pub subsections: Vec<Subsection>,
    /// Collection of all labels defined in subsections
    pub sub_labels: NameSet,
    /// Collection of all subsection names in here
    pub sub_names: NameSet,
    pub subscopes: HashMap<ScopeId, usize, BuildHasherDefault<IdentityHasher>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subsection {
    pub name: Name,
    pub name_id: DefId,
    pub scope_id: ScopeId,
    pub params: NameMap<Vec1<DefId>>,
    pub body: Body,
}

impl Section {
    fn new(name: Name, name_id: DefId, section_id: ScopeId) -> Self {
        Self {
            name,
            name_id,
            scope_id: section_id,
            section_kind: Default::default(),
            params: Default::default(),
            body: Default::default(),
            subsections: Default::default(),
            sub_names: Default::default(),
            sub_labels: Default::default(),
            subscopes: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionKind {
    #[default]
    Knot,
    Stitch,
    Function,
}

struct Vstr<'a> {
    doc: &'a InkDocument,
    /// When walking through a qualified name, we need to keep track of its start, so
    /// that we can properly make substrings.
    qname: Option<ink_syntax::QualifiedName<'a>>,
    scope: ScopeId,
    /// In some definitons we descend but want to ignore the name of the thing itself
    /// from being a usage.
    ignore: Option<DefId>,
}

impl<'a> Vstr<'a> {
    fn new(doc: &'a InkDocument) -> Self {
        Self {
            doc,
            qname: None,
            scope: doc.root().into(),
            ignore: Default::default(),
        }
    }

    fn text<N: Node<'a>>(&self, node: N) -> Name {
        Name(ustr(self.doc.node_text(node)))
    }

    fn text_between(&self, start: usize, end: usize) -> Name {
        Name(ustr(self.doc.text(start..end)))
    }

    pub fn knot_section(&self, node: ink_syntax::Knot<'a>) -> Section {
        Section::new(self.text(node.name()), node.into(), self.scope).tap_mut(|it| {
            it.section_kind = if node.function().is_some() {
                SectionKind::Function
            } else {
                SectionKind::Knot
            }
        })
    }
    pub fn toplevel_stitch(&self, node: ink_syntax::Stitch<'a>) -> Section {
        Section::new(self.text(node.name()), node.into(), self.scope)
            .tap_mut(|it| it.section_kind = SectionKind::Stitch)
    }
    pub fn child_stitch(&self, node: ink_syntax::Stitch<'a>) -> Subsection {
        Subsection {
            name: self.text(node.name()),
            name_id: node.into(),
            scope_id: self.scope,
            params: Default::default(),
            body: Default::default(),
        }
    }
}

impl InkInventory {
    pub fn new(scope_id: ScopeId) -> Self {
        Self {
            scope_id,
            lists: Default::default(),
            vars: Default::default(),
            consts: Default::default(),
            sections: Default::default(),
            externals: Default::default(),
            body: Default::default(),
        }
    }

    fn current_body(&mut self) -> &mut Body {
        if let Some(section) = self.sections.last_mut() {
            if let Some(sub) = section.subsections.last_mut() {
                &mut sub.body
            } else {
                &mut section.body
            }
        } else {
            &mut self.body
        }
    }
}

impl<'a> Visitor<'a, AllNamed<'a>> for Vstr<'a> {
    type State = InkInventory;

    fn visit(
        &mut self,
        node: AllNamed<'a>,
        state: &mut Self::State,
    ) -> VisitInstruction<Self::State> {
        use VisitInstruction::{Descend, Ignore};

        match node {
            /*** Scopes ***/
            AllNamed::KnotBlock(block) => {
                self.scope = block.into();
                Descend
            }
            AllNamed::StitchBlock(block) => {
                self.scope = block.into();
                Descend
            }

            /*** Names ***/
            AllNamed::Knot(knot) => {
                self.ignore = Some(knot.into());
                let section = self.knot_section(knot);
                state.sections.push(section);
                Descend
            }

            AllNamed::Stitch(stitch) => {
                self.ignore = Some(stitch.into());
                if let Some(parent) = state.sections.last_mut() {
                    let sub = self.child_stitch(stitch);
                    parent.sub_names.insert(sub.name);
                    let scope_index = parent.subsections.len(); // len before pushing == index of element we're about to push
                    parent.subscopes.insert(sub.scope_id, scope_index);
                    parent.subsections.push(sub);
                } else {
                    let mut stitch = self.toplevel_stitch(stitch);
                    stitch.section_kind = SectionKind::Stitch;
                    state.sections.push(stitch);
                }
                Descend
            }

            AllNamed::Label(label) => {
                let text = self.text(label.name());
                state.current_body().labels.register(text, label);
                if let Some(knot) = state.sections.last_mut() {
                    knot.sub_labels.insert(text);
                }
                Ignore
            }

            AllNamed::External(ext) => {
                let name = self.text(ext.name());
                state.externals.register(name, ext);
                Ignore // No need to look at the params, as an External has no body
            }

            AllNamed::Param(param) => {
                let name = match param.value() {
                    Ok(ink_syntax::ParamValue::Divert(div)) => self.text(div.target()),
                    Ok(ink_syntax::ParamValue::Identifier(ident)) => self.text(ident),
                    Err(_) => self.text(param), // weird, but let's muddle through
                };
                state
                    .sections
                    .last_mut() // reminder: We never enter Params for Externals, so this is fine.
                    .unwrap()
                    .params
                    .register(name, param);
                Ignore
            }

            AllNamed::TempDef(temp) => {
                let text = self.text(temp.name());
                self.ignore = Some(temp.into());
                state.current_body().temps.register(text, temp);
                Descend
            }

            /*** Globals ***/
            AllNamed::Global(global) => {
                let name = self.text(global.name());
                self.ignore = Some(global.into());
                if global.keyword().is_ok_and(|it| it.as_const().is_some()) {
                    state.consts.register(name, global)
                } else {
                    state.vars.register(name, global)
                };
                Descend
            }

            AllNamed::List(list) => {
                self.ignore = Some(list.into());
                state.lists.push(List {
                    name: self.text(list.name()),
                    id: list.into(),
                    items: Default::default(),
                });
                Descend
            }
            AllNamed::ListValueDefs(_) => Descend,

            AllNamed::ListValueDef(def) => {
                let name = self.text(def.name());
                state
                    .lists
                    .last_mut()
                    .expect("Must have entered a List")
                    .items
                    .register(name, def);
                Ignore
            }

            /*** Usages ***/
            AllNamed::Divert(_)
            | AllNamed::Tunnel(_)
            | AllNamed::Thread(_)
            | AllNamed::Call(_)
            | AllNamed::Args(_)
            | AllNamed::ListValues(_) => Descend,

            // XXX: There is a bug(?) somewhere that causes qualified names and identifiers to be wrapped in an expr.
            // Not sure why, but we work around this here:
            AllNamed::QualifiedName(qname)
            | AllNamed::Expr(ink_syntax::Expr::QualifiedName(qname)) => {
                self.qname = Some(qname);
                Descend
            }

            AllNamed::Identifier(identifier)
            | AllNamed::Expr(ink_syntax::Expr::Identifier(identifier)) => {
                if self.ignore.is_some_and(|it| it == identifier) {
                    self.ignore = None; // OK, we've ignored it now.
                    return Ignore;
                }
                let text = if let Some(qname) = self.qname {
                    self.text_between(qname.start_byte(), identifier.end_byte())
                } else {
                    self.text(identifier)
                };
                let body = state.current_body();
                body.usages.register(text, identifier);
                body.names.insert(identifier.into(), text);
                Ignore
            }

            AllNamed::Include(_) => Ignore,

            /*** Unused ***/
            AllNamed::AltArm(_) => Descend,
            AllNamed::Alternatives(_) => Descend,
            AllNamed::Assignment(_) => Descend,
            AllNamed::Binary(_) => Descend,
            AllNamed::BlockComment(_) => Ignore,
            AllNamed::Boolean(_) => Ignore,
            AllNamed::ChoiceBlock(_) => Descend,
            AllNamed::Choice(_) => Descend,
            AllNamed::ChoiceMark(_) => Ignore,
            AllNamed::ChoiceMarks(_) => Ignore,
            AllNamed::ChoiceOnly(_) => Descend,
            AllNamed::Code(_) => Descend,
            AllNamed::CondArm(_) => Descend,
            AllNamed::CondBlock(_) => Descend,
            AllNamed::ConditionalText(_) => Descend,
            AllNamed::Condition(_) => Descend,
            AllNamed::Content(_) => Descend,
            AllNamed::Else(_) => Ignore,
            AllNamed::Eol(_) => Ignore,
            AllNamed::Eval(_) => Descend,
            AllNamed::Expr(_) => Descend,
            AllNamed::GatherBlock(_) => Descend,
            AllNamed::Gather(_) => Descend,
            AllNamed::GatherMark(_) => Ignore,
            AllNamed::GatherMarks(_) => Ignore,
            AllNamed::Glue(_) => Ignore,
            AllNamed::Ink(_) => Descend,
            AllNamed::LineComment(_) => Ignore,
            AllNamed::MultilineAlternatives(_) => Descend,
            AllNamed::Number(_) => Ignore,
            AllNamed::Paragraph(_) => Descend,
            AllNamed::Params(_) => Descend,
            AllNamed::Paren(_) => Descend,
            AllNamed::Path(_) => Ignore,
            AllNamed::Postfix(_) => Descend,
            AllNamed::Return(_) => Descend,
            AllNamed::String(_) => Descend, // because String interpolation/evaluation
            AllNamed::Tag(_) => Descend,
            AllNamed::Text(_) => Ignore,
            AllNamed::TodoComment(_) => Ignore,
            AllNamed::Unary(_) => Descend,
        }
    }

    fn leave(&mut self, node: AllNamed<'a>, _: &mut Self::State) {
        match node {
            AllNamed::QualifiedName(_) => self.qname = None,
            _ => {}
        }
    }

    fn combine(_: &mut Self::State, _: Self::State) {
        // SAFETY: We never DescendWith, therefore we never combine.
        unsafe { std::hint::unreachable_unchecked() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn try_it() {
        let doc = InkDocument::new(
            indoc! {"
                VAR x = 3
                CONST y = x
                LIST lst = arg, (barg), zarg = 3

                -> knot(4)

                - (toblwl) Hi!

                = toplevel_stitch(p1, p2)
                Hey, I'm using {x}.

                == knot(ref this) ==
                ~ temp horp = this + 3
                I'm using {y}
                - (notlabel) Ho!

                = stitch(-> return_to)
                I'm using {y}
                - (stirtchlbl) Hoi!
                ~ temp hirp = \"Hey {you}! {y} are you doing this?!\"
                -> return_to
            "}
            .into(),
            None,
        );

        let result = traverse(&doc);
        panic!("{result:#?}");
    }
}
