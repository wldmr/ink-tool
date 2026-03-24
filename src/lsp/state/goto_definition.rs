use std::collections::HashSet;

use crate::lsp::{
    salsa::InkGetters as _,
    state::{DocumentNotFound, GotoLocationError},
};
use lsp_types::{Location, Position, Uri};

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
                .any(|it| it.start.line == pos.line); // imports are always a single line.

            if is_import {
                let stories = self.db.stories_of(this_docid);

                // In case this file is in several stories, we don’t want to add repeated target
                // files several times. Therefore we must keep track of the target files we’ve
                // already seen.
                let mut seen = HashSet::new();
                for story in stories.iter().copied() {
                    let transitive_imports = self.db.transitive_imports(story);
                    let targets = transitive_imports
                        .iter()
                        .filter(|import| {
                            log::debug!("Considering import {import:?}");
                            import.importer == this_docid && import.range.start.line == pos.line
                        })
                        .filter_map(|import| import.target)
                        .filter(|target| *target != this_docid);

                    for target in targets {
                        if seen.insert(target) {
                            // we put the user at the start of the file
                            defs.push(Location::new(
                                docs[target].clone(),
                                lsp_types::Range::default(),
                            ));
                        }
                    }
                }
            }
        }

        Ok(defs)
    }
}
