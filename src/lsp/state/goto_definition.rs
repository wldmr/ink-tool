use crate::lsp::{
    salsa::InkGetters as _,
    state::{DocumentNotFound, GotoLocationError},
};
use lsp_types::{Position, Uri};

impl super::State {
    pub fn goto_definition(
        &self,
        uri: Uri,
        pos: Position,
    ) -> Result<Vec<lsp_types::Location>, GotoLocationError> {
        let location = lsp_types::Location::new;
        let docs = self.db.doc_ids();
        let this = docs
            .get_id(&uri)
            .ok_or_else(|| DocumentNotFound(uri.clone()))?;
        let doc = self.db.document(this);

        let mut found = Vec::new();

        if let Some(usage) = doc.usage_at(pos) {
            // We "allow" abiguous definitions, since we can't know which definition the user meant
            // (there'll be an error message and they'll have to fix it).
            if let Some(local_def) = self.db.locals(this).name_at(usage.term, pos) {
                for (range, _kind) in local_def {
                    found.push(location(uri.clone(), *range));
                }
            } else {
                for docid in docs.ids() {
                    if let Some(defs) = self.db.globals(docid).get(usage.term) {
                        for (range, _kind) in defs {
                            found.push(location(docs[docid].clone(), *range));
                        }
                    }
                }
            }
        }

        Ok(found)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lsp::state::tests::{new_state, uri},
        test_utils::Compact,
    };
    use assert2::check;
    use itertools::Itertools;
    use std::collections::HashMap;

    #[test]
    fn next_label() {
        let text = "\
            -> knot.stitch.label
            //             ^^^^^ usage label
            == knot
            = stitch
            -> (label)
            //  ^^^^^ usage label

            - (label)
            // ^^^^^ definition label
            // ^^^^^ usage label
        ";
        let mut state = new_state().with_comment_separated_files(text);
        state.edit(uri("main.ink"), text);

        let mut defs: HashMap<&str, lsp_types::Range> = HashMap::new();
        let mut usages: HashMap<&str, Vec<lsp_types::Range>> = HashMap::new();

        for ann in text_annotations::scan_default_annotations(text) {
            match ann.claim().split_once(' ') {
                Some(("definition", name)) => {
                    defs.insert(name, ann.text_location.into());
                }
                Some(("goto-definition", name)) => {
                    usages
                        .entry(name)
                        .or_default()
                        .push(ann.text_location.into());
                }
                _ => {}
            }
        }

        let actual = state
            .goto_references(uri("main.ink"), defs["label"].start)
            .unwrap()
            .into_iter()
            .map(|it| Compact(it.range))
            .collect_vec();
        let expected = usages["label"].iter().copied().map(Compact).collect_vec();

        check!(actual == expected);
    }
}
