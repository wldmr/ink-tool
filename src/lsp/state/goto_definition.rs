use crate::lsp::{
    salsa::InkGetters as _,
    state::{DocumentNotFound, GotoLocationError},
};
use itertools::Itertools;
use lsp_types::{Location, Position, Range, Uri};

impl super::State {
    pub fn goto_definition(
        &self,
        uri: Uri,
        pos: Position,
    ) -> Result<Vec<Location>, GotoLocationError> {
        let docs = self.db.doc_ids();
        let this_docid = docs
            .get_id(&uri)
            .ok_or_else(|| DocumentNotFound(uri.clone()))?;
        let doc = self.db.document(this_docid);

        let mut defs = Vec::new();

        if let Some(usage) = doc.usage_at(pos) {
            let found = self.db.definition(this_docid, usage.ident.into());
            for (defdoc, defid) in found.iter() {
                let uri = docs[*defdoc].clone();
                let range = self.db.node_locations(*defdoc)[*defid].into();
                defs.push(Location::new(uri, range))
            }
        } else {
            // Maybe we're over an import line?
            let is_import = self
                .db
                .imported_files(this_docid)
                .iter()
                .any(|(_, range)| range.start.line == pos.line); // imports always occupy the whole line, no need to be pernickety

            if is_import {
                let stories = self.db.stories();
                let parents = self.db.stories_of(this_docid);

                let targets = parents.iter().flat_map(|root| {
                    stories[root]
                        .resolved
                        .iter()
                        .filter(|(target, _)| **target != this_docid) // ignore the implicit "self import"
                        .flat_map(|(target, defs)| defs.iter().copied().map(|it| (*target, it)))
                        .filter(|(_, def)| {
                            def.file == this_docid && def.range.start.line == pos.line
                        })
                        .map(|(target, _)| target)
                });

                let targets = targets
                    .unique() // Deduplicate in case of multiple overlapping stories (or accidental overlapping imports)
                    .map(|target| docs[target].clone())
                    .map(|uri| Location::new(uri, Range::default())); // we send the user to the start of the file.
                defs.extend(targets);
            }
        }

        Ok(defs)
    }
}
