#![doc = include_str!("./root_detection_algorithm.md")]

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::lsp::{
    idset::Id,
    salsa::{stories_of, story_roots, transitive_imports, DocId, InkGetters, Ops},
};
use lsp_types::Uri;
use mini_milc::{Db, Old, Subquery, Updated};
use util::nonempty::Vec1;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, derive_more::Into)]
pub struct StoryRoot(Id<Uri>);
impl PartialEq<Id<Uri>> for StoryRoot {
    fn eq(&self, other: &Id<Uri>) -> bool {
        self.0 == *other
    }
}

pub(crate) type StoryRoots = HashSet<StoryRoot>;
pub(crate) type TransitiveImports = HashSet<TransitiveImport>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TransitiveImport {
    pub target: Option<DocId>,
    pub importer: DocId,
    pub range: lsp_types::Range,
}

impl Subquery<Ops, TransitiveImports> for transitive_imports {
    fn value(&self, db: &impl Db<Ops>, old: Old<TransitiveImports>) -> Updated<TransitiveImports> {
        let doc_ids = db.doc_ids();
        let path = Path::new(doc_ids[self.root.0].path().as_str());
        let root_dir = path.parent().expect("Each uri must point to a file");

        let ids: HashMap<&Path, Id<Uri>> = doc_ids
            .pairs()
            .map(|(id, uri)| (Path::new(uri.path().as_str()), id))
            .collect();

        let mut result = TransitiveImports::new();
        fill_transitive(&mut result, db, &ids, root_dir, self.root.0);
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
    imported.insert(TransitiveImport {
        target: Some(current_file),
        importer: current_file,
        range: Default::default(),
    });

    let doc = db.document(current_file);
    let infos = db.node_infos(current_file);

    for range in infos.imported_files() {
        let import = doc.lsp_text(range);
        let path = root_dir.join(import);
        let maybe_target = ids.get(path.as_path());

        imported.insert(TransitiveImport {
            target: maybe_target.cloned(), // None means unresolved
            importer: current_file,
            range: range.into(),
        });

        if let Some(target) = maybe_target {
            fill_transitive(imported, db, ids, root_dir, *target);
        };
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
                    .filter_map(|it| it.target.as_ref())
                    .filter(|it| **it != id) // "self-imports" aren't considered here
                    .copied(),
            );
        }
        let roots = docs
            .ids()
            .filter(|it| !imported.contains(it))
            .map(StoryRoot)
            .collect::<StoryRoots>();
        old.update(roots)
    }
}

impl Subquery<Ops, Vec1<StoryRoot>> for stories_of {
    fn value(&self, db: &impl Db<Ops>, old: Old<Vec1<StoryRoot>>) -> Updated<Vec1<StoryRoot>> {
        let roots = db.story_roots();
        let iter = roots.iter().filter(|it| {
            db.transitive_imports(**it)
                .iter()
                .any(|import| import.target.is_some_and(|it| it == self.docid))
        });
        let new = Vec1::from_iter(iter.copied())
            .expect("Every document must belong to at least one story, by definition");
        old.update(new)
    }
}
