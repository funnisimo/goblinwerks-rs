use super::ReadOnly;
use crate::{
    component::Component,
    refcell::{AtomicRef3, AtomicRefMut3},
    resource::{panic_nonexistent_resource, ResourceSet, ResourceTypeId, UnsafeResources},
    storage::DenseStorage,
    Levels,
};
use std::marker::PhantomData;

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct Comp<T>(PhantomData<*const T>);

impl<T> Default for Comp<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T> ReadOnly for Comp<T> {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, C: Component> ResourceSet<'a> for Comp<C> {
    type Result = AtomicRef3<'a, DenseStorage<C>>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        let (levels, root) = resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
            .destructure();

        let (level, parent) = levels.current().destructure();
        let unique = level.get_component::<C>().unwrap();
        AtomicRef3::new(root, parent, unique)
    }
}

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct CompMut<T>(PhantomData<*const T>);

impl<T> Default for CompMut<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'a, C: Component> ResourceSet<'a> for CompMut<C> {
    type Result = AtomicRefMut3<'a, DenseStorage<C>>;

    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        let (levels, root) = resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
            .destructure();

        let (level, parent) = levels.current().destructure();
        let comp = level.get_component_mut::<C>().unwrap();

        AtomicRefMut3::new(root, parent, comp)
    }
}
