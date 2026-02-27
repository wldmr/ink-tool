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
        }

        Ok(defs)
    }
}
