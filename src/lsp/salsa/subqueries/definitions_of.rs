use crate::lsp::salsa::{resolve_definition, DocId, InkGetters as _, Ops};
use lsp_types::Range;
use mini_milc::{Db, Old, Subquery, Updated};
use tree_traversal::TreeTraversal as _;

impl Subquery<Ops, Vec<(DocId, Range)>> for resolve_definition {
    fn value(
        &self,
        db: &impl Db<Ops>,
        old: Old<Vec<(DocId, Range)>>,
    ) -> Updated<Vec<(DocId, Range)>> {
        let doc = db.document(self.docid);
        let Some(usage) = doc.usage_at(self.pos) else {
            return old.reuse_or_default();
        };
        let defs = db.definitions(self.docid);

        let mut found = Vec::new();

        // We "allow" ambiguous definitions, since we can't know which definition the user meant
        // (there'll be an error message and they'll have to fix it).

        // Walk up the tree, stop once we find matching locals.

        for (nth, parent) in usage.ident.ascend_to(doc.root()).enumerate() {
            if let Some(local_defs) = defs.local(usage.term, parent) {
                for (range, kind) in local_defs {
                    // Only the innermost scope includes temps
                    if !kind.is_temp() || nth == 0 {
                        found.push((self.docid, *range));
                    }
                }
                if !found.is_empty() {
                    return old.update(found);
                }
            }
        }

        // No locals found, so let's look for globals.
        for docid in db.doc_ids().ids() {
            if let Some(defs) = db.definitions(docid).global(usage.term) {
                for (range, _kind) in defs {
                    found.push((docid, *range));
                }
            }
        }
        old.update(found)
    }
}
