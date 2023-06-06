//! Component storage types, implementations for component joins, etc.

use super::{Added, Changed, Drain};
use crate::components::{CompMut, CompRef, Component};
use crate::entity::EntitiesRes;
use crate::join::JoinExt;
use crate::resources::ResMut;
use crate::tick::ComponentTicks;
use crate::{components::CastFrom, resources::ResRef, world::World};
use crate::{
    entity::{Entity, EntityBuilder, Index},
    error::{Error, WrongGeneration},
    join::Join,
};
use hibitset::{BitSet, BitSetLike, BitSetNot};
use std::{
    self,
    marker::PhantomData,
    ops::{Deref, DerefMut, Not},
};

#[cfg(feature = "nightly")]
pub use self::deref_flagged::{DerefFlaggedStorage, FlaggedAccessMut};

#[cfg(feature = "parallel")]
use crate::specs::join::ParJoin;

// #[cfg(feature = "nightly")]
// pub type AccessMutReturn<'a, T> =
//     <<T as Component>::Storage as UnprotectedStorage<T>>::AccessMut<'a>;
// #[cfg(not(feature = "nightly"))]
// pub type AccessMutReturn<'a, T> = &'a mut T;

/// Some storages can provide slices to access the underlying data.
///
/// The underlying data may be of type `T`, or it may be of a type
/// which wraps `T`. The associated type `Element` identifies what
/// the slices will contain.
pub trait SliceAccess<T> {
    type Element;

    fn as_slice(&self) -> &[Self::Element];
    fn as_mut_slice(&mut self) -> &mut [Self::Element];
}

/// An inverted storage type, only useful to iterate entities
/// that do not have a particular component type.
pub struct AntiStorage<'a>(pub &'a BitSet, u32, u32);

impl<'a> Join for AntiStorage<'a> {
    type Mask = BitSetNot<&'a BitSet>;
    type Item = ();
    type Storage = ();

    // SAFETY: No invariants to meet and no unsafe code.
    unsafe fn open(self) -> (Self::Mask, (), u32, u32) {
        (BitSetNot(self.0), (), self.1, self.2)
    }

    // SAFETY: No invariants to meet and no unsafe code.
    unsafe fn get(_: &mut (), _: Index, _: u32, _: u32) -> Option<()> {
        Some(())
    }
}

// SAFETY: Since `get` does not do any memory access, this is safe to implement.
unsafe impl<'a> DistinctStorage for AntiStorage<'a> {}

// SAFETY: Since `get` does not do any memory access, this is safe to implement.
#[cfg(feature = "parallel")]
unsafe impl<'a> ParJoin for AntiStorage<'a> {}

/// A dynamic storage.
pub trait AnyStorage {
    /// Drop components of given entities.
    fn drop(&mut self, entities: &[Entity], world_tick: u32);

    /// Registers the component in the world - for registry copy
    fn register(&self, world: &mut World);

    fn maintain(&mut self, world_ticks: u32);

    /// Moves the component of the given entity to the other world
    fn try_move_component(&mut self, entity: Entity, source_tick: u32, dest: &mut EntityBuilder);
}

unsafe impl<T> CastFrom<T> for dyn AnyStorage
where
    T: AnyStorage + 'static,
{
    fn cast(t: &T) -> &Self {
        t
    }

    fn cast_mut(t: &mut T) -> &mut Self {
        t
    }
}

impl<T> AnyStorage for MaskedStorage<T>
where
    T: Component,
{
    fn drop(&mut self, entities: &[Entity], world_tick: u32) {
        for entity in entities {
            MaskedStorage::drop(self, entity.id(), world_tick);
        }
    }

    fn register(&self, world: &mut World) {
        world.register::<T>();
    }

    fn maintain(&mut self, world_ticks: u32) {
        self.check_change_ticks(world_ticks);
    }

    fn try_move_component(&mut self, entity: Entity, source_tick: u32, dest: &mut EntityBuilder) {
        dest.maybe_insert::<T>(self.remove(entity.id(), source_tick));
    }
}

/// This is a marker trait which requires you to uphold the following guarantee:
///
/// > Multiple threads may call `get_mut()` with distinct indices without
/// causing > undefined behavior.
///
/// This is for example valid for `Vec`:
///
/// ```rust
/// vec![1, 2, 3];
/// ```
///
/// We may modify both element 1 and 2 at the same time; indexing the vector
/// mutably does not modify anything else than the respective elements.
///
/// As a counter example, we may have some kind of cached storage; it caches
/// elements when they're retrieved, so pushes a new element to some
/// cache-vector. This storage is not allowed to implement `DistinctStorage`.
///
/// Implementing this trait marks the storage safe for concurrent mutation (of
/// distinct elements), thus allows `join_par()`.
pub unsafe trait DistinctStorage {}

/// The status of an `insert()`ion into a storage.
/// If the insertion was successful then the Ok value will
/// contain the component that was replaced (if any).
pub type InsertResult<T> = Result<Option<T>, Error>;

/// The `UnprotectedStorage` together with the `BitSet` that knows
/// about which elements are stored, and which are not.
pub struct MaskedStorage<T: Component> {
    pub(super) mask: BitSet,
    pub(super) inner: T::Storage,
}

impl<T: Component> Default for MaskedStorage<T>
where
    T::Storage: Default,
{
    fn default() -> Self {
        Self {
            mask: Default::default(),
            inner: Default::default(),
        }
    }
}

impl<T: Component> MaskedStorage<T> {
    /// Creates a new `MaskedStorage`. This is called when you register
    /// a new component type within the world.
    pub fn new(inner: T::Storage) -> MaskedStorage<T> {
        MaskedStorage {
            mask: BitSet::new(),
            inner,
        }
    }

    pub(crate) fn open_mut(&mut self) -> (&BitSet, &mut T::Storage) {
        (&self.mask, &mut self.inner)
    }

    /// Clear the contents of this storage.
    pub fn clear(&mut self) {
        // SAFETY: `self.mask` is the correct mask as specified.
        unsafe {
            self.inner.clean(&self.mask);
        }
        self.mask.clear();
    }

    /// Remove an element by a given index.
    pub fn remove(&mut self, id: Index, _world_tick: u32) -> Option<T> {
        if self.mask.remove(id) {
            // SAFETY: We checked the mask (`remove` returned `true`)
            Some(unsafe { self.inner.remove(id).data })
        } else {
            None
        }
    }

    /// Drop an element by a given index.
    pub fn drop(&mut self, id: Index, _world_tick: u32) {
        if self.mask.remove(id) {
            // SAFETY: We checked the mask (`remove` returned `true`)
            unsafe {
                self.inner.drop(id);
            }
        }
    }

    fn check_change_ticks(&mut self, world_tick: u32) {
        let MaskedStorage { mask, inner } = self;
        for id in mask.iter() {
            let comp = unsafe { inner.get_mut(id) };
            comp.ticks.check_ticks(world_tick);
        }
    }
}

impl<T: Component> Drop for MaskedStorage<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

/// A wrapper around the masked storage and the generations vector.
/// Can be used for safe lookup of components, insertions and removes.
/// This is what `World::read/write` fetches for the user.
pub struct Storage<'e, T, D> {
    pub(super) data: D,
    pub(crate) entities: ResRef<'e, EntitiesRes>,
    phantom: PhantomData<T>,
    pub(crate) last_system_tick: u32,
    pub(crate) world_tick: u32,
}

impl<'e, T, D> Storage<'e, T, D> {
    /// Creates a new `Storage` from a fetched allocator and a immutable or
    /// mutable `MaskedStorage`, named `data`.
    pub fn new(
        entities: ResRef<'e, EntitiesRes>,
        data: D,
        last_system_tick: u32,
        current_world_tick: u32,
    ) -> Storage<'e, T, D> {
        Storage {
            data,
            entities,
            phantom: PhantomData,
            last_system_tick,
            world_tick: current_world_tick,
        }
    }
}

impl<'e, T, D> Storage<'e, T, D>
where
    T: Component,
    D: Deref<Target = MaskedStorage<T>>,
{
    /// Gets the wrapped storage.
    pub fn unprotected_storage(&self) -> &T::Storage {
        &self.data.inner
    }

    /// Returns the `EntitiesRes` resource fetched by this storage.
    /// **This does not have anything to do with the components inside.**
    /// You only want to use this when implementing additional methods
    /// for `Storage` via an extension trait.
    pub fn fetched_entities(&self) -> &EntitiesRes {
        &self.entities
    }

    /// Tries to read the data associated with an `Entity`.
    pub fn get(&self, e: Entity) -> Option<CompRef<'_, T>> {
        if self.data.mask.contains(e.id()) && self.entities.is_alive(e) {
            // SAFETY: We checked the mask, so all invariants are met.
            let cell = unsafe { self.data.inner.get(e.id()) };
            let (d, t) = cell.destructure();
            Some(CompRef::new(d, t, self.last_system_tick, self.world_tick))
        } else {
            None
        }
    }

    /// Computes the number of elements this `Storage` contains by counting the
    /// bits in the bit set. This operation will never be performed in
    /// constant time.
    pub fn count(&self) -> usize {
        self.mask().iter().count()
    }

    /// Checks whether this `Storage` is empty. This operation is very cheap.
    pub fn is_empty(&self) -> bool {
        self.mask().is_empty()
    }

    /// Returns true if the storage has a component for this entity, and that
    /// entity is alive.
    pub fn contains(&self, e: Entity) -> bool {
        self.data.mask.contains(e.id()) && self.entities.is_alive(e)
    }

    /// Returns a reference to the bitset of this storage which allows filtering
    /// by the component type without actually getting the component.
    pub fn mask(&self) -> &BitSet {
        &self.data.mask
    }
}

impl<'e, T> Storage<'e, T, ResRef<'e, MaskedStorage<T>>>
where
    T: Component,
{
    pub fn added(&self) -> Added<&Self> {
        Added::new(self)
    }

    pub fn changed(&self) -> Changed<&Self> {
        Changed::new(self)
    }
}

impl<'e, T> Storage<'e, T, ResMut<'e, MaskedStorage<T>>>
where
    T: Component,
{
    pub fn added(&mut self) -> Added<&mut Self> {
        Added::new(self)
    }

    pub fn changed(&mut self) -> Changed<&mut Self> {
        Changed::new(self)
    }
}

impl<'e, T, D> Storage<'e, T, D>
where
    T: Component,
    D: Deref<Target = MaskedStorage<T>>,
    T::Storage: SliceAccess<T>,
{
    /// Returns the component data as a slice.
    ///
    /// The indices of this slice may not correspond to anything in particular.
    /// Check the underlying storage documentation for details.
    pub fn as_slice(&self) -> &[<T::Storage as SliceAccess<T>>::Element] {
        self.data.inner.as_slice()
    }
}

impl<'e, T, D> Storage<'e, T, D>
where
    T: Component,
    D: DerefMut<Target = MaskedStorage<T>>,
    T::Storage: SliceAccess<T>,
{
    /// Returns the component data as a slice.
    ///
    /// The indices of this slice may not correspond to anything in particular.
    /// Check the underlying storage documentation for details.
    pub fn as_mut_slice(&mut self) -> &mut [<T::Storage as SliceAccess<T>>::Element] {
        self.data.inner.as_mut_slice()
    }
}

impl<'e, T, D> Storage<'e, T, D>
where
    T: Component,
    D: DerefMut<Target = MaskedStorage<T>>,
{
    /// Gets mutable access to the wrapped storage.
    ///
    /// # Safety
    ///
    /// This is unsafe because modifying the wrapped storage without also
    /// updating the mask bitset accordingly can result in illegal memory
    /// access.
    pub unsafe fn unprotected_storage_mut(&mut self) -> &mut T::Storage {
        &mut self.data.inner
    }

    /// Tries to mutate the data associated with an `Entity`.
    pub fn get_mut(&mut self, e: Entity) -> Option<CompMut<'_, T>> {
        if self.data.mask.contains(e.id()) && self.entities.is_alive(e) {
            // SAFETY: We checked the mask, so all invariants are met.
            let cell = unsafe { self.data.inner.get_mut(e.id()) };
            let (d, t) = cell.destructure_mut();
            Some(CompMut::new(d, t, self.last_system_tick, self.world_tick))
        } else {
            None
        }
    }

    /// Inserts new data for a given `Entity`.
    /// Returns the result of the operation as a `InsertResult<T>`
    ///
    /// If a component already existed for the given `Entity`, then it will
    /// be overwritten with the new component. If it did overwrite, then the
    /// result will contain `Some(T)` where `T` is the previous component.
    pub fn insert(&mut self, e: Entity, v: T) -> InsertResult<T> {
        if self.entities.is_alive(e) {
            let id = e.id();
            if self.data.mask.contains(id) {
                // TODO - set_changed needs to be done!!!
                // SAFETY: We checked the mask, so all invariants are met.
                let mut cell = StorageCell::new(v, self.world_tick);
                std::mem::swap(&mut cell, unsafe {
                    self.data.inner.get_mut(id).deref_mut()
                });
                Ok(Some(cell.data))
            } else {
                self.data.mask.add(id);
                // SAFETY: The mask was previously empty, so it is safe to insert.
                let cell = StorageCell::new(v, self.world_tick);
                unsafe { self.data.inner.insert(id, cell) };
                Ok(None)
            }
        } else {
            Err(Error::WrongGeneration(WrongGeneration {
                action: "insert component for entity",
                actual_gen: self.entities.entity(e.id()).gen(),
                entity: e,
            }))
        }
    }

    /// Removes the data associated with an `Entity`.
    pub fn remove(&mut self, e: Entity) -> Option<T> {
        if self.entities.is_alive(e) {
            self.data.remove(e.id(), self.world_tick)
        } else {
            None
        }
    }

    /// Clears the contents of the storage.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Creates a draining storage wrapper which can be `.join`ed
    /// to get a draining iterator.
    pub fn drain(&mut self) -> Drain<T> {
        Drain {
            data: &mut self.data,
            last_system_tick: self.last_system_tick,
            world_tick: self.world_tick,
        }
    }
}

impl<'a, T, D: Clone> Clone for Storage<'a, T, D> {
    fn clone(&self) -> Self {
        Storage::new(
            ResRef::clone(&self.entities),
            self.data.clone(),
            self.last_system_tick,
            self.world_tick,
        )
    }
}

// SAFETY: This is safe, since `T::Storage` is `DistinctStorage` and `Join::get`
// only accesses the storage and nothing else.
unsafe impl<'a, T: Component, D> DistinctStorage for Storage<'a, T, D> where
    T::Storage: DistinctStorage
{
}

impl<'a, 'e, T, D> Join for &'a Storage<'e, T, D>
where
    T: Component,
    D: Deref<Target = MaskedStorage<T>>,
{
    type Mask = &'a BitSet;
    type Item = CompRef<'a, T>;
    type Storage = &'a T::Storage;

    // SAFETY: No unsafe code and no invariants.
    unsafe fn open(self) -> (Self::Mask, Self::Storage, u32, u32) {
        (
            &self.data.mask,
            &self.data.inner,
            self.last_system_tick,
            self.world_tick,
        )
    }

    // SAFETY: Since we require that the mask was checked, an element for `i` must
    // have been inserted without being removed.
    unsafe fn get(
        v: &mut Self::Storage,
        i: Index,
        last_system_tick: u32,
        world_tick: u32,
    ) -> Option<CompRef<'a, T>> {
        let value: *const Self::Storage = v as *const Self::Storage;
        let (d, t) = (*value).get(i).destructure();
        Some(CompRef::new(d, t, last_system_tick, world_tick))
    }
}

impl<'a, 'e, T, D> JoinExt for &'a Storage<'e, T, D>
where
    T: Component,
    D: Deref<Target = MaskedStorage<T>>,
{
}

impl<'a, 'e, T, D> Not for &'a Storage<'e, T, D>
where
    T: Component,
    D: Deref<Target = MaskedStorage<T>>,
{
    type Output = AntiStorage<'a>;

    fn not(self) -> Self::Output {
        AntiStorage(&self.data.mask, self.last_system_tick, self.world_tick)
    }
}

// SAFETY: This is always safe because immutable access can in no case cause
// memory issues, even if access to common memory occurs.
#[cfg(feature = "parallel")]
unsafe impl<'a, 'e, T, D> ParJoin for &'a Storage<'e, T, D>
where
    T: Component,
    D: Deref<Target = MaskedStorage<T>>,
    T::Storage: Sync,
{
}

impl<'a, 'e, T, D> Join for &'a mut Storage<'e, T, D>
where
    T: Component,
    D: DerefMut<Target = MaskedStorage<T>>,
{
    type Mask = &'a BitSet;
    type Item = CompMut<'a, T>;
    type Storage = &'a mut T::Storage;

    // SAFETY: No unsafe code and no invariants to fulfill.
    unsafe fn open(self) -> (Self::Mask, Self::Storage, u32, u32) {
        let (a, b) = self.data.open_mut();
        (a, b, self.last_system_tick, self.world_tick)
    }

    // TODO: audit unsafe
    unsafe fn get(
        v: &mut Self::Storage,
        i: Index,
        last_system_tick: u32,
        world_tick: u32,
    ) -> Option<CompMut<'a, T>> {
        // This is horribly unsafe. Unfortunately, Rust doesn't provide a way
        // to abstract mutable/immutable state at the moment, so we have to hack
        // our way through it.
        let value: *mut Self::Storage = v as *mut Self::Storage;
        let (d, t) = (*value).get_mut(i).destructure_mut();
        Some(CompMut::new(d, t, last_system_tick, world_tick))
    }
}

impl<'a, 'e, T, D> JoinExt for &'a mut Storage<'e, T, D>
where
    T: Component,
    D: DerefMut<Target = MaskedStorage<T>>,
{
}

// SAFETY: This is safe because of the `DistinctStorage` guarantees.
#[cfg(feature = "parallel")]
unsafe impl<'a, 'e, T, D> ParJoin for &'a mut Storage<'e, T, D>
where
    T: Component,
    D: DerefMut<Target = MaskedStorage<T>>,
    T::Storage: Sync + DistinctStorage,
{
}

/// Tries to create a default value, returns an `Err` with the name of the
/// storage and/or component if there's no default.
pub trait TryDefault: Sized {
    /// Tries to create the default.
    fn try_default() -> Result<Self, String>;

    /// Calls `try_default` and panics on an error case.
    fn unwrap_default() -> Self {
        match Self::try_default() {
            Ok(x) => x,
            Err(e) => panic!("Failed to create a default value for storage ({:?})", e),
        }
    }
}

impl<T> TryDefault for T
where
    T: Default,
{
    fn try_default() -> Result<Self, String> {
        Ok(T::default())
    }
}

/// Used by the framework to quickly join components.
pub trait UnprotectedStorage<T>: Default {
    // / The wrapper through with mutable access of a component is performed.
    // #[cfg(feature = "nightly")]
    // type AccessMut<'a>: DerefMut<Target = T>
    // where
    //     Self: 'a;

    /// Clean the storage given a bitset with bits set for valid indices.
    /// Allows us to safely drop the storage.
    ///
    /// # Safety
    ///
    /// May only be called with the mask which keeps track of the elements
    /// existing in this storage.
    unsafe fn clean<B>(&mut self, has: B)
    where
        B: BitSetLike;

    /// Tries reading the data associated with an `Index`.
    /// This is unsafe because the external set used
    /// to protect this storage is absent.
    ///
    /// # Safety
    ///
    /// May only be called after a call to `insert` with `id` and
    /// no following call to `remove` with `id`.
    ///
    /// A mask should keep track of those states, and an `id` being contained
    /// in the tracking mask is sufficient to call this method.
    unsafe fn get(&self, id: Index) -> &StorageCell<T>;

    unsafe fn raw(&self, id: Index) -> &T {
        &self.get(id).data
    }

    // / Tries mutating the data associated with an `Index`.
    // / This is unsafe because the external set used
    // / to protect this storage is absent.
    // /
    // / # Safety
    // /
    // / May only be called after a call to `insert` with `id` and
    // / no following call to `remove` with `id`.
    // /
    // / A mask should keep track of those states, and an `id` being contained
    // / in the tracking mask is sufficient to call this method.
    // #[cfg(feature = "nightly")]
    // unsafe fn get_mut(&mut self, id: Index) -> Self::AccessMut<'_>;

    /// Tries mutating the data associated with an `Index`.
    /// This is unsafe because the external set used
    /// to protect this storage is absent.
    ///
    /// # Safety
    ///
    /// May only be called after a call to `insert` with `id` and
    /// no following call to `remove` with `id`.
    ///
    /// A mask should keep track of those states, and an `id` being contained
    /// in the tracking mask is sufficient to call this method.
    unsafe fn get_mut(&mut self, id: Index) -> &mut StorageCell<T>;

    unsafe fn raw_mut(&mut self, id: Index) -> &mut T {
        &mut self.get_mut(id).data
    }

    /// Inserts new data for a given `Index`.
    ///
    /// # Safety
    ///
    /// May only be called if `insert` was not called with `id` before, or
    /// was reverted by a call to `remove` with `id.
    ///
    /// A mask should keep track of those states, and an `id` missing from the
    /// mask is sufficient to call `insert`.
    unsafe fn insert(&mut self, id: Index, value: StorageCell<T>);

    /// Removes the data associated with an `Index`.
    ///
    /// # Safety
    ///
    /// May only be called if an element with `id` was `insert`ed and not yet
    /// removed / dropped.
    unsafe fn remove(&mut self, id: Index) -> StorageCell<T>;

    /// Drops the data associated with an `Index`.
    /// This could be used when a more efficient implementation for it exists
    /// than `remove` when the data is no longer needed.
    /// Defaults to simply calling `remove`.
    ///
    /// # Safety
    ///
    /// May only be called if an element with `id` was `insert`ed and not yet
    /// removed / dropped.
    unsafe fn drop(&mut self, id: Index) {
        self.remove(id);
    }
}

pub struct StorageCell<T> {
    data: T,
    ticks: ComponentTicks,
}

impl<T> StorageCell<T> {
    pub fn new(data: T, world_tick: u32) -> Self {
        StorageCell {
            data,
            ticks: ComponentTicks::new(world_tick),
        }
    }

    pub fn destructure(&self) -> (&T, &ComponentTicks) {
        (&self.data, &self.ticks)
    }

    pub fn destructure_mut(&mut self) -> (&mut T, &mut ComponentTicks) {
        (&mut self.data, &mut self.ticks)
    }
}

#[cfg(test)]
#[cfg(feature = "parallel")]
mod tests_inline {

    use crate::specs::{Builder, Component, Entities, ParJoin};
    use crate::{DenseVecStorage, ReadComp, World};
    use rayon::iter::ParallelIterator;

    struct Pos;

    impl Component for Pos {
        type Storage = DenseVecStorage<Self>;
    }

    #[test]
    fn test_anti_par_join() {
        let mut world = World::empty(0);
        world.create_entity().build();
        world.exec(|(entities, pos): (Entities, ReadComp<Pos>)| {
            (&entities, !&pos).par_join().for_each(|(ent, ())| {
                println!("Processing entity: {:?}", ent);
            });
        });
    }
}
