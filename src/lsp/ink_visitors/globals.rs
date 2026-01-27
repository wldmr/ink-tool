use ink_document::InkDocument;
use ink_syntax::AllNamed;
use lsp_types::Range;
use std::{collections::HashMap, ops::Index};
use tree_traversal::{VisitInstruction, Visitor};
use type_sitter::Node;
use util::nonempty::Vec1;

/// Since ink is line-based we can get away with definiing the extent of names in terms of lines only.
///
/// These ranges are non-overlapping; we enforce this by only refering to the first line.
#[derive(Clone, PartialEq, Eq)]
pub struct Globals(HashMap<String, Vec1<(Range, GlobalKind)>>);

impl Globals {
    pub fn get(&self, name: impl AsRef<str>) -> Option<&Vec1<(Range, GlobalKind)>> {
        self.0.get(name.as_ref())
    }
}

impl Index<&str> for Globals {
    type Output = Vec1<(Range, GlobalKind)>;

    fn index(&self, index: &str) -> &Self::Output {
        &self.0[index]
    }
}

impl std::fmt::Debug for Globals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Locals ")?;
        let mut sections = f.debug_map();
        for sec in &self.0 {
            sections.entry(&sec.0, &sec.1);
        }
        sections.finish()
    }
}

impl Default for Globals {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

/// Private Construction helpers
impl Globals {
    fn add_definition(
        &mut self,
        name: impl Into<String>,
        range: lsp_types::Range,
        kind: GlobalKind,
    ) {
        self.0
            .entry(name.into()) // Duplicates will be rare, so we just clone liberally.
            .and_modify(|it| it.push((range, kind)))
            .or_insert_with(|| Vec1::new((range, kind)));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalKind {
    Knot,
    Stitch,
    Label,
    Function,
    External,
    Var,
    Const,
    List,
    ListItem,
}

pub fn globals(doc: &InkDocument) -> Globals {
    LocalsVisitor::new(doc).traverse(doc.root())
}

struct LocalsVisitor<'a> {
    doc: &'a InkDocument,
    knot: Option<&'a str>,
    stitch: Option<&'a str>,
    list: &'a str,
}

impl<'a> LocalsVisitor<'a> {
    pub fn new(doc: &'a InkDocument) -> Self {
        Self {
            doc,
            knot: None,
            stitch: None,
            list: "",
        }
    }
}

impl<'a> Visitor<'a, AllNamed<'a>> for LocalsVisitor<'a> {
    type State = Globals;

    fn visit(
        &mut self,
        node: AllNamed<'a>,
        state: &mut Self::State,
    ) -> VisitInstruction<Self::State> {
        use AllNamed::*;
        use VisitInstruction::{Descend, Ignore};
        match node {
            Knot(knot) => {
                if let Ok(identifier) = knot.name() {
                    let name = self.text(identifier);
                    state.add_definition(name, self.site(identifier), GlobalKind::Knot);
                    self.knot = Some(name);
                } else {
                    self.knot = None; // most certainly an error, but what are we going to do?
                };
                self.stitch = None;
                Ignore
            }
            Stitch(stitch) => {
                if let Ok(identifier) = stitch.name() {
                    let name = self.text(identifier);
                    let site = self.site(identifier);
                    self.stitch = Some(name);
                    let global_name = if let Some(knot) = self.knot {
                        format!("{knot}.{name}")
                    } else {
                        format!("{name}")
                    };
                    state.add_definition(global_name, site, GlobalKind::Stitch);
                }
                Ignore
            }
            Label(label) => {
                if let Ok(identifier) = label.name() {
                    let site = self.site(identifier);
                    let name = self.text(identifier);
                    let kind = GlobalKind::Label;
                    match (&self.knot, &self.stitch) {
                        (None, None) => state.add_definition(name, site, kind),
                        (None, Some(toplevel)) | (Some(toplevel), None) => {
                            state.add_definition(format!("{toplevel}.{name}"), site, kind);
                        }
                        (Some(knot), Some(stitch)) => {
                            state.add_definition(format!("{knot}.{name}"), site, kind);
                            state.add_definition(format!("{knot}.{stitch}.{name}"), site, kind);
                        }
                    }
                }
                Ignore
            }

            Global(global) => {
                if let Ok(identifier) = global.name() {
                    state.add_definition(
                        self.text(identifier),
                        self.site(identifier),
                        global
                            .keyword()
                            .ok()
                            .and_then(|kw| kw.as_const())
                            .is_some()
                            .then_some(GlobalKind::Const)
                            .unwrap_or(GlobalKind::Var),
                    );
                }
                Ignore
            }

            List(list) => {
                if let Ok(identifier) = list.name() {
                    self.list = self.text(identifier);
                    state.add_definition(self.list, self.site(identifier), GlobalKind::List);
                    Descend
                } else {
                    Ignore
                }
            }
            ListValueDef(def) => {
                if let Ok(identifier) = def.name() {
                    let list = self.list;
                    let item = self.text(identifier);
                    let site = self.site(identifier);
                    let kind = GlobalKind::ListItem;
                    // Non-qualified list item names are allowed to be ambiguous. That means, multiple
                    // list items of different lists are allowed to share the same name, but in that
                    // case their *references* must be unambiguous. This, however, is not our concern
                    // here, so we just define both names.
                    state.add_definition(format!("{list}.{item}"), site, kind);
                    state.add_definition(format!("{item}"), site, kind);
                }
                Ignore
            }

            AltArm(_)
            | Ink(_)
            | KnotBlock(_)
            | StitchBlock(_)
            | Choice(_)
            | ChoiceBlock(_)
            | ChoiceOnly(_)
            | CondArm(_)
            | CondBlock(_)
            | Gather(_)
            | GatherBlock(_)
            | ListValueDefs(_)
            | MultilineAlternatives(_) => Descend,

            BlockComment(_) | Boolean(_) | ChoiceMark(_) | ChoiceMarks(_) | Else(_) | Eol(_)
            | Alternatives(_) | Divert(_) | Content(_) | Paragraph(_) | ListValues(_)
            | Condition(_) | Binary(_) | ConditionalText(_) | Eval(_) | Code(_) | Unary(_)
            | Expr(_) | Postfix(_) | Return(_) | Tag(_) | Paren(_) | Thread(_) | Tunnel(_)
            | Assignment(_) | Call(_) | Args(_) | Params(_) | External(_) | GatherMark(_)
            | GatherMarks(_) | Glue(_) | Identifier(_) | Include(_) | LineComment(_)
            | Number(_) | Param(_) | Path(_) | QualifiedName(_) | String(_) | TempDef(_)
            | Text(_) | TodoComment(_) => Ignore,
        }
    }

    fn visit_error(&mut self, err: type_sitter::IncorrectKind) -> VisitInstruction<Self::State> {
        match err.cause() {
            type_sitter::IncorrectKindCause::Error => VisitInstruction::Descend,
            type_sitter::IncorrectKindCause::Missing => VisitInstruction::Ignore,
            type_sitter::IncorrectKindCause::OtherKind(_) => VisitInstruction::Descend,
        }
    }

    fn combine(_: &mut Self::State, _: Self::State) {
        // parent.append(&mut children); // More of a failsafe; we shouldn't actuall nest these.
        unreachable!("We don't have sub-states")
    }
}

/// Private Helpers
impl<'a> LocalsVisitor<'a> {
    fn text<N: Node<'a>>(&self, n: N) -> &'a str {
        self.doc.node_text(n)
    }

    fn site<N: Node<'a>>(&self, n: N) -> lsp_types::Range {
        self.doc.lsp_range(n.range())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use assert2::check;
    use indoc::indoc;

    #[test]
    fn gobals_may_be_nested() {
        let text = indoc! {"
            CONST const = true
            //    ^^^^^ const

            === knot ===
            = stitch
            * Globals can be defined anywhere, even nested in choices/gathers.
              VAR var = 1
            - Yes, also in gathers
              LIST however = you, (shouldnt), define, nested, globals
        "};
        let doc = InkDocument::new(text.into(), None);

        let expected = BTreeSet::from([
            "const", "var", "however", "you", "shouldnt", "define", "nested", "globals",
        ]);
        let globals = globals(&doc);
        let keys: BTreeSet<&str> = globals.0.keys().map(String::as_str).collect();
        let found_keys: BTreeSet<_> = keys.intersection(&expected).map(|it| *it).collect();
        check!(found_keys == expected);
        // NOTE: With the `asserting` crate, we could simply do this:
        // assert_that!(globals(&doc).0).contains_keys([
        //     "const", "var", "however", "you", "shouldnt", "define", "nested", "globals",
        // ]);
    }

    fn globals_for(text: &str) -> (Globals, HashMap<&str, Range>) {
        let doc = InkDocument::new(text.into(), None);
        let locs = text_annotations::scan_default_annotations(text)
            .map(|ann| (ann.claim(), ann.text_location.into()))
            .collect();
        let gbl = globals(&doc);
        (gbl, locs)
    }

    mod lists {
        use super::*;

        #[test]
        fn basic_names() {
            let (gbl, locs) = globals_for(indoc! {"
                LIST list = (item1), item2
                //   |  |    |   |   ^^^^^ item2
                //   |  |    ^^^^^ item1
                //   ^^^^ list
            "});

            check!(gbl["list"] == [(locs["list"], GlobalKind::List)]);
            check!(gbl["item1"] == [(locs["item1"], GlobalKind::ListItem)]);
            check!(gbl["item2"] == [(locs["item2"], GlobalKind::ListItem)]);
        }

        #[test]
        fn items_have_two_synonymous_global_names() {
            let (gbl, _) = globals_for("LIST list = item1, item2");
            check!(gbl["item1"] == gbl["list.item1"]);
            check!(gbl["item2"] == gbl["list.item2"]);
        }

        #[test]
        fn items_may_be_ambiguous() {
            let (gbl, _) = globals_for(indoc! {"
                LIST a = aa, shared
                LIST b = bb, shared
            "});
            check!(gbl["shared"].len() == 2);
        }
    }

    mod sections {
        use super::*;

        #[test]
        fn stitches_are_namespaced_by_knots() {
            let (gbl, loc) = globals_for(indoc! {"
                === knot_a ===
                //  ^^^^^^ knot:a
                = stitch
                //^^^^^^ stitch:a
                === knot_b ===
                //  ^^^^^^ knot:b
                = stitch
                //^^^^^^ stitch:b

            "});

            check!(gbl["knot_a"] == [(loc["knot:a"], GlobalKind::Knot)]);
            check!(gbl["knot_b"] == [(loc["knot:b"], GlobalKind::Knot)]);
            check!(gbl["knot_a.stitch"] == [(loc["stitch:a"], GlobalKind::Stitch)]);
            check!(gbl["knot_b.stitch"] == [(loc["stitch:b"], GlobalKind::Stitch)]);
        }

        #[test]
        fn labels_are_namespaced_by_knots_and_additionally_stitches() {
            let (gbl, locs) = globals_for(indoc! {"
                === knot_a ===
                - (label_1) Text
                // ^^^^^^^ a1
                = stitch
                - (label_2) Text
                // ^^^^^^^ a2

                === knot_b ===
                = stitch
                - (label_1) Text
                // ^^^^^^^ b1

            "});

            check!(gbl["knot_a.label_1"] == [(locs["a1"], GlobalKind::Label)]);

            // Labels inside stitches have 2 names:
            check!(gbl["knot_a.label_2"] == [(locs["a2"], GlobalKind::Label)]);
            check!(gbl["knot_a.stitch.label_2"] == [(locs["a2"], GlobalKind::Label)]);

            check!(gbl["knot_b.label_1"] == [(locs["b1"], GlobalKind::Label)]);
            check!(gbl["knot_b.stitch.label_1"] == [(locs["b1"], GlobalKind::Label)]);
        }
    }

    #[test]
    fn labels_outside_sections_are_not_namespaced() {
        let (gbl, locs) = globals_for(indoc! {"
            - (label) Text
            // ^^^^^ global:label
            === knot ===
            //  ^^^^ knot
            - (label) text
            // ^^^^^ knot:label
        "});

        check!(gbl.0.len() == 3);
        check!(gbl["label"] == [(locs["global:label"], GlobalKind::Label)]);
        check!(gbl["knot"] == [(locs["knot"], GlobalKind::Knot)]);
        check!(gbl["knot.label"] == [(locs["knot:label"], GlobalKind::Label)]);
    }
}
