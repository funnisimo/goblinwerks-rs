use crate::atomic_refcell::{AtomicBorrowRef, AtomicRefCell};
use crate::prelude::DetectChanges;
use crate::resources::{ResMut, ResRef, Resource, Resources};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct Globals {
    resources: Arc<AtomicRefCell<Resources>>,
}

impl Globals {
    pub fn new() -> Self {
        Globals {
            resources: Arc::new(AtomicRefCell::new(Resources::empty())),
        }
    }

    pub fn empty() -> Self {
        Globals::new()
    }

    /// Returns true if the resource is in the Globals
    pub fn contains<G: Resource>(&self) -> bool {
        self.resources.borrow().contains::<G>()
    }

    /// Ensures that the resource is in the Globals or enters
    /// the value from the function.
    pub fn ensure_with<G: Resource + Send + Sync, F: FnOnce() -> G>(
        &mut self,
        func: F,
        world_tick: u32,
    ) {
        self.resources.borrow_mut().ensure_with(func, world_tick);
    }

    /// Ensures that the resource is in the Globals or enters
    /// the value from the function.
    pub fn ensure_non_send_with<G: Resource, F: FnOnce() -> G>(
        &mut self,
        func: F,
        world_tick: u32,
    ) {
        self.resources
            .borrow_mut()
            .ensure_non_send_with(func, world_tick);
    }

    /// Inserts a global
    pub fn insert<G: Resource + Send + Sync>(&mut self, global: G, world_tick: u32) {
        self.resources.borrow_mut().insert(global, world_tick);
    }

    pub fn insert_non_send<G: Resource>(&mut self, global: G, world_tick: u32) {
        self.resources
            .borrow_mut()
            .insert_non_send(global, world_tick);
    }

    /// Removes a global
    pub fn remove<G: Resource>(&mut self, world_tick: u32) -> Option<G> {
        self.resources.borrow_mut().remove::<G>(world_tick)
    }

    pub fn fetch<'b, G: Resource>(
        &'b self,
        last_system_tick: u32,
        world_tick: u32,
    ) -> GlobalRef<'b, G> {
        let (globals, borrow) = self.resources.borrow().destructure();
        let fetch = globals.get::<G>(last_system_tick, world_tick).unwrap();
        GlobalRef::new(borrow, fetch)
    }

    pub(crate) fn try_fetch<G: Resource>(
        &self,
        last_system_tick: u32,
        world_tick: u32,
    ) -> Option<GlobalRef<G>> {
        let (globals, borrow) = self.resources.borrow().destructure();
        match globals.get::<G>(last_system_tick, world_tick) {
            None => None,
            Some(fetch) => Some(GlobalRef::new(borrow, fetch)),
        }
    }

    pub fn fetch_mut<G: Resource>(&self, last_system_tick: u32, world_tick: u32) -> GlobalMut<G> {
        let (globals, borrow) = self.resources.borrow().destructure();
        let fetch = globals.get_mut::<G>(last_system_tick, world_tick).unwrap();
        GlobalMut::new(borrow, fetch)
    }

    pub fn try_fetch_mut<G: Resource>(
        &self,
        last_system_tick: u32,
        world_tick: u32,
    ) -> Option<GlobalMut<G>> {
        let (globals, borrow) = self.resources.borrow().destructure();
        match globals.get_mut::<G>(last_system_tick, world_tick) {
            None => None,
            Some(fetch) => Some(GlobalMut::new(borrow, fetch)),
        }
    }

    pub fn maintain(&mut self, world_tick: u32) {
        self.resources.borrow_mut().maintain(world_tick);
    }

    // pub fn deleted<G: Resource>(&self) -> Option<Tick> {
    //     self.resources.borrow().deleted::<G>()
    // }

    pub fn clear(&mut self) {
        self.resources.borrow_mut().clear();
    }
}

impl Default for Globals {
    fn default() -> Self {
        Globals::new()
    }
}

impl Clone for Globals {
    fn clone(&self) -> Self {
        Globals {
            resources: Arc::clone(&self.resources),
        }
    }
}

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<GlobalFetch<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
pub struct GlobalRef<'a, T: 'a> {
    borrow: AtomicBorrowRef<'a>, // borrow of globals
    fetch: ResRef<'a, T>,
}

impl<'a, T: 'a> GlobalRef<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(borrow: AtomicBorrowRef<'a>, fetch: ResRef<'a, T>) -> Self {
        GlobalRef { borrow, fetch }
    }

    pub fn last_system_tick(&self) -> u32 {
        self.fetch.last_system_tick
    }

    pub fn world_tick(&self) -> u32 {
        self.fetch.world_tick
    }
}

impl<'w, T> DetectChanges for GlobalRef<'w, T>
where
    T: Resource + Send + Sync,
{
    /// Returns `true` if the resource was added after the system last ran.
    fn is_added(&self) -> bool {
        self.fetch
            .ticks
            .is_added(self.last_system_tick(), self.world_tick())
    }

    /// Returns `true` if the resource was added or mutably dereferenced after the system last ran.
    fn is_changed(&self) -> bool {
        self.fetch
            .ticks
            .is_changed(self.last_system_tick(), self.world_tick())
    }

    #[inline]
    fn last_changed(&self) -> u32 {
        self.fetch.ticks.changed.tick
    }
}

impl<'a, T> Deref for GlobalRef<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.fetch.deref()
    }
}

impl<'a, T> Clone for GlobalRef<'a, T> {
    fn clone(&self) -> Self {
        GlobalRef {
            borrow: AtomicBorrowRef::clone(&self.borrow),
            fetch: ResRef::clone(&self.fetch),
        }
    }
}

/// Allows to fetch a resource in a system mutably.
///
/// If the resource isn't strictly required, you should use
/// `Option<GlobalFetchMut<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
pub struct GlobalMut<'a, T: 'a> {
    #[allow(dead_code)]
    borrow: AtomicBorrowRef<'a>, // borrow of globals
    fetch: ResMut<'a, T>,
}

impl<'a, T: 'a> GlobalMut<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(borrow: AtomicBorrowRef<'a>, fetch: ResMut<'a, T>) -> Self {
        GlobalMut { borrow, fetch }
    }

    pub fn last_system_tick(&self) -> u32 {
        self.fetch.last_system_tick
    }

    pub fn world_tick(&self) -> u32 {
        self.fetch.world_tick
    }
}

impl<'w, T> DetectChanges for GlobalMut<'w, T>
where
    T: Resource + Send + Sync,
{
    /// Returns `true` if the resource was added after the system last ran.
    fn is_added(&self) -> bool {
        self.fetch
            .ticks
            .is_added(self.last_system_tick(), self.world_tick())
    }

    /// Returns `true` if the resource was added or mutably dereferenced after the system last ran.
    fn is_changed(&self) -> bool {
        self.fetch
            .ticks
            .is_changed(self.last_system_tick(), self.world_tick())
    }

    #[inline]
    fn last_changed(&self) -> u32 {
        self.fetch.ticks.changed.tick
    }
}

impl<'a, T> Deref for GlobalMut<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.fetch.deref()
    }
}

impl<'a, T> DerefMut for GlobalMut<'a, T>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        self.fetch.deref_mut()
    }
}

// pub(crate) struct GlobalRes<T>(PhantomData<T>);
// pub(crate) struct GlobalSet;

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ReadGlobal<'a, T: Resource + Send + Sync> {
    fetch: GlobalRef<'a, T>,
}

impl<'a, T> ReadGlobal<'a, T>
where
    T: Resource + Send + Sync,
{
    pub(crate) fn new(fetch: GlobalRef<'a, T>) -> Self {
        ReadGlobal { fetch }
    }
}

impl<'w, T> DetectChanges for ReadGlobal<'w, T>
where
    T: Resource + Send + Sync,
{
    /// Returns `true` if the resource was added after the system last ran.
    fn is_added(&self) -> bool {
        self.fetch
            .fetch
            .ticks
            .is_added(self.last_system_tick(), self.world_tick())
    }

    /// Returns `true` if the resource was added or mutably dereferenced after the system last ran.
    fn is_changed(&self) -> bool {
        self.fetch
            .fetch
            .ticks
            .is_changed(self.last_system_tick(), self.world_tick())
    }

    #[inline]
    fn last_changed(&self) -> u32 {
        self.fetch.fetch.ticks.changed.tick
    }
}

impl<'w, T> Debug for ReadGlobal<'w, T>
where
    T: Debug + Resource + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReadGlobal")
            .field(self.fetch.deref())
            .finish()
    }
}

impl<'a, T> Deref for ReadGlobal<'a, T>
where
    T: Resource + Send + Sync,
{
    type Target = GlobalRef<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.fetch
    }
}

// impl<'a, T, F> From<GlobalRef<'a, T>> for ReadGlobal<'a, T, F> {
//     fn from(fetch: GlobalRef<'a, T>) -> Self {
//         ReadGlobal {
//             fetch,
//             phantom: PhantomData,
//         }
//     }
// }

// impl<'a, T, F> SystemData<'a> for ReadGlobal<'a, T, F>
// where
//     T: Resource,
//     F: SetupHandler<T>,
// {
//     fn setup(world: &mut World) {
//         F::setup(world)
//     }

//     fn fetch(world: &'a World) -> Self {
//         ReadGlobal::<'a, T, F> {
//             fetch: world.read_global::<T>(),
//             phantom: PhantomData,
//         }
//     }

//     fn reads() -> HashSet<ResourceId> {
//         let mut reads = HashSet::new();
//         reads.insert(ResourceId::new::<Globals>());
//         reads.insert(ResourceId::new::<GlobalRes<T>>());
//         reads
//     }

//     // fn writes() -> HashSet<ResourceId> {
//     //     vec![]
//     // }
// }

/// Allows to fetch a resource in a system mutably.
///
/// If the resource isn't strictly required, you should use `Option<Write<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct WriteGlobal<'a, T: Resource + Send + Sync> {
    fetch: GlobalMut<'a, T>,
}

impl<'a, T> WriteGlobal<'a, T>
where
    T: Resource + Send + Sync,
{
    pub(crate) fn new(fetch: GlobalMut<'a, T>) -> Self {
        WriteGlobal { fetch }
    }

    #[inline]
    pub fn last_system_tick(&self) -> u32 {
        self.fetch.last_system_tick()
    }

    #[inline]
    pub fn world_tick(&self) -> u32 {
        self.fetch.world_tick()
    }
}

impl<'w, T> DetectChanges for WriteGlobal<'w, T>
where
    T: Resource + Send + Sync,
{
    /// Returns `true` if the resource was added after the system last ran.
    fn is_added(&self) -> bool {
        self.fetch
            .fetch
            .ticks
            .is_added(self.last_system_tick(), self.world_tick())
    }

    /// Returns `true` if the resource was added or mutably dereferenced after the system last ran.
    fn is_changed(&self) -> bool {
        self.fetch
            .fetch
            .ticks
            .is_changed(self.last_system_tick(), self.world_tick())
    }

    #[inline]
    fn last_changed(&self) -> u32 {
        self.fetch.fetch.ticks.changed.tick
    }
}

impl<'w, T> Debug for WriteGlobal<'w, T>
where
    T: Debug + Resource + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WriteGlobal")
            .field(self.fetch.deref())
            .finish()
    }
}

impl<'a, T> Deref for WriteGlobal<'a, T>
where
    T: Resource + Send + Sync,
{
    type Target = GlobalMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.fetch
    }
}

impl<'a, T> DerefMut for WriteGlobal<'a, T>
where
    T: Resource + Send + Sync,
{
    fn deref_mut(&mut self) -> &mut GlobalMut<'a, T> {
        &mut self.fetch
    }
}

// impl<'a, T, F> From<GlobalMut<'a, T>> for WriteGlobal<'a, T, F> {
//     fn from(fetch: GlobalMut<'a, T>) -> Self {
//         WriteGlobal {
//             fetch,
//             phantom: PhantomData,
//         }
//     }
// }

// impl<'a, T, F> SystemData<'a> for WriteGlobal<'a, T, F>
// where
//     T: Resource,
//     F: SetupHandler<T>,
// {
//     fn setup(world: &mut World) {
//         F::setup(world)
//     }

//     fn fetch(world: &'a World) -> Self {
//         WriteGlobal::<'a, T, F> {
//             fetch: world.write_global::<T>(),
//             phantom: PhantomData,
//         }
//     }

//     fn reads() -> HashSet<ResourceId> {
//         let mut reads = HashSet::new();
//         reads.insert(ResourceId::new::<Globals>());
//         reads
//     }

//     fn writes() -> HashSet<ResourceId> {
//         let mut writes = HashSet::new();
//         writes.insert(ResourceId::new::<GlobalRes<T>>());
//         writes
//     }
// }

// ------------------

// impl<'a, T, F> SystemData<'a> for Option<ReadGlobal<'a, T, F>>
// where
//     T: Resource,
// {
//     fn setup(_: &mut World) {}

//     fn fetch(world: &'a World) -> Self {
//         match world.try_read_global::<T>() {
//             None => None,
//             Some(fetch) => Some(ReadGlobal::<'a, T, F> {
//                 fetch: fetch,
//                 phantom: PhantomData,
//             }),
//         }
//     }

//     fn reads() -> HashSet<ResourceId> {
//         let mut reads = HashSet::new();
//         reads.insert(ResourceId::new::<Globals>());
//         reads.insert(ResourceId::new::<GlobalRes<T>>());
//         reads
//     }

//     // fn writes() -> Vec<ResourceId> {
//     //     vec![]
//     // }
// }

// impl<'a, T, F> SystemData<'a> for Option<WriteGlobal<'a, T, F>>
// where
//     T: Resource,
// {
//     fn setup(_: &mut World) {}

//     fn fetch(world: &'a World) -> Self {
//         match world.try_write_global::<T>() {
//             None => None,
//             Some(fetch) => Some(WriteGlobal::<'a, T, F> {
//                 fetch: fetch,
//                 phantom: PhantomData,
//             }),
//         }
//     }

//     fn reads() -> HashSet<ResourceId> {
//         let mut reads = HashSet::new();
//         reads.insert(ResourceId::new::<Globals>());
//         reads
//     }

//     fn writes() -> HashSet<ResourceId> {
//         let mut writes = HashSet::new();
//         writes.insert(ResourceId::new::<GlobalRes<T>>());
//         writes
//     }
// }

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct ReadNonSendGlobal<'a, T: 'a> {
    fetch: GlobalRef<'a, T>,
    phantom: PhantomData<*mut ()>,
}

impl<'a, T> ReadNonSendGlobal<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(fetch: GlobalRef<'a, T>) -> Self {
        ReadNonSendGlobal {
            fetch,
            phantom: PhantomData,
        }
    }

    pub fn last_system_tick(&self) -> u32 {
        self.fetch.last_system_tick()
    }

    pub fn world_tick(&self) -> u32 {
        self.fetch.world_tick()
    }
}

impl<'w, T> DetectChanges for ReadNonSendGlobal<'w, T>
where
    T: Resource,
{
    /// Returns `true` if the resource was added after the system last ran.
    fn is_added(&self) -> bool {
        self.fetch
            .fetch
            .ticks
            .is_added(self.last_system_tick(), self.world_tick())
    }

    /// Returns `true` if the resource was added or mutably dereferenced after the system last ran.
    fn is_changed(&self) -> bool {
        self.fetch
            .fetch
            .ticks
            .is_changed(self.last_system_tick(), self.world_tick())
    }

    #[inline]
    fn last_changed(&self) -> u32 {
        self.fetch.fetch.ticks.changed.tick
    }
}

impl<'w, T> Debug for ReadNonSendGlobal<'w, T>
where
    T: Debug + Resource,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReadNonSendGlobal")
            .field(self.fetch.deref())
            .finish()
    }
}

impl<'a, T> Deref for ReadNonSendGlobal<'a, T>
where
    T: Resource,
{
    type Target = GlobalRef<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.fetch
    }
}

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
pub struct WriteNonSendGlobal<'a, T: 'a> {
    fetch: GlobalMut<'a, T>,
    phantom: PhantomData<*mut ()>,
}

impl<'a, T> WriteNonSendGlobal<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(fetch: GlobalMut<'a, T>) -> Self {
        WriteNonSendGlobal {
            fetch,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn last_system_tick(&self) -> u32 {
        self.fetch.last_system_tick()
    }

    #[inline]
    pub fn world_tick(&self) -> u32 {
        self.fetch.world_tick()
    }
}

impl<'w, T> DetectChanges for WriteNonSendGlobal<'w, T>
where
    T: Resource,
{
    /// Returns `true` if the resource was added after the system last ran.
    fn is_added(&self) -> bool {
        self.fetch
            .fetch
            .ticks
            .is_added(self.last_system_tick(), self.world_tick())
    }

    /// Returns `true` if the resource was added or mutably dereferenced after the system last ran.
    fn is_changed(&self) -> bool {
        self.fetch
            .fetch
            .ticks
            .is_changed(self.last_system_tick(), self.world_tick())
    }

    #[inline]
    fn last_changed(&self) -> u32 {
        self.fetch.fetch.ticks.changed.tick
    }
}

impl<'w, T> Debug for WriteNonSendGlobal<'w, T>
where
    T: Debug + Resource,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WriteGlobal")
            .field(self.fetch.deref())
            .finish()
    }
}

impl<'a, T> Deref for WriteNonSendGlobal<'a, T>
where
    T: Resource,
{
    type Target = GlobalMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.fetch
    }
}

// /// Allows to fetch a resource in a system immutably.
// /// **This will add a default value in a `System` setup if the resource does not exist.**
// pub type ReadGlobalDefault<'a, T> = ReadGlobal<'a, T, SetupDefault>;

// /// Allows to fetch a resource in a system mutably.
// /// **This will add a default value in a `System` setup if the resource does not exist.**
// pub type WriteGlobalDefault<'a, T> = WriteGlobal<'a, T, SetupDefault>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::change_detection::DetectChanges;
    // use crate::shred::{RunNow, System};

    #[derive(Default)]
    struct Res;

    // #[test]
    // fn fetch_aspects() {
    //     assert_eq!(Read::<Res>::reads(), vec![ResourceId::new::<Res>()]);
    //     assert_eq!(Read::<Res>::writes(), vec![]);

    //     let mut world = World::empty(0);
    //     world.insert(Res);
    //     <Read<Res> as SystemData>::fetch(&world);
    // }

    // #[test]
    // fn fetch_mut_aspects() {
    //     assert_eq!(Write::<Res>::reads(), vec![]);
    //     assert_eq!(Write::<Res>::writes(), vec![ResourceId::new::<Res>()]);

    //     let mut world = World::empty(0);
    //     world.insert(Res);
    //     <Write<Res> as SystemData>::fetch(&world);
    // }

    // #[test]
    // fn fetch_by_id() {
    //     #![allow(clippy::map_clone)] // False positive

    //     let mut globals = Globals::empty();

    //     globals.insert_by_id(ResourceId::new_with_dynamic_id::<i32>(1), 5);
    //     globals.insert_by_id(ResourceId::new_with_dynamic_id::<i32>(2), 15);
    //     globals.insert_by_id(ResourceId::new_with_dynamic_id::<i32>(3), 45);

    //     assert_eq!(
    //         globals
    //             .get_by_id::<i32>(ResourceId::new_with_dynamic_id::<i32>(2))
    //             .map(|x| *x),
    //         Some(15)
    //     );
    //     assert_eq!(
    //         globals
    //             .get_by_id::<i32>(ResourceId::new_with_dynamic_id::<i32>(1))
    //             .map(|x| *x),
    //         Some(5)
    //     );
    //     assert_eq!(
    //         globals
    //             .get_by_id::<i32>(ResourceId::new_with_dynamic_id::<i32>(3))
    //             .map(|x| *x),
    //         Some(45)
    //     );
    // }

    // #[test]
    // fn system_data() {
    //     let mut resources = World::empty(0);

    //     resources.insert(5u32);
    //     let x = *resources.system_data::<Read<u32>>();
    //     assert_eq!(x, 5);
    // }

    // #[test]
    // fn setup() {
    //     let mut resources = World::empty(0);

    //     resources.insert(5u32);
    //     resources.setup::<Read<u32>>();
    //     let x = *resources.system_data::<Read<u32>>();
    //     assert_eq!(x, 5);

    //     resources.remove::<u32>();
    //     resources.setup::<Read<u32>>();
    //     let x = *resources.system_data::<Read<u32>>();
    //     assert_eq!(x, 0);
    // }

    // #[test]
    // fn exec() {
    //     #![allow(clippy::float_cmp)]

    //     let mut resources = World::empty(0);

    //     resources.exec(|(float, boolean): (Read<f32>, Read<bool>)| {
    //         assert_eq!(*float, 0.0);
    //         assert!(!*boolean);
    //     });

    //     resources.exec(|(mut float, mut boolean): (Write<f32>, Write<bool>)| {
    //         *float = 4.3;
    //         *boolean = true;
    //     });

    //     resources.exec(|(float, boolean): (Read<f32>, ReadExpect<bool>)| {
    //         assert_eq!(*float, 4.3);
    //         assert!(*boolean);
    //     });
    // }

    // #[test]
    // #[should_panic]
    // fn exec_panic() {
    //     let mut resources = World::empty(0);

    //     resources.exec(|(_float, _boolean): (Write<f32>, Write<bool>)| {
    //         panic!();
    //     });
    // }

    // #[test]
    // fn invalid_fetch_by_id0() {
    //     let mut globals = Globals::empty();

    //     globals.insert(5i32);

    //     assert!(globals
    //         .get_by_id::<u32>(ResourceId::new_with_dynamic_id::<i32>(111))
    //         .is_none());
    // }

    // #[test]
    // fn invalid_fetch_by_id1() {
    //     let mut globals = Globals::empty();

    //     globals.insert(5i32);

    //     assert!(globals
    //         .get_by_id::<i32>(ResourceId::new_with_dynamic_id::<u32>(111))
    //         .is_none());
    // }

    #[test]
    fn add() {
        struct Foo;

        let mut globals = Globals::empty();
        globals.insert(Res, 99);

        assert!(globals.contains::<Res>());
        assert!(!globals.contains::<Foo>());
    }

    #[allow(unused)]
    #[test]
    #[should_panic(expected = "already immutably borrowed")]
    fn read_write_fails() {
        let mut globals = Globals::empty();
        globals.insert(Res, 99);

        let read: GlobalRef<Res> = globals.fetch::<Res>(99, 99);
        let write: GlobalMut<Res> = globals.fetch_mut::<Res>(99, 99);
    }

    #[allow(unused)]
    #[test]
    #[should_panic(expected = "already mutably borrowed")]
    fn write_read_fails() {
        let mut globals = Globals::empty();
        globals.insert(Res, 99);

        let write: GlobalMut<Res> = globals.fetch_mut::<Res>(99, 99);
        let read: GlobalRef<Res> = globals.fetch::<Res>(99, 99);
    }

    #[test]
    fn remove_insert() {
        let mut globals = Globals::empty();

        globals.insert(Res, 99);

        assert!(globals.contains::<Res>());

        // println!("{:#?}", resources.hashmap.keys().collect::<Vec<_>>());

        globals.remove::<Res>(100).unwrap();

        assert!(!globals.contains::<Res>());

        globals.insert(Res, 102);

        assert!(globals.contains::<Res>());
    }

    // #[test]
    // fn default_works() {
    //     struct Sys;

    //     impl<'a> System<'a> for Sys {
    //         type SystemData = Write<'a, i32>;

    //         fn run(&mut self, mut data: Self::SystemData) {
    //             assert_eq!(*data, 0);

    //             *data = 33;
    //         }
    //     }

    //     let mut resources = Globals::empty();
    //     assert!(resources.try_fetch::<i32>().is_none());

    //     let mut sys = Sys;
    //     RunNow::setup(&mut sys, &mut resources);

    //     sys.run_now(&resources);

    //     assert!(resources.try_fetch::<i32>().is_some());
    //     assert_eq!(*resources.fetch::<i32>(), 33);
    // }

    #[test]
    fn simple_read_write_test() {
        struct TestOne {
            value: String,
        }

        struct TestTwo {
            value: String,
        }

        struct NotSync {
            ptr: *const u8,
        }

        let mut globals = Globals::default();
        globals.insert(
            TestOne {
                value: "one".to_string(),
            },
            99,
        );

        globals.insert(
            TestTwo {
                value: "two".to_string(),
            },
            99,
        );

        globals.insert_non_send(
            NotSync {
                ptr: std::ptr::null(),
            },
            99,
        );

        assert_eq!(globals.fetch::<TestOne>(100, 102).value, "one");
        assert_eq!(globals.fetch::<TestTwo>(100, 102).value, "two");
        assert_eq!(globals.fetch::<NotSync>(100, 102).ptr, std::ptr::null());

        // test re-ownership
        let owned = globals.remove::<TestTwo>(104);
        assert_eq!(owned.unwrap().value, "two");
    }

    #[test]
    fn last_changed() {
        struct Data(u32);

        let mut globals = Globals::default();

        globals.maintain(99);

        globals.insert(Data(5), 99);

        globals.maintain(102);

        {
            let borrow = globals.fetch::<Data>(100, 102);
            assert_eq!(borrow.0, 5);
            assert_eq!(borrow.last_changed(), 99);
            // assert_eq!(borrow.inserted_tick().tick, 99);
        }

        globals.maintain(103);

        {
            let mut borrow = globals.fetch_mut::<Data>(102, 106);
            assert_eq!(borrow.0, 5);
            // assert_eq!(borrow.inserted_tick().tick, 99);
            assert_eq!(borrow.last_changed(), 99);
            borrow.0 = 8;
            // assert_eq!(borrow.inserted_tick().tick, 99);
            assert_eq!(borrow.last_changed(), 106);
            assert_eq!(borrow.0, 8);
        }

        globals.maintain(107);

        {
            let borrow = globals.fetch::<Data>(106, 110);
            assert_eq!(borrow.0, 8);
            // assert_eq!(borrow.inserted_tick().tick, 99);
            assert_eq!(borrow.last_changed(), 106);
        }

        {
            globals.remove::<Data>(114);
            // assert_eq!(globals.deleted::<Data>().unwrap(), 126);
        }
    }
}
