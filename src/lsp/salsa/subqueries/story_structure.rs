#![doc = include_str!("./root_detection_algorithm.md")]

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::lsp::{
    idset::Id,
    location::{FileTextRange, TextRange},
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TransitiveImports {
    /// Imported file -> Location of the import statement
    pub resolved: HashMap<DocId, Vec1<FileTextRange>>,
    /// File with unresolved imports -> Locations of import statements which couldn't be resolved
    pub unresolved: HashMap<DocId, Vec<TextRange>>,
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

        let mut result = TransitiveImports::default();
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
    imported
        .resolved
        .entry(current_file)
        .and_modify(|vec1| vec1.push(FileTextRange::start_of(current_file)))
        .or_insert_with(|| Vec1::new(FileTextRange::start_of(current_file)));

    let doc = db.document(current_file);
    let infos = db.node_infos(current_file);

    for range in infos.imported_files() {
        let import = doc.lsp_text(range);
        let path = root_dir.join(import);
        if let Some(resolved) = ids.get(path.as_path()).copied() {
            let import_location = FileTextRange::new(current_file, range);
            let imports = imported
                .resolved
                .entry(resolved)
                .and_modify(|vec1| vec1.push(import_location))
                .or_insert_with(|| Vec1::new(import_location));
            if imports.len() > 1 {
                let uris = db.doc_ids();
                let resolved_uri = uris[resolved].path().as_str();
                let source_uri = uris[import_location.file].path().as_str();
                let source_range = import_location.range;
                log::warn!("{source_uri}:{source_range} : Duplicate (or circular) import of {resolved_uri}");
            } else {
                fill_transitive(imported, db, ids, root_dir, resolved);
            }
        } else {
            imported
                .unresolved
                .entry(current_file)
                .or_default()
                .push(range);
        }
    }
}

impl Subquery<Ops, StoryRoots> for story_roots {
    fn value(&self, db: &impl Db<Ops>, old: Old<StoryRoots>) -> Updated<StoryRoots> {
        let docs = db.doc_ids();
        let mut imported = HashSet::new();
        let mut circular = HashSet::new();
        for id in docs.ids() {
            let transitive_imports = db.transitive_imports(StoryRoot(id));
            imported.extend(
                transitive_imports
                    .resolved
                    .keys()
                    .copied()
                    .filter(|import| *import != id), // "self-imports" aren't considered here
            );
            if let Some(self_imports) = transitive_imports.resolved.get(&id) {
                let default = FileTextRange::start_of(id);
                let is_circular = self_imports.iter().any(|it| *it != default);
                if is_circular {
                    circular.insert(id);
                }
            }
        }
        let roots = docs
            .ids()
            .filter(|it| !imported.contains(it))
            .chain(circular)
            .map(StoryRoot)
            .collect::<StoryRoots>();
        old.update(roots)
    }
}

impl Subquery<Ops, Vec1<StoryRoot>> for stories_of {
    fn value(&self, db: &impl Db<Ops>, old: Old<Vec1<StoryRoot>>) -> Updated<Vec1<StoryRoot>> {
        let roots = db.story_roots();
        let parents = roots.iter().filter(|it| {
            db.transitive_imports(**it)
                .resolved
                .contains_key(&self.docid)
        });
        let new = Vec1::from_iter(parents.copied())
            .expect("Every document must belong to at least one story, by definition");
        old.update(new)
    }
}
