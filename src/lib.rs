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
        result: String::with_capacity(source.len() * 1.2 as usize), // Just a bit bigger, in case we add a bunch of stuff.
        line_prefixes: Vec::new(),
    };

    fmt.ink(ink)?;
    Ok(fmt.result)
}
pub mod cst {
    include!(concat!(env!("OUT_DIR"), "/type_sitter_ink.rs"));
}

use cst::{
    self as t,
    anon_unions::{ContentBlock_Knot_StitchBlock, ContentBlock_Stitch},
};
use t::anon_unions::{
    ChoiceBlock_Code_Comment_External_GatherBlock_Global_Include_List_Paragraph_TodoComment as ContentItem,
    ContentBlock_KnotBlock_StitchBlock,
};
use tree_sitter::TreeCursor;
use type_sitter_lib::{ExtraOr, IncorrectKind, TypedNode};

use crate::config::FormatConfig;

struct Formatter<'source, 'config, 'tree> {
    source: &'source str,
    config: &'config FormatConfig,
    cursor: TreeCursor<'tree>,
    result: String,
    line_prefixes: Vec<String>,
}

type FmtResult = Result<(), String>;

macro_rules! handle_children {
    ($self:ident$(,)? $parent:ident.$children:ident: $($regular:path => $result:ident),+) => {
        let children = guard_all_valid($parent.$children(&mut $self.cursor))?;
        for child in children {
            match child {
                ExtraOr::Extra(extra) => $self.push_verbatim(extra)?,
                $( ExtraOr::Regular( $regular (thing) ) => $self.$result (thing) ? ),+
            }
        }
    };
}

impl<'source, 'config, 'tree> Formatter<'source, 'config, 'tree>
where
    'source: 'tree,
{
    fn ink(&mut self, ink: cst::Ink<'tree>) -> FmtResult {
        handle_children! { self, ink.children:
            ContentBlock_KnotBlock_StitchBlock::ContentBlock => content_block,
            ContentBlock_KnotBlock_StitchBlock::KnotBlock => knot_block,
            ContentBlock_KnotBlock_StitchBlock::StitchBlock => stitch_block
        }
        Ok(())
    }

    fn knot_block(&mut self, block: cst::KnotBlock<'tree>) -> FmtResult {
        handle_children! { self, block.children:
            ContentBlock_Knot_StitchBlock::Knot => knot,
            ContentBlock_Knot_StitchBlock::ContentBlock => content_block,
            ContentBlock_Knot_StitchBlock::StitchBlock => stitch_block
        }
        Ok(())
    }

    fn stitch_block(&mut self, block: cst::StitchBlock<'tree>) -> FmtResult {
        handle_children! { self, block.children:
            ContentBlock_Stitch::ContentBlock => content_block,
            ContentBlock_Stitch::Stitch =>  stitch
        }
        Ok(())
    }

    fn content_block(&mut self, block: cst::ContentBlock<'tree>) -> FmtResult {
        handle_children! { self, block.children:
            ContentItem::ChoiceBlock => todo_verbatim,
            ContentItem::Code => todo_verbatim,
            ContentItem::Comment => todo_verbatim,
            ContentItem::External => todo_verbatim,
            ContentItem::GatherBlock => todo_verbatim,
            ContentItem::Global => todo_verbatim,
            ContentItem::Include => todo_verbatim,
            ContentItem::List => todo_verbatim,
            ContentItem::Paragraph => todo_verbatim,
            ContentItem::TodoComment => todo_verbatim
        }
        Ok(())
    }

    fn knot(&mut self, knot: cst::Knot<'tree>) -> FmtResult {
        self.push("===");
        self.identifier(guard_valid(knot.name())?)?;
        let iter = guard_all_valid(knot.paramss(&mut self.cursor))?;
        self.params(iter?)?;
        self.push("===");
        self.push("\n");
        Ok(())
    }

    fn stitch(&mut self, stitch: cst::Stitch<'tree>) -> FmtResult {
        self.todo_verbatim(stitch)
    }

    fn todo_verbatim(&mut self, node: impl type_sitter_lib::TypedNode<'tree>) -> FmtResult {
        let node = node.into_node();
        Ok(self
            .result
            .push_str(&self.source[node.start_byte()..node.end_byte()]))
    }

    fn push_verbatim(&mut self, node: tree_sitter::Node<'tree>) -> FmtResult {
        Ok(self
            .result
            .push_str(&self.source[node.start_byte()..node.end_byte()]))
    }

    fn push(&mut self, mbs: impl MightBeString) {
        if let Some(s) = mbs.mbs() {
            self.result.push_str(s);
        }
    }

    fn identifier(&mut self, ident: cst::Identifier<'tree>) -> FmtResult {
        self.push_verbatim(ident.into_node())
    }

    fn params(&mut self, params: cst::Params<'tree>) -> FmtResult {
        self.push_verbatim(params.into_node())
    }
}

trait MightBeString {
    fn mbs(&self) -> Option<&str>;
}

impl<'a> MightBeString for &'a str {
    fn mbs(&self) -> Option<&str> {
        Some(&self)
    }
}

impl MightBeString for String {
    fn mbs(&self) -> Option<&str> {
        Some(&self)
    }
}

impl<T: AsRef<str>> MightBeString for Option<T> {
    fn mbs(&self) -> Option<&str> {
        self.as_ref().map(|it| it.as_ref())
    }
}

fn guard_valid<'tree, T>(value: Result<T, IncorrectKind<'tree>>) -> Result<T, String> {
    value.map_err(|e| {
        format!(
            "node: {}, expected {}, found {}",
            e.node,
            e.kind,
            e.actual_kind()
        )
    })
}

fn guard_all_valid<'tree, T, I>(iter: I) -> Result<Vec<T>, String>
where
    I: Iterator<Item = Result<T, IncorrectKind<'tree>>>,
{
    guard_valid(iter.collect::<Result<Vec<_>, _>>())
}
