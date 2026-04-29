use std::{collections::HashMap, iter};

use enumflags2::BitFlags;
use ink_document::{ids::DefId, InkDocument};
use lsp_types::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location};
use mini_milc::{subquery, Db, Old, Subquery, Updated};
use util::nonempty::Vec1;

use crate::lsp::{
    ink_visitors::parse_errors::parse_errors,
    location::FileTextRange,
    salsa::{
        duplicate_definitions, duplicate_imports, file_diagnostics,
        subqueries::node_info::{match_flags, NodeFlag},
        DocId, InkGetters as _, Name, NodeInfos, Ops,
    },
};

pub type FileDiagnostics = Vec<Diagnostic>;
pub type DuplicateImports = HashMap<DocId, Vec1<FileTextRange>>;
pub type DuplicateDefinitions = HashMap<Name, Vec<((DocId, DefId), BitFlags<NodeFlag>)>>;

subquery!(Ops, file_diagnostics, FileDiagnostics, |self, db| {
    let doc = db.document(self.docid);
    let node_infos = db.node_infos(self.docid);
    let mut errors = parse_errors(&doc);
    add_unused(&mut errors, db, &doc, self.docid, &node_infos);
    add_illegal_targets(&mut errors, db, self.docid, &node_infos);
    add_duplicate_definitions(&mut errors, db, self.docid);
    add_duplicate_imports(&mut errors, db, self.docid);
    add_unresolved_imports(&mut errors, db, self.docid);
    errors
});

impl Subquery<Ops, DuplicateDefinitions> for duplicate_definitions {
    fn value(
        &self,
        db: &impl Db<Ops>,
        old: Old<DuplicateDefinitions>,
    ) -> Updated<DuplicateDefinitions> {
        use NodeFlag::*;
        let mut duplicates = DuplicateDefinitions::new();

        for file in db.stories()[&self.story].resolved.keys() {
            let globals = db.file_globals(*file);
            let infos = db.node_infos(*file);
            for (name, defs) in globals.iter() {
                for def in defs {
                    let loc = ((*file, *def), infos.flags(*def));
                    match duplicates.get_mut(name) {
                        Some(vec) => vec.push(loc),
                        None => {
                            duplicates.insert(*name, vec![loc]);
                        }
                    }
                }
            }
        }

        duplicates.retain(|_, defs| defs.len() > 1);
        duplicates.retain(|_, defs| {
            // Two functions, with exactly one External
            if defs.len() == 2
                && defs.iter().all(|it| it.1.contains(Definition | Function))
                && defs.iter().filter(|it| it.1.contains(External)).count() == 1
            {
                false
            }
            // List item don't conflict with each other, so if all names are List Items, then that's fine.
            else if defs.iter().all(|it| it.1.contains(ListItem)) {
                false
            } else {
                true
            }
        });

        old.update(duplicates)
    }
}

impl Subquery<Ops, DuplicateImports> for duplicate_imports {
    fn value(&self, db: &impl Db<Ops>, old: Old<DuplicateImports>) -> Updated<DuplicateImports> {
        let transitive_imports = &db.stories()[&self.story];

        let new: DuplicateImports = transitive_imports
            .resolved
            .iter()
            .filter(|(_, defs)| defs.len() > 1)
            .map(|(target, defs)| (*target, defs.clone()))
            .collect();

        old.update(new)
    }
}

fn add_unused(
    diags: &mut FileDiagnostics,
    db: &impl Db<Ops>,
    doc: &InkDocument,
    docid: DocId,
    node_infos: &NodeInfos,
) {
    let defs = node_infos.iter_definitions();

    for (defid, flags) in defs {
        if db.usages(docid, defid).len() <= 1 {
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
            let locs = db.node_locations(docid);
            let range = locs[defid].into();
            let name = doc.lsp_text(range);
            diags.push(Diagnostic {
                range,
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
    docid: DocId,
    node_infos: &NodeInfos,
) {
    use NodeFlag::*;

    let usages = node_infos.iter_flags().filter(|(_range, flags)| {
        flags.intersects(Usage) && !flags.intersects(Definition | Builtin)
    });

    let uris = db.doc_ids();
    let node_text = db.node_text(docid);
    let locs = db.node_locations(docid);

    for (usage, flags) in usages {
        let definition = db.definition(docid, usage);
        let text = node_text[usage.as_ref()];

        if definition.is_empty() {
            let kind = match_flags!(match (flags) {
                Redirect => "location",
                Call => "function",
                _ => "name",
            });
            diags.push(Diagnostic {
                range: locs[usage].into(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!(r#"Undefined {kind} "{text}""#),
                ..Default::default()
            });
        } else {
            let mut illegal_targets = Vec::new();

            for (def_doc, def_id) in definition.iter().copied() {
                let def_flags = db.node_infos(def_doc).flags(def_id);
                let locs = db.node_locations(def_doc);

                let def_kind = match_flags!(match (def_flags) {
                    Function | External => "external function",
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

                if flags.contains(Redirect) {
                    if !def_flags.intersects(Knot | Stitch | Label | Var | Param | Temp) {
                        // Var’s, Temps, Params *might* contain an address. We could get more granular
                        // here, but this would get us in the weeds of infering types in a dynamically
                        // typed language.
                        illegal_targets.push(DiagnosticRelatedInformation {
                            location: Location::new(uris[def_doc].clone(), locs[def_id].into()),
                            message: format!("a {def_kind} is not an address"),
                        });
                    }
                }

                if flags.contains(Call) {
                    if !def_flags.intersects(Function | HasParams) {
                        illegal_targets.push(DiagnosticRelatedInformation {
                            location: Location::new(uris[def_doc].clone(), locs[def_id].into()),
                            message: format!("a {def_kind} can not be called"),
                        });
                    }
                }
            }

            if illegal_targets.len() != 0 {
                diags.push(Diagnostic {
                    range: locs[usage].into(),
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

fn add_duplicate_definitions(diags: &mut FileDiagnostics, db: &impl Db<Ops>, docid: DocId) {
    use NodeFlag::*;

    // TODO: We should certainly catch duplicate *local* definitions, too.

    let uris = db.doc_ids();
    let parents = db.stories_of(docid);
    for story in parents.iter().copied() {
        let duplicates = db.duplicate_definitions(story);
        let story_suffix = if db.stories().len() > 1 {
            let story_path = db.short_path(story.into());
            format!(" in story {}", story_path.as_str())
        } else {
            String::new()
        };

        for (name, dups) in duplicates.iter() {
            for (this, this_flags) in dups.iter() {
                let locs = db.node_locations(this.0);
                if this.0 != docid {
                    continue;
                }
                diags.push(Diagnostic {
                    range: locs[this.1].into(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!("Multiple definitions of {name}{story_suffix}."),
                    related_information: Some(
                        dups.iter()
                            .filter(|(other, _)| other != this)
                            .filter(|(_, other_flags)| {
                                !(*this_flags & *other_flags).contains(ListItem)
                                // List items don't conflict with each other
                            })
                            .map(|(other, flags)| {
                                let locs = db.node_locations(other.0);
                                DiagnosticRelatedInformation {
                                    location: Location::new(
                                        uris[other.0].clone(),
                                        locs[other.1].into(),
                                    ),
                                    message: format!(
                                        "Also a {} here",
                                        flag_to_kind(*flags)
                                            .unwrap_or("unknown kind of thing (this is a bug)")
                                    ),
                                }
                            })
                            .collect(),
                    ),
                    ..Diagnostic::default()
                });
            }
        }
    }
}

fn flag_to_kind(flags: BitFlags<NodeFlag>) -> Option<&'static str> {
    use NodeFlag::*;
    match_flags!(match (flags) {
        Function | External => "external function",
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
    })
}

fn add_unresolved_imports(diags: &mut FileDiagnostics, db: &impl Db<Ops>, docid: DocId) {
    let stories = db.stories();
    for story in db.stories_of(docid).iter().copied() {
        let transitive_imports = &stories[&story];

        if let Some(unresolved) = transitive_imports.unresolved.get(&docid) {
            let uris = db.doc_ids();
            let story_uri = &uris[story];
            let story_path = db.short_path(story.into());
            let story_path = story_path.as_str();
            for range in unresolved.iter().copied() {
                diags.push(Diagnostic {
                    range: range.into(),
                    message: format!("Import not found relative to story root {story_path}"),
                    severity: Some(DiagnosticSeverity::ERROR),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            uri: story_uri.clone(),
                            range: lsp_types::Range::default(),
                        },
                        message: format!("This story root file"),
                    }]),
                    ..Default::default()
                });
            }
        }
    }
}

fn add_duplicate_imports(diags: &mut FileDiagnostics, db: &impl Db<Ops>, this_doc: DocId) {
    for story in db.stories_of(this_doc).iter().copied() {
        let dupl = db.duplicate_imports(story);

        for (target, dups) in dupl.iter() {
            let (me, others) = dups
                .iter()
                .partition::<Vec<FileTextRange>, _>(|it| it.file == this_doc);

            let uris = db.doc_ids();
            let story_uri = &uris[story];
            let target_path = db.short_path(*target);
            let target_path = target_path.as_str();

            let story_suffix = if db.stories().len() > 1 {
                let story_path = db.short_path(story.into());
                format!(" in story {}", story_path.as_str())
            } else {
                String::new()
            };

            for import in me {
                diags.push(Diagnostic {
                    range: import.range.into(),
                    message: format!("Duplicate or cyclic import{story_suffix}"),
                    severity: Some(DiagnosticSeverity::ERROR),
                    related_information: Some(
                        iter::once(DiagnosticRelatedInformation {
                            location: Location::new(story_uri.clone(), lsp_types::Range::default()),
                            message: format!("story root"),
                        })
                        .chain(others.iter().map(|other| DiagnosticRelatedInformation {
                            location: Location::new(uris[other.file].clone(), other.range.into()),
                            message: format!("{target_path} also imported here"),
                        }))
                        .collect(),
                    ),
                    ..Default::default()
                });
            }
        }
    }
}
