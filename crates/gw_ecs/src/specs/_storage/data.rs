use crate::legion::{ResMut, ResRef};
use crate::shred::{ResourceId, SystemData};
use crate::specs::{
    storage::{MaskedStorage, Storage},
    world::{Component, EntitiesRes},
};
use crate::World;
use std::collections::HashSet;

/// A storage with read access.
///
/// This is just a type alias for a fetched component storage.
///
/// The main functionality it provides is listed in the following,
/// however make sure to also check out the documentation for the
/// respective methods on `Storage`.
///
/// ## Aliasing
///
/// **It is strictly disallowed to get both a `ReadStorage` and a `WriteStorage`
/// of the same component.**
/// Because Specs uses interior mutability for its resources, we can't check
/// this at compile time. If you try to do this, you will get a panic.
///
/// It is explicitly allowed to get multiple `ReadStorage`s for the same
/// component.
///
/// ## Joining storages
///
/// `&ReadStorage` implements `Join`, which allows to do
/// something like this:
///
/// ```
/// use specs::prelude::*;
///
/// struct Pos;
/// impl Component for Pos {
///     type Storage = VecStorage<Self>;
/// }
/// struct Vel;
/// impl Component for Vel {
///     type Storage = VecStorage<Self>;
/// }
///
/// let mut world = World::new();
/// world.register::<Pos>();
/// world.register::<Vel>();
/// let pos_storage = world.read_storage::<Pos>();
/// let vel_storage = world.read_storage::<Vel>();
///
/// for (pos, vel) in (&pos_storage, &vel_storage).join() {}
/// ```
///
/// This joins the position and the velocity storage, which means it only
/// iterates over the components of entities that have both a position
/// **and** a velocity.
///
/// ## Retrieving single components
///
/// If you have an entity (for example because you stored it before
/// or because you're joining over `Entities`), you can get a single
/// component by calling `Storage::get`:
///
/// ```
/// # use specs::prelude::*;
/// # #[derive(Debug, PartialEq)]
/// # struct Pos; impl Component for Pos { type Storage = VecStorage<Self>; }
/// # #[derive(Debug, PartialEq)]
/// # struct Vel; impl Component for Vel { type Storage = VecStorage<Self>; }
/// #
/// # let mut world = World::new(); world.register::<Pos>(); world.register::<Vel>();
/// let entity1 = world.create_entity().with(Pos).build();
/// let entity2 = world.create_entity().with(Vel).build();
///
/// # let pos_storage = world.read_storage::<Pos>();
/// # let vel_storage = world.read_storage::<Vel>();
/// assert_eq!(pos_storage.get(entity1), Some(&Pos));
/// assert_eq!(pos_storage.get(entity2), None);
///
/// assert_eq!(vel_storage.get(entity1), None);
/// assert_eq!(vel_storage.get(entity2), Some(&Vel));
/// ```
///
/// ## Usage as `SystemData`
///
/// `ReadStorage` implements `SystemData` which allows you to
/// get it inside a system by simply adding it to the tuple:
///
/// ```
/// # use specs::prelude::*;
/// #[derive(Debug)]
/// struct Pos {
///     x: f32,
///     y: f32,
/// }
///
/// impl Component for Pos {
///     type Storage = VecStorage<Self>;
/// }
///
/// struct Sys;
///
/// impl<'a> System<'a> for Sys {
///     type SystemData = (Entities<'a>, ReadStorage<'a, Pos>);
///
///     fn run(&mut self, (ent, pos): Self::SystemData) {
///         for (ent, pos) in (&*ent, &pos).join() {
///             println!("Entitiy with id {} has a position of {:?}", ent.id(), pos);
///         }
///     }
/// }
/// ```
///
/// These operations can't mutate anything; if you want to do
/// insertions or modify components, you need to use `WriteStorage`.
/// Note that you can also use `LazyUpdate` , which does insertions on
/// `World::maintain`. This allows more concurrency and is designed
/// to be used for entity initialization.
pub type ReadComp<'a, T> = Storage<'a, T, ResRef<'a, MaskedStorage<T>>>;

impl<'a, T> SystemData<'a> for ReadComp<'a, T>
where
    T: Component,
{
    fn setup(world: &mut World) {
        // world.resources.get_or_insert_with(|| {
        //     MaskedStorage::<T>::new(<T::Storage as TryDefault>::unwrap_default())
        // });
        // world
        //     .resources
        //     .get_mut::<MetaTable<dyn AnyStorage>>()
        //     .unwrap()
        //     .register(&*world.resources.get::<MaskedStorage<T>>().unwrap());
        world.register::<T>();
    }

    fn fetch(world: &'a World) -> Self {
        world.read_component::<T>()
    }

    fn reads() -> HashSet<ResourceId> {
        let mut reads = HashSet::new();
        reads.insert(ResourceId::new::<EntitiesRes>());
        reads.insert(ResourceId::new::<MaskedStorage<T>>());
        reads
    }

    // fn writes() -> Vec<ResourceId> {
    //     vec![]
    // }
}

/// A storage with read and write access.
///
/// Additionally to what `ReadStorage` can do a storage with mutable access
/// allows:
///
/// ## Aliasing
///
/// **It is strictly disallowed to fetch both a `ReadStorage` and a
/// `WriteStorage` of the same component.**
/// Because Specs uses interior mutability for its resources, we can't check
/// this at compile time. If you try to do this, you will get a panic.
///
/// It is also disallowed to fetch multiple `WriteStorage`s for the same
/// component.
///
/// ## Retrieve components mutably
///
/// This works just like `Storage::get`, but returns a mutable reference:
///
/// ```
/// # use specs::prelude::*;
/// # #[derive(Debug, PartialEq)]
/// # struct Pos(f32); impl Component for Pos { type Storage = VecStorage<Self>; }
/// #
/// # let mut world = World::new(); world.register::<Pos>();
/// let entity = world.create_entity().with(Pos(2.0)).build();
/// # let mut pos_storage = world.write_storage::<Pos>();
///
/// assert_eq!(pos_storage.get_mut(entity), Some(&mut Pos(2.0)));
/// if let Some(pos) = pos_storage.get_mut(entity) {
///     *pos = Pos(4.5);
/// }
///
/// assert_eq!(pos_storage.get(entity), Some(&Pos(4.5)));
/// ```
///
/// ## Inserting and removing components
///
/// You can insert components using `Storage::insert` and remove them
/// again with `Storage::remove`.
///
/// ```
/// # use specs::prelude::*;
/// # use specs::storage::InsertResult;
/// # #[derive(Debug, PartialEq)]
/// # struct Pos(f32); impl Component for Pos { type Storage = VecStorage<Self>; }
/// #
/// # let mut world = World::new(); world.register::<Pos>();
/// let entity = world.create_entity().with(Pos(0.1)).build();
/// # let mut pos_storage = world.write_storage::<Pos>();
///
/// if let Ok(Some(p)) = pos_storage.insert(entity, Pos(4.0)) {
///     println!("Overwrote {:?} with a new position", p);
/// }
/// ```
///
/// There's also an Entry-API similar to the one provided by
/// `std::collections::HashMap`.
pub type WriteComp<'a, T> = Storage<'a, T, ResMut<'a, MaskedStorage<T>>>;

impl<'a, T> SystemData<'a> for WriteComp<'a, T>
where
    T: Component,
{
    fn setup(world: &mut World) {
        // world.resources.get_or_insert_with(|| {
        //     MaskedStorage::<T>::new(<T::Storage as TryDefault>::unwrap_default())
        // });
        // world
        //     .resources
        //     .get_mut::<MetaTable<dyn AnyStorage>>()
        //     .unwrap()
        //     .register(&*world.resources.get::<MaskedStorage<T>>().unwrap());
        world.register::<T>();
    }

    fn fetch(world: &'a World) -> Self {
        world.write_component::<T>()
    }

    fn reads() -> HashSet<ResourceId> {
        let mut reads = HashSet::new();
        reads.insert(ResourceId::new::<EntitiesRes>());
        reads
    }

    fn writes() -> HashSet<ResourceId> {
        let mut writes = HashSet::new();
        writes.insert(ResourceId::new::<MaskedStorage<T>>());
        writes
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        schedule::Schedule,
        specs::{prelude::*, storage::MaskedStorage},
    };

    struct Foo;
    impl Component for Foo {
        type Storage = VecStorage<Self>;
    }

    struct Sys;
    impl<'a> System<'a> for Sys {
        type SystemData = ReadComp<'a, Foo>;

        fn run(&mut self, _data: <Self as System>::SystemData) {
            unimplemented!()
        }
    }

    #[test]
    fn uses_setup() {
        let mut w = World::empty(1);

        let mut d = Schedule::new().with("UPDATE", Sys);

        assert!(!w.has_resource::<MaskedStorage<Foo>>());

        d.setup(&mut w);

        assert!(w.has_resource::<MaskedStorage<Foo>>());
    }
}