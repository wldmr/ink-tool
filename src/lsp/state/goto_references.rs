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
            let def = self.db.definition_of(docid, usage.range.into());
            for (def_doc, def) in def.iter().copied() {
                let usages = self.db.usages_of(def_doc, def);
                references.extend(
                    usages
                        .iter()
                        .copied()
                        .map(|(docid, range)| Location::new(docs[docid].clone(), range.into())),
                );
            }
        }
        Ok(references)
    }
}
