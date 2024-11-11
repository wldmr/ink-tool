use std::collections::HashMap;

use super::location::{specification::LocationThat, Location, LocationId};

#[derive(Default)]
pub(crate) struct Locations {
    locs: HashMap<LocationId, Location>,
}

impl Locations {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    // Insert or update location. Returns previous location on update.
    pub(crate) fn put(&mut self, loc: Location) -> Option<Location> {
        todo!()
    }

    // Find by specification. Returns references.
    pub(crate) fn find(&self, spec: LocationThat) -> Vec<&Location> {
        todo!()
    }

    // Delete by specification. Returns deleted entries.
    pub(crate) fn delete(&self, spec: LocationThat) -> Vec<Location> {
        todo!()
    }
}
