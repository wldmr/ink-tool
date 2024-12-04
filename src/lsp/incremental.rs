///! Very basic incremental computation facilities, based on the following articles:
///! * [Salsa Algorithm Explained](https://medium.com/@eliah.lakhin/salsa-algorithm-explained-c5d6df1dd291)
///! * [How to recalculate a Spreadsheet]<https://lord.io/spreadsheets/>
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Revision(usize);
impl Revision {
    fn inc(&mut self) {
        self.0 += 1;
    }
}

#[derive(Clone, Debug)]
enum Computation<K, V> {
    Const {
        value: V,
        next: Option<Box<ComputationStep<K, V>>>,
    },
    Derive {
        input: K,
        derive: fn(&V) -> V,
        next: Option<Box<ComputationStep<K, V>>>,
    },
}

#[derive(Clone, Debug)]
struct ComputationStep<K, V> {
    input: K,
    /// Function from (previous output, value for input) to new output
    combine: fn(&V, &V) -> V,
    next: Option<Box<Self>>,
}

impl<K: Key, V: Value> Computation<K, V> {
    /// A computation (after a fashion) that simply returns `value`
    pub(crate) fn constant(value: V) -> Self {
        Self::Const { value, next: None }
    }

    /// Convenience function to a computation that simply contains the value behind `input`.
    pub(crate) fn fetch(input: K) -> Self {
        Self::derive(input, |value| value.clone())
    }

    /// Convenience function to a computation that simply contains the value behind `input`.
    pub(crate) fn derive(input: K, derive: fn(&V) -> V) -> Self {
        Self::Derive {
            input,
            derive,
            next: None,
        }
    }

    /// Combines the result of the previous step and the value behind `input` using the `combine` operation.
    pub(crate) fn and_combine(mut self, input: K, combine: fn(&V, &V) -> V) -> Self {
        match &mut self {
            Computation::Const { next, .. } | Computation::Derive { next, .. } => {
                let new_step = ComputationStep {
                    input,
                    combine,
                    next: None,
                };
                if let Some(step) = next {
                    step.set_next(new_step);
                } else {
                    *next = Some(Box::new(new_step));
                }
            }
        }
        self
    }
}

impl<K, V> ComputationStep<K, V> {
    // Helper function: Find the last element in the computation chain and add `step`
    fn set_next(&mut self, step: Self) {
        if let Some(ref mut inner) = self.next {
            inner.set_next(step);
        } else {
            self.next = Some(Box::new(step));
        }
    }
}

#[derive(Debug)]
pub(crate) struct Db<K, V> {
    revision: Revision,
    cache: HashMap<K, (u64, V)>,
    computation: HashMap<K, Computation<K, V>>,
    updated_at: HashMap<K, Revision>,
    verified_at: HashMap<K, Revision>,
}

trait Key: Hash + Eq + Copy + Debug {}
impl<K: Hash + Eq + Copy + Debug> Key for K {}

trait Value: Hash + Clone {}
impl<V: Hash + Clone> Value for V {}

impl<K: Key, V: Value> Db<K, V> {
    pub(crate) fn new() -> Self {
        Self {
            revision: Revision(0),
            cache: HashMap::new(),
            computation: HashMap::new(),
            updated_at: HashMap::new(),
            verified_at: HashMap::new(),
        }
    }

    /// Get the current value behind `key`, if any.
    pub(crate) fn value(&mut self, key: K) -> Option<&V> {
        self.verify(key);
        self.cache.get(&key).map(|it| &it.1)
    }

    /// Verify and update cell behind `key`. Return the revision of the latest update
    /// (which may be the current revision, if any input has changed (recursively),
    /// or an earlier one, if no update was required).
    fn verify(&mut self, key: K) -> Option<&Revision> {
        // We're not really doing anything useful with the `Option` return value, but it enables the early return, so there's that.
        let current_cell_verified_at = *self.verified_at.get(&key)?;
        if current_cell_verified_at == self.revision {
            // eprintln!("verify {key:?}: up to date at {:?}", self.revision);
            return self.updated_at.get(&key);
        }

        // Couldn't shortcut, so: Verify/update all inputs, …
        let (mut accumulator, mut next) = match self.computation.get(&key).cloned()? {
            Computation::Const { value, next } => (value.clone(), next),
            Computation::Derive {
                input,
                derive,
                next,
            } => {
                let input_value = self.value(input)?;
                (derive(input_value), next)
            }
        };
        while let Some(ref step) = next {
            let input_value = self.value(step.input)?;
            accumulator = (step.combine)(&accumulator, input_value);
            next = step.next.clone();
        }

        let new_hash = hash(&accumulator);
        if self.cache.get(&key).is_some_and(|it| it.0 == new_hash) {
            //eprintln!("verify {key:?}: hash hasn't changed -> set verified_at to {:?}", self.revision);
            self.verified_at.insert(key, self.revision);
        } else {
            // eprintln!("verify {key:?}: hash has changed -> insert new value set revisions to {:?}", self.revision);
            self.cache.insert(key, (new_hash, accumulator));
            self.verified_at.insert(key, self.revision);
            self.updated_at.insert(key, self.revision);
        }
        self.updated_at.get(&key)
    }

    pub(crate) fn set_value(&mut self, key: K, computation: impl Into<Computation<K, V>>) {
        self.computation.insert(key, computation.into());
        self.verified_at.insert(key, self.revision);
        self.updated_at.insert(key, self.revision);
        self.revision.inc(); // Increase _after_ setting the timestamps so that the values are calculated when next observed.
    }
}

/// Create a constant computation from a value
impl<K: Key, V: Value> From<V> for Computation<K, V> {
    fn from(value: V) -> Self {
        Computation::Const { value, next: None }
    }
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    (*t).hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Direct translation of the example in <https://medium.com/@eliah.lakhin/salsa-algorithm-explained-c5d6df1dd291>
    #[test]
    fn spreadsheet() {
        #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
        #[rustfmt::skip]
        enum Keys { A, B, C, D }

        let mut db: Db<Keys, i32> = Db::new();
        db.set_value(Keys::A, 10);
        db.set_value(Keys::B, 20);
        db.set_value(Keys::C, Computation::derive(Keys::A, |a| a + 5));
        db.set_value(
            Keys::D,
            Computation::fetch(Keys::B).and_combine(Keys::C, |b, c| b + c),
        );
        eprintln!("{db:#?}");
        assert_eq!(db.value(Keys::D), Some(&35));

        db.set_value(Keys::B, 23);
        assert_eq!(db.value(Keys::D), Some(&38));

        db.set_value(Keys::B, 23);
        assert_eq!(db.value(Keys::D), Some(&38));

        db.set_value(Keys::A, 11);
        assert_eq!(db.value(Keys::D), Some(&39));

        db.set_value(Keys::A, 11);
        assert_eq!(db.value(Keys::D), Some(&39));
    }

    /// Direct translation of the example in <https://lord.io/spreadsheets/>
    /// Since this is a more elaborate example, we can whitebox-test which cells are actually touched for an update.
    #[test]
    fn burrito() {
        #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
        enum Keys {
            BurritoPrice,
            BurritoPriceWithShip,
            NumBurritos,
            SalsaPerBurrito,
            Total,
            SalsaInOrder,
        }

        #[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
        enum Values {
            Dollars(u32),
            Amount(u32),
            Grams(u32),
        }

        impl std::ops::Add for Values {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                use Values::*;
                match (self, rhs) {
                    (Dollars(a), Dollars(b)) => Dollars(a + b),
                    (Amount(a), Amount(b)) => Amount(a + b),
                    (Grams(a), Grams(b)) => Amount(a + b),
                    (a, b) => panic!("Cannot add incompatible values {a:?} and {b:?}"),
                }
            }
        }
        impl std::ops::Mul for Values {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                use Values::*;
                match (self, rhs) {
                    (Dollars(a), Amount(b)) | (Amount(a), Dollars(b)) => Dollars(a * b),
                    (Grams(a), Amount(b)) | (Amount(a), Grams(b)) => Grams(a * b),
                    (Amount(a), Amount(b)) => Amount(a * b),
                    (a, b) => panic!("Cannot multiply incompatible values {a:?} and {b:?}"),
                }
            }
        }

        let mut db = Db::new();
        db.set_value(Keys::BurritoPrice, Values::Dollars(8));
        db.set_value(Keys::NumBurritos, Values::Amount(3));
        db.set_value(Keys::SalsaPerBurrito, Values::Grams(40));
        db.set_value(
            Keys::BurritoPriceWithShip,
            Computation::derive(Keys::BurritoPrice, |&price| price + Values::Dollars(2)),
        );
        db.set_value(
            Keys::Total,
            Computation::fetch(Keys::BurritoPriceWithShip)
                .and_combine(Keys::NumBurritos, |&price, &num| price * num),
        );
        db.set_value(
            Keys::SalsaInOrder,
            Computation::fetch(Keys::NumBurritos)
                .and_combine(Keys::SalsaPerBurrito, |&num, &salsa| num * salsa),
        );
        let revision_1 = db.revision;

        assert_eq!(db.value(Keys::Total), Some(&Values::Dollars(30)));
        assert_eq!(db.value(Keys::SalsaInOrder), Some(&Values::Grams(120)));
        assert!(db.updated_at.values().all(|&it| it == revision_1));
        assert!(db.verified_at.values().all(|&it| it == revision_1));

        db.set_value(Keys::BurritoPrice, Values::Dollars(9));
        db.set_value(
            Keys::Total,
            Computation::fetch(Keys::BurritoPriceWithShip)
                .and_combine(Keys::NumBurritos, |&price, &num| {
                    (price * num) + Values::Dollars(2)
                }),
        );
        let revision_2 = db.revision;

        assert_eq!(db.value(Keys::Total), Some(&Values::Dollars(35)));
        assert!(
            db.verified_at.values().any(|&it| it== revision_2)
                && !db.verified_at.values().all(|&it| it== revision_2),
            "some (but not all) are verified at revision 2 because we haven't observed SalsaInOrder yet."
        );
        assert_eq!(db.value(Keys::SalsaInOrder), Some(&Values::Grams(120)));
        assert!(
            db.verified_at.values().all(|&it| it == revision_2),
            "now all are verified at revision 2, because we *have* observed all output cells."
        );
        assert_eq!(db.updated_at[&Keys::Total], revision_2, "total was updated");
        assert_eq!(
            db.updated_at[&Keys::SalsaInOrder],
            revision_1,
            "salsa in order remains untouched"
        );
    }
}
