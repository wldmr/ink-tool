use crate::lsp::{
    salsa::InkGetters as _,
    state::{DocumentNotFound, GotoLocationError, InvalidPosition},
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
        let usage = doc.usage_at(pos).ok_or_else(|| InvalidPosition(pos))?;

        Ok(self
            .db
            .resolve_definition(this_docid, usage.range.start)
            .iter()
            .copied()
            .map(|(docid, range)| Location::new(docs[docid].clone(), range))
            .collect())
    }
}
