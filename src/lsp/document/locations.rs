use crate::lsp::document::InkDocument;
use crate::{
    ink_syntax::{types::AllNamed, VisitInstruction, Visitor},
    lsp::location::{Location, LocationKind},
};
use lsp_types::Uri;
use std::ops::Range;
use type_sitter_lib::{IncorrectKindCause, Node};

pub(crate) struct Locations<'a> {
    pub(crate) doc: &'a InkDocument,
    pub(crate) uri: &'a Uri,
    pub(crate) namespace: Option<String>,
    pub(crate) locs: Vec<Location>,
}

impl<'a> Locations<'a> {
    pub(crate) fn new(uri: &'a Uri, doc: &'a InkDocument) -> Self {
        Self {
            uri,
            doc,
            namespace: None,
            locs: Vec::new(),
        }
    }

    fn new_loc(&mut self, kind: LocationKind, name: String, byte_range: Range<usize>) -> Self {
        self.locs.push(Location {
            file: self.uri.clone(),
            name,
            namespace: self.namespace.clone(),
            byte_range,
            kind,
        });
        Self {
            uri: self.uri,
            doc: self.doc,
            namespace: self.namespace.clone(),
            locs: Vec::new(),
        }
    }
}

impl<'tree> Visitor<'tree, AllNamed<'tree>> for Locations<'tree> {
    fn visit(&mut self, node: AllNamed) -> VisitInstruction<Self> {
        use VisitInstruction::*;
        match node {
            // recurse into these without creating a new level
            AllNamed::Choice(_)
            | AllNamed::ChoiceBlock(_)
            | AllNamed::Code(_)
            | AllNamed::Content(_)
            | AllNamed::Gather(_)
            | AllNamed::GatherBlock(_)
            | AllNamed::Ink(_)
            | AllNamed::KnotBlock(_)
            | AllNamed::ListValueDefs(_)
            | AllNamed::StitchBlock(_) => Descend,

            // children of these don't have interesting symbols, so don't recurse
            AllNamed::AltArm(_)
            | AllNamed::Alternatives(_)
            | AllNamed::Args(_)
            | AllNamed::Assignment(_)
            | AllNamed::Binary(_)
            | AllNamed::BlockComment(_)
            | AllNamed::Boolean(_)
            | AllNamed::Call(_)
            | AllNamed::ChoiceMark(_)
            | AllNamed::ChoiceMarks(_)
            | AllNamed::ChoiceOnly(_)
            | AllNamed::CodeStmt(_)
            | AllNamed::Comment(_)
            | AllNamed::CondArm(_)
            | AllNamed::CondBlock(_)
            | AllNamed::Condition(_)
            | AllNamed::ConditionalText(_)
            | AllNamed::Divert(_)
            | AllNamed::DivertTarget(_)
            | AllNamed::Else(_)
            | AllNamed::Eol(_)
            | AllNamed::Eval(_)
            | AllNamed::Expr(_)
            | AllNamed::GatherMark(_)
            | AllNamed::GatherMarks(_)
            | AllNamed::GlobalKeyword(_)
            | AllNamed::Glue(_)
            | AllNamed::Identifier(_)
            | AllNamed::Include(_)
            | AllNamed::LineComment(_)
            | AllNamed::ListValues(_)
            | AllNamed::Logic(_)
            | AllNamed::MultilineAlternatives(_)
            | AllNamed::Number(_)
            | AllNamed::Paragraph(_)
            | AllNamed::Param(_)
            | AllNamed::ParamValue(_)
            | AllNamed::Params(_)
            | AllNamed::Paren(_)
            | AllNamed::Path(_)
            | AllNamed::Postfix(_)
            | AllNamed::QualifiedName(_)
            | AllNamed::Redirect(_)
            | AllNamed::Return(_)
            | AllNamed::String(_)
            | AllNamed::Tag(_)
            | AllNamed::Text(_)
            | AllNamed::Thread(_)
            | AllNamed::TodoComment(_)
            | AllNamed::Tunnel(_)
            | AllNamed::Unary(_) => Ignore,

            AllNamed::Knot(knot) => {
                let kind = if knot.function().is_some() {
                    LocationKind::Function
                } else {
                    LocationKind::Knot
                };
                let mut byte_range = knot.name().byte_range();
                if let Some(Ok(params)) = knot.params() {
                    // dirty trick to get the params in there. We should do this better
                    byte_range.end = params.end_byte();
                }
                let name = self.doc.text[byte_range.clone()].to_string();
                let mut new_loc = self.new_loc(kind, name.clone(), byte_range);
                new_loc.namespace = Some(name);
                Return(new_loc)
            }

            AllNamed::Stitch(stitch) => {
                let mut byte_range = stitch.name().byte_range();
                if let Some(Ok(params)) = stitch.params() {
                    // dirty trick to get the params in there. We should do this better
                    byte_range.end = params.end_byte();
                }
                let name = self.doc.text[byte_range.clone()].to_string();
                let mut new_loc = self.new_loc(LocationKind::Stitch, name.clone(), byte_range);
                new_loc.namespace = if let Some(ref knot) = self.namespace {
                    Some(format!("{knot}.{name}"))
                } else {
                    Some(name)
                };
                Return(new_loc)
            }

            AllNamed::External(external) => {
                let mut byte_range = external.name().byte_range();
                if let Ok(params) = external.params() {
                    // dirty trick to get the params in there. We should do this better
                    byte_range.end = params.end_byte();
                }
                let name = self.doc.text[byte_range.clone()].to_string();
                Return(self.new_loc(LocationKind::Function, name, byte_range))
            }

            AllNamed::Label(label) => {
                let byte_range = label.name().byte_range();
                let name = self.doc.text[byte_range.clone()].to_string();
                Return(self.new_loc(LocationKind::Label, name, byte_range))
            }

            AllNamed::Global(global) => {
                let byte_range = global.name().byte_range();
                let name = self.doc.text[byte_range.clone()].to_owned();
                let mut new_loc = self.new_loc(LocationKind::Variable, name, byte_range);
                new_loc.namespace = None;
                Return(new_loc)
            }

            AllNamed::List(list) => {
                let byte_range = list.name().byte_range();
                let name = self.doc.text[byte_range.clone()].to_owned();
                let mut new_loc = self.new_loc(LocationKind::Variable, name.clone(), byte_range);
                new_loc.namespace = Some(name);
                DescendWith(new_loc)
            }

            AllNamed::ListValueDef(def) => {
                let byte_range = def.name().byte_range();
                let name = self.doc.text[byte_range.clone().clone()].to_owned();
                Return(self.new_loc(LocationKind::Variable, name, byte_range))
            }

            AllNamed::Temp(temp) => {
                let byte_range = temp.name().byte_range();
                let name = self.doc.text[byte_range.clone()].to_owned();
                Return(self.new_loc(LocationKind::Variable, name, byte_range))
            }
        }
    }

    fn combine(&mut self, child: Self) {
        self.locs.extend(child.locs);
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
