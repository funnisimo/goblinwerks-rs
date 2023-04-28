use crate::globals::{GlobalFetch, GlobalFetchMut, Globals};
use crate::shred::cell::TrustCell;
use crate::shred::{Fetch, FetchMut, SystemData, World as Resources};
use crate::{Resource, ResourceId};

pub use crate::shred::Entry;

#[derive(Default)]
pub struct World {
    resources: Resources,
    globals: Globals,
}

impl World {
    /// Creates a new, empty resource container.
    ///
    /// Note that if you're using Specs, you should use `WorldExt::new` instead.
    pub fn empty() -> Self {
        Default::default()
    }

    pub fn new(globals: Globals) -> Self {
        World {
            resources: Resources::empty(),
            globals,
        }
    }

    pub(crate) fn set_globals(&mut self, globals: Globals) {
        self.globals = globals;
    }

    /// Inserts a resource into this container. If the resource existed before,
    /// it will be overwritten.
    ///
    /// # Examples
    ///
    /// Every type satisfying `Any + Send + Sync` automatically
    /// implements `Resource`, thus can be added:
    ///
    /// ```rust
    /// # #![allow(dead_code)]
    /// struct MyRes(i32);
    /// ```
    ///
    /// When you have a resource, simply insert it like this:
    ///
    /// ```rust
    /// # struct MyRes(i32);
    /// use shred::World;
    ///
    /// let mut world = World::empty();
    /// world.insert(MyRes(5));
    /// ```
    pub fn insert<R>(&mut self, r: R)
    where
        R: Resource,
    {
        self.insert_by_id(ResourceId::new::<R>(), r);
    }

    /// Removes a resource of type `R` from the `World` and returns its
    /// ownership to the caller. In case there is no such resource in this
    /// `World`, `None` will be returned.
    ///
    /// Use this method with caution; other functions and systems might assume
    /// this resource still exists. Thus, only use this if you're sure no
    /// system will try to access this resource after you removed it (or else
    /// you will get a panic).
    pub fn remove<R>(&mut self) -> Option<R>
    where
        R: Resource,
    {
        self.remove_by_id(ResourceId::new::<R>())
    }

    /// Returns true if the specified resource type `R` exists in `self`.
    pub fn has_value<R>(&self) -> bool
    where
        R: Resource,
    {
        self.has_value_raw(ResourceId::new::<R>())
    }

    /// Returns true if the specified resource type exists in `self`.
    pub fn has_value_raw(&self, id: ResourceId) -> bool {
        self.resources.has_value_raw(id)
    }

    /// Returns an entry for the resource with type `R`.
    pub fn entry<R>(&mut self) -> Entry<R>
    where
        R: Resource,
    {
        self.resources.entry::<R>()
    }

    /// Gets `SystemData` `T` from the `World`. This can be used to retrieve
    /// data just like in [System](crate::System)s.
    ///
    /// This will not setup the system data, i.e. resources fetched here must
    /// exist already.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shred::*;
    /// # #[derive(Default)] struct Timer; #[derive(Default)] struct AnotherResource;
    ///
    /// // NOTE: If you use Specs, use `World::new` instead.
    /// let mut world = World::empty();
    /// world.insert(Timer);
    /// world.insert(AnotherResource);
    /// let system_data: (Read<Timer>, Read<AnotherResource>) = world.system_data();
    /// ```
    ///
    /// # Panics
    ///
    /// * Panics if `T` is already borrowed in an incompatible way.
    pub fn system_data<'a, T>(&'a self) -> T
    where
        T: SystemData<'a>,
    {
        SystemData::fetch(self)
    }

    /// Sets up system data `T` for fetching afterwards.
    ///
    /// Most `SystemData` implementations will insert a sensible default value,
    /// by implementing [SystemData::setup]. However, it is not guaranteed to
    /// do that; if there is no sensible default, `setup` might not do anything.
    ///
    /// # Examples
    ///
    /// ```
    /// use shred::{Read, World};
    ///
    /// #[derive(Default)]
    /// struct MyCounter(u32);
    ///
    /// // NOTE: If you use Specs, use `World::new` instead.
    /// let mut world = World::empty();
    /// assert!(!world.has_value::<MyCounter>());
    ///
    /// // `Read<MyCounter>` requires a `Default` implementation, and uses
    /// // that to initialize the resource
    /// world.setup::<Read<MyCounter>>();
    /// assert!(world.has_value::<MyCounter>());
    /// ```
    ///
    /// Here's another example, showing the case where no resource gets
    /// initialized:
    ///
    /// ```
    /// use shred::{ReadExpect, World};
    ///
    /// struct MyCounter(u32);
    ///
    /// // NOTE: If you use Specs, use `World::new` instead.
    /// let mut world = World::empty();
    ///
    /// world.setup::<ReadExpect<MyCounter>>();
    /// ```
    pub fn setup<'a, T: SystemData<'a>>(&mut self) {
        T::setup(self);
    }

    /// Executes `f` once, right now and with the specified system data.
    ///
    /// This sets up the system data `f` expects, fetches it and then
    /// executes `f`. This is essentially like a one-time
    /// [System](crate::System).
    ///
    /// This is especially useful if you either need a lot of system data or,
    /// with Specs, if you want to build an entity and for that you need to
    /// access resources first - just fetching the resources and building
    /// the entity would cause a double borrow.
    ///
    /// **Calling this method is equivalent to:**
    ///
    /// ```
    /// # use shred::*;
    /// # struct MySystemData; impl MySystemData { fn do_something(&self) {} }
    /// # impl<'a> SystemData<'a> for MySystemData {
    /// #     fn fetch(res: &World) -> Self { MySystemData }
    /// #     fn reads() -> Vec<ResourceId> { vec![] }
    /// #     fn writes() -> Vec<ResourceId> { vec![] }
    /// #     fn setup(res: &mut World) {}
    /// # }
    /// # let mut world = World::empty();
    /// {
    ///     // note the extra scope
    ///     world.setup::<MySystemData>();
    ///     let my_data: MySystemData = world.system_data();
    ///     my_data.do_something();
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use shred::*;
    /// // NOTE: If you use Specs, use `World::new` instead.
    /// let mut world = World::empty();
    ///
    /// #[derive(Default)]
    /// struct MyRes {
    ///     field: i32,
    /// }
    ///
    /// world.exec(|(mut my_res,): (Write<MyRes>,)| {
    ///     assert_eq!(my_res.field, 0);
    ///     my_res.field = 5;
    /// });
    ///
    /// assert_eq!(world.fetch::<MyRes>().field, 5);
    /// ```
    pub fn exec<'a, F, R, T>(&'a mut self, f: F) -> R
    where
        F: FnOnce(T) -> R,
        T: SystemData<'a>,
    {
        self.setup::<T>();
        f(self.system_data())
    }

    /// Fetches the resource with the specified type `T` or panics if it doesn't
    /// exist.
    ///
    /// # Panics
    ///
    /// Panics if the resource doesn't exist.
    /// Panics if the resource is being accessed mutably.
    pub fn fetch<T>(&self) -> Fetch<T>
    where
        T: Resource,
    {
        self.try_fetch().unwrap_or_else(|| {
            if self.resources.is_empty() {
                eprintln!(
                    "Note: Could not find a resource (see the following panic);\
                     the `World` is completely empty. Did you accidentally create a fresh `World`?"
                )
            }

            fetch_panic!()
        })
    }

    /// Like `fetch`, but returns an `Option` instead of inserting a default
    /// value in case the resource does not exist.
    pub fn try_fetch<T>(&self) -> Option<Fetch<T>>
    where
        T: Resource,
    {
        self.resources.try_fetch::<T>()
    }

    /// Like `try_fetch`, but fetches the resource by its `ResourceId` which
    /// allows using a dynamic ID.
    ///
    /// This is usually not what you need; please read the type-level
    /// documentation of `ResourceId`.
    ///
    /// # Panics
    ///
    /// This method panics if `id` refers to a different type ID than `T`.
    pub fn try_fetch_by_id<T>(&self, id: ResourceId) -> Option<Fetch<T>>
    where
        T: Resource,
    {
        self.resources.try_fetch_by_id(id)
    }

    /// Fetches the resource with the specified type `T` mutably.
    ///
    /// Please see `fetch` for details.
    ///
    /// # Panics
    ///
    /// Panics if the resource doesn't exist.
    /// Panics if the resource is already being accessed.
    pub fn fetch_mut<T>(&self) -> FetchMut<T>
    where
        T: Resource,
    {
        self.try_fetch_mut().unwrap_or_else(|| fetch_panic!())
    }

    /// Like `fetch_mut`, but returns an `Option` instead of inserting a default
    /// value in case the resource does not exist.
    pub fn try_fetch_mut<T>(&self) -> Option<FetchMut<T>>
    where
        T: Resource,
    {
        self.resources.try_fetch_mut()
    }

    /// Like `try_fetch_mut`, but fetches the resource by its `ResourceId` which
    /// allows using a dynamic ID.
    ///
    /// This is usually not what you need; please read the type-level
    /// documentation of `ResourceId`.
    ///
    /// # Panics
    ///
    /// This method panics if `id` refers to a different type ID than `T`.
    pub fn try_fetch_mut_by_id<T>(&self, id: ResourceId) -> Option<FetchMut<T>>
    where
        T: Resource,
    {
        self.resources.try_fetch_mut_by_id(id)
    }

    /// Internal function for inserting resources, should only be used if you
    /// know what you're doing.
    ///
    /// This is useful for inserting resources with a custom `ResourceId`.
    ///
    /// # Panics
    ///
    /// This method panics if `id` refers to a different type ID than `R`.
    pub fn insert_by_id<R>(&mut self, id: ResourceId, r: R)
    where
        R: Resource,
    {
        self.resources.insert_by_id(id, r)
    }

    /// Internal function for removing resources, should only be used if you
    /// know what you're doing.
    ///
    /// This is useful for removing resources with a custom `ResourceId`.
    ///
    /// # Panics
    ///
    /// This method panics if `id` refers to a different type ID than `R`.
    pub fn remove_by_id<R>(&mut self, id: ResourceId) -> Option<R>
    where
        R: Resource,
    {
        self.resources.remove_by_id(id)
    }

    /// Internal function for fetching resources, should only be used if you
    /// know what you're doing.
    pub(crate) fn try_fetch_internal(
        &self,
        id: ResourceId,
    ) -> Option<&TrustCell<Box<dyn Resource>>> {
        self.resources.try_fetch_internal(id)
    }

    // /// Retrieves a resource without fetching, which is cheaper, but only
    // /// available with `&mut self`.
    // pub fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
    //     self.get_mut_raw(ResourceId::new::<T>())
    //         .map(|res| unsafe { res.downcast_mut_unchecked() })
    // }

    // /// Retrieves a resource without fetching, which is cheaper, but only
    // /// available with `&mut self`.
    // pub fn get_mut_raw(&mut self, id: ResourceId) -> Option<&mut dyn Resource> {
    //     self.resources
    //         .get_mut(&id)
    //         .map(TrustCell::get_mut)
    //         .map(Box::as_mut)
    // }

    // GLOBALS

    pub fn has_global<G: Resource>(&self) -> bool {
        self.globals.has_value::<G>()
    }

    /// Inserts a global
    pub fn insert_global<G: Resource>(&mut self, global: G) {
        self.globals.insert(global);
    }

    /// Removes a global
    pub fn remove_global<G: Resource>(&mut self) -> Option<G> {
        self.globals.remove::<G>()
    }

    /// Fetch a global value
    pub fn fetch_global<G: Resource>(&self) -> GlobalFetch<G> {
        self.globals.fetch::<G>()
    }

    /// Fetch a global value as mutable
    pub fn fetch_global_mut<G: Resource>(&self) -> GlobalFetchMut<G> {
        self.globals.fetch_mut::<G>()
    }

    /// Try to fetch a global value
    pub fn try_fetch_global<G: Resource>(&self) -> Option<GlobalFetch<G>> {
        self.globals.try_fetch::<G>()
    }

    /// Try to fetch a global value mutably
    pub fn try_fetch_global_mut<G: Resource>(&self) -> Option<GlobalFetchMut<G>> {
        self.globals.try_fetch_mut::<G>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shred::{Read, ReadExpect, Write};
    use crate::shred::{RunNow, System, SystemData};

    #[derive(Default)]
    struct Res;

    #[test]
    fn fetch_aspects() {
        assert_eq!(Read::<Res>::reads(), vec![ResourceId::new::<Res>()]);
        assert_eq!(Read::<Res>::writes(), vec![]);

        let mut world = World::empty();
        world.insert(Res);
        <Read<Res> as SystemData>::fetch(&world);
    }

    #[test]
    fn fetch_mut_aspects() {
        assert_eq!(Write::<Res>::reads(), vec![]);
        assert_eq!(Write::<Res>::writes(), vec![ResourceId::new::<Res>()]);

        let mut world = World::empty();
        world.insert(Res);
        <Write<Res> as SystemData>::fetch(&world);
    }

    #[test]
    fn system_data() {
        let mut world = World::empty();

        world.insert(5u32);
        let x = *world.system_data::<Read<u32>>();
        assert_eq!(x, 5);
    }

    #[test]
    fn setup() {
        let mut world = World::empty();

        world.insert(5u32);
        world.setup::<Read<u32>>();
        let x = *world.system_data::<Read<u32>>();
        assert_eq!(x, 5);

        world.remove::<u32>();
        world.setup::<Read<u32>>();
        let x = *world.system_data::<Read<u32>>();
        assert_eq!(x, 0);
    }

    #[test]
    fn exec() {
        #![allow(clippy::float_cmp)]

        let mut world = World::empty();

        world.exec(|(float, boolean): (Read<f32>, Read<bool>)| {
            assert_eq!(*float, 0.0);
            assert!(!*boolean);
        });

        world.exec(|(mut float, mut boolean): (Write<f32>, Write<bool>)| {
            *float = 4.3;
            *boolean = true;
        });

        world.exec(|(float, boolean): (Read<f32>, ReadExpect<bool>)| {
            assert_eq!(*float, 4.3);
            assert!(*boolean);
        });
    }

    #[test]
    #[should_panic]
    fn exec_panic() {
        let mut world = World::empty();

        world.exec(|(_float, _boolean): (Write<f32>, Write<bool>)| {
            panic!();
        });
    }

    #[test]
    fn default_works() {
        struct Sys;

        impl<'a> System<'a> for Sys {
            type SystemData = Write<'a, i32>;

            fn run(&mut self, mut data: Self::SystemData) {
                assert_eq!(*data, 0);

                *data = 33;
            }
        }

        let mut world = World::empty();
        assert!(world.try_fetch::<i32>().is_none());

        let mut sys = Sys;
        RunNow::setup(&mut sys, &mut world);

        sys.run_now(&world);

        assert!(world.try_fetch::<i32>().is_some());
        assert_eq!(*world.fetch::<i32>(), 33);
    }
}
