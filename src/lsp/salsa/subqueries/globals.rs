use std::collections::HashMap;

use ink_document::ids::DefId;
use mini_milc::{subquery, Db, Old, Subquery, Updated};
use util::nonempty::{MapOfNonEmpty as _, Vec1};

use crate::lsp::{
    salsa::{file_globals, global_names, globals, Def, Name, NameMap},
    InkGetters as _, Ops,
};

impl Subquery<Ops, NameMap<Vec1<DefId>>> for file_globals {
    fn value(
        &self,
        db: &impl Db<Ops>,
        old: Old<NameMap<Vec1<DefId>>>,
    ) -> Updated<NameMap<Vec1<DefId>>> {
        let mut result = NameMap::default();
        let inv = db.ink_inventory(self.docid);

        for list in &inv.lists {
            result.register(list.name, list.id);
            // List items are globally visible without the preceding list name
            for (item, defs) in &list.items {
                for def in defs {
                    result.register(*item, *def);
                    result.register(format!("{list}.{item}"), *def);
                }
            }
        }

        let globals = std::iter::empty() // just so the chain looks more uniform ;)
            .chain(&inv.vars)
            .chain(&inv.consts)
            .chain(&inv.externals)
            .chain(&inv.body.labels);

        for (name, defs) in globals {
            for def in defs {
                result.register(*name, *def);
            }
        }

        for toplevel in &inv.sections {
            result.register(toplevel.name, toplevel.name_id);
            for (label, defs) in &toplevel.body.labels {
                // Subsection names take precedence over labels
                if !toplevel.sub_names.contains(label) {
                    for def in defs {
                        result.register(format!("{toplevel}.{label}"), *def);
                    }
                }
            }

            for subsection in &toplevel.subsections {
                result.register(format!("{toplevel}.{subsection}"), subsection.name_id);
                for (label, defs) in &subsection.body.labels {
                    for def in defs {
                        result.register(format!("{toplevel}.{subsection}.{label}"), *def);

                        // The logic for the shortcut version is a bit convoluted.
                        let shortcut = Name::from(format!("{toplevel}.{label}"));
                        // Shortcut is allowed if there's no subsection with that name.
                        let mut shortcut_allowed = !toplevel.sub_names.contains(label);

                        // And if we haven’t defined that shortcut already. (Multiple subsection labels may
                        // exist, but only the first one can be the shortcut.)
                        shortcut_allowed &= !result.contains_key(&shortcut);

                        // However, if the main section already defines such a label, then that’s an error.
                        // In that case, we (counter-intuitively) *do* allow all shortcuts, to allow the
                        // user to navigate between the duplicate definitions.
                        if toplevel.body.labels.contains_key(label)
                            && !toplevel.sub_names.contains(label)
                        {
                            shortcut_allowed = true;
                        }

                        if shortcut_allowed {
                            result.register(shortcut, *def);
                        }
                    }
                }
            }
        }
        old.update(result)
    }
}

subquery!(Ops, globals, NameMap<Vec1<Def>>, |self, db| {
    let mut result = NameMap::default();
    let stories = db.stories();
    for docid in stories[&self.story].resolved.keys() {
        let globals = db.file_globals(*docid);
        for (name, defs) in globals.iter() {
            result.register_extend(*name, defs.into_iter().map(|def| (*docid, *def)));
        }
    }
    result
});

type DefMap = HashMap<Def, Vec1<Name>>; // Seems like rust-analyzer can't format the following without this alias.
subquery!(Ops, global_names, DefMap, |self, db| {
    let mut result = HashMap::new();
    let globals = db.globals(self.story);
    for (name, defs) in globals.iter() {
        for def in defs {
            result.register(*def, *name);
        }
    }
    result
});
