use super::ReadOnly;
use crate::{
    refcell::{AtomicRef, AtomicRefMut},
    resource::{
        panic_nonexistent_resource, Resource, ResourceSet, ResourceTypeId, UnsafeResources,
    },
};
use std::marker::PhantomData;

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct Global<T>(PhantomData<*const T>);

impl<T> Default for Global<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T> ReadOnly for Global<T> {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, T: Resource> ResourceSet<'a> for Global<T> {
    type Result = AtomicRef<'a, T>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<T>();
        resources
            .get(&type_id)
            .map(|x| x.get::<T>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct TryGlobal<T>(PhantomData<*const T>);

impl<T> Default for TryGlobal<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T> ReadOnly for TryGlobal<T> {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, T: Resource> ResourceSet<'a> for TryGlobal<T> {
    type Result = Option<AtomicRef<'a, T>>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<T>();
        resources.get(&type_id).map(|x| x.get::<T>())
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct GlobalMut<T>(PhantomData<*const T>);

impl<T> Default for GlobalMut<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'a, T: Resource> ResourceSet<'a> for GlobalMut<T> {
    type Result = AtomicRefMut<'a, T>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<T>();
        resources
            .get(&type_id)
            .map(|x| x.get_mut::<T>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct TryGlobalMut<T>(PhantomData<*const T>);

impl<T> Default for TryGlobalMut<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'a, T: Resource> ResourceSet<'a> for TryGlobalMut<T> {
    type Result = Option<AtomicRefMut<'a, T>>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<T>();
        resources.get(&type_id).map(|x| x.get_mut::<T>())
    }
}
