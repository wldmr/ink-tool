use ink_document::ids::{DefId, ScopeId, UsageId};
use itertools::Itertools as _;
use mini_milc::{Db, Old, Subquery, Updated};
use util::nonempty::{MapOfNonEmpty, Vec1};

use crate::lsp::{
    salsa::{
        local_resolutions,
        subqueries::ink_inventory::{IMap, Name, NameMap},
    },
    InkGetters as _, Ops,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LocalResolutions {
    /// Local usage → definitions mapping.
    pub definitions: IMap<UsageId, Vec1<DefId>>,
    /// Local definition → usages mapping.
    pub usages: IMap<DefId, Vec1<UsageId>>,
    /// The *locally* unresolved names in this file (i.e. they might refer to globals)
    pub unresolved: IMap<Name, Vec1<UsageId>>,
    /// The local names defined in a particular scope, including “scoped” ones like
    /// “subsection.label”
    ///
    /// Note that these are not pared down by whether the are resolvable or not (such as
    /// a knot label being shadowed by a subsection name). This is mostly for simplicity
    /// and owed to the fact that these names are meant for completions, where
    /// overlapping names will usually result in the “correct” thing anyway.
    pub in_scope: IMap<ScopeId, Vec<(Name, DefId)>>,
}

impl Subquery<Ops, LocalResolutions> for local_resolutions {
    fn value(&self, db: &impl Db<Ops>, old: Old<LocalResolutions>) -> Updated<LocalResolutions> {
        let mut result = LocalResolutions::default();
        let inv = db.ink_inventory(self.docid);

        result.in_scope.entry(inv.scope_id).or_default().extend(
            inv.body
                .temps
                .iter()
                .flat_map(|(name, defs)| defs.iter().map(|def| (*name, *def))),
        );

        for (name, usages) in &inv.body.usages {
            let resolved = result.resolve(&inv.body.temps, name, usages);
            if !resolved {
                result.unresolved(name, usages);
            }
        }

        for section in &inv.sections {
            // We need to know about subsections and their labels to determine what a name actually means.
            let mut sub_names: NameMap<Vec1<DefId>> = NameMap::default();
            let mut sub_labels: NameMap<Vec1<DefId>> = NameMap::default();
            for sub in &section.subsections {
                let subname = sub.name;
                sub_names.register(sub.name, sub.name_id);
                for (labelname, defs) in &sub.body.labels {
                    let labelname = *labelname;
                    let namespaced_label = Name::from(format!("{subname}.{labelname}"));
                    for def in defs.iter().copied() {
                        sub_labels.register(labelname, def);
                        sub_labels.register(namespaced_label, def);
                    }
                }
            }

            result.in_scope.entry(section.scope_id).or_default().extend(
                std::iter::empty()
                    .chain(&sub_labels)
                    .chain(&sub_names)
                    .chain(&section.params)
                    .chain(&section.body.temps)
                    .flat_map(|(name, defs)| defs.iter().map(|def| (*name, *def))),
            );

            for (name, usages) in &section.body.usages {
                let mut resolved = false;
                resolved |= result.resolve(&section.params, name, usages);
                resolved |= result.resolve(&section.body.temps, name, usages);
                resolved |= result.resolve(&section.body.labels, name, usages);
                if !resolved {
                    resolved |= result.resolve(&sub_names, name, usages);
                }
                if !resolved {
                    resolved |= result.resolve(&sub_labels, name, usages);
                }
                if !resolved {
                    result.unresolved(name, usages);
                }
            }

            for subsec in &section.subsections {
                result.in_scope.entry(subsec.scope_id).or_default().extend(
                    std::iter::empty()
                        .chain(&subsec.params)
                        .chain(&subsec.body.temps)
                        // subsection and label already visibility handled at the section level.
                        .flat_map(|(name, defs)| defs.iter().map(|def| (*name, *def))),
                );

                for (name, usages) in &subsec.body.usages {
                    let mut resolved = false;
                    resolved |= result.resolve(&subsec.params, name, usages);
                    resolved |= result.resolve(&subsec.body.temps, name, usages);
                    resolved |= result.resolve(&subsec.body.labels, name, usages);
                    // XXX: I *think* the priority here is section body labels > subsections > subsection labels
                    if !resolved {
                        resolved |= result.resolve(&section.body.labels, name, usages);
                    }
                    if !resolved {
                        resolved |= result.resolve(&sub_names, name, usages);
                    }
                    if !resolved {
                        resolved |= result.resolve(&sub_labels, name, usages);
                    }
                    if !resolved {
                        result.unresolved(name, usages);
                    }
                }
            }
        }
        old.update(result)
    }
}

impl LocalResolutions {
    /// If `name` is contained in `names`, resolve it to its definiton and return `true`.
    /// Otherwise return `false.`
    fn resolve(
        &mut self,
        names: &NameMap<Vec1<DefId>>,
        name: &Name,
        usages: &Vec1<UsageId>,
    ) -> bool {
        if let Some(defs) = names.get(name) {
            for (def, usage) in defs.iter().cartesian_product(usages) {
                self.definitions.register(*usage, *def);
                self.usages.register(*def, *usage);
            }
            true
        } else {
            false
        }
    }

    fn unresolved(&mut self, name: &Name, usages: &Vec1<UsageId>) {
        self.unresolved
            .register_extend(*name, usages.iter().copied());
    }
}
