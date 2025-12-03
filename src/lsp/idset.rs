//! Our own kind of “interning”, so that we can use the IDs (which are [`Copy`]),
//! instead of the values themselves.

use indexmap::{Equivalent, IndexMap, IndexSet};
use std::{hash::Hash, marker::PhantomData};

/// A typed ID.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn contains<Q: Hash + Equivalent<T>>(&self, item: &Q) -> bool {
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

    pub fn remove<Q: Hash + Equivalent<T>>(&mut self, value: &Q) -> bool {
        self.0.shift_remove(value)
    }

    pub fn remove_id(&mut self, id: Id<T>) -> Option<T> {
        self.0.swap_remove_index(id.0)
    }
}

pub struct IdMap<K, V>(IndexMap<K, V>);

impl<K, V> Default for IdMap<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K: Eq + Hash, V> IdMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> (Id<K>, Option<V>) {
        let (idx, old) = self.0.insert_full(key, value);
        (id(idx), old)
    }

    pub fn id_of<Q: Eq + Hash + Equivalent<K>>(&self, key: &Q) -> Option<Id<K>> {
        self.0.get_index_of(key).map(id)
    }

    pub fn contains_key<Q: Eq + Hash + Equivalent<K>>(&self, key: &Q) -> bool {
        self.0.contains_key(key)
    }

    pub fn get_by_key<Q: Eq + Hash + Equivalent<K>>(&self, key: &Q) -> Option<&V> {
        self.0.get(key)
    }

    pub fn get_mut_by_key<Q: Eq + Hash + Equivalent<K>>(&mut self, key: &Q) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    pub fn get_mut_or_insert(&mut self, key: K, make_value: impl FnOnce() -> V) -> &mut V {
        self.0.entry(key).or_insert_with(make_value)
    }

    pub fn get(&self, id: Id<K>) -> Option<&V> {
        self.0.get_index(id.0).map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, id: Id<K>) -> Option<&mut V> {
        self.0.get_index_mut(id.0).map(|(_, v)| v)
    }

    pub fn remove<Q: Eq + Hash + Equivalent<K>>(&mut self, key: &Q) -> Option<V> {
        self.0.swap_remove(key)
    }

    pub fn ids(&self) -> impl Iterator<Item = Id<K>> + use<'_, K, V> {
        (0..self.0.len()).map(id)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> + use<'_, K, V> {
        self.0.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> + use<'_, K, V> {
        self.0.values()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::Arbitrary;

    impl<T: 'static> Arbitrary for Id<T> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            id(usize::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new((self.0).shrink().map(id))
        }
    }
}
