use ink_document::InkDocument;
use lsp_types::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location};
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
    add_unused(&mut errors, db, &doc, self.docid, &node_infos);
    add_illegal_targets(&mut errors, db, &doc, self.docid, &node_infos);
    errors
});

fn add_unused(
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
                _ => "unknown kind of definition (This is likely a bug in ink-tool.)",
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

fn add_illegal_targets(
    diags: &mut FileDiagnostics,
    db: &impl Db<Ops>,
    doc: &InkDocument,
    docid: DocId,
    node_infos: &NodeInfos,
) {
    use NodeFlag::*;

    let usages = node_infos.iter_flags().filter(|(_range, flags)| {
        flags.intersects(Usage) && !flags.intersects(Definition | Builtin)
    });

    let uris = db.doc_ids();
    for (usage, flags) in usages {
        let definition = db.definition_of(docid, usage);
        let text = doc.text(doc.byte_range(usage.into()));

        if definition.is_empty() {
            let kind = match_flags!(match (flags) {
                Redirect => "location",
                Call => "function",
                _ => "name",
            });
            diags.push(Diagnostic {
                range: usage.into(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!(r#"Undefined {kind} "{text}""#),
                ..Default::default()
            });
        } else {
            let mut ambiguous_targets = Vec::new();
            let mut illegal_targets = Vec::new();

            for (def_doc, def_range) in definition.iter().copied() {
                let def_flags = db.node_infos(def_doc).flags(*def_range);

                let def_kind = match_flags!(match (def_flags) {
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

                ambiguous_targets.push(DiagnosticRelatedInformation {
                    location: Location::new(uris[def_doc].clone(), def_range.into()),
                    message: format!("{def_kind} defined here"),
                });

                if flags.contains(Redirect) {
                    if !def_flags.intersects(Knot | Stitch | Label | Var | Param | Temp) {
                        // Var’s, Temps, Params *might* contain an address. We could get more granular
                        // here, but this would get us in the weeds of infering types in a dynamically
                        // typed language.
                        illegal_targets.push(DiagnosticRelatedInformation {
                            location: Location::new(uris[def_doc].clone(), def_range.into()),
                            message: format!("a {def_kind} is not an address"),
                        });
                    }
                }

                if flags.contains(Call) {
                    if !def_flags.intersects(Function | HasParams) {
                        illegal_targets.push(DiagnosticRelatedInformation {
                            location: Location::new(uris[def_doc].clone(), def_range.into()),
                            message: format!("a {def_kind} can not be called"),
                        });
                    }
                }
            }

            if ambiguous_targets.len() >= 2 {
                diags.push(Diagnostic {
                    range: usage.into(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!(r#"Multiple definitions for "{text}""#),
                    related_information: Some(ambiguous_targets),
                    ..Default::default()
                });
            }

            if illegal_targets.len() != 0 {
                diags.push(Diagnostic {
                    range: usage.into(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: match_flags!(match (flags) {
                        Redirect => format!(r#"Can not redirect to {text}"#),
                        Call => format!(r#"Can not call {text}"#),
                        _ => format! {r#"Some problem with "{text}"

(This message should be more specific. You've likely found a bug in ink-tool)"#
                        },
                    }),
                    related_information: Some(illegal_targets),
                    ..Default::default()
                });
            }
        }
    }
}
