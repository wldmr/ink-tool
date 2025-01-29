use std::collections::HashMap;

pub trait LinkLocation: Eq + Clone {}
impl<T: Eq + Clone> LinkLocation for T {}

/// Result of of run of the Visitor for a particular node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Links<'a, Loc> {
    /// locations both defined and referenced within this ink.
    /// The tuple means (kind, definition, usage)
    pub resolved: Vec<(Loc, Loc)>,
    /// names referenced, but not defined, within this ink
    pub resolvable: Vec<(Loc, &'a str)>,
    /// names defined within and visible to the outside world
    pub provided_names: HashMap<String, Vec<Loc>>,
}

// need manual impl because for `derive(Default)` needlessly requires `T` to be default as well.
impl<'a, Loc> Default for Links<'a, Loc> {
    fn default() -> Self {
        Self {
            resolved: Default::default(),
            resolvable: Default::default(),
            provided_names: Default::default(),
        }
    }
}

/// IMPORTANT: Adding Links does not resolve any names.
impl<'a, Loc> std::ops::AddAssign for Links<'a, Loc> {
    fn add_assign(&mut self, mut rhs: Self) {
        self.resolved.append(&mut rhs.resolved);
        self.resolvable.append(&mut rhs.resolvable);
        for (name, mut locations) in rhs.provided_names {
            self.provided_names
                .entry(name)
                .or_default()
                .append(&mut locations);
        }
    }
}

impl<'a, Loc> std::ops::Add for Links<'a, Loc> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a, L: LinkLocation> std::iter::Sum for Links<'a, L> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), std::ops::Add::add)
    }
}

impl<'a, L: LinkLocation> Links<'a, L> {
    /// Transform the locations to a new representation.
    pub fn transform_locations<U>(self, f: impl Fn(L) -> U) -> Links<'a, U> {
        Links {
            resolved: self
                .resolved
                .into_iter()
                .map(|(a, b)| (f(a), f(b)))
                .collect(),
            resolvable: self
                .resolvable
                .into_iter()
                .map(|(loc, name)| (f(loc), name))
                .collect(),
            provided_names: self
                .provided_names
                .into_iter()
                .map(|(name, defs)| (name.clone(), defs.into_iter().map(|loc| (f(loc))).collect()))
                .collect(),
        }
    }

    pub fn provide<N>(&mut self, name: N, loc: impl Into<L>)
    where
        N: AsRef<str> + Into<String>,
    {
        if let Some(existing) = self.provided_names.get_mut(name.as_ref()) {
            existing.push(loc.into());
        } else {
            self.provided_names.insert(name.into(), vec![loc.into()]);
        }
    }

    /// Remove definitions for which `predicate` is true.
    /// The predicate gets references to both the name and the location.
    /// NOTE: You'll probably want to call `self.resolve()` beforehand,
    /// otherwise the names you remove will remain … well, unresolved.
    pub fn unprovide(&mut self, should_remove: impl Fn(&str, &L) -> bool) {
        // a "nested" removal:
        self.provided_names.retain(|name, defs| {
            // retain defs which do _not_ match the predicate.
            defs.retain(|def| !should_remove(name, def));
            // and retain the name _only_ if there are any defs left
            !defs.is_empty()
        });
    }

    /// Register a reference to a name.
    pub fn reference(&mut self, name: &'a str, node: L) {
        self.resolvable.push((node, name));
    }

    /// Resolve any hitherto unresolved references.
    /// A reference is resolvable if its kind matches with a definition,
    /// or its kind is `None`.
    ///
    /// Note that matches are not removed from `self.resolvable`,
    /// because in the general case we might want to allow ambiguity.
    /// Unprovide names from  manually if they go out of scope.
    pub fn resolve(&mut self) {
        for (reference, name) in &self.resolvable {
            if let Some(definitions) = self.provided_names.get(*name) {
                for def in definitions {
                    // XXX: This'll result in duplicate definitions. Probably should prevent that.
                    self.resolved.push((def.clone(), reference.clone()));
                }
            }
        }
    }

    pub fn usages<'s>(&'s self, loc: &'s L) -> impl Iterator<Item = &'s L> {
        self.resolved
            .iter()
            .filter(move |(def, _)| def == loc)
            .map(|(_, usage)| usage)
    }

    pub fn definitions<'s>(&'s self, loc: &'s L) -> impl Iterator<Item = &'s L> {
        self.resolved
            .iter()
            .filter(move |(_, usage)| usage == loc)
            .map(|(def, _usage)| def)
    }
}
