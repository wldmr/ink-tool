use crate::lsp::{
    salsa::InkGetters as _,
    state::{DocumentNotFound, GotoLocationError},
};
use lsp_types::{Location, Position, Range, Uri};
use tree_traversal::TreeTraversal;

impl super::State {
    pub fn goto_definition(
        &self,
        uri: Uri,
        pos: Position,
    ) -> Result<Vec<Location>, GotoLocationError> {
        let location = |uri: &Uri, range: &Range| Location::new(uri.clone(), *range);
        let docs = self.db.doc_ids();
        let this = docs
            .get_id(&uri)
            .ok_or_else(|| DocumentNotFound(uri.clone()))?;
        let doc = self.db.document(this);

        let mut found = Vec::new();

        let Some(usage) = doc.usage_at(pos) else {
            return Ok(found); // no usage under cursor => we found nothing
        };

        // We "allow" abiguous definitions, since we can't know which definition the user meant
        // (there'll be an error message and they'll have to fix it).

        let defs = self.db.definitions(this);
        // Walk up the tree, stop once we find matching locals.
        for (nth, parent) in usage.ident.ascend_to(doc.root()).enumerate() {
            if let Some(local_defs) = defs.local(usage.term, parent) {
                for (range, kind) in local_defs {
                    // Only the innermost scope includes temps
                    if !kind.is_temp() || nth == 0 {
                        found.push(location(&uri, range));
                    }
                }
                if !found.is_empty() {
                    return Ok(found);
                }
            }
        }

        // No locals found, so let's look for globals.
        for docid in docs.ids() {
            if let Some(defs) = self.db.definitions(docid).global(usage.term) {
                for (range, _kind) in defs {
                    found.push(location(&docs[docid], range));
                }
            }
        }

        return Ok(found);
    }
}

