use super::ReadOnly;
use crate::refcell::AtomicRef3;
use crate::refcell::AtomicRefMut3;
use crate::resource::*;
use crate::Levels;
use crate::Unique;
use std::marker::PhantomData;

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct UniqueRef<T>(PhantomData<*const T>);

impl<T> Default for UniqueRef<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T> ReadOnly for UniqueRef<T> {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, U: Unique> ResourceSet<'a> for UniqueRef<U> {
    type Result = AtomicRef3<'a, U>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        let (levels, root) = resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
            .destructure();

        let (level, parent) = levels.current().destructure();
        let unique = level.get_unique::<U>().unwrap();
        AtomicRef3::new(root, parent, unique)
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct TryUniqueRef<T>(PhantomData<*const T>);

impl<T> Default for TryUniqueRef<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T> ReadOnly for TryUniqueRef<T> {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, U: Unique> ResourceSet<'a> for TryUniqueRef<U> {
    type Result = Option<AtomicRef3<'a, U>>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        let (levels, root) = resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
            .destructure();

        let (level, parent) = levels.current().destructure();
        level
            .get_unique::<U>()
            .map(|unique| AtomicRef3::new(root, parent, unique))
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct UniqueMut<T>(PhantomData<*const T>);

impl<T> Default for UniqueMut<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'a, U: Unique> ResourceSet<'a> for UniqueMut<U> {
    type Result = AtomicRefMut3<'a, U>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        let (levels, root) = resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
            .destructure();

        let (level, parent) = levels.current().destructure();
        let unique = level.get_unique_mut::<U>().unwrap();

        AtomicRefMut3::new(root, parent, unique)
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct TryUniqueMut<T>(PhantomData<*const T>);

impl<T> Default for TryUniqueMut<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'a, U: Unique> ResourceSet<'a> for TryUniqueMut<U> {
    type Result = Option<AtomicRefMut3<'a, U>>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        let (levels, root) = resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
            .destructure();

        let (level, parent) = levels.current().destructure();
        level
            .get_unique_mut::<U>()
            .map(|unique| AtomicRefMut3::new(root, parent, unique))
    }
}
