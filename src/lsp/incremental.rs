///! Very basic incremental computation facilities, based on this article:
///! <https://medium.com/@eliah.lakhin/salsa-algorithm-explained-c5d6df1dd291>
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

struct Cell<K, V> {
    updated_at: Revision,
    verified_at: Revision,
    cache: Option<(u64, V)>,
    /// Which keys does this cell depend on?
    inputs: fn() -> Vec<K>,
    /// Compute the value given all the input values (corresponding to the result of `inputs`)
    compute: fn(Vec<V>) -> V,
}

pub(crate) struct Db<K, V> {
    revision: Revision,
    cells: HashMap<K, Cell<K, V>>,
}

trait Key: Hash + Eq + Copy + Debug {}
impl<K: Hash + Eq + Copy + Debug> Key for K {}

trait Value: Hash + Clone {}
impl<V: Hash + Clone> Value for V {}

impl<K: Key, V: Value> Db<K, V> {
    pub(crate) fn new() -> Self {
        Self {
            revision: Revision(0),
            cells: HashMap::new(),
        }
    }

    pub(crate) fn value(&mut self, key: K) -> V {
        self.verify(key);
        self.cells
            .get(&key)
            .expect("algorithm must guarantee that verified cache is filled")
            .cache
            .clone()
            .unwrap()
            .1
    }

    /// Verify and update cell behind `key`. Return the revision of the latest update
    /// (which may be the current revision, if any input has changed (recursively)).
    fn verify(&mut self, key: K) -> Revision {
        let cell_verified_at = self
            .cells
            .get(&key)
            .expect("mustn't get cells that don't exists")
            .verified_at;
        if cell_verified_at == self.revision {
            // eprintln!("verify {key:?}: verified == {:?}", self.revision);
            return self
                .cells
                .get(&key)
                .expect("mustn't get cells that don't exists")
                .updated_at;
        }

        if self.cells.get(&key).unwrap().cache.is_none() {
            // eprintln!("verify {key:?}: empty cache -> recompute");
            let input_keys = (self.cells.get(&key).unwrap().inputs)();
            let input_values = input_keys
                .iter()
                .map(|key| self.value(*key).clone())
                .collect();
            let cell = self.cells.get_mut(&key).unwrap();
            let new_value = (cell.compute)(input_values);
            let new_hash = hash(&new_value);
            cell.cache = Some((new_hash, new_value));
            cell.updated_at = self.revision;
            cell.verified_at = self.revision;
            return self.revision;
        }

        // Couldn't shortcut, so: Verify/update all inputs, …
        let mut encountered_newer_input = false;
        for input_key in self.input_keys_for(&key) {
            let is_newer = self.verify(input_key) > cell_verified_at;
            // eprintln!("verify {key:?}: input {input_key:?} newer? {is_newer}");
            encountered_newer_input |= is_newer;
        }

        // … and recompute current cell if any input cell was newer than this cell.
        if encountered_newer_input {
            // eprintln!("verify {key:?}: outated because some input is newer -> recompute");
            let input_values = self
                .input_keys_for(&key)
                .iter()
                .map(|input_key| self.cells.get(input_key).unwrap().cache.clone().unwrap().1)
                .collect();
            let cell = self.cells.get_mut(&key).unwrap();
            let new_value = (cell.compute)(input_values);
            let new_hash = hash(&new_value);
            let old_hash = cell.cache.as_ref().unwrap().0;
            if new_hash == old_hash {
                //eprintln!("verify {key:?}: hash hasn't changed -> set verified_at to {:?}", self.revision);
                cell.verified_at = self.revision;
                cell.updated_at
            } else {
                // eprintln!("verify {key:?}: hash has changed -> insert new value set revisions to {:?}", self.revision);
                cell.cache = Some((new_hash, new_value));
                cell.updated_at = self.revision;
                cell.verified_at = self.revision;
                cell.updated_at
            }
        } else {
            let cell = self.cells.get_mut(&key).unwrap();
            cell.verified_at = self.revision;
            cell.updated_at
        }
    }

    fn input_keys_for(&self, key: &K) -> Vec<K> {
        (self.cells.get(key).unwrap().inputs)()
    }

    pub(crate) fn set_input(&mut self, key: K, compute: fn(Vec<V>) -> V) {
        self.set_derived(key, Vec::new, compute)
    }

    pub(crate) fn set_derived(&mut self, key: K, inputs: fn() -> Vec<K>, compute: fn(Vec<V>) -> V) {
        self.cells.insert(
            key,
            Cell {
                updated_at: self.revision,
                verified_at: self.revision,
                cache: None,
                inputs,
                compute,
            },
        );
        self.revision.inc();
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
    fn example_spreadsheet() {
        #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
        #[rustfmt::skip]
        enum Keys { A, B, C, D }

        let mut db = Db::new();
        db.set_input(Keys::A, |_| 10);
        db.set_input(Keys::B, |_| 20);
        db.set_derived(Keys::C, || vec![Keys::A], |v| v[0] + 5);
        db.set_derived(Keys::D, || vec![Keys::B, Keys::C], |v| v[0] + v[1]);
        assert_eq!(db.value(Keys::D), 35);

        db.set_input(Keys::B, |_| 23);
        assert_eq!(db.value(Keys::D), 38);

        db.set_input(Keys::B, |_| 23);
        assert_eq!(db.value(Keys::D), 38);

        db.set_input(Keys::A, |_| 11);
        assert_eq!(db.value(Keys::D), 39);

        db.set_input(Keys::A, |_| 11);
        assert_eq!(db.value(Keys::D), 39);
    }

    /// Direct translation of the example in <https://lord.io/spreadsheets/>
    /// Since this is a more elaborate example, we can whitebox-test which cells are actually touched for an update.
    #[test]
    fn example_burrito() {
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
        db.set_input(Keys::BurritoPrice, |_| Values::Dollars(8));
        db.set_input(Keys::NumBurritos, |_| Values::Amount(3));
        db.set_input(Keys::SalsaPerBurrito, |_| Values::Grams(40));
        db.set_derived(
            Keys::BurritoPriceWithShip,
            || vec![Keys::BurritoPrice],
            |v| v[0] + Values::Dollars(2),
        );
        db.set_derived(
            Keys::Total,
            || vec![Keys::BurritoPriceWithShip, Keys::NumBurritos],
            |v| v[0] * v[1],
        );
        db.set_derived(
            Keys::SalsaInOrder,
            || vec![Keys::NumBurritos, Keys::SalsaPerBurrito],
            |v| v[0] * v[1],
        );
        let revision_1 = db.revision;

        assert_eq!(db.value(Keys::Total), Values::Dollars(30));
        assert_eq!(db.value(Keys::SalsaInOrder), Values::Grams(120));
        assert!(db.cells.values().all(|it| it.updated_at == revision_1));
        assert!(db.cells.values().all(|it| it.verified_at == revision_1));

        db.set_input(Keys::BurritoPrice, |_| Values::Dollars(9));
        db.set_derived(
            Keys::Total,
            || vec![Keys::BurritoPriceWithShip, Keys::NumBurritos],
            |v| (v[0] * v[1]) + Values::Dollars(2),
        );
        let revision_2 = db.revision;

        assert_eq!(db.value(Keys::Total), Values::Dollars(35));
        assert!(
            db.cells.values().any(|it| it.verified_at == revision_2)
                && !db.cells.values().all(|it| it.verified_at == revision_2),
            "some (but not all) are verified at revision 2 because we haven't observed SalsaInOrder yet."
        );
        assert_eq!(db.value(Keys::SalsaInOrder), Values::Grams(120));
        assert!(
            db.cells.values().all(|it| it.verified_at == revision_2),
            "now all are verified at revision 2, because we *have* observed all output cells."
        );
        assert_eq!(
            db.cells[&Keys::Total].updated_at,
            revision_2,
            "total was updated"
        );
        assert_eq!(
            db.cells[&Keys::SalsaInOrder].updated_at,
            revision_1,
            "salsa in order remains untouched"
        );
    }
}
