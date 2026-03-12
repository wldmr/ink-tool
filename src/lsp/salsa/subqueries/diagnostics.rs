use ink_document::InkDocument;
use lsp_types::{Diagnostic, DiagnosticSeverity};
use mini_milc::{subquery, Db};

use crate::lsp::{
    ink_visitors::parse_errors::parse_errors,
    salsa::{
        file_diagnostics,
        subqueries::node_info::{match_flags, NodeFlag},
        DocId, InkGetters as _, NodeInfos, Ops,
    },
};

pub(crate) type FileDiagnostics = Vec<Diagnostic>;

subquery!(Ops, file_diagnostics, FileDiagnostics, |self, db| {
    let doc = db.document(self.docid);
    let node_infos = db.node_infos(self.docid);
    let mut errors = parse_errors(&doc);
    find_unused(&mut errors, db, &doc, self.docid, &node_infos);
    find_unresolved(&mut errors, db, &doc, self.docid, &node_infos);
    errors
});

fn find_unused(
    diags: &mut FileDiagnostics,
    db: &impl Db<Ops>,
    doc: &InkDocument,
    docid: DocId,
    node_infos: &NodeInfos,
) {
    let defs = node_infos.iter_definitions();

    for (range, flags) in defs {
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
            diags.push(Diagnostic {
                range: range.into(),
                severity: Some(DiagnosticSeverity::WARNING),
                message: format!(r#"Unused {kind} "{name}""#),
                ..Default::default()
            });
        }
    }
}

fn find_unresolved(
    diags: &mut FileDiagnostics,
    db: &impl Db<Ops>,
    doc: &InkDocument,
    docid: DocId,
    node_infos: &NodeInfos,
) {
    use NodeFlag::*;

    let usages = node_infos
        .iter_flags()
        .filter(|(_range, flags)| flags.contains(Usage) && !flags.contains(Definition));

    for (usage, flags) in usages {
        if db.definition_of(docid, usage).is_empty() {
            let name = doc.text(doc.byte_range(usage.into()));
            if flags.contains(Redirect) && (name == "DONE" || name == "END") {
                continue; // These are special, they're never unresolved
            }
            diags.push(Diagnostic {
                range: usage.into(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!(r#"Undefined name "{name}""#),
                ..Default::default()
            });
        }
    }
}
