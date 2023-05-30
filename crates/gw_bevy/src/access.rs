use crate::resources::ResourceId;
use std::{collections::HashSet, fmt::Debug};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum AccessItem {
    Global(ResourceId),
    Unique(ResourceId),
    Component(ResourceId),
}

#[derive(Default, Debug, Clone)]
pub struct AccessTracker {
    reads: HashSet<AccessItem>,
    writes: HashSet<AccessItem>,
    reads_all: bool,
}

impl AccessTracker {
    pub fn new() -> Self {
        AccessTracker::default()
    }

    pub fn add_read(&mut self, item: AccessItem) {
        self.reads.insert(item);
    }

    pub fn add_write(&mut self, item: AccessItem) {
        self.writes.insert(item);
    }

    pub fn has_read(&self, item: &AccessItem) -> bool {
        self.reads_all || self.reads.contains(item)
    }

    pub fn has_write(&self, item: &AccessItem) -> bool {
        self.writes.contains(item)
    }

    pub fn set_reads_all(&mut self) {
        self.reads_all = true;
    }

    pub fn has_reads_all(&self) -> bool {
        self.reads_all
    }

    pub fn clear(&mut self) {
        self.reads.clear();
        self.writes.clear();
        self.reads_all = false;
    }

    pub fn extend(&mut self, other: &AccessTracker) {
        self.reads_all = self.reads_all || other.reads_all;
        for val in other.reads.iter() {
            self.reads.insert(val.clone());
        }
        for val in other.writes.iter() {
            self.writes.insert(val.clone());
        }
    }

    pub fn is_compatible(&self, other: &AccessTracker) -> bool {
        if self.reads_all {
            return other.writes.len() == 0;
        }

        if other.reads_all {
            return self.writes.len() == 0;
        }

        self.writes.is_disjoint(&other.reads) && self.reads.is_disjoint(&other.writes)
    }

    pub fn get_conflicts(&self, other: &AccessTracker) -> Vec<AccessItem> {
        let mut conflicts: Vec<AccessItem> = Vec::new();

        if self.reads_all {
            conflicts.extend(other.writes.iter().cloned());
        }
        if other.reads_all {
            conflicts.extend(self.writes.iter().cloned());
        }
        conflicts.extend(self.writes.intersection(&other.reads).cloned());
        conflicts.extend(self.reads.intersection(&other.writes).cloned());

        conflicts
    }

    pub fn reads(&self) -> impl Iterator<Item = &AccessItem> {
        self.reads.iter()
    }

    pub fn writes(&self) -> impl Iterator<Item = &AccessItem> {
        self.writes.iter()
    }
}
