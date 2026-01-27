use std::{
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
        let mut vec = f.debug_list();
        vec.entry(&self.first);
        vec.entries(&self.rest);
        vec.finish()
    }
}

impl<T> TryFrom<Option<T>> for Vec1<T> {
    type Error = ();

    fn try_from(value: Option<T>) -> Result<Self, Self::Error> {
        value.map(Vec1::new).ok_or(())
    }
}

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
