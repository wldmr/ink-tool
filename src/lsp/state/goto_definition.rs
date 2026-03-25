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
            let found = self.db.definition_of(this_docid, usage.range.into());
            let locations = found
                .iter()
                .copied()
                .map(|(docid, range)| Location::new(docs[docid].clone(), range.into()));
            defs.extend(locations)
        } else {
            // Maybe we're over an import line?
            let is_import = self
                .db
                .node_infos(this_docid)
                .imported_files()
                .any(|(_, range)| range.contains_pos(pos));

            if is_import {
                let stories = self.db.stories();
                let parents = self.db.stories_of(this_docid);

                let targets = parents.iter().flat_map(|root| {
                    stories[root]
                        .resolved
                        .iter()
                        .filter(|(target, _)| **target != this_docid) // ignore the implicit "self import"
                        .flat_map(|(target, defs)| defs.iter().copied().map(|it| (*target, it)))
                        .filter(|(_, def)| def.file == this_docid && def.range.contains_pos(pos))
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
