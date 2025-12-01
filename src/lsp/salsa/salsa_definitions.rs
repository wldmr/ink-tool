use crate::ink_syntax::{
    self,
    types::{AllNamed, Definitions},
    VisitInstruction, Visitor,
};
use type_sitter_lib::{IncorrectKindCause, Node};

use super::{Db, Doc, SalsaBlock, SalsaDefinition};

pub struct DefinitionVisitor<'a> {
    db: &'a dyn Db,
    start_node_id: usize,
    cst: Doc,
    defs: Vec<SalsaDefinition<'a>>,
    blocks: Vec<SalsaBlock<'a>>,
}

impl<'a> DefinitionVisitor<'a> {
    pub fn new(db: &'a dyn Db, start_node: SalsaBlock<'a>) -> Self {
        Self {
            db,
            start_node_id: start_node.node(db).raw().id(),
            cst: start_node.cst(db),
            defs: Vec::new(),
            blocks: Vec::new(),
        }
    }

    pub fn finish(self) -> (Vec<SalsaDefinition<'a>>, Vec<SalsaBlock<'a>>) {
        (self.defs, self.blocks)
    }

    fn define(&mut self, def: ink_syntax::types::Definitions<'a>) {
        self.defs.push(SalsaDefinition::new(self.db, self.cst, def));
    }

    fn block(&mut self, block: ink_syntax::types::Block<'a>) {
        self.blocks.push(SalsaBlock::new(self.db, self.cst, block));
    }
}

impl<'a> Visitor<'a, AllNamed<'a>> for DefinitionVisitor<'a> {
    fn visit(&mut self, visit: AllNamed<'a>) -> VisitInstruction<Self> {
        use ink_syntax::types::AllNamed::*;
        use VisitInstruction::*;
        match visit {
            // Definitions
            TempDef(node) => {
                self.define(Definitions::TempDef(node));
                Ignore
            }

            External(node) => {
                self.define(Definitions::External(node));
                Ignore // Externals have no body, therefore we don't need to include param definitions.
            }

            Global(node) => {
                self.define(Definitions::Global(node));
                Ignore
            }

            List(node) => {
                self.define(Definitions::List(node));
                Descend
            }
            ListValueDef(node) => {
                self.define(Definitions::ListValueDef(node));
                Ignore
            }

            Knot(node) => {
                self.define(Definitions::Knot(node));
                Descend
            }

            Stitch(node) => {
                self.define(Definitions::Stitch(node));
                Descend
            }

            Param(node) => {
                self.define(Definitions::Param(node));
                Ignore
            }

            Label(node) => {
                self.define(Definitions::Label(node));
                Ignore
            }

            // Others
            Content(_) => Ignore, // Content can not have definitions
            other => {
                let raw = other.into_raw();
                if raw.id() == self.start_node_id {
                    Descend
                } else if let Ok(block) = ink_syntax::types::Block::try_from_raw(raw) {
                    self.block(block);
                    Ignore // not our job
                } else {
                    Descend
                }
            }
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
