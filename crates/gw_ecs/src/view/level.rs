use super::ReadOnly;
use crate::{
    refcell::{AtomicRef, AtomicRef2, AtomicRefMut, AtomicRefMut2},
    resource::*,
    Level, Levels,
};

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct LevelRef;

impl Default for LevelRef {
    fn default() -> Self {
        Self
    }
}
impl ReadOnly for LevelRef {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a> ResourceSet<'a> for LevelRef {
    type Result = AtomicRef2<'a, Level>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        let (levels, parent) = resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
            .destructure();

        let level = levels.current();
        AtomicRef2::new(parent, level)
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct LevelMut;

impl Default for LevelMut {
    fn default() -> Self {
        Self
    }
}

impl<'a> ResourceSet<'a> for LevelMut {
    type Result = AtomicRefMut2<'a, Level>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        let (levels, parent) = resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
            .destructure();

        let level = levels.current_mut();
        AtomicRefMut2::new(parent, level)
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct LevelsRef;

impl Default for LevelsRef {
    fn default() -> Self {
        Self
    }
}
impl ReadOnly for LevelsRef {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a> ResourceSet<'a> for LevelsRef {
    type Result = AtomicRef<'a, Levels>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct LevelsMut;

impl Default for LevelsMut {
    fn default() -> Self {
        Self
    }
}

impl<'a> ResourceSet<'a> for LevelsMut {
    type Result = AtomicRefMut<'a, Levels>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        resources
            .get(&type_id)
            .map(|x| x.get_mut::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
    }
}
