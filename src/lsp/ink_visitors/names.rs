use std::ops::Deref;

use ink_document::{
    ids::{self, DefinitionInfo, NodeId},
    InkDocument,
};
use ink_syntax::{AllNamed, ScopeBlock};
use lsp_types::{Position, Range};
use tree_traversal::{VisitInstruction, Visitor};
use type_sitter::Node;

pub type Name = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meta {
    pub id: ids::Definition,
    pub site: Range,
    pub extent: Option<Range>,
}

pub fn document_names(doc: &impl Deref<Target = InkDocument>) -> Vec<(Name, Meta)> {
    Names::new(doc.deref()).traverse(doc.root())
}

impl Meta {
    pub fn is_locally_visible_at(&self, pos: Position) -> bool {
        match self.extent {
            Some(Range { start, end }) => start <= pos && pos <= end,
            None => false,
        }
    }

    pub fn is_global(&self) -> bool {
        self.extent.is_none()
    }

    pub fn cmp_extent(&self, other: &Meta) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match (self.extent, other.extent) {
            (Some(here), Some(there)) => {
                let here_lines = here.end.line - here.start.line;
                let there_lines = there.end.line - there.start.line;
                // this is a little fragile as it depends on the fact that
                // scopes are tied to lines and are strictly nested!
                here_lines.cmp(&there_lines)
            }
            (None, None) => Equal,
            (None, Some(_)) => Greater, // self extent is global, i.e. "greater"
            (Some(_), None) => Less,
        }
    }
}

struct Environment {
    name: String,
    nodeid: NodeId,
    extent: Range,
    temp_extent: Range,
}

struct Names<'a> {
    doc: &'a InkDocument,
    ink_temp_extent: Option<Range>,
    knot: Option<Environment>,
    stitch: Option<Environment>,
    list: Option<Environment>,
}

impl<'a> Names<'a> {
    pub fn new(doc: &'a InkDocument) -> Self {
        Self {
            doc,
            ink_temp_extent: None,
            knot: None,
            stitch: None,
            list: None,
        }
    }
}

impl<'a> Visitor<'a, AllNamed<'a>> for Names<'a> {
    type State = Vec<(Name, Meta)>;

    fn visit(
        &mut self,
        node: AllNamed<'a>,
        names: &mut Self::State,
    ) -> VisitInstruction<Self::State> {
        use AllNamed::*;
        use VisitInstruction::{Descend, Ignore};
        match node {
            AltArm(_)
            | Choice(_)
            | ChoiceBlock(_)
            | Code(_)
            | CondArm(_)
            | CondBlock(_)
            | Content(_)
            | Gather(_)
            | GatherBlock(_)
            | Knot(_)
            | ListValueDefs(_)
            | MultilineAlternatives(_)
            | Paragraph(_)
            | Params(_)
            | Stitch(_) => Descend,

            Global(def) => {
                names.push(self.global(
                    def.name(),
                    match def.keyword().ok() {
                        Some(kw) if kw.as_const().is_some() => DefinitionInfo::Const,
                        _ => DefinitionInfo::Var,
                    },
                ));
                Ignore
            }

            List(list) => {
                names.push(self.global(list.name(), DefinitionInfo::List));
                let extent = self.lsp_range(list);
                self.list = Some(Environment {
                    nodeid: NodeId::new(list.name()),
                    name: self.text(list.name()),
                    extent,
                    temp_extent: extent, // doesn't really make sense, but we don't want to define a new environment type just for lists.
                });
                // Ideally we’d unset this when we leave the definition, but we shouldn’t access
                // this field without first coming through here and setting it to the correct
                // value.
                Descend
            }
            ListValueDef(def) => {
                let item = self.text(def.name());
                let site = def.name();
                let list = self.list.as_ref().expect("must have been set");
                names.extend([
                    self.name(
                        format!("{item}"),
                        site,
                        None,
                        DefinitionInfo::ListItem { list: list.nodeid },
                    ), // Yes, you read that right.
                    self.name(
                        format!("{}.{item}", list.name),
                        site,
                        None,
                        DefinitionInfo::List,
                    ),
                ]);
                Ignore
            }

            Ink(ink) => {
                // Annoyingly, we need to determine the temp scope ourselves, when it should be its
                // own node. See <https://github.com/wldmr/tree-sitter-ink/issues/12>
                let (end_byte, end_position) = ink
                    .raw()
                    .children(&mut ink.raw().walk())
                    .find_map(|it| ScopeBlock::try_from_raw(it).ok())
                    .map(|it| (it.start_byte(), it.start_position()))
                    .unwrap_or_else(|| (ink.end_byte(), ink.end_position()));
                let temp_extent = self.lsp_range_between(
                    ink.start_byte(),
                    end_byte,
                    ink.start_position(),
                    end_position,
                );
                self.ink_temp_extent = Some(temp_extent);
                Descend
            }

            KnotBlock(block) => {
                let name = block.header().map(|it| it.name());
                self.stitch = None;
                self.knot = Some(Environment {
                    nodeid: NodeId::new(name),
                    name: self.text(name),
                    extent: self.lsp_range(block),
                    temp_extent: {
                        let (end_byte, end_position) = block
                            .others(&mut block.walk())
                            .filter_map(Result::ok)
                            .find_map(|it| it.as_stitch_block())
                            .map(|it| (it.start_byte(), it.start_position()))
                            .unwrap_or_else(|| (block.end_byte(), block.end_position()));
                        self.lsp_range_between(
                            block.start_byte(),
                            end_byte,
                            block.start_position(),
                            end_position,
                        )
                    },
                });

                let is_function = block.header().is_ok_and(|it| it.function().is_some());
                let name = if is_function {
                    self.global(name, DefinitionInfo::Function)
                } else {
                    self.global(
                        name,
                        DefinitionInfo::ToplevelScope {
                            stitch: false,
                            params: block
                                .header()
                                .map(|header| header.params().is_some())
                                .unwrap_or_default(),
                        },
                    )
                };
                names.push(name);
                Descend
            }

            StitchBlock(block) => {
                let name = block.header().map(|it| it.name());
                let extent = self.lsp_range(block);

                self.stitch = Some(Environment {
                    nodeid: NodeId::new(name),
                    name: block
                        .header()
                        .map(|it| self.text(it.name()))
                        .unwrap_or("ERROR".into()),
                    extent,
                    temp_extent: extent, // Stitches can't have subsections
                });

                let kind = DefinitionInfo::SubScope {
                    parent: self.knot.as_ref().map(|it| it.nodeid),
                    params: block
                        .header()
                        .map(|it| it.params().is_some())
                        .unwrap_or_default(),
                };
                if let Some(knot) = &self.knot {
                    let k = &knot.name;
                    let s = self.text(name);
                    let extent = Some(knot.extent);
                    names.extend([
                        self.name(format!("{s}"), name, extent, kind),
                        self.name(format!("{k}.{s}"), name, None, kind),
                    ]);
                } else {
                    names.push(self.global(name, kind));
                }
                Descend
            }

            Label(label) => {
                let name = label.name();
                let l = self.text(name);
                let kind = DefinitionInfo::Label;
                match (&self.knot, &self.stitch) {
                    (None, None) => names.push(self.global(name, kind)),
                    (None, Some(stitch)) => {
                        let s = &stitch.name;
                        names.extend([
                            self.local(name, stitch.extent, kind),
                            self.name(format!("{s}.{l}"), name, Some(stitch.extent), kind),
                        ]);
                    }
                    (Some(knot), None) => {
                        let k = &knot.name;
                        names.extend([
                            self.local(name, knot.extent, kind),
                            self.name(format!("{k}.{l}"), name, None, kind),
                        ]);
                    }
                    (Some(knot), Some(stitch)) => {
                        // This is where it gets confusing. Labels are *uniquely* namespaced by their knot
                        // name, but allow for an optional stitch name.
                        let k = &knot.name;
                        let s = &stitch.name;
                        let l = &self.text(name);
                        names.extend([
                            self.name(format!("{k}.{l}"), name, None, kind),
                            self.name(format!("{k}.{s}.{l}"), name, None, kind),
                            self.name(format!("{l}"), name, Some(knot.extent), kind),
                            self.name(format!("{s}.{l}"), name, Some(knot.extent), kind),
                        ]);
                    }
                }
                Ignore
            }

            Param(param) => {
                use ink_syntax::ParamValue::*;
                let name = param.value().map(|it| match it {
                    Divert(divert) => divert.target().upcast(),
                    Identifier(identifier) => identifier.upcast(),
                });
                let extent = match (&self.knot, &self.stitch) {
                    (_, Some(stitch)) => stitch.extent,
                    (Some(knot), _) => knot.extent,
                    (None, None) => unreachable!("Must be inside block to find param"),
                };
                names.push(
                    self.local(
                        name,
                        extent,
                        DefinitionInfo::Param {
                            is_ref: param.r#ref().is_some(),
                            is_divert: param
                                .value()
                                .map(|it| it.as_divert().is_some())
                                .unwrap_or_default(),
                        },
                    ),
                );
                Ignore
            }

            TempDef(temp) => {
                let extent = match (&self.knot, &self.stitch) {
                    (None, None) => self.ink_temp_extent.expect("must have been set"),
                    (_, Some(stitch)) => stitch.temp_extent,
                    (Some(knot), _) => knot.temp_extent,
                };
                names.push(self.local(temp.name(), extent, DefinitionInfo::Temp));
                Ignore
            }

            External(external) => {
                names.push(self.global(external.name(), DefinitionInfo::External));
                Ignore // Doesn't have body, so we don't need to define names for them
            }

            // … and the rest …
            Alternatives(_) | Args(_) | Assignment(_) | Binary(_) | BlockComment(_)
            | Boolean(_) | Call(_) | ChoiceMark(_) | ChoiceMarks(_) | ChoiceOnly(_)
            | ConditionalText(_) | Condition(_) | Divert(_) | Else(_) | Eol(_) | Eval(_)
            | Expr(_) | GatherMark(_) | GatherMarks(_) | Glue(_) | Identifier(_) | Include(_)
            | LineComment(_) | ListValues(_) | Number(_) | Paren(_) | Path(_) | Postfix(_)
            | QualifiedName(_) | Return(_) | String(_) | Tag(_) | Text(_) | Thread(_)
            | TodoComment(_) | Tunnel(_) | Unary(_) => Ignore,
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
        // parent.append(&mut children); // More of a failsafe; we shouldn't actuall nest these.
        unreachable!("We don't have sub-states")
    }
}

/// Private Helpers
impl<'a> Names<'a> {
    fn text<N: Node<'a>>(&self, n: N) -> String {
        self.doc.node_text(n).to_owned()
    }

    fn lsp_range<N: Node<'a>>(&self, n: N) -> lsp_types::Range {
        self.doc.lsp_range(n.range())
    }

    fn lsp_range_between(
        &self,
        start_byte: usize,
        end_byte: usize,
        start_point: tree_sitter::Point,
        end_point: tree_sitter::Point,
    ) -> lsp_types::Range {
        self.doc.lsp_range(tree_sitter::Range {
            start_byte,
            end_byte,
            start_point,
            end_point,
        })
    }

    fn global<N: Node<'a>>(&self, n: N, kind: DefinitionInfo) -> (Name, Meta) {
        self.name(self.text(n), n, None, kind)
    }

    fn local<N: Node<'a>>(&self, n: N, extent: Range, kind: DefinitionInfo) -> (Name, Meta) {
        self.name(self.text(n), n, Some(extent), kind)
    }

    fn name(
        &self,
        name: String,
        site: impl type_sitter::Node<'a>,
        extent: Option<Range>,
        info: DefinitionInfo,
    ) -> (Name, Meta) {
        (
            name,
            Meta {
                id: ids::Definition::new(site, info),
                extent: extent,
                site: self.lsp_range(site),
            },
        )
    }
}
