use crate::lsp::{
    salsa::InkGetters,
    state::{DocumentNotFound, GotoLocationError},
};
use lsp_types::{Location, Position, Uri};

impl super::State {
    pub fn goto_references(
        &self,
        from_uri: Uri,
        from_position: Position,
    ) -> Result<Vec<Location>, GotoLocationError> {
        let docs = self.db.doc_ids();
        let docid = docs
            .get_id(&from_uri)
            .ok_or_else(|| DocumentNotFound(from_uri.clone()))?;
        let doc = self.db.document(docid);

        let mut references = Vec::new();

        if let Some(usage) = doc.usage_at(from_position) {
            let def = self.db.definition(docid, usage.ident.into());
            for (def_doc, def) in def.iter().copied() {
                let usages = self.db.usages(def_doc, def);
                for (usgdoc, usgid) in usages.iter() {
                    let locs = self.db.node_locations(*usgdoc);
                    let uri = docs[*usgdoc].clone();
                    let range = locs[*usgid].into();
                    references.push(Location::new(uri, range));
                }
            }
        }
        Ok(references)
    }
}
