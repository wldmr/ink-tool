use ink_document::{ids::NodeId, InkDocument};
use ink_syntax::AllNamed;
use lsp_types::{Range, Uri};
use std::collections::HashMap;
use tap::TapOptional;
use tree_traversal::{VisitInstruction, Visitor};
use type_sitter::Node;
use util::{nonempty::Vec1, testing::Compact};

pub fn document_definitions(uri: &Uri, doc: &InkDocument) -> Defs {
    DefinitionVisitor::new(uri, doc).traverse(doc.root())
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Defs {
    locals: HashMap<NodeId, NameMapping>,
    globals: NameMapping,
}
pub type NameMapping = HashMap<String, Vec1<(Range, DefKind)>>;

impl Defs {
    pub fn local(
        &self,
        term: &str,
        block: ink_syntax::ScopeBlock,
    ) -> Option<&Vec1<(Range, DefKind)>> {
        self.locals
            .get(&NodeId::new(block))
            .and_then(|it| it.get(term))
    }

    pub fn global(&self, term: &str) -> Option<&Vec1<(Range, DefKind)>> {
        self.globals.get(term)
    }
}

// private helpers
impl Defs {
    fn add_local(
        &mut self,
        kind: DefKind,
        range: Range,
        parent: impl Into<NodeId>,
        name: impl Into<String>,
    ) {
        Self::add_name(
            self.locals.entry(parent.into()).or_default(),
            name,
            range,
            kind,
        );
    }

    fn add_global(&mut self, kind: DefKind, range: Range, name: impl Into<String>) {
        Self::add_name(&mut self.globals, name, range, kind);
    }

    fn add_name(names: &mut NameMapping, name: impl Into<String>, range: Range, kind: DefKind) {
        let item = (range, kind);
        let entry = names.entry(name.into());
        match entry {
            std::collections::hash_map::Entry::Occupied(mut occupied) => {
                occupied.get_mut().push(item);
            }
            std::collections::hash_map::Entry::Vacant(vacant) => {
                vacant.insert(Vec1::new(item));
            }
        };
    }
}

/// The kinds of things we can give names to in Ink
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefKind {
    Knot,
    Stitch,
    Label,
    List,
    ListItem,
    Function,
    External,
    Param,
    Var,
    Const,
    Temp,
}

impl DefKind {
    pub fn is_temp(&self) -> bool {
        matches!(self, DefKind::Temp)
    }
}

struct DefinitionVisitor<'a> {
    uri: &'a Uri,
    doc: &'a InkDocument,
    knot: Option<Context<'a>>,
    stitch: Option<Context<'a>>,
    list: Option<Context<'a>>,
}

/// Crawls a document, collects all names and where they are defined
impl<'a> DefinitionVisitor<'a> {
    pub fn new(uri: &'a Uri, doc: &'a InkDocument) -> Self {
        Self {
            uri,
            doc,
            knot: None,
            stitch: None,
            list: None,
        }
    }

    fn section(&self) -> Option<Context<'_>> {
        self.stitch.or(self.knot)
    }

    fn err(&self, msg: &str, node: impl Node<'a>) {
        log::error!(
            "{msg}: {}:{}:{}–{}:{} ({})",
            self.uri.path().as_str(),
            node.range().start_point.row + 1,
            node.range().start_point.column + 1,
            node.range().end_point.row + 1,
            node.range().end_point.column + 1,
            node.kind()
        )
    }
}

/// Keep track of the current parent node and its associated name.

/*
We use a custom type so that we can ad add conversion trait impls, which enables
us to use this directly in places where we need the NodeId or the name, without
having to pattern match first.

Note how below we can give the context to to the `add_*` functions which
extracts the NodeId, but also in the `format!(…)` invocations to generate the
qualified names.

Yes, this may be a bit too “cute”, but it just makes the code in the visitor so
much easier to read.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Context<'a>(NodeId, &'a str);
impl<'a> Context<'a> {
    fn new(node: impl Node<'a>, name: &'a str) -> Self {
        Self(NodeId::new(node), name)
    }
}
impl<'a> Into<NodeId> for Context<'a> {
    fn into(self) -> NodeId {
        self.0
    }
}
impl<'a> std::fmt::Display for Context<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.1)
    }
}

impl<'a> Visitor<'a, AllNamed<'a>> for DefinitionVisitor<'a> {
    type State = Defs;

    fn visit(
        &mut self,
        node: AllNamed<'a>,
        defs: &mut Self::State,
    ) -> VisitInstruction<Self::State> {
        use AllNamed::*;
        use VisitInstruction::{Descend, Ignore};
        match node {
            Ink(_) => Descend,

            KnotBlock(block) => {
                self.knot = Some(Context::new(block, "ERR")); // will update actual name when we get to the knot
                self.stitch = None;
                Descend
            }

            StitchBlock(block) => {
                self.stitch = Some(Context::new(block, "ERR")); // ditto
                Descend
            }

            Knot(knot) => {
                if let Ok(ident) = knot.name() {
                    let name = self.text(ident);
                    let site = self.site(ident);
                    self.knot
                        .as_mut()
                        .map(|it| it.1 = name)
                        .tap_none(|| self.err("Found section but no parent scope", knot));
                    let kind = if knot.function().is_some() {
                        DefKind::Function
                    } else {
                        DefKind::Knot
                    };
                    defs.add_global(kind, site, name);
                };
                if knot.params().is_some() {
                    Descend
                } else {
                    Ignore
                }
            }

            Stitch(stitch) => {
                if let Ok(ident) = stitch.name() {
                    const KIND: DefKind = DefKind::Stitch;
                    let name = self.text(ident);
                    let site = self.site(ident);
                    self.stitch
                        .as_mut()
                        .map(|it| it.1 = name)
                        .tap_none(|| self.err("Found section but no parent scope", stitch));
                    if let Some(knot) = self.knot {
                        defs.add_local(KIND, site, knot, name);
                        defs.add_global(KIND, site, format!("{knot}.{name}"));
                    } else {
                        defs.add_global(KIND, site, name);
                    }
                }
                if stitch.params().is_some() {
                    Descend
                } else {
                    Ignore
                }
            }

            Label(label) => {
                const KIND: DefKind = DefKind::Label;
                if let Ok(ident) = label.name() {
                    let label = self.text(ident);
                    let site = self.site(ident);
                    match (self.knot, self.stitch) {
                        // Global label
                        (None, None) => defs.add_global(KIND, site, label),

                        // Singly nested label (toplevel stitch and toplevel knot work the same)
                        (None, Some(parent)) | (Some(parent), None) => {
                            defs.add_local(KIND, site, parent, label);
                            defs.add_global(KIND, site, format!("{parent}.{label}"));
                        }

                        // Doubly nested label has many different names in different contexts:
                        (Some(knot), Some(stitch)) => {
                            defs.add_global(KIND, site, format!("{knot}.{label}"));
                            defs.add_global(KIND, site, format!("{knot}.{stitch}.{label}"));

                            defs.add_local(KIND, site, knot, label);
                            defs.add_local(KIND, site, knot, format!("{stitch}.{label}"));

                            defs.add_local(KIND, site, stitch, label);
                        }
                    }
                }
                Ignore
            }

            Param(param) => {
                use ink_syntax::ParamValue::*;
                let ident = param
                    .value()
                    .ok()
                    .map(|param| match param {
                        Divert(divert) => divert.target().ok().and_then(|it| it.as_identifier()),
                        Identifier(identifier) => Some(identifier),
                    })
                    .flatten();
                if let Some(ident) = ident {
                    let block = self.section().map(|it| it.0).unwrap_or_else(|| {
                        self.err("Found parameter but no section/function", param);
                        NodeId::new(self.doc.root())
                    });
                    defs.add_local(DefKind::Param, self.site(ident), block, self.text(ident));
                }
                Ignore
            }

            TempDef(temp) => {
                if let Ok(ident) = temp.name() {
                    let block = self.section().map(Into::into).unwrap_or_else(|| {
                        self.err("Found temp but no section", temp);
                        NodeId::new(self.doc.root())
                    });
                    defs.add_local(DefKind::Temp, self.site(ident), block, self.text(ident));
                }
                Ignore
            }

            List(list) => {
                if let Ok(ident) = list.name() {
                    let name = self.text(ident);
                    self.list = Some(Context::new(list, name));
                    defs.add_global(DefKind::List, self.site(ident), name);
                    Descend
                } else {
                    Ignore
                }
            }
            ListValueDefs(_) => Descend,
            ListValueDef(def) => {
                if let Ok(ident) = def.name() {
                    let list = self.list.map(|it| it.1).unwrap_or_else(|| {
                        self.err("Found list item without parent list", def);
                        "ERR"
                    });
                    let site = self.site(ident);
                    let item = self.text(ident);
                    defs.add_global(DefKind::ListItem, site, format!("{item}"));
                    defs.add_global(DefKind::ListItem, site, format!("{list}.{item}"));
                }
                Ignore
            }

            Global(global) => {
                if let Ok(ident) = global.name() {
                    let name = self.text(ident);
                    let site = self.site(ident);
                    let is_const = global.keyword().ok().and_then(|it| it.as_const()).is_some();
                    let kind = if is_const {
                        DefKind::Const
                    } else {
                        DefKind::Var
                    };
                    defs.add_global(kind, site, name);
                };
                Ignore
            }

            External(external) => {
                if let Ok(ident) = external.name() {
                    let name = self.text(ident);
                    let site = self.site(ident);
                    defs.add_global(DefKind::External, site, name);
                };
                Ignore // Arguments for externals aren't actually used anywhere
            }

            // These can contain locals:
            AltArm(_) => Descend, // Might contain temps, annoyingly
            Choice(_) => Descend,
            ChoiceBlock(_) => Descend,
            Code(_) => Descend, // might contain temp
            CondBlock(_) => Descend,
            Gather(_) => Descend,
            GatherBlock(_) => Descend,
            MultilineAlternatives(_) => Descend,
            Params(_) => Descend,

            // These can't contain definitions
            Alternatives(_) | Args(_) | Assignment(_) | Binary(_) | BlockComment(_)
            | Boolean(_) | Call(_) | ChoiceMark(_) | ChoiceMarks(_) | ChoiceOnly(_)
            | CondArm(_) | Condition(_) | ConditionalText(_) | Content(_) | Divert(_) | Else(_)
            | Eol(_) | Eval(_) | Expr(_) | GatherMark(_) | GatherMarks(_) | Glue(_)
            | Identifier(_) | Include(_) | LineComment(_) | ListValues(_) | Number(_)
            | Paragraph(_) | Paren(_) | Path(_) | Postfix(_) | QualifiedName(_) | Return(_)
            | String(_) | Tag(_) | Text(_) | Thread(_) | TodoComment(_) | Tunnel(_) | Unary(_) => {
                Ignore
            }
        }
    }

    fn visit_error(&mut self, err: type_sitter::IncorrectKind) -> VisitInstruction<Self::State> {
        match err.cause() {
            type_sitter::IncorrectKindCause::Error => VisitInstruction::Descend,
            type_sitter::IncorrectKindCause::Missing => VisitInstruction::Ignore,
            type_sitter::IncorrectKindCause::OtherKind(_) => VisitInstruction::Descend,
        }
    }

    fn combine(_: &mut Self::State, _: Self::State) {
        unreachable!("We don't have sub-states")
    }
}

/// Private Helpers
impl<'a> DefinitionVisitor<'a> {
    fn text<N: Node<'a>>(&self, n: N) -> &'a str {
        self.doc.node_text(n)
    }

    fn site<N: Node<'a>>(&self, n: N) -> lsp_types::Range {
        self.doc.lsp_range(n.range())
    }
}
