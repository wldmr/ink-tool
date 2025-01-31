use crate::{
    ink_syntax::{self, types::AllNamed, Visit, VisitInstruction, Visitor},
    lsp::links::Links,
};
use type_sitter_lib::IncorrectKindCause;

pub struct LinkVisitor<'a> {
    text: &'a str,
    current_list: Option<&'a str>,
    current_knot: Option<&'a str>,
    current_stitch: Option<&'a str>,
    links: Links<'a, Scoped<tree_sitter::Node<'a>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Scoped<T> {
    Global(T),
    WithinKnot(T),
    WithinStitch(T),
    Temp(T),
    /// A reference to a definition.
    Usage(T),
}

impl<'a> Scoped<tree_sitter::Node<'a>> {
    fn global(node: impl type_sitter_lib::Node<'a>) -> Self {
        Self::Global(node.into_raw())
    }
    fn within_knot(node: impl type_sitter_lib::Node<'a>) -> Self {
        Self::WithinKnot(node.into_raw())
    }
    fn within_stitch(node: impl type_sitter_lib::Node<'a>) -> Self {
        Self::WithinStitch(node.into_raw())
    }
    fn temp(node: impl type_sitter_lib::Node<'a>) -> Self {
        Self::Temp(node.into_raw())
    }
    fn usage(node: impl type_sitter_lib::Node<'a>) -> Self {
        Self::Usage(node.into_raw())
    }
}

impl<'a> LinkVisitor<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            current_list: Default::default(),
            current_knot: Default::default(),
            current_stitch: Default::default(),
            links: Default::default(),
        }
    }

    pub fn into_links(mut self) -> Links<'a, tree_sitter::Node<'a>> {
        self.links.resolve();
        self.links.transform_locations(|it| match it {
            Scoped::Global(it)
            | Scoped::WithinKnot(it)
            | Scoped::WithinStitch(it)
            | Scoped::Temp(it)
            | Scoped::Usage(it) => it,
        })
    }

    fn text(&self, node: impl type_sitter_lib::Node<'a>) -> &'a str {
        &self.text[node.byte_range()]
    }
    fn reference_expr(&mut self, expr: ink_syntax::types::Expr<'a>) -> VisitInstruction<Self> {
        if let Some(qname) = expr.as_qualified_name() {
            // TODO: resolve leading parts?
            self.links.reference(self.text(qname), Scoped::usage(qname));
            // ... if we descend, then each name will be resolved individually, which will generate false positives
            VisitInstruction::Ignore
        } else if let Some(ident) = expr.as_identifier() {
            // TODO: resolve leading parts?
            self.links.reference(self.text(ident), Scoped::usage(ident));
            VisitInstruction::Ignore
        } else {
            VisitInstruction::Descend
        }
    }
}

impl<'a> Visitor<'a, AllNamed<'a>> for LinkVisitor<'a> {
    fn voyage(&mut self, visit: Visit<AllNamed<'a>>) -> VisitInstruction<Self> {
        use ink_syntax::types::AllNamed::*;
        use Visit::*;
        use VisitInstruction::*;
        match visit {
            // Definitions
            Enter(TempDef(node)) => {
                let name = self.text(node.name());
                self.links.provide(name, Scoped::temp(node.name()));
                Ignore
            }

            Enter(External(node)) => {
                let name = self.text(node.name());
                self.links.provide(name, Scoped::global(node.name()));
                Ignore // can't have a body, therefore
            }

            Enter(Global(node)) => {
                let name = self.text(node.name());
                self.links.provide(name, Scoped::global(node.name()));
                Ignore
            }

            Enter(List(node)) => {
                let name = self.text(node.name());
                self.links.provide(name, Scoped::global(node.name()));
                self.current_list = Some(name);
                Descend
            }
            Leave(List(_)) => {
                self.current_list = None;
                Ignore
            }
            Enter(ListValueDef(node)) => {
                let list = self
                    .current_list
                    .expect("list items can only be defined inside a list node");
                let name = self.text(node.name());
                // List items have two global names:
                let global_def = Scoped::global(node.name());
                self.links.provide(name, global_def); // naked
                self.links.provide(format!("{list}.{name}"), global_def); // qualified
                Ignore
            }

            Enter(Ink(_)) | Enter(KnotBlock(_)) | Enter(StitchBlock(_)) => {
                // We don't really have a node for temp scope in the tree-sitter syntax,
                // so we resolve, then clear the temps at the _start_ of the scope where the next temps starts.
                self.links.resolve();
                self.links
                    .unprovide(|_name, def| matches!(*def, Scoped::Temp(_)));
                Descend
            }

            Enter(Knot(node)) => {
                let name = self.text(node.name());
                self.links.provide(name, Scoped::global(node.name()));
                self.current_knot = Some(name);
                self.current_stitch = None;
                Descend // collect params
            }
            Leave(KnotBlock(_)) => {
                self.links.resolve();
                self.links
                    .unprovide(|_name, def| matches!(*def, Scoped::WithinKnot(_)));
                self.current_knot = None;
                Ignore
            }

            Enter(Stitch(node)) => {
                let name_node = node.name();
                let name = self.text(name_node);
                if let Some(knot) = self.current_knot {
                    self.links
                        .provide(format!("{name}"), Scoped::within_knot(name_node));
                    self.links
                        .provide(format!("{knot}.{name}"), Scoped::global(name_node));
                } else {
                    self.links
                        .provide(format!("{name}"), Scoped::global(name_node));
                }
                self.current_stitch = Some(name);
                Descend // collect params
            }
            Leave(StitchBlock(_)) => {
                self.links.resolve();
                self.links
                    .unprovide(|_name, def| matches!(*def, Scoped::WithinStitch(_)));
                self.current_stitch = None;
                Ignore
            }

            Enter(Param(node)) => {
                let param_value = node.value();
                let name = self.text(param_value);

                if self.current_stitch.is_some() {
                    self.links.provide(name, Scoped::within_stitch(param_value));
                } else if self.current_knot.is_some() {
                    self.links.provide(name, Scoped::within_knot(param_value));
                } else {
                    unreachable!("Param should only exist within Knots or Stitches");
                };
                Ignore
            }

            Enter(Label(node)) => {
                let name_node = node.name();
                let label = self.text(node);
                let links = &mut self.links;
                match (self.current_knot, self.current_stitch) {
                    (None, None) => {
                        links.provide(format!("{label}"), Scoped::global(name_node));
                    }
                    (Some(knot), None) => {
                        links.provide(format!("{label}"), Scoped::within_knot(name_node));
                        links.provide(format!("{knot}.{label}"), Scoped::global(name_node));
                    }
                    (None, Some(stitch)) => {
                        links.provide(format!("{label}"), Scoped::within_stitch(name_node));
                        links.provide(format!("{stitch}.{label}"), Scoped::global(name_node));
                    }
                    (Some(knot), Some(stitch)) => {
                        links.provide(format!("{label}"), Scoped::within_stitch(name_node));
                        links.provide(format!("{stitch}.{label}"), Scoped::within_knot(name_node));
                        // this is the weird one; labels are uniqe per Knot, even within nested stitches:
                        links.provide(format!("{knot}.{label}"), Scoped::global(name_node));
                        links.provide(
                            format!("{knot}.{stitch}.{label}"),
                            Scoped::global(name_node),
                        );
                    }
                }
                Ignore
            }

            // Usages
            Enter(Expr(expr)) => self.reference_expr(expr),
            Enter(Eval(eval)) => {
                // Eval "hides" its inner expr by making it anonymous.
                // Somehow, this results in Eval nodes not descending into that "hidden" expr,
                // so `Enter(Expr(_))` doesn't fire for Evals. I don't get it.
                // Anyway, we work around this by unpacking the eval ourselves.
                let Ok(expr) = eval.expr() else { return Ignore };
                self.reference_expr(expr)
            }

            // Others
            Enter(Text(_)) => Ignore, // Text is an "atom"
            Enter(_) => Descend,
            Leave(_) => Ignore,
        }
    }

    fn combine(&mut self, _child: Self) {
        // nothing to do
    }

    fn visit_error(&mut self, err: type_sitter_lib::IncorrectKind) -> VisitInstruction<Self> {
        match err.cause() {
            // Error nodes might have children
            IncorrectKindCause::Error => VisitInstruction::Descend,
            // Missing nodes don't have children
            IncorrectKindCause::Missing => VisitInstruction::Ignore,
            // The node couldn't be converted to an AllNamed; unnamed nodes don't have any interesting children
            IncorrectKindCause::OtherKind(_) => VisitInstruction::Ignore,
        }
    }
}
