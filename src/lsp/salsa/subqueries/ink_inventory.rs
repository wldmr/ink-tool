use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::BuildHasherDefault,
};

use derive_more::derive::{AsRef, Debug, Display, Into};
use ink_document::{
    ids::{DefId, ScopeId, UsageId},
    InkDocument,
};
use ink_syntax::AllNamed;
use mini_milc::subquery;
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

impl Name {
    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }
}

impl<T: AsRef<str>> From<T> for Name {
    fn from(value: T) -> Self {
        Name(ustr(value.as_ref()))
    }
}

impl Into<&'static str> for Name {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

impl Into<Cow<'static, str>> for Name {
    fn into(self) -> Cow<'static, str> {
        Cow::Borrowed(self.as_str())
    }
}

impl Into<String> for Name {
    fn into(self) -> String {
        self.0.to_string()
    }
}

pub type ISet<T> = HashSet<T, BuildHasherDefault<IdentityHasher>>;
pub type NameSet = ISet<Name>;
pub type IMap<K, V> = HashMap<K, V, BuildHasherDefault<IdentityHasher>>;
pub type NameMap<T> = IMap<Name, T>;

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

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_ref())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Body {
    pub temps: NameMap<Vec1<DefId>>,
    pub labels: NameMap<Vec1<DefId>>,
    pub usages: NameMap<Vec1<UsageId>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub name: Name,
    pub name_id: DefId,
    pub scope_id: ScopeId,
    pub params: NameMap<Vec1<DefId>>,
    pub body: Body,
    pub subsections: Vec<Subsection>,
    /// Collection of all labels defined in subsections.
    /// The values of the map are the subsection names that the keys are contained in.
    pub sub_labels: NameSet,
    /// Collection of all subsection names in here
    pub sub_names: NameSet,
    pub subscopes: HashMap<ScopeId, usize, BuildHasherDefault<IdentityHasher>>,
}

impl Section {
    fn new(name: Name, name_id: impl Into<DefId>, section_id: impl Into<ScopeId>) -> Self {
        Self {
            name,
            name_id: name_id.into(),
            scope_id: section_id.into(),
            params: Default::default(),
            body: Default::default(),
            subsections: Default::default(),
            sub_names: Default::default(),
            sub_labels: Default::default(),
            subscopes: Default::default(),
        }
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subsection {
    pub name: Name,
    pub name_id: DefId,
    pub scope_id: ScopeId,
    pub params: NameMap<Vec1<DefId>>,
    pub body: Body,
}

impl Subsection {
    pub fn new(name: Name, name_id: impl Into<DefId>, scope_id: impl Into<ScopeId>) -> Self {
        Self {
            name,
            name_id: name_id.into(),
            scope_id: scope_id.into(),
            params: Default::default(),
            body: Default::default(),
        }
    }
}

impl Display for Subsection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_ref())
    }
}

struct Vstr<'a> {
    doc: &'a InkDocument,
    /// When walking through a qualified name, we need to keep track of its start, so
    /// that we can properly make substrings.
    qname: Option<ink_syntax::QualifiedName<'a>>,
    scope: ScopeId,
}

impl<'a> Vstr<'a> {
    fn new(doc: &'a InkDocument) -> Self {
        Self {
            doc,
            qname: None,
            scope: doc.root().into(),
        }
    }

    fn text<N: Node<'a>>(&self, node: N) -> Name {
        Name(ustr(self.doc.node_text(node)))
    }

    fn text_between(&self, start: usize, end: usize) -> Name {
        Name(ustr(self.doc.text(start..end)))
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

    fn current_params(&mut self) -> Option<&mut NameMap<Vec1<DefId>>> {
        if let Some(section) = self.sections.last_mut() {
            if let Some(sub) = section.subsections.last_mut() {
                Some(&mut sub.params)
            } else {
                Some(&mut section.params)
            }
        } else {
            None
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
                let section = Section::new(self.text(knot.name()), knot, self.scope);
                state.sections.push(section);
                Descend
            }

            AllNamed::Stitch(stitch) => {
                if let Some(parent) = state.sections.last_mut() {
                    let sub = Subsection::new(self.text(stitch.name()), stitch, self.scope);
                    parent.sub_names.insert(sub.name);
                    let scope_index = parent.subsections.len(); // len before pushing == index of element we're about to push
                    parent.subscopes.insert(sub.scope_id, scope_index);
                    parent.subsections.push(sub);
                } else {
                    let stitch = Section::new(self.text(stitch.name()), stitch, self.scope);
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
                Descend
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
                // Reminder: We never enter Params for Externals, so this is fine.
                if let Some(params) = state.current_params() {
                    params.register(name, param);
                }
                Descend
            }

            AllNamed::TempDef(temp) => {
                let text = self.text(temp.name());
                state.current_body().temps.register(text, temp);
                Descend
            }

            /*** Globals ***/
            AllNamed::Global(global) => {
                let name = self.text(global.name());
                if global.keyword().is_ok_and(|it| it.as_const().is_some()) {
                    state.consts.register(name, global)
                } else {
                    state.vars.register(name, global)
                };
                Descend
            }

            AllNamed::List(list) => {
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
                if let Some(last) = state.lists.last_mut() {
                    // Technically it’s an error to not find a list here, but let’s not kick up a fuss.
                    last.items.register(name, def);
                };
                Descend
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
                let start = if let Some(qname) = self.qname {
                    qname.start_byte()
                } else {
                    identifier.start_byte()
                };
                let end = identifier.end_byte();
                let text = self.text_between(start, end);
                state.current_body().usages.register(text, identifier);
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
            AllNamed::QualifiedName(_) | AllNamed::Expr(ink_syntax::Expr::QualifiedName(_)) => {
                self.qname = None
            }
            _ => {}
        }
    }

    fn combine(_: &mut Self::State, _: Self::State) {
        // SAFETY: We never DescendWith, therefore we never combine.
        unsafe { std::hint::unreachable_unchecked() }
    }
}
