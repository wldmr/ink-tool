use crate::ink_syntax::{types::AllNamed, VisitInstruction, Visitor};
use crate::lsp::document::InkDocument;
use crate::lsp::location::{FileId, LocationId};
use crate::lsp::scopes::{ScopeId, Scopes};
use type_sitter_lib::{IncorrectKindCause, Node};

pub(super) struct ScopedNamesVisitor<'tree, 'scopes> {
    file: FileId,
    doc: &'tree InkDocument,
    current_list: Option<String>,
    current_knot: Option<String>,
    current_stitch: Option<String>,
    scopes: &'scopes mut Scopes,
    current_scope: ScopeId,
}

impl<'tree, 'scopes> ScopedNamesVisitor<'tree, 'scopes> {
    pub(crate) fn new(doc: &'tree InkDocument, scopes: &'scopes mut Scopes) -> Self {
        let file = doc.uri.clone().into();
        scopes.remove_all(&file);
        Self {
            file,
            doc,
            scopes,
            current_scope: ScopeId::global(),
            current_list: None,
            current_knot: None,
            current_stitch: None,
        }
    }
}

impl<'tree, 'scopes> Visitor<'tree, AllNamed<'tree>> for ScopedNamesVisitor<'tree, 'scopes> {
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
            | AllNamed::ListValueDefs(_) => Descend,

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

            AllNamed::KnotBlock(block) => {
                match self.scopes.define_scope(
                    self.file.clone(),
                    self.doc.lsp_range(&block.range()),
                    ScopeId::global(),
                ) {
                    Ok(id) => self.current_scope = id,
                    Err(e) => println!("{e}"),
                };
                Descend
            }
            AllNamed::Knot(knot) => {
                let name = &self.doc.text[knot.name().byte_range()];
                let range = self.doc.lsp_range(&knot.name().range());
                let id = LocationId::new(self.file.clone(), range);
                self.current_knot = Some(name.to_string());
                self.current_stitch = None;
                self.scopes.define_name(&self.current_scope, name, id);
                Descend
            }

            AllNamed::StitchBlock(block) => {
                match self.scopes.define_scope(
                    self.file.clone(),
                    self.doc.lsp_range(&block.range()),
                    self.current_scope.clone(),
                ) {
                    Ok(id) => self.current_scope = id,
                    Err(e) => println!("{e}"),
                };
                Descend
            }
            AllNamed::Stitch(stitch) => {
                let byte_range = stitch.name().byte_range();
                let stitch_name = &self.doc.text[byte_range];
                let id = LocationId::new(
                    self.file.clone(),
                    self.doc.lsp_range(&stitch.name().range()),
                );
                self.current_stitch = Some(stitch_name.to_string());
                if let Some(ref knot) = self.current_knot {
                    self.scopes.define_name(
                        &ScopeId::global(),
                        format!("{knot}.{stitch_name}"),
                        id.clone(),
                    );
                }
                self.scopes
                    .define_name(&self.current_scope, stitch_name, id);
                Descend
            }

            AllNamed::External(external) => {
                let byte_range = external.name().byte_range();
                let name = self.doc.text[byte_range].to_string();
                let id = LocationId::new(
                    self.file.clone(),
                    self.doc.lsp_range(&external.name().range()),
                );
                self.scopes.define_name(&ScopeId::global(), name, id);
                Ignore
            }

            AllNamed::Label(label) => {
                let byte_range = label.name().byte_range();
                let label_name = self.doc.text[byte_range.clone()].to_string();
                let id =
                    LocationId::new(self.file.clone(), self.doc.lsp_range(&label.name().range()));
                // Generate all the names for this label.
                // Reminder:
                // - The cannonical name is `knot.label`, or `label` if toplevel.
                // - Another optional name is `knot.stitch.label` if defined inside a stitch
                let label_name = &label_name; // makes the format strings more readable
                match (self.current_knot.as_ref(), self.current_stitch.as_ref()) {
                    (None, None) => {}
                    (None, Some(stitch)) => {
                        self.scopes.define_name(
                            &ScopeId::global(),
                            format!("{stitch}.{label_name}"),
                            id.clone(),
                        );
                    }
                    (Some(knot), None) => {
                        self.scopes.define_name(
                            &ScopeId::global(),
                            format!("{knot}.{label_name}"),
                            id.clone(),
                        );
                    }
                    (Some(knot), Some(stitch)) => {
                        self.scopes.define_name(
                            &ScopeId::global(),
                            format!("{knot}.{label_name}"),
                            id.clone(),
                        );
                        self.scopes.define_name(
                            &ScopeId::global(),
                            format!("{knot}.{stitch}.{label_name}"),
                            id.clone(),
                        );
                    }
                }
                // Finally, the innermost scope (whatever it is, including global) always gets to see the plain `label_name`
                self.scopes.define_name(&self.current_scope, label_name, id);
                Ignore
            }

            AllNamed::Global(global) => {
                let byte_range = global.name().byte_range();
                let name = self.doc.text[byte_range.clone()].to_string();
                let id = LocationId::new(
                    self.file.clone(),
                    self.doc.lsp_range(&global.name().range()),
                );
                self.scopes.define_name(&ScopeId::global(), name, id);
                Ignore
            }

            AllNamed::List(list) => {
                let byte_range = list.name().byte_range();
                let name = self.doc.text[byte_range.clone()].to_string();
                let id =
                    LocationId::new(self.file.clone(), self.doc.lsp_range(&list.name().range()));
                self.scopes.define_name(&ScopeId::global(), &name, id);
                self.current_list = Some(name);
                Descend
            }

            AllNamed::ListValueDef(def) => {
                let byte_range = def.name().byte_range();
                let value_name = &self.doc.text[byte_range.clone()];
                let id =
                    LocationId::new(self.file.clone(), self.doc.lsp_range(&def.name().range()));
                let list_name = self
                    .current_list
                    .as_ref()
                    .expect("must have set this before getting here");
                self.scopes.define_name(
                    &ScopeId::global(),
                    format!("{list_name}.{value_name}"),
                    id,
                );
                Ignore
            }

            AllNamed::Temp(temp) => {
                let byte_range = temp.name().byte_range();
                let name = self.doc.text[byte_range.clone()].to_string();
                let id =
                    LocationId::new(self.file.clone(), self.doc.lsp_range(&temp.name().range()));
                self.scopes.define_temp(&self.current_scope, name, id);
                Ignore
            }
        }
    }

    fn combine(&mut self, _child: Self) {
        // nothing to do, we mutate scopes in-place
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
