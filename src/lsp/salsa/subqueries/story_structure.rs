#![doc = include_str!("./root_detection_algorithm.md")]

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::lsp::{
    idset::Id,
    salsa::{
        relative_imports, stories_of, story_roots, transitive_imports, DocId, InkGetters, Ops,
    },
};
use lsp_types::Uri;
use mini_milc::{Db, Old, Subquery, Updated};
use tree_traversal::TreeTraversal;
use util::nonempty::Vec1;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, derive_more::Into)]
pub struct StoryRoot(Id<Uri>);
impl PartialEq<Id<Uri>> for StoryRoot {
    fn eq(&self, other: &Id<Uri>) -> bool {
        self.0 == *other
    }
}

type StoryRoots = HashSet<StoryRoot>;
type TransitiveImports = HashSet<Result<DocId, DocId>>;

impl Subquery<Ops, TransitiveImports> for transitive_imports {
    fn value(&self, db: &impl Db<Ops>, old: Old<TransitiveImports>) -> Updated<TransitiveImports> {
        let doc_ids = db.doc_ids();
        let path = Path::new(doc_ids[self.root.0].path().as_str());
        let root_dir = path.parent().expect("Each uri must point to a file");
        log::debug!(
            "Creating transitive imports for {:?}={path:?} with root dir {root_dir:?}",
            self.root
        );

        let ids: HashMap<&Path, Id<Uri>> = doc_ids
            .pairs()
            .map(|(id, uri)| (Path::new(uri.path().as_str()), id))
            .collect();

        let mut result = TransitiveImports::new();
        fill_transitive(&mut result, db, &ids, root_dir, self.root.0);
        log::debug!("Transitive imports: {result:?}");
        old.update(result)
    }
}

fn fill_transitive(
    imported: &mut TransitiveImports,
    db: &impl Db<Ops>,
    ids: &HashMap<&Path, Id<Uri>>,
    root_dir: &Path,
    current_file: DocId,
) {
    let current_path = db.short_path(current_file);
    let current_path = current_path.as_str();
    log::debug!("Adding current file: {current_file:?}:{current_path}",);
    imported.insert(Ok(current_file));

    for import in db.relative_imports(current_file).iter() {
        let path = root_dir.join(import);
        log::debug!("Resolved import {import} to {path:?}");
        let imported_file_id = ids.get(path.as_path());
        log::debug!("Resolved import {import} to {imported_file_id:?}");

        if let Some(child) = imported_file_id {
            fill_transitive(imported, db, ids, root_dir, *child);
        } else {
            imported.insert(Err(current_file));
            log::warn!("Could not resolve import {import} in {current_file:?}:{current_path}",);
            continue;
        };
    }
}

impl Subquery<Ops, HashSet<String>> for relative_imports {
    fn value(&self, db: &impl Db<Ops>, old: Old<HashSet<String>>) -> Updated<HashSet<String>> {
        log::debug!("Finding realitve imports for {:?}", self.docid);
        let doc = db.document(self.docid);
        let new: HashSet<String> = doc
            .root()
            .depth_first::<ink_syntax::Include>()
            .inspect(|it| log::debug!("Found {it:?}"))
            .map(|it| doc.node_text(it.path()).to_string())
            .collect();
        log::debug!("Relative imports: {new:?}");
        old.update(new)
    }
}

impl Subquery<Ops, StoryRoots> for story_roots {
    fn value(&self, db: &impl Db<Ops>, old: Old<StoryRoots>) -> Updated<StoryRoots> {
        let docs = db.doc_ids();
        let mut imported = HashSet::new();
        for id in docs.ids() {
            imported.extend(
                db.transitive_imports(StoryRoot(id))
                    .iter()
                    .filter_map(|it| it.as_ref().ok())
                    .filter(|it| **it != id) // "self-imports" aren't considered here
                    .copied(),
            );
        }
        log::debug!("All imported somewhere {imported:?}");
        let roots = docs
            .ids()
            .filter(|it| !imported.contains(it))
            .map(StoryRoot)
            .collect::<StoryRoots>();
        log::debug!("All roots {roots:?}");
        old.update(roots)
    }
}

impl Subquery<Ops, Vec1<StoryRoot>> for stories_of {
    fn value(&self, db: &impl Db<Ops>, old: Old<Vec1<StoryRoot>>) -> Updated<Vec1<StoryRoot>> {
        let roots = db.story_roots();
        let iter = roots.iter().filter(|it| {
            db.transitive_imports(**it)
                .iter()
                .any(|any| any.is_ok_and(|it| it == self.docid))
        });
        let new = Vec1::from_iter(iter.copied())
            .expect("Every document must belong to at least one story, by definition");
        old.update(new)
    }
}
