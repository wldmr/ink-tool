use super::location::{FileId, FilePos, FileRange, LocationId};
use derive_more::derive::{Display, Error};
use std::collections::HashMap;

///! # Namespacing
///!
///! SUMMARY: Ink namespacing is *fucking bananas*, man!
///!
///! ## Observations about addresses:
///!
///! - Local addresses can't shadow global addresses; as soon as a global address exists, it prevents defining the same name again anywhere (neither global nor local)
///! - Addresses seem to be namespaced by the knot only.
///!   - Stitch names seem to be *optional* namespace components
///!   
///! More formally:
///!
///! - Every address has a *globally unique name* (GUN). Attempts to define another address resulting in the same GUN will fail.
///! - There are two namespaces: top-level and knot-level:
///!   - At the top level: The following would all result in the same global name `foo` and would therefore clash:
///!     - Knot: `== foo`
///!     - Stitch: `= foo`
///!     - Label: `(foo)`
///!   - Inside a knot `== foo`:
///!     - The following would all create the same global name `foo.bar`:
///!       - Stitch: `= bar`
///!       - Label: `(bar)`
///!       - Label `(bar)` inside stitch `= baz`.
///!         - It then has the _additional_ name `foo.baz.bar`. `foo.bar` and `foo.baz.bar` are synonyms
///!         - This means there can't be another `(bar)` *anywhere* inside `foo`, not even in another stitch `= qux`: `foo.qux.bar` would be synonymous with `foo.bar` and therefore clash.
///!  - Referencing an address:
///!    - At the top level or when addressing another knot: Can only use the GUN, i.e. `foo.bar`
///!    - From inside a knot `foo` when addressing `foo.bar`:
///!      - GUN, e.g. `-> foo.bar`
///!      - Knot name is optional, e.g. `-> bar`
///!      - Stitch names are always optional: `-> foo.baz.bar`, `-> foo.bar`, and `-> bar` are all equivalent
///!      
///! ## Observations about variable names:
///!
///! - `LIST`, `VAR` & `CONST` all introduce global names, wherever they are located
///! - `LIST` values have two global names:
///!   - `list.value` always exists (it's the GUN)
///!   - `value` also exists, but does not need to be unique among list element names
///!     - It is only an an error if a non-unique `value` is _referenced_ anywhere.
///!     - However, `value` must be unique with regard to any other global name (knots, labels, `VAR`s)
///! - `temp` names are only visible in the _exact_ scope they are defined in
///!   - that is, `temp` variable at the top level are not visible in knots, and knot level temps are not visible in nested stitches
///!   - `temp` is apparently visible before it is declared.
///! - parameter names are visible in the scope they are defined in and lower (i.e., nested stitches see knot params)
///!   - stitch parameter names can actually shadow knot parameter names (IDEA: maybe emit a warning?)

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct ScopeId(ScopeIdInner);

impl ScopeId {
    /// The only scope that is guaranteed to exist.
    pub(crate) fn global() -> Self {
        Self(ScopeIdInner::Global)
    }

    /// Private! Local scopes can only be created from within this module.
    fn local(location: LocationId) -> Self {
        Self(ScopeIdInner::Local(location))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ScopeIdInner {
    Global,
    Local(LocationId),
}

#[derive(Debug, Clone)]
struct Names {
    names: HashMap<String, LocationId>,
    temps: HashMap<String, LocationId>,
    parent: Option<ScopeId>,
}

impl Names {
    fn new_global() -> Self {
        Self {
            names: HashMap::new(),
            temps: HashMap::new(),
            parent: None,
        }
    }

    fn new(parent: ScopeId) -> Self {
        Self {
            names: HashMap::new(),
            temps: HashMap::new(),
            parent: Some(parent),
        }
    }
}

/// What a name points to.
/// This adds two pieces of information: What kind of thing the target is, and how many possible targets there are.
/// We only need the second part because unqualified list values can point to similarly named values in different lists.
/// If that wasn't the case then we could just use `LocationId` directly.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
enum InkNameDefinition {
    Knot(String),
    Stitch(String),
    Label(String),
    List(String),
    Var(String),
    Const(String),
    ListValue(String),
    Parameter(String),
    Temp(String),
}

enum NameResolution {
    Unique(LocationId),
    Shadowable(LocationId),
    Ambiguous(Vec<LocationId>),
    Temp(LocationId),
}

#[derive(Debug, Clone)]
pub(crate) struct Scopes {
    global: Names,
    scopes: HashMap<LocationId, Names>,
}

impl Scopes {
    pub(crate) fn new() -> Self {
        Self {
            global: Names::new_global(),
            scopes: HashMap::new(),
        }
    }

    /// Define a range within a file
    pub(crate) fn define_scope(
        &mut self,
        file: impl Into<FileId>,
        range: impl Into<FileRange>,
        parent: ScopeId,
    ) -> Result<ScopeId, ScopeDefError> {
        // Error if parent does not point to a valid scope
        match parent.0 {
            ScopeIdInner::Global => {}
            ScopeIdInner::Local(ref location) => {
                if !self.scopes.contains_key(location) {
                    return Err(ScopeDefError::NoSuchScope);
                }
            }
        };
        let id = LocationId::new(file, range);
        for existing in self.scopes.keys() {
            if existing.overlaps(&id) {
                return Err(ScopeDefError::OverlappingScope(OverlappingScope {
                    existing: existing.clone(),
                    new: id.clone(),
                }));
            }
        }
        self.scopes.insert(id.clone(), Names::new(parent));
        Ok(ScopeId::local(id))
    }

    /// Define a name that is visible in `scope` and below.
    pub(crate) fn define_name(
        &mut self,
        scope: &ScopeId,
        name: impl Into<String>,
        target: impl Into<LocationId>,
    ) -> Result<(), ScopeDefError> {
        self.define_name_internal(scope, name, target, false)
    }

    /// Define a name that is *only* visible in `scope`, and nowhere else.
    pub(crate) fn define_temp(
        &mut self,
        scope: &ScopeId,
        name: impl Into<String>,
        target: impl Into<LocationId>,
    ) -> Result<(), ScopeDefError> {
        self.define_name_internal(scope, name, target, true)
    }

    /// From the vantage point of `cursor` in `file`, where does `name` point to?
    pub(crate) fn visible_names(&self, file: FileId, cursor: FilePos) -> Vec<(&str, &LocationId)> {
        let range = FileRange::new(cursor.clone(), cursor);
        let location_probe = LocationId::new(file, range);
        self.scopes
            .keys()
            .into_iter()
            .find(|loc| loc.overlaps(&location_probe))
            .cloned()
            .map(|id| ScopeId(ScopeIdInner::Local(id)))
            .map(|id| self.find_names(&id, true))
            .unwrap_or_default()
    }

    // Remove all registered names defined in `file` and all links to it.
    pub(crate) fn remove_all(&mut self, file: &FileId) {
        self.global
            .names
            .retain(|_name, loc| !loc.is_in_file(&file));
        self.global
            .temps
            .retain(|_name, loc| !loc.is_in_file(&file)); // this really shouldn't be filled, ever.
        self.scopes.retain(|loc, _names| !loc.is_in_file(&file));
        for (_, names) in self.scopes.iter_mut() {
            names.names.retain(|_name, loc| !loc.is_in_file(&file));
            names.temps.retain(|_name, loc| !loc.is_in_file(&file)); // again, while these can be filled, they shouldn't point to anything outside their own files.
        }
    }

    /// Due to the bonkers namespacing in Ink, temporary variables are not visible from inner scopes.
    /// That's what the `include_temp` paramter governs: Call it with `true` to get the temporary variales of `scope`, but no others.
    fn find_names(&self, scope: &ScopeId, include_temp: bool) -> Vec<(&str, &LocationId)> {
        let names = match scope.0 {
            ScopeIdInner::Global => &self.global,
            ScopeIdInner::Local(ref location) => {
                let Some(names) = self.scopes.get(location) else {
                    return Vec::new();
                };
                names
            }
        };
        let mut pairs = Vec::new();
        if include_temp {
            pairs.extend(names.temps.iter().map(|(name, loc)| (name.as_str(), loc)));
        }
        pairs.extend(names.names.iter().map(|(name, loc)| (name.as_str(), loc)));
        if let Some(ref parent) = names.parent {
            pairs.extend(self.find_names(parent, false));
        }
        pairs
    }

    fn define_name_internal(
        &mut self,
        scope: &ScopeId,
        name: impl Into<String>,
        target: impl Into<LocationId>,
        temp: bool,
    ) -> Result<(), ScopeDefError> {
        let names = match scope.0 {
            ScopeIdInner::Global => &mut self.global,
            ScopeIdInner::Local(ref location) => self
                .scopes
                .get_mut(location)
                .ok_or(ScopeDefError::NoSuchScope)?,
        };
        let names = if temp {
            &mut names.temps
        } else {
            &mut names.names
        };
        let name = name.into();
        let target = target.into();
        if names.contains_key(&name) {
            return Err(ScopeDefError::DuplicateName);
        } else {
            names.insert(name, target);
        }
        Ok(())
    }
}

#[derive(Debug, Display, Error)]
#[display("Given scope {new} overlaps with existing scope {existing}")]
struct OverlappingScope {
    existing: LocationId,
    new: LocationId,
}

#[derive(Debug, Display, Error)]
pub(crate) enum ScopeDefError {
    #[display("No such scope. Did you keep this ID around after a delete?")]
    NoSuchScope,
    #[display("{_0}")]
    OverlappingScope(OverlappingScope),
    #[display("Name must be unique within this scope")]
    DuplicateName,
}

/* #[cfg(test)]
mod tests {
    use super::*;
    use crate::{lsp::location::FileRange, test_utils::in_case};
    use quickcheck::{quickcheck, TestResult};

    quickcheck! {
        fn impossible_to_add_overlapping_scopes(scopes: Scopes, file: FileId, r1: FileRange, r2: FileRange, parent: usize) -> TestResult {
            in_case!{ r1.overlaps(&r2) =>
                let all_scopes = scopes.all_scopes();
                let parent = all_scopes[parent % all_scopes.len()].clone();
                let mut scopes = scopes;
                assert!(scopes.define_scope(file.clone(), r1, &parent).is_ok());
                let new_definition = scopes.define_scope(file, r2, &parent);
                matches!(new_definition, Err(ScopeDefError::OverlappingScope(_)))
            }
        }

        fn redefining_shadowable_names(l: LocationId, n: InkNameResolution) -> bool {
            let mut scopes = Scopes::new();
            let can_be_shadowedd = matches!(n, InkNameResolution::Parameter(_));
            assert!(scopes.define_scope(l.clone(), "some_name", n.clone()).is_ok());

            let redefinition = scopes.define_scope(l, "some_name", n);
            if can_be_shadowedd {
                matches!(redefinition, Ok(_))
            } else {
                matches!(redefinition, Err(ScopeDefError::DuplicateName))
            }
        }

        fn adding_ambiguous_names(l: LocationId, n: InkNameResolution) -> bool {
            let mut scopes = Scopes::new();
            let ambiguous = matches!(n, InkNameResolution::Parameter(_));
            assert!(scopes.define_scope(l.clone(), "some_name", n.clone()).is_ok());

            let redefinition = scopes.define_scope(l, "some_name", n);
            if ambiguous {
                matches!(redefinition, Ok(_))
            } else {
                matches!(redefinition, Err(ScopeDefError::DuplicateName))
            }
        }

    }
}

#[cfg(test)]
mod arbitrary {
    use super::{InkNameResolution, Scopes};
    use crate::lsp::location::LocationId;
    use quickcheck::Arbitrary;

    impl Arbitrary for Scopes {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut me = Self::new();
            while me.scopes.len() < g.size() {
                // Ignore errors until we have the specified length
                let _ = me.define_scope(
                    LocationId::arbitrary(g),
                    String::arbitrary(g),
                    InkNameResolution::arbitrary(g),
                );
            }
            me
        }
    }
}
*/
