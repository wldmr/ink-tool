use super::location::{Link, LinkKind, LocationId};
use std::collections::HashSet;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct Links {
    links: HashSet<Link>,
}

impl Links {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    // Insert or update link. Returns true if link is new, false if link has been updated.
    pub(crate) fn link(&mut self, source: LocationId, kind: LinkKind, target: LocationId) -> bool {
        self.links.insert(Link {
            source,
            target,
            kind,
        })
    }

    pub(crate) fn remove_any(&mut self, id: &LocationId) -> bool {
        let len_orig = self.links.len();
        self.links.retain(|it| it.source != *id && it.target != *id);
        len_orig != self.links.len()
    }

    pub(crate) fn locations(&self) -> HashSet<LocationId> {
        let mut locs: HashSet<LocationId> = HashSet::new();
        for item in self.links.iter() {
            if !locs.contains(&item.source) {
                locs.insert(item.source.clone());
            }
            if !locs.contains(&item.target) {
                locs.insert(item.target.clone());
            }
        }
        locs
    }

    pub(crate) fn outgoing<'s: 'l, 'l>(&'s self, source: &'l LocationId) -> HashSet<&'l Link> {
        self.links
            .iter()
            .filter(|it| it.source == *source)
            .collect()
    }

    pub(crate) fn incoming<'s: 'l, 'l>(&'s self, target: &'l LocationId) -> HashSet<&'l Link> {
        self.links
            .iter()
            .filter(|it| it.target == *target)
            .collect()
    }
}

impl IntoIterator for Links {
    type Item = Link;
    type IntoIter = <HashSet<Link> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.links.into_iter()
    }
}

impl FromIterator<Link> for Links {
    fn from_iter<T: IntoIterator<Item = Link>>(iter: T) -> Self {
        let mut links = Links::new();
        for item in iter {
            links.link(item.source, item.kind, item.target);
        }
        links
    }
}

#[cfg(test)]
mod arbitrary {
    use super::Links;
    use crate::lsp::location::{Link, LinkKind, LocationId};

    impl quickcheck::Arbitrary for Links {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut links = Links::new();
            let some_size = usize::arbitrary(g) % g.size();
            for _ in 0..some_size {
                let locations: Vec<LocationId> = links.locations().into_iter().collect();
                // sometimes re-use an existing value
                let source = bool::arbitrary(g)
                    .then(|| g.choose(&locations))
                    .and_then(|it| it.cloned())
                    .unwrap_or_else(|| LocationId::arbitrary(g));
                let target = bool::arbitrary(g)
                    .then(|| g.choose(&locations))
                    .and_then(|it| it.cloned())
                    .unwrap_or_else(|| LocationId::arbitrary(g));
                let kind = LinkKind::arbitrary(g);
                links.link(source, kind, target);
            }
            links
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(
                self.clone()
                    .into_iter()
                    .collect::<Vec<Link>>() // piggybacking on shrinking the vector of indivdual links
                    .shrink()
                    .map(Links::from_iter),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::in_case;
    use quickcheck::{quickcheck, TestResult};

    quickcheck! {
        fn iter_collect_is_idempotent(links: Links) -> bool {
            let rechts: Links = links.clone().into_iter().collect();
            links == rechts
        }

        fn all_incoming_equals_all_outgoing(links: Links) -> bool {
            let mut inc = HashSet::new();
            let mut out = HashSet::new();
            let keys = links.locations();
            for key in keys {
                inc.extend(links.incoming(&key).into_iter().cloned());
                out.extend(links.outgoing(&key).into_iter().cloned());
            }
            inc == out
        }

        fn adding_links_is_idempotent(links: Links, a: LocationId, b: LocationId, kind: LinkKind) -> bool {
            let mut links_1 = links;
            links_1.link(a.clone(), kind.clone(), b.clone());

            let mut links_2 = links_1.clone();
            links_2.link(a.clone(), kind.clone(), b.clone());

            links_1 == links_2
        }

        fn link_outgoing(links: Links, source: LocationId, target: LocationId, kind: LinkKind) -> bool {
            let mut links = links;
            links.link(source.clone(), kind.clone(), target.clone());
            links.outgoing(&source)
                 .into_iter()
                 .filter(|it| it.target == target && it.kind == kind)
                 .count() == 1
        }

        fn link_incoming(links: Links, source: LocationId, target: LocationId, kind: LinkKind) -> bool {
            let mut links = links;
            links.link(source.clone(), kind.clone(), target.clone());
            links.incoming(&target)
                 .into_iter()
                 .filter(|it| it.source == source && it.kind == kind)
                 .count() == 1
        }

        fn outgoing_id_equals_source(links: Links, n: usize) -> TestResult {
            in_case! { links.locations().len() > 0 =>
                let location = get_probe(&links, n);
                links.outgoing(&location.clone())
                     .into_iter()
                     .all(|link| link.source == location)
            }
        }

        fn incoming_id_equals_target(links: Links, n: usize) -> TestResult {
            in_case! { links.locations().len() > 0 =>
                let location = get_probe(&links, n);
                links.incoming(&location.clone())
                     .into_iter()
                     .all(|link| link.target == location)
            }
        }

        fn remove_any_removes_keys(links: Links, n: usize) -> TestResult {
            in_case! { links.locations().len() > 0 =>
                let location = get_probe(&links, n);
                let mut links = links;
                assert!(links.remove_any(&location));
                links.incoming(&location).is_empty() && links.outgoing(&location).is_empty()
            }
        }

        fn remove_any_removes_values(links: Links, n: usize) -> TestResult {
            in_case! { links.locations().len() > 0 =>
                let location = get_probe(&links, n);
                let mut links = links;
                assert!(links.remove_any(&location));
                links.into_iter().all(|link| link.source != location && link.target != location)
            }
        }
    }

    fn get_probe(links: &Links, n: usize) -> LocationId {
        let keys: Vec<LocationId> = links.locations().into_iter().collect();
        keys.get(n % keys.len()).unwrap().clone()
    }
}
