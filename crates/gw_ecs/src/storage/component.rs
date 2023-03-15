use super::VecStorage;
use atomic_refcell::{AtomicRef, AtomicRefMut};
// use std::ops::Deref;

/// A marker trait for all types which can be attached to an entity.
///
/// This trait has a blanket impl for all applicable types.
pub trait Component: 'static + Default + Sized + Send + Sync {
    /// The storage type required to hold all instances of this component in a world.
    type Storage: for<'a> ComponentStorage<'a, Self>;
}

impl<T: 'static + Default + Sized + Send + Sync> Component for T {
    type Storage = VecStorage<T>;
}

// /// An accessor for a shared slice reference of components for a single archetype.
// pub struct ComponentSlice<'a, T: Component> {
//     pub(crate) components: &'a [T],
//     // pub(crate) version: &'a Version,
// }

// impl<'a, T: Component> ComponentSlice<'a, T> {
//     pub(crate) fn new(components: &'a [T] /* , version: &'a Version */) -> Self {
//         Self {
//             components,
//             // version,
//         }
//     }

//     /// Converts this slice into its inner value.
//     pub fn into_slice(self) -> &'a [T] {
//         self.components
//     }
// }

// impl<'a, T: Component> From<ComponentSlice<'a, T>> for &'a [T] {
//     fn from(slice: ComponentSlice<'a, T>) -> Self {
//         slice.components
//     }
// }

// impl<'a, T: Component> Deref for ComponentSlice<'a, T> {
//     type Target = [T];

//     fn deref(&self) -> &Self::Target {
//         &self.components
//     }
// }

// impl<'a, T: Component> Index<ComponentIndex> for ComponentSlice<'a, T> {
//     type Output = T;
//     fn index(&self, index: ComponentIndex) -> &Self::Output {
//         &self.components[index.0]
//     }
// }

// /// An accessor for a mutable slice reference of components for a single archetype.
// pub struct ComponentSliceMut<'a, T: Component> {
//     // todo would be better if these were private and we controlled version increments more centrally
//     pub(crate) components: &'a mut [T],
//     // pub(crate) version: &'a mut Version,
// }

// impl<'a, T: Component> ComponentSliceMut<'a, T> {
//     pub(crate) fn new(components: &'a mut [T] /*, version: &'a mut Version */) -> Self {
//         Self {
//             components,
//             // version,
//         }
//     }

//     /// Converts this slice into its inner value.
//     /// This increments the slice's version.
//     pub fn into_slice(self) -> &'a mut [T] {
//         // *self.version = next_component_version();
//         self.components
//     }
// }

// impl<'a, T: Component> Deref for ComponentSliceMut<'a, T> {
//     type Target = [T];

//     fn deref(&self) -> &Self::Target {
//         &self.components
//     }
// }

// impl<'a, T: Component> Index<ComponentIndex> for ComponentSliceMut<'a, T> {
//     type Output = T;
//     fn index(&self, index: ComponentIndex) -> &Self::Output {
//         &self.components[index.0]
//     }
// }

// impl<'a, T: Component> IndexMut<ComponentIndex> for ComponentSliceMut<'a, T> {
//     fn index_mut(&mut self, index: ComponentIndex) -> &mut Self::Output {
//         &mut self.components[index.0]
//     }
// }

/// A storage location for component data slices. Each component storage may hold once slice for
/// each archetype inserted into the storage.
pub trait ComponentStorage<'a, T: Component>: Default {
    /// An iterator of shared archetype slice references.
    type Iter: Iterator<Item = AtomicRef<'a, T>>;

    /// An iterator of mutable archetype slice references.
    type IterMut: Iterator<Item = AtomicRefMut<'a, T>>;

    /// Returns the number of archetype slices stored.
    fn len(&self) -> usize;

    /// Returns `true` if the storage contains no archetypes.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Copies new components into the specified archetype slice.
    ///
    /// # Safety
    /// The components located at `ptr` are memcopied into the storage. If `T` is not `Copy`, then the
    /// previous memory location should no longer be accessed.
    // unsafe fn extend_memcopy(&mut self, archetype: ArchetypeIndex, ptr: *const T, len: usize);

    /// Gets the component slice for the specified archetype.
    // fn get(&'a self, archetype: ArchetypeIndex) -> Option<ComponentSlice<'a, T>>;
    fn get(&'a self, index: usize) -> Option<AtomicRef<'a, T>>;

    /// Gets a mutable component slice for the specified archetype.
    ///
    /// # Safety
    /// Ensure that the requested archetype slice is not concurrently borrowed anywhere else such that memory
    /// is not mutably aliased.
    // unsafe fn get_mut(&'a self, archetype: ArchetypeIndex) -> Option<ComponentSliceMut<'a, T>>;
    fn get_mut(&'a self, index: usize) -> Option<AtomicRefMut<'a, T>>;

    /// Iterates through all archetype component slices.
    fn iter(&'a self, start_inclusive: usize, end_exclusive: usize) -> Self::Iter;

    /// Iterates through all mutable archetype component slices.
    ///
    /// # Safety
    /// Ensure that all requested archetype slices are not concurrently borrowed anywhere else such that memory
    /// is not mutably aliased.
    fn iter_mut(&'a self, start_inclusive: usize, end_exclusive: usize) -> Self::IterMut;
}
