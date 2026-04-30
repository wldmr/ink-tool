use std::{
    collections::HashMap,
    hash::BuildHasher,
    iter::{Chain, Once},
    slice, vec,
};

#[macro_export]
macro_rules! vec1 {
    ($item:expr) => {
        Vec1::new($item)
    };
    ($item:expr, $($rest:expr),+) => {
        {
            let mut vec = Vec1::new($item);
            $(vec.push($rest);)+
            vec
        }
    };
}

/// A vector that is guaranteed to have at least one entry.
#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct Vec1<T> {
    first: T,
    rest: Vec<T>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vec1<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rest.is_empty() {
            write!(f, "{:?}", self.first)
        } else {
            let mut vec = f.debug_list();
            vec.entry(&self.first);
            vec.entries(&self.rest);
            vec.finish()
        }
    }
}

impl<T> TryFrom<Option<T>> for Vec1<T> {
    type Error = ();

    fn try_from(value: Option<T>) -> Result<Self, Self::Error> {
        value.map(Vec1::new).ok_or(())
    }
}

impl<T> From<[T; 1]> for Vec1<T> {
    fn from([single_value]: [T; 1]) -> Self {
        Vec1::new(single_value)
    }
}

macro_rules! from_array {
    ($($n:literal),+) => {
        $(
            impl<T> From<[T; $n]> for Vec1<T> {
                fn from([first, tail @ ..]: [T; $n]) -> Self {
                    Self {
                        first,
                        rest: tail.into_iter().collect(),
                    }
                }
            }
        )+
    };
}
from_array!(2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20);

impl<T> TryFrom<Vec<T>> for Vec1<T> {
    type Error = Vec<T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(value)
        } else {
            let mut iter = value.into_iter();
            Ok(Self {
                first: iter.next().unwrap(),
                rest: iter.collect(),
            })
        }
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for Vec1<T> {
    fn eq(&self, other: &[T; N]) -> bool {
        self.iter().eq(other)
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for Vec1<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.iter().eq(other)
    }
}

impl<T> Vec1<T> {
    pub fn new(first: T) -> Self {
        Self {
            first,
            rest: Vec::new(),
        }
    }

    pub fn and(mut self, value: T) -> Self {
        self.push(value);
        self
    }

    pub fn push(&mut self, value: T) {
        self.rest.push(value);
    }

    pub fn len(&self) -> usize {
        self.rest.len() + 1
    }

    pub fn single(&self) -> Option<&T> {
        self.try_single().ok()
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn first_mut(&mut self) -> &mut T {
        &mut self.first
    }

    pub fn try_single(&self) -> Result<&T, &Self> {
        if self.rest.is_empty() {
            Ok(&self.first)
        } else {
            Err(self)
        }
    }

    pub fn single_mut(&mut self) -> Option<&mut T> {
        self.rest.is_empty().then_some(&mut self.first)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.into_iter()
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        let first = iter.next()?;
        Some(Self {
            first,
            rest: iter.collect(),
        })
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.rest.extend(iter);
    }

    /// Retain in place; panic on empty!
    pub fn retain<F: Fn(&T) -> bool>(&mut self, pred: F) {
        if pred(&self.first) {
            self.rest.retain(pred);
        } else {
            let mut rest = self.rest.drain(..).filter(pred);
            self.first = rest.next().expect("Must retain at least one element");
            self.rest = rest.collect();
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec1<T> {
    type Item = &'a T;
    type IntoIter = Chain<Once<&'a T>, slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(&self.first).chain(self.rest.iter())
    }
}

impl<'a, T> IntoIterator for &'a mut Vec1<T> {
    type Item = &'a mut T;
    type IntoIter = Chain<Once<&'a mut T>, slice::IterMut<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(&mut self.first).chain(self.rest.iter_mut())
    }
}

impl<T> IntoIterator for Vec1<T> {
    type Item = T;
    type IntoIter = Chain<Once<T>, vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.first).chain(self.rest.into_iter())
    }
}

pub trait MapOfNonEmpty<K, V> {
    fn register(&mut self, key: impl Into<K>, value: impl Into<V>);
    fn register_extend<I: Into<V>>(
        &mut self,
        key: impl Into<K>,
        value: impl IntoIterator<Item = I>,
    );
}

impl<K: Eq + std::hash::Hash, T, S: BuildHasher> MapOfNonEmpty<K, T> for HashMap<K, Vec1<T>, S> {
    fn register(&mut self, key: impl Into<K>, value: impl Into<T>) {
        match self.entry(key.into()) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                occupied_entry.into_mut().push(value.into());
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(Vec1::new(value.into()));
            }
        }
    }

    fn register_extend<I: Into<T>>(
        &mut self,
        key: impl Into<K>,
        value: impl IntoIterator<Item = I>,
    ) {
        let iter = value.into_iter().map(|it| it.into());
        match self.entry(key.into()) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                occupied_entry.into_mut().extend(iter);
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(Vec1::from_iter(iter).expect("Must have at least one item"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro1() {
        pretty_assertions::assert_eq!(vec1![1], Vec1::new(1));
    }

    #[test]
    fn macro3() {
        pretty_assertions::assert_eq!(vec1![1, 2, 3], Vec1::new(1).and(2).and(3));
    }
}
