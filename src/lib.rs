use tree_sitter::Parser;

pub mod config;
pub mod edit;
pub mod rules;

pub fn format(config: config::FormatConfig, source: String) -> Result<String, String> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_ink::language())
        .expect("We should be ablet to load an Ink grammar.");

    let tree = parser
        .parse(&source, None)
        .expect("There should be a tree here.");

    let ink: cst::Ink = tree
        .root_node()
        .try_into()
        .expect("If the syntax don't fit, you must, ah …, quit.");

    let mut fmt = Formatter {
        source: &source,
        config: &config,
        cursor: ink.walk(),
        result: String::with_capacity(source.len() * 1.2 as usize),
        line_prefixes: Vec::new(),
    };

    fmt.ink(&ink)?;
    Ok(fmt.result)
}
pub mod cst {
    include!(concat!(env!("OUT_DIR"), "/type_sitter_ink.rs"));
}

use cst as t;
use t::anon_unions::ContentBlock_KnotBlock_StitchBlock;
use tree_sitter::TreeCursor;
use type_sitter_lib::{ExtraOr, TypedNode};

use crate::config::FormatConfig;

struct Formatter<'a, 'tree> {
    source: &'a str,
    config: &'a FormatConfig,
    cursor: TreeCursor<'tree>,
    result: String,
    line_prefixes: Vec<String>,
}

impl<'a, 'tree> Formatter<'a, 'tree>
where
    'a: 'tree,
{
    fn ink(&mut self, ink: &cst::Ink<'tree>) -> Result<(), String> {
        let children: Vec<_> = ink.children(&mut self.cursor).collect();
        for child in children {
            let ele = child.map_err(|e| format!("expected extra or blocks, found {}", e.kind))?;
            dbg!(ele);
            match ele {
                ExtraOr::Extra(extra) => self.take_verbatim(extra)?,
                ExtraOr::Regular(regular) => match regular {
                    ContentBlock_KnotBlock_StitchBlock::ContentBlock(block) => {
                        self.take_verbatim(block.into_node())?
                    }
                    ContentBlock_KnotBlock_StitchBlock::KnotBlock(block) => {
                        self.take_verbatim(block.into_node())?
                    }
                    ContentBlock_KnotBlock_StitchBlock::StitchBlock(block) => {
                        self.take_verbatim(block.into_node())?
                    }
                },
            }
        }
        Ok(())
    }

    fn knot_block(&'a mut self, k: cst::KnotBlock<'_>) {
        todo!()
    }

    fn stitch_block(&mut self, s: cst::StitchBlock<'_>) {
        todo!()
    }

    fn content_block(&mut self, c: cst::ContentBlock<'_>) {
        todo!()
    }

    fn take_verbatim(&mut self, node: tree_sitter::Node<'_>) -> Result<(), String> {
        Ok(self
            .result
            .push_str(&self.source[node.start_byte()..node.end_byte()]))
    }
}
