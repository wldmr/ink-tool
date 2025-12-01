use crate::ink_syntax::{
    self,
    types::{Block, Definitions, OfInterest, Usages},
    VisitInstruction, Visitor,
};
use type_sitter_lib::{IncorrectKindCause, Node};

use super::{Db, Doc, SalsaBlock, SalsaDefinition, SalsaUsage};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct BlockItems<'a> {
    pub defs: Vec<SalsaDefinition<'a>>,
    pub usages: Vec<SalsaUsage<'a>>,
    pub blocks: Vec<SalsaBlock<'a>>,
}

impl<'a> BlockItems<'a> {
    pub fn has_any_items(&self) -> bool {
        !self.defs.is_empty() || !self.usages.is_empty() || !self.blocks.is_empty()
    }
}

pub struct BlockVisitor<'a> {
    db: &'a dyn Db,
    cst: Doc,
    start_node: ink_syntax::types::Block<'a>,
    items: BlockItems<'a>,
}

impl<'a> BlockVisitor<'a> {
    pub fn new(db: &'a dyn Db, start_node: SalsaBlock<'a>) -> Self {
        Self {
            db,
            cst: start_node.cst(db),
            start_node: start_node.node(db),
            items: Default::default(),
        }
    }

    pub fn finish(self) -> BlockItems<'a> {
        self.items
    }

    fn definition(&mut self, def: ink_syntax::types::Definitions<'a>) {
        self.items
            .defs
            .push(SalsaDefinition::new(self.db, self.cst, def));
    }

    fn usage(&mut self, usage: Usages<'a>) {
        self.items
            .usages
            .push(SalsaUsage::new(self.db, self.cst, usage));
    }

    fn block(&mut self, block: Block<'a>) {
        self.items
            .blocks
            .push(SalsaBlock::new(self.db, self.cst, block));
    }
}

impl<'a> Visitor<'a, OfInterest<'a>> for BlockVisitor<'a> {
    fn visit(&mut self, visit: OfInterest<'a>) -> VisitInstruction<Self> {
        use VisitInstruction::*;
        log::debug!("visiting {:?}", visit.raw());
        match visit {
            OfInterest::Definitions(definitions) => {
                self.definition(definitions);
                match definitions {
                    // XXX: I think these would then report their own names as usages.
                    Definitions::Knot(_)
                    | Definitions::TempDef(_)
                    | Definitions::List(_)
                    | Definitions::Global(_)
                    | Definitions::Stitch(_) => Descend,
                    // Can't have more interesting items:
                    Definitions::External(_)
                    | Definitions::Label(_)
                    | Definitions::ListValueDef(_)
                    | Definitions::Param(_) => Ignore,
                }
            }
            OfInterest::Usages(usages) => {
                self.usage(usages);
                match usages {
                    // bit of a pointless match, but I want to stumble over this if/when this changes
                    Usages::QualifiedName(_) | Usages::Identifier(_) => Ignore,
                }
            }
            OfInterest::Block(block) => {
                if block == self.start_node {
                    Descend
                } else {
                    self.block(block);
                    // Blocks are units of caching, and the entry point for other BlockVisitors,
                    // so we definitely don't need to descend
                    Ignore
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
            // The node couldn't be converted to an OfInterest, but they might have interesting children
            IncorrectKindCause::OtherKind(_) => VisitInstruction::Descend,
        }
    }
}
