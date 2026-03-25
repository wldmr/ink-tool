#![doc = include_str!("./root_detection_algorithm.md")]

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::lsp::{
    idset::Id,
    location::{FileTextRange, TextRange},
    salsa::{stories, stories_of, DocId, InkGetters, Ops},
};
use derive_more::{AsRef, Into};
use itertools::Itertools;
use lsp_types::Uri;
use mini_milc::{Db, Old, Subquery, Updated};
use util::nonempty::Vec1;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Into, AsRef)]
pub struct StoryRoot(DocId);

impl PartialEq<Id<Uri>> for StoryRoot {
    fn eq(&self, other: &DocId) -> bool {
        self.0 == *other
    }
}
impl std::fmt::Debug for StoryRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Root-{:?}", self.0)
    }
}

pub type StoryRoots = HashMap<StoryRoot, TransitiveImports>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TransitiveImports {
    /// Imported file -> Location of the import statement
    pub resolved: HashMap<DocId, Vec1<FileTextRange>>,
    /// File with unresolved imports -> Locations of import statements which couldn't be resolved
    pub unresolved: HashMap<DocId, Vec<TextRange>>,
}

impl TransitiveImports {
    fn fill_transitive(
        &mut self,
        db: &impl Db<Ops>,
        ids: &HashMap<&Path, Id<Uri>>,
        root_dir: &Path,
        current_file: DocId,
    ) {
        self.resolved
            .entry(current_file)
            .and_modify(|vec1| vec1.push(FileTextRange::start_of(current_file)))
            .or_insert_with(|| Vec1::new(FileTextRange::start_of(current_file)));

        let infos = db.node_infos(current_file);
        for (import, range) in infos.imported_files() {
            let path = root_dir.join(import);
            if let Some(resolved) = ids.get(path.as_path()).copied() {
                let import_location = FileTextRange::new(current_file, range);
                let imports = self
                    .resolved
                    .entry(resolved)
                    .and_modify(|vec1| vec1.push(import_location))
                    .or_insert_with(|| Vec1::new(import_location));
                if imports.len() > 1 {
                    continue; // we've already sees this import.
                } else {
                    self.fill_transitive(db, ids, root_dir, resolved);
                }
            } else {
                self.unresolved.entry(current_file).or_default().push(range);
            }
        }
    }
}

impl Subquery<Ops, StoryRoots> for stories {
    fn value(&self, db: &impl Db<Ops>, old: Old<StoryRoots>) -> Updated<StoryRoots> {
        let docs = db.doc_ids();
        let ids: HashMap<&Path, Id<Uri>> = docs
            .pairs()
            .map(|(id, uri)| (Path::new(uri.path().as_str()), id))
            .collect();
        let paths: HashMap<Id<Uri>, &Path> = ids.iter().map(|(path, id)| (*id, *path)).collect();

        let mut roots = StoryRoots::new();
        let mut imported = HashSet::<DocId>::new();
        let mut circular = HashSet::<DocId>::new();

        for candidate in docs.ids() {
            let root_dir = paths[&candidate]
                .parent()
                .expect("Each uri must point to a file");

            let mut imports = TransitiveImports::default();
            imports.fill_transitive(db, &ids, root_dir, candidate);

            if imports.resolved[&candidate].len() > 1 {
                circular.insert(candidate);
            }

            imported.extend(imports.resolved.keys().filter(|key| **key != candidate));
            roots.insert(StoryRoot(candidate), imports);
        }

        roots.retain(|root, _| circular.contains(&root.0) || !imported.contains(&root.0));
        old.update(roots)
    }
}

impl Subquery<Ops, Vec1<StoryRoot>> for stories_of {
    fn value(&self, db: &impl Db<Ops>, old: Old<Vec1<StoryRoot>>) -> Updated<Vec1<StoryRoot>> {
        let roots = db.stories();
        let parents = roots
            .iter()
            .filter_map(|(root, closure)| {
                closure.resolved.contains_key(&self.docid).then_some(root)
            })
            .unique();
        let new = Vec1::from_iter(parents.copied())
            .expect("Every document must belong to at least one story, by definition");
        old.update(new)
    }
}
