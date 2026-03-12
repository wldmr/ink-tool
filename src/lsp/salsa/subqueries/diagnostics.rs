use ink_document::InkDocument;
use lsp_types::Diagnostic;
use mini_milc::{subquery, Db};

use crate::lsp::{
    ink_visitors::parse_errors::parse_errors,
    salsa::{
        file_diagnostics,
        subqueries::node_info::{match_flags, NodeFlag},
        DocId, InkGetters as _, Ops,
    },
};

pub(crate) type FileDiagnostics = Vec<Diagnostic>;

subquery!(Ops, file_diagnostics, FileDiagnostics, |self, db| {
    let doc = db.document(self.docid);
    let mut errors = parse_errors(&doc);
    find_unused(&mut errors, db, &doc, self.docid);
    errors
});

fn find_unused(diags: &mut FileDiagnostics, db: &impl Db<Ops>, doc: &InkDocument, docid: DocId) {
    let node_infos = db.node_infos(docid);
    let filter = node_infos.iter_definitions();
    for (range, flags) in filter {
        if db.usages_of(docid, range).len() <= 1 {
            use NodeFlag::*;
            let kind = match_flags!(match (flags) {
                // We don't consider external parameters unused, because EXTERNALs have no body anyway.
                External | Param => continue,
                External | Function => "external function",
                Function => "function",
                Knot => "knot",
                Stitch => "stitch",
                Label => "label",
                Temp => "temporary variable",
                Param => "parameter",
                Var => "variable",
                Const => "constant",
                List => "list",
                ListItem => "list item",
                _ => "unknown kind of definition (this is a bug)",
            });
            let name = doc.text(doc.byte_range(range.into()));
            diags.push(lsp_types::Diagnostic {
                range: range.into(),
                severity: Some(lsp_types::DiagnosticSeverity::WARNING),
                message: format!(r#"Unused {kind} "{name}""#),
                ..Default::default()
            });
        }
    }
}
