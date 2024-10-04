use std::ops::ControlFlow;

use crate::ink_syntax::types::{AllNamed, Knot, Label, Redirect, Stitch};
use crate::ink_syntax::Visitor;
use tree_sitter::Parser;
use type_sitter_lib::{Node, NodeResult};

fn ink_parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::LANGUAGE.into())
        .expect("setting the language mustn't fail");
    parser
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AddressDef<'a> {
    Knot {
        knot: Knot<'a>,
    },
    Stitch {
        stitch: Stitch<'a>,
        knot: Option<Knot<'a>>,
    },
    Label {
        label: Label<'a>,
        knot: Option<Knot<'a>>,
        stitch: Option<Stitch<'a>>,
    },
}

#[derive(Debug, Default)]
pub struct InkAddressCollector<'a> {
    pub defs: Vec<AddressDef<'a>>,
    pub refs: Vec<Redirect<'a>>,
    current_knot: Option<Knot<'a>>,
    current_stitch: Option<Stitch<'a>>,
}

impl<'a> Visitor<'a, AllNamed<'a>> for InkAddressCollector<'a> {
    type Output = ();

    fn visit(&mut self, node: NodeResult<AllNamed<'a>>) -> std::ops::ControlFlow<Self::Output> {
        let result = match node {
            Ok(ok) => match ok {
                // Definitions
                AllNamed::Knot(knot) if knot.function().is_none() => {
                    self.current_knot = Some(knot);
                    self.defs.push(AddressDef::Knot { knot });
                    ControlFlow::Break(())
                }
                AllNamed::Stitch(stitch) => {
                    self.current_stitch = Some(stitch);
                    self.defs.push(AddressDef::Stitch {
                        stitch,
                        knot: self.current_knot,
                    });
                    ControlFlow::Break(())
                }
                AllNamed::Choice(choice) => {
                    if let Some(Ok(label)) = choice.label() {
                        self.defs.push(AddressDef::Label {
                            label,
                            knot: self.current_knot,
                            stitch: self.current_stitch,
                        })
                    }
                    ControlFlow::Break(())
                }
                AllNamed::Gather(gather) => {
                    if let Some(Ok(label)) = gather.label() {
                        self.defs.push(AddressDef::Label {
                            label,
                            knot: self.current_knot,
                            stitch: self.current_stitch,
                        })
                    }
                    ControlFlow::Break(())
                }

                // References
                AllNamed::Thread(it) => {
                    self.refs.push(Redirect::Thread(it));
                    ControlFlow::Break(())
                }
                AllNamed::Divert(it) => {
                    self.refs.push(Redirect::Divert(it));
                    ControlFlow::Break(())
                }
                AllNamed::Tunnel(it) => {
                    self.refs.push(Redirect::Tunnel(it));
                    ControlFlow::Break(())
                }

                it if it.named_child_count() == 0 => ControlFlow::Break(()),

                _ => ControlFlow::Continue(()),
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                ControlFlow::Continue(())
            }
        };
        eprintln!(
            "{}: {} {}",
            if result.is_break() { "Visit" } else { "Enter" },
            node.raw().kind(),
            node.raw().start_position()
        );
        result
    }

    fn leave(&mut self, node: NodeResult<AllNamed<'a>>) {
        eprintln!(
            "Leave: {} {}",
            node.raw().kind(),
            node.raw().start_position()
        );
        // this might not be necessary, because you can't reference anything between leaving a block and entering a new one. I thing …
        match node {
            Ok(AllNamed::KnotBlock(_)) => {
                self.current_knot = None;
            }
            Ok(AllNamed::StitchBlock(_)) => {
                self.current_stitch = None;
            }
            _ => (),
        };
    }
}
