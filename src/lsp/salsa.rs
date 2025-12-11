#![allow(non_camel_case_types)]

use super::state::InvalidPosition;
use crate::{
    ink_syntax::{self, traversal, Visitor},
    lsp::{
        document::InkDocument,
        idset::{Id, IdSet},
        salsa::{doc_symbols::DocumentSymbolsQ, ws_symbols::WorkspaceSymbolsQ},
    },
};
use lsp_types::{DocumentSymbol, Uri, WorkspaceSymbol};
use mini_milc::{composite_query, subquery, Cached, Db, HasChanged};
use names::Meta;
use std::collections::HashMap;
use type_sitter::Node as _;

mod doc_symbols;
pub mod names;
mod ws_symbols;

#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("Could not go to node.")]
pub enum GetNodeError {
    #[display("Node type didn't match")]
    InvalidType,
    PositionOutOfBounds(InvalidPosition),
}

pub(crate) type DocId = Id<Uri>;
pub(crate) type DocIds = IdSet<Uri>;

type Names = Vec<(String, Meta)>;
type WorkspaceNames = HashMap<String, Vec<Meta>>;

type Usages = Vec<(String, lsp_types::Range)>;
type WorkspaceUsages = HashMap<String, Vec<(DocId, lsp_types::Range)>>;

composite_query! {
    #[derive(Hash, Copy)]
    pub enum Ops -> OpsV;

    use DocumentSymbolsQ -> Option<DocumentSymbol>,
        WorkspaceSymbolsQ -> Vec<WorkspaceSymbol>;

    #[derive(Hash, Copy)]
    struct document -> InkDocument {(DocId);}

    #[derive(Hash, Copy)]
    struct doc_ids -> DocIds {;}

    #[derive(Hash, Copy)]
    struct document_names -> Names {(DocId);}

    #[derive(Hash, Copy)]
    struct workspace_names -> WorkspaceNames {;}

    #[derive(Hash, Copy)]
    struct document_usages -> Usages {(DocId);}

    #[derive(Hash, Copy)]
    struct workspace_usages -> WorkspaceUsages {;}
}

// Inputs
subquery!(Ops, document, InkDocument);
subquery!(Ops, doc_ids, DocIds);

subquery!(Ops, document_names, Names, |self, db| {
    let doc = db.document(self.0);
    let to_lsp_range = |r| doc.lsp_range(&r);
    let mut visitor = names::Names::new(self.0, &doc.text, &to_lsp_range);
    visitor.traverse(&mut doc.tree.root_node().walk());
    visitor.into_names()
});

subquery!(Ops, workspace_names, WorkspaceNames, |self, db| {
    let mut names = WorkspaceNames::new();
    // liberally clone the things, because we don’t expect many global name clashes
    // (that is, we will mostly encounter each string once and must clone anyway).
    for id in db.doc_ids().ids() {
        for (name, meta) in db.document_names(id).iter().cloned() {
            names.entry(name).or_default().push(meta);
        }
    }
    names
});

subquery!(Ops, document_usages, Usages, |self, db| {
    let doc = db.document(self.0);
    let ink = type_sitter::UntypedNode::new(doc.tree.root_node());
    traversal::depth_first::<_, ink_syntax::types::Usages>(ink)
        .map(|node| match node {
            ink_syntax::types::Usages::Identifier(ident) => ident.range(),
            ink_syntax::types::Usages::QualifiedName(qname) => qname.range(),
        })
        .map(|range| {
            (
                doc.text[range.start_byte..range.end_byte].to_owned(),
                doc.lsp_range(&range),
            )
        })
        .collect::<Usages>()
});

subquery!(Ops, workspace_usages, WorkspaceUsages, |self, db| {
    let mut usages = WorkspaceUsages::new();
    for id in db.doc_ids().ids() {
        for (name, range) in db.document_usages(id).iter() {
            // Ensure that the entry exists, only cloning the name if necessary.
            // (The entry API requires an owned key, but given that the same name will be
            // referenced quite often, we don’t want to eagerly clone all the occurrences)
            if !usages.contains_key(name) {
                usages.insert(name.clone(), Default::default());
            }
            usages.get_mut(name).unwrap().push((id, *range));
        }
    }
    usages
});

// Extension traits
pub trait InkGetters: Db<Ops> {
    fn doc_ids(&self) -> Cached<'_, Ops, DocIds> {
        self.get(doc_ids)
    }

    fn document(&self, id: DocId) -> Cached<'_, Ops, InkDocument> {
        self.get(document(id))
    }

    fn document_symbols(&self, id: DocId) -> Cached<'_, Ops, Option<DocumentSymbol>> {
        self.get(doc_symbols::DocumentSymbolsQ(id))
    }

    fn workspace_symbols(&self, id: DocId) -> Cached<'_, Ops, Vec<WorkspaceSymbol>> {
        self.get(ws_symbols::WorkspaceSymbolsQ(id))
    }

    fn document_names(&self, id: DocId) -> Cached<'_, Ops, Names> {
        self.get(document_names(id))
    }

    fn workspace_names(&self) -> Cached<'_, Ops, WorkspaceNames> {
        self.get(workspace_names)
    }

    fn document_usages(&self, id: DocId) -> Cached<'_, Ops, Usages> {
        self.get(document_usages(id))
    }

    fn workspace_usages(&self) -> Cached<'_, Ops, WorkspaceUsages> {
        self.get(workspace_usages)
    }
}
impl<D: Db<Ops>> InkGetters for D {}

pub trait InkSetters: Db<Ops> {
    fn modify_docs<C: HasChanged>(&mut self, f: impl FnOnce(&mut DocIds) -> C) -> bool {
        self.modify(doc_ids, f)
    }

    fn modify_document<C: HasChanged>(
        &mut self,
        id: DocId,
        default: impl FnOnce() -> InkDocument,
        update: impl FnOnce(&mut InkDocument) -> C,
    ) -> bool {
        self.modify_with_default(document(id), default, update)
    }
}
impl<D: InkGetters> InkSetters for D {}

pub struct LspDiagnostic {
    doc: DocId,
    diagnostic: lsp_types::Diagnostic,
}
