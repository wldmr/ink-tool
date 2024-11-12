use super::location::{Link, LinkKind, LocationId};
use std::collections::BTreeSet;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct Links {
    items: BTreeSet<Link>,
}

impl Links {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    // Insert or update relation. Returns true if relation is new, false if relation has been updated.
    pub(crate) fn put(&mut self, source: LocationId, kind: LinkKind, target: LocationId) -> bool {
        self.items.insert(Link {
            source,
            target,
            kind,
        })
    }

    pub(crate) fn remove_any(&mut self, id: &LocationId) -> bool {
        let len_orig = self.items.len();
        self.items.retain(|it| it.source != *id && it.target != *id);
        len_orig != self.items.len()
    }

    pub(crate) fn keys(&self) -> BTreeSet<LocationId> {
        let mut locs: BTreeSet<LocationId> = BTreeSet::new();
        for item in self.items.iter() {
            locs.insert(item.source.clone());
            locs.insert(item.target.clone());
        }
        locs
    }

    pub(crate) fn outgoing<'s: 'l, 'l>(&'s self, source: &'l LocationId) -> BTreeSet<&'l Link> {
        self.items
            .iter()
            .filter(|it| it.source == *source)
            .collect()
    }

    pub(crate) fn incoming<'s: 'l, 'l>(&'s self, target: &'l LocationId) -> BTreeSet<&'l Link> {
        self.items
            .iter()
            .filter(|it| it.target == *target)
            .collect()
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Links {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        use itertools::Itertools as _;
        let mut relations = Links::new();
        let some_size = usize::arbitrary(g) % g.size();
        for _ in 0..some_size {
            let source = LocationId::arbitrary(g);
            let target = if bool::arbitrary(g) {
                LocationId::arbitrary(g)
            } else {
                // sometimes re-use an existing value
                let random_key = g
                    .choose(&relations.keys().into_iter().collect_vec())
                    .cloned();
                random_key.unwrap_or_else(|| LocationId::arbitrary(g))
            };
            let kind = LinkKind::arbitrary(g);
            relations.put(source, kind, target);
        }
        relations
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        use itertools::Itertools as _;
        let half = self.keys().into_iter().collect_vec().len() / 2;

        match half {
            0 | 1 => quickcheck::empty_shrinker(),
            _ => {
                let mut front_fell_off = self.clone();
                let front = self.keys().into_iter().take(half).collect_vec();
                for key in front {
                    front_fell_off.remove_any(&key);
                }
                Box::new(vec![front_fell_off].into_iter())
            }
        }
    }
}

impl IntoIterator for Links {
    type Item = Link;
    type IntoIter = <BTreeSet<Link> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl FromIterator<Link> for Links {
    fn from_iter<T: IntoIterator<Item = Link>>(iter: T) -> Self {
        let mut links = Links::new();
        for item in iter {
            links.put(item.source, item.kind, item.target);
        }
        links
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::in_case;
    use itertools::Itertools;
    use quickcheck::{quickcheck, TestResult};

    quickcheck! {
        fn iter_collect_is_idempotent(links: Links) -> bool {
            let rechts: Links = links.clone().into_iter().collect();
            links == rechts
        }

        fn all_incoming_equals_all_outgoing(links: Links) -> bool {
            let mut inc = BTreeSet::new();
            let mut out = BTreeSet::new();
            let keys = links.keys();
            for key in keys {
                inc.extend(links.incoming(&key).into_iter().cloned());
                out.extend(links.outgoing(&key).into_iter().cloned());
            }
            inc == out
        }

        fn put_adds_forwards_link_to_outgoing(links: Links, a: LocationId, b: LocationId, kind: LinkKind) -> bool {
            let mut links = links;
            links.put(a.clone(), kind.clone(), b.clone());
            links.outgoing(&a)
                 .into_iter()
                 .filter(|it| it.target == b && it.kind == kind)
                 .count() == 1
        }

        fn put_adds_backwards_link_to_incoming(links: Links, a: LocationId, b: LocationId, kind: LinkKind) -> bool {
            let mut links = links;
            links.put(a.clone(), kind.clone(), b.clone());
            links.incoming(&b)
                 .into_iter()
                 .filter(|it| it.source == a && it.kind == kind)
                 .count() == 1
        }

        fn outgoing_id_equals_source(links: Links, n: usize) -> TestResult {
            in_case! { links.keys().len() > 0 =>
                let probe = get_probe(&links, n);
                links.outgoing(&probe.clone())
                     .into_iter()
                     .all(|link| link.source == probe)
            }
        }

        fn incoming_id_equals_target(links: Links, n: usize) -> TestResult {
            in_case! { links.keys().len() > 0 =>
                let probe = get_probe(&links, n);
                links.incoming(&probe.clone())
                     .into_iter()
                     .all(|link| link.target == probe)
            }
        }

        fn remove_any_removes_keys(links: Links, n: usize) -> TestResult {
            in_case! { links.keys().len() > 0 =>
                let probe = get_probe(&links, n);
                let mut rels = links;
                assert!(rels.remove_any(&probe));
                rels.incoming(&probe).is_empty() && rels.outgoing(&probe).is_empty()
            }
        }

        fn remove_any_removes_values(links: Links, n: usize) -> TestResult {
            in_case! { links.keys().len() > 0 =>
                let probe = get_probe(&links, n);
                let mut links = links;
                assert!(links.remove_any(&probe));
                links.into_iter().all(|link| link.source != probe && link.target != probe)
            }
        }
    }

    fn get_probe(links: &Links, n: usize) -> LocationId {
        let keys = links.keys().into_iter().collect_vec();
        keys.get(n % keys.len()).unwrap().clone()
    }
}
