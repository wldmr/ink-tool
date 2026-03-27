#![doc = include_str!("./root_detection_algorithm.md")]

use std::{
    collections::{HashMap, HashSet},
    mem,
    path::Path,
};

use crate::lsp::{
    idset::Id,
    location::{FileTextRange, TextPos, TextRange},
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

/// The conventional location of where the root file "imports itself"
pub static START_OF_FILE: TextRange = TextRange {
    start: TextPos {
        line: 0,
        character: 0,
    },
    end: TextPos {
        line: 0,
        character: 0,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitiveImports {
    /// Imported file -> Location of the import statement
    pub resolved: HashMap<DocId, Vec1<FileTextRange>>,
    /// File with unresolved imports -> Locations of import statements which couldn't be resolved
    pub unresolved: HashMap<DocId, Vec<TextRange>>,
}

impl TransitiveImports {
    pub fn new_root(docid: DocId) -> Self {
        let mut this = Self {
            resolved: Default::default(),
            unresolved: Default::default(),
        };
        this.resolved
            .insert(docid, Vec1::new(FileTextRange::start_of(docid)));
        this
    }

    /// Fills `self` with the transitive import relative to `root` dir.
    ///
    /// 1.  Adds `current_file`.
    ///
    /// 2.  Walks through `current_file`s include statements and tries to resolve them
    ///     relative to `root_dir`.
    ///
    ///     - If it could be resolved:
    ///
    ///       - If the resolved file turns out to already be in `roots`, take it out and
    ///         absorb it into `self`.
    ///
    ///         This is because, in a flat directory, we don’t know which one(s) is/are
    ///         the actual root file(s). So we treat each one as a root, until we learn
    ///         otherwise, and then correct it.
    ///
    ///       - Otherwise: Add it to the imports list for that target. If that list has
    ///         to be created, that means we haven’t traversed that file yet, so we
    ///         recurse into it.
    fn fill_transitive(
        &mut self,
        db: &impl Db<Ops>,
        ids: &HashMap<&Path, DocId>,
        roots: &mut StoryRoots,
        root_dir: &Path,
        current_file: DocId,
    ) {
        use std::collections::hash_map::Entry::*;

        let infos = db.node_infos(current_file);

        for (import_path, range) in infos.imported_files() {
            // BUG: We're joining URI paths with the OS path separator! This will break on windows!
            let path = root_dir.join(import_path);

            if let Some(resolved) = ids.get(path.as_path()).copied() {
                let import_location = FileTextRange::new(current_file, range);

                if let Some(mut existing) = roots.remove(&StoryRoot(resolved)) {
                    // If we import it, then it isn't really a root anymore.
                    // Therefore we have to undo the "self import" we did at the start.
                    if let Some(self_import) = existing.resolved.get_mut(&resolved) {
                        let old_first = mem::replace(self_import.first_mut(), import_location);
                        assert!(old_first.file == resolved);
                        assert!(old_first.range == START_OF_FILE);
                    }

                    for (target_file, def_sites) in existing.resolved.drain() {
                        match self.resolved.entry(target_file) {
                            Occupied(it) => it.into_mut().extend(def_sites),
                            Vacant(it) => {
                                it.insert(def_sites);
                            }
                        }
                    }

                    for (source_file, import_sites) in existing.unresolved.drain() {
                        match (&mut *self).unresolved.entry(source_file) {
                            Occupied(it) => it.into_mut().extend(import_sites),
                            Vacant(it) => {
                                it.insert(import_sites);
                            }
                        }
                    }
                } else {
                    match self.resolved.entry(resolved) {
                        Occupied(it) => {
                            it.into_mut().push(import_location);
                        }
                        Vacant(it) => {
                            it.insert(Vec1::new(import_location));
                            // We haven't seen this file before, so we recursively add its imports.
                            self.fill_transitive(db, ids, roots, root_dir, resolved);
                        }
                    };
                };
            } else {
                self.unresolved.entry(current_file).or_default().push(range);
            }
        }
    }
}

impl Subquery<Ops, StoryRoots> for stories {
    fn value(&self, db: &impl Db<Ops>, old: Old<StoryRoots>) -> Updated<StoryRoots> {
        let docs = db.doc_ids();

        // We sort the candidates by depth, then by name. This gives us two properties:
        //
        // 1.  Files early in this iteration are more likely to be roots.
        // 2.  Each transitive closure can only contain files at the same or higher depth.
        let mut candidates = docs
            .pairs()
            .map(|(id, uri)| {
                let path = Path::new(uri.path().as_str());
                let dir = path.parent().expect("Each uri must point to a file");
                let fname = path.file_name().expect("Each uri must point to a file");
                let depth = dir.components().count();
                (depth, dir, fname, path, id)
            })
            .collect_vec();
        candidates.sort_unstable();

        let paths: HashMap<&Path, DocId> = candidates
            .iter()
            .map(|(_, _, _, path, id)| (*path, *id))
            .collect();

        let mut roots = StoryRoots::new();
        let mut already_imported = HashSet::<DocId>::new();

        for (_, root_dir, _, _, id) in candidates {
            if already_imported.contains(&id) {
                continue;
            }

            let mut imports = TransitiveImports::new_root(id);
            imports.fill_transitive(db, &paths, &mut roots, root_dir, id);

            already_imported.extend(imports.resolved.keys());
            roots.insert(StoryRoot(id), imports);
        }

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
