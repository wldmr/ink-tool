//! Our own kind of “interning”, so that we can use the IDs (which are [`Copy`]),
//! instead of the values themselves.

use indexmap::IndexSet;
use std::{hash::Hash, marker::PhantomData};

/// A typed ID.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Id<T>(usize, PhantomData<T>);

fn id<T>(n: usize) -> Id<T> {
    Id(n, PhantomData)
}

impl<T> Copy for Id<T> {}
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

/// A unique set of [`T`]s, indexable by [`Id<T>`].
pub struct IdSet<T>(IndexSet<T>);

impl<T> Default for IdSet<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Eq + Hash> IdSet<T> {
    pub fn insert(&mut self, item: T) -> bool {
        self.0.insert(item)
    }

    pub fn insert_new(&mut self, item: &T) -> (Id<T>, bool)
    where
        T: Clone,
    {
        let (n, is_new) = self.0.insert_full(item.clone());
        (id(n), is_new)
    }

    pub fn contains_id(&self, id: Id<T>) -> bool {
        self.get(id).is_some()
    }

    pub fn contains(&self, item: &T) -> bool {
        self.0.contains(item)
    }

    pub fn get_id(&self, target: &T) -> Option<Id<T>> {
        self.0.get_index_of(target).map(id)
    }

    pub fn get(&self, id: Id<T>) -> Option<&T> {
        self.0.get_index(id.0)
    }

    pub fn pairs(&self) -> impl Iterator<Item = (Id<T>, &T)> {
        self.0.iter().enumerate().map(|(n, v)| (id(n), v))
    }

    pub fn ids(&self) -> impl Iterator<Item = Id<T>> + use<'_, T> {
        (0..self.0.len()).map(id)
    }

    pub fn values(&self) -> impl Iterator<Item = &T> + use<'_, T> {
        self.0.iter()
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.0.shift_remove(value)
    }

    pub fn remove_id(&mut self, id: Id<T>) -> Option<T> {
        self.0.swap_remove_index(id.0)
    }
}
