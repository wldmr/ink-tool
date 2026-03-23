#![allow(unused)]

#[doc = include_str!("./root_detection.md")]
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct InkFile {
    path: PathBuf,
    imports: Vec<PathBuf>,
}

impl InkFile {
    fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            imports: Default::default(),
        }
    }

    fn imports(mut self, path: impl Into<PathBuf>) -> Self {
        self.imports.push(path.into());
        self
    }
}

type TransitiveClosure = BTreeSet<PathBuf>;
type ImportMap = BTreeMap<PathBuf, BTreeSet<PathBuf>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Story {
    path: PathBuf,
    transitive_closure: TransitiveClosure,
}

impl Story {
    fn new<T: Into<PathBuf>>(
        path: impl Into<PathBuf>,
        transitive_closure: impl IntoIterator<Item = T>,
    ) -> Self {
        Self {
            path: path.into(),
            transitive_closure: transitive_closure.into_iter().map(Into::into).collect(),
        }
    }
}

fn find_roots(imports: ImportMap) -> BTreeSet<Story> {
    let mut roots: BTreeSet<Story> = imports
        .keys()
        .map(|root| {
            let root_dir = root.parent().expect("Root must be a file");
            let transitive_closure = transitive(&imports, root_dir, root.clone());
            Story {
                path: root.clone(),
                transitive_closure,
            }
        })
        .collect();

    let imported: BTreeSet<PathBuf> = roots
        .iter()
        .flat_map(|root| {
            root.transitive_closure
                .iter()
                .filter(|it| **it != root.path)
                .cloned()
        })
        .collect();

    roots.retain(|it| !imported.contains(&it.path));
    roots
}

fn transitive(imports: &ImportMap, root_dir: &Path, current_file: PathBuf) -> TransitiveClosure {
    let mut closure = TransitiveClosure::new();

    closure.insert(current_file.clone());

    for import in &imports[&current_file] {
        let import_relative_to_root = root_dir.join(import);
        closure.insert(import_relative_to_root.clone());

        if let Some(children) = imports.get(import_relative_to_root.as_path()) {
            for child in children {
                closure.extend(transitive(imports, root_dir, child.clone()));
            }
        } else {
            // TODO: what to do when the file can't be found?
            eprintln!("{import_relative_to_root:?} not found");
        }
    }
    closure
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map;

    use assert2::check;

    use super::*;

    fn inkfile<'a>(
        path: &'a str,
        imports: impl IntoIterator<Item = &'a str>,
    ) -> (PathBuf, BTreeSet<PathBuf>) {
        (
            path.into(),
            imports.into_iter().map(|it| it.into()).collect(),
        )
    }

    #[test]
    fn root1() {
        let files = [inkfile("main.ink", [])].into();
        check!(find_roots(files) == [Story::new("main.ink", ["main.ink"])].into());
    }

    #[test]
    fn root2() {
        let files = [
            inkfile("main.ink", ["chapter1.ink"]),
            inkfile(
                "chapter1.ink",
                [
                    "chapter1/beginning.ink",
                    "chapter1/middle.ink",
                    "chapter1/end.ink",
                ],
            ),
            inkfile("chapter1/beginning.ink", []),
            inkfile("chapter1/middle.ink", []),
            inkfile("chapter1/end.ink", []),
        ]
        .into();

        let expected_roots = [Story::new(
            "main.ink",
            [
                "main.ink",
                "chapter1.ink",
                "chapter1/beginning.ink",
                "chapter1/middle.ink",
                "chapter1/end.ink",
            ],
        )]
        .into();

        check!(find_roots(files) == expected_roots);
    }

    #[test]
    fn root3() {
        let files = [
            inkfile(
                "main.ink",
                ["chapter1.ink", "chapter2.ink", "lib/stats.ink"],
            ),
            inkfile("secondary.ink", ["lib/stats.ink"]),
            inkfile(
                "chapter1.ink",
                [
                    "chapter1/beginning.ink",
                    "chapter1/middle.ink",
                    "chapter1/end.ink",
                ],
            ),
            inkfile(
                "chapter2.ink",
                [
                    "chapter2/beginning.ink",
                    "chapter2/middle.ink",
                    "chapter2/end.ink",
                ],
            ),
            inkfile("chapter1/beginning.ink", []),
            inkfile("chapter1/middle.ink", []),
            inkfile("chapter1/end.ink", []),
            inkfile("chapter2/beginning.ink", []),
            inkfile("chapter2/middle.ink", []),
            inkfile("chapter2/end.ink", []),
            inkfile("lib/stats.ink", ["lib/helper.ink"]),
            inkfile("lib/helper.ink", []),
        ]
        .into();

        let expected = [
            Story::new(
                "main.ink",
                [
                    "main.ink",
                    "chapter1.ink",
                    "chapter2.ink",
                    "chapter1/beginning.ink",
                    "chapter1/middle.ink",
                    "chapter1/end.ink",
                    "chapter2/beginning.ink",
                    "chapter2/middle.ink",
                    "chapter2/end.ink",
                    "lib/stats.ink",
                    "lib/helper.ink",
                ],
            ),
            Story::new(
                "secondary.ink",
                ["secondary.ink", "lib/stats.ink", "lib/helper.ink"],
            ),
        ]
        .into();

        check!(find_roots(files) == expected);
    }
}
