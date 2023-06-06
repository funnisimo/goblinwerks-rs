//! Entity handling types.
//!
//! An **entity** exclusively owns zero or more [component] instances, all of different types, and can dynamically acquire or lose them over its lifetime.
//!
//! **empty entity**: Entity with zero components.
//! **pending entity**: Entity reserved, but not flushed yet (see [`Entities::flush`] docs for reference).
//! **reserved entity**: same as **pending entity**.
//! **invalid entity**: **pending entity** flushed with invalid (see [`Entities::flush_as_invalid`] docs for reference).
//!
//! See [`Entity`] to learn more.
//!
//! [component]: crate::component::Component
//!
//! # Usage
//!
//! Operations involving entities and their components are performed either from a system by submitting commands,
//! or from the outside (or from an exclusive system) by directly using [`World`] methods:
//!
//! |Operation|Command|Method|
//! |:---:|:---:|:---:|
//! |Spawn an entity with components|[`Commands::spawn`]|[`World::spawn`]|
//! |Spawn an entity without components|[`Commands::spawn_empty`]|[`World::spawn_empty`]|
//! |Despawn an entity|[`EntityCommands::despawn`]|[`World::despawn`]|
//! |Insert a component, bundle, or tuple of components and bundles to an entity|[`EntityCommands::insert`]|[`EntityMut::insert`]|
//! |Remove a component, bundle, or tuple of components and bundles from an entity|[`EntityCommands::remove`]|[`EntityMut::remove`]|
//!
//! [`World`]: crate::world::World
//! [`Commands::spawn`]: crate::system::Commands::spawn
//! [`Commands::spawn_empty`]: crate::system::Commands::spawn_empty
//! [`EntityCommands::despawn`]: crate::system::EntityCommands::despawn
//! [`EntityCommands::insert`]: crate::system::EntityCommands::insert
//! [`EntityCommands::remove`]: crate::system::EntityCommands::remove
//! [`World::spawn`]: crate::world::World::spawn
//! [`World::spawn_empty`]: crate::world::World::spawn_empty
//! [`World::despawn`]: crate::world::World::despawn
//! [`EntityMut::insert`]: crate::world::EntityMut::insert
//! [`EntityMut::remove`]: crate::world::EntityMut::remove
mod builder;
mod entities;
mod generation;
mod map_entities;

pub use builder::*;
pub use entities::*;
pub use generation::*;
pub use map_entities::*;

use hibitset::{AtomicBitSet, BitSet};
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "parallel")]
use crate::specs::join::ParJoin;

/// An index is basically the id of an `Entity`.
pub type Index = u32;

/// Internally used structure for `Entity` allocation.
#[derive(Default, Debug)]
pub(crate) struct EntityAllocator {
    generations: Vec<ZeroableGeneration>,

    alive: BitSet,
    raised: AtomicBitSet,
    killed: AtomicBitSet,
    cache: EntityCache,
    max_id: AtomicUsize,
}

impl EntityAllocator {
    /// Kills a list of entities immediately.
    /// Returns true if all of the entities were alive and are now dead.
    pub fn kill(&mut self, delete: &[Entity]) -> bool {
        let mut all_killed = true;
        for &entity in delete {
            let id = entity.id() as usize;

            if !self.is_alive(entity) {
                // TODO - Log?  Warn?
                all_killed = false;
                continue;
            }

            self.alive.remove(entity.id());
            // If the `Entity` was killed by `kill_atomic`, remove the bit set by it.
            self.killed.remove(entity.id());

            self.update_generation_length(id);

            if self.raised.remove(entity.id()) {
                self.generations[id].raise();
            }
            self.generations[id].die();

            self.cache.push(entity.id());
        }

        // self.cache.extend(delete.iter().map(|e| e.0));

        all_killed
    }

    /// Kills and entity atomically (will be updated when the allocator is
    /// maintained).
    /// Returns true if entity was alive and is now dead.
    pub fn kill_atomic(&self, e: Entity) -> bool {
        if !self.is_alive(e) {
            return false;
        }

        self.killed.add_atomic(e.id());
        true
    }

    // pub(crate) fn del_err(&self, e: Entity) -> Result<(), WrongGeneration> {
    //     Err(WrongGeneration {
    //         action: "delete",
    //         actual_gen: self.generations[e.id() as usize]
    //             .0
    //             .unwrap_or_else(Generation::one),
    //         entity: e,
    //     })
    // }

    /// Return `true` if the entity is alive.
    pub fn is_alive(&self, e: Entity) -> bool {
        e.gen()
            == match self.generations.get(e.id() as usize) {
                Some(g) if !g.is_alive() && self.raised.contains(e.id()) => g.raised(),
                Some(g) => g.0.unwrap_or_else(Generation::one),
                None => Generation::one(),
            }
    }

    /// Returns the `Generation` of the given `Index`, if any.
    pub fn generation(&self, id: Index) -> Option<Generation> {
        self.generations
            .get(id as usize)
            .cloned()
            .and_then(|gen| gen.0)
    }

    /// Returns the current alive entity with the given `Index`.
    pub fn entity(&self, id: Index) -> Entity {
        let gen = match self.generations.get(id as usize) {
            Some(g) if !g.is_alive() && self.raised.contains(id) => g.raised(),
            Some(g) => g.0.unwrap_or_else(Generation::one),
            None => Generation::one(),
        };

        Entity(id, gen)
    }

    /// Allocate a new entity
    pub fn allocate_atomic(&self) -> Entity {
        let id = self.cache.pop_atomic().unwrap_or_else(|| {
            atomic_increment(&self.max_id).expect("No entity left to allocate") as Index
        });

        self.raised.add_atomic(id);
        let gen = self
            .generation(id)
            .map(|gen| if gen.is_alive() { gen } else { gen.raised() })
            .unwrap_or_else(Generation::one);
        Entity(id, gen)
    }

    /// Allocate a new entity
    #[allow(dead_code)]
    pub fn allocate(&mut self) -> Entity {
        let id = self.cache.pop().unwrap_or_else(|| {
            let id = *self.max_id.get_mut();
            *self.max_id.get_mut() = id.checked_add(1).expect("No entity left to allocate");
            id as Index
        });

        self.update_generation_length(id as usize);

        self.alive.add(id as Index);

        let gen = self.generations[id as usize].raise();

        Entity(id as Index, gen)
    }

    /// Maintains the allocated entities, mainly dealing with atomically
    /// allocated or killed entities.
    pub fn merge(&mut self) -> Vec<Entity> {
        use hibitset::BitSetLike;

        let mut deleted = vec![];

        let max_id = *self.max_id.get_mut();
        self.update_generation_length(max_id + 1);

        for i in (&self.raised).iter() {
            self.generations[i as usize].raise();
            self.alive.add(i);
        }
        self.raised.clear();

        for i in (&self.killed).iter() {
            self.alive.remove(i);
            deleted.push(Entity(i, self.generations[i as usize].0.unwrap()));
            self.generations[i as usize].die();
        }
        self.killed.clear();

        self.cache.extend(deleted.iter().map(|e| e.0));

        deleted
    }

    fn update_generation_length(&mut self, i: usize) {
        if self.generations.len() <= i as usize {
            self.generations
                .resize(i as usize + 1, ZeroableGeneration(None));
        }
    }
}

/// An iterator for entity creation.
/// Please note that you have to consume
/// it because iterators are lazy.
///
/// Returned from `Entities::create_iter`.
pub struct CreateIterAtomic<'a>(&'a EntityAllocator);

impl<'a> Iterator for CreateIterAtomic<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        Some(self.0.allocate_atomic())
    }
}

/// `Entity` type, as seen by the user.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entity(Index, Generation);

impl Entity {
    /// Creates a new entity (externally from ECS).
    #[cfg(test)]
    pub fn new(index: Index, gen: Generation) -> Self {
        Self(index, gen)
    }

    /// Creates a dead entity that can be a placeholder in structs.
    pub fn dead() -> Self {
        Self(0, Generation::zero())
    }

    /// Returns the index of the `Entity`.
    #[inline]
    pub fn id(self) -> Index {
        self.0
    }

    /// Returns the `Generation` of the `Entity`.
    #[inline]
    pub fn gen(self) -> Generation {
        self.1
    }
}

/// The entities of this ECS. This is a resource, stored in the `World`.
/// If you just want to access it in your system, you can also use the
/// `Entities` type def.
///
/// **Please note that you should never get
/// this mutably in a system, because it would
/// block all the other systems.**
///
/// You need to call `World::maintain` after creating / deleting
/// entities with this struct.
#[derive(Debug, Default)]
pub struct EntitiesRes {
    pub(crate) alloc: EntityAllocator,
}

impl EntitiesRes {
    /// Creates a new entity atomically.
    /// This will be persistent as soon
    /// as you call `World::maintain`.
    ///
    /// If you want a lazy entity builder, take a look
    /// at `LazyUpdate::create_entity`.
    ///
    /// In case you have access to the `World`,
    /// you can also use `World::create_entity` which
    /// creates the entity and the components immediately.
    pub fn create(&self) -> Entity {
        self.alloc.allocate_atomic()
    }

    /// Returns an iterator which creates
    /// new entities atomically.
    /// They will be persistent as soon
    /// as you call `World::maintain`.
    pub fn create_iter(&self) -> CreateIterAtomic {
        CreateIterAtomic(&self.alloc)
    }

    // /// Similar to the `create` method above this
    // /// creates an entity atomically, and then returns a
    // /// builder which can be used to insert components into
    // /// various storages if available.
    // pub fn build_entity(&self) -> EntityResBuilder {
    //     let entity = self.create();
    //     EntityResBuilder {
    //         entity,
    //         entities: self,
    //         built: false,
    //     }
    // }

    /// Deletes an entity atomically.
    /// The associated components will be
    /// deleted as soon as you call `World::maintain`.
    pub fn delete(&self, e: Entity) {
        let _ = self.alloc.kill_atomic(e);
    }

    /// Returns an entity with a given `id`. There's no guarantee for validity,
    /// meaning the entity could be not alive.
    pub fn entity(&self, id: Index) -> Entity {
        self.alloc.entity(id)
    }

    /// Returns `true` if the specified entity is alive.
    #[inline]
    pub fn is_alive(&self, e: Entity) -> bool {
        self.alloc.is_alive(e)
    }

    /// Merges in the deleted entities and returns them
    pub fn maintain(&mut self) -> Vec<Entity> {
        self.alloc.merge()
    }
}

// /// An entity builder from `EntitiesRes`.  Allows building an entity with its
// /// components if you have mutable access to the component storages.
// #[must_use = "Please call .build() on this to finish building it."]
// pub struct EntityResBuilder<'a> {
//     /// The entity being built
//     pub entity: Entity,
//     /// The active borrow to `EntitiesRes`, used to delete the entity if the
//     /// builder is dropped without called `build()`.
//     pub entities: &'a EntitiesRes,
//     built: bool,
// }

// impl<'a> EntityResBuilder<'a> {
//     /// Appends a component and associates it with the entity.
//     pub fn with<T: Component>(self, c: T, storage: &mut WriteComp<T>) -> Self {
//         storage.insert(self.entity, c).unwrap();
//         self
//     }

//     /// Finishes the building and returns the entity.
//     pub fn build(mut self) -> Entity {
//         self.built = true;
//         self.entity
//     }
// }

// impl<'a> Drop for EntityResBuilder<'a> {
//     fn drop(&mut self) {
//         if !self.built {
//             self.entities.delete(self.entity).unwrap();
//         }
//     }
// }

#[derive(Default, Debug)]
struct EntityCache {
    cache: Vec<Index>,
    len: AtomicUsize,
}

impl EntityCache {
    fn pop_atomic(&self) -> Option<Index> {
        atomic_decrement(&self.len).map(|x| self.cache[x - 1])
    }

    fn pop(&mut self) -> Option<Index> {
        self.maintain();
        let x = self.cache.pop();
        *self.len.get_mut() = self.cache.len();
        x
    }

    fn push(&mut self, val: Index) {
        self.maintain();
        self.cache.push(val);
        *self.len.get_mut() = self.cache.len();
    }

    fn maintain(&mut self) {
        self.cache.truncate(*(self.len.get_mut()));
    }
}

impl Extend<Index> for EntityCache {
    fn extend<T: IntoIterator<Item = Index>>(&mut self, iter: T) {
        self.maintain();
        self.cache.extend(iter);
        *self.len.get_mut() = self.cache.len();
    }
}

/// Increments `i` atomically without wrapping on overflow.
/// Resembles a `fetch_add(1, Ordering::Relaxed)` with
/// checked overflow, returning `None` instead.
fn atomic_increment(i: &AtomicUsize) -> Option<usize> {
    use std::usize;
    let mut prev = i.load(Ordering::Relaxed);
    while prev != usize::MAX {
        match i.compare_exchange_weak(prev, prev + 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(x) => return Some(x),
            Err(next_prev) => prev = next_prev,
        }
    }
    None
}

/// Increments `i` atomically without wrapping on overflow.
/// Resembles a `fetch_sub(1, Ordering::Relaxed)` with
/// checked underflow, returning `None` instead.
fn atomic_decrement(i: &AtomicUsize) -> Option<usize> {
    let mut prev = i.load(Ordering::Relaxed);
    while prev != 0 {
        match i.compare_exchange_weak(prev, prev - 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(x) => return Some(x),
            Err(next_prev) => prev = next_prev,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonzero_optimization() {
        use std::mem::size_of;
        assert_eq!(size_of::<Option<Generation>>(), size_of::<Generation>());
        assert_eq!(size_of::<Option<Entity>>(), size_of::<Entity>());
    }

    #[test]
    fn kill_atomic_create_merge() {
        let mut allocator = EntityAllocator::default();

        let entity = allocator.allocate();
        assert_eq!(entity.id(), 0);

        allocator.kill_atomic(entity);

        assert_ne!(allocator.allocate(), entity);

        assert_eq!(allocator.killed.contains(entity.id()), true);
        assert_eq!(allocator.merge(), vec![entity]);
    }

    #[test]
    fn kill_atomic_kill_now_create_merge() {
        let mut allocator = EntityAllocator::default();

        let entity = allocator.allocate();

        allocator.kill_atomic(entity);

        assert_ne!(allocator.allocate(), entity);

        allocator.kill(&[entity]);

        allocator.allocate();

        assert_eq!(allocator.killed.contains(entity.id()), false);
        assert_eq!(allocator.merge(), vec![]);
    }

    #[test]
    fn dead_entity() {
        let mut entities = EntitiesRes::default();

        let dead = Entity::dead();

        assert!(!entities.is_alive(dead));

        let a = entities.create();
        let b = entities.create();

        assert!(!entities.is_alive(dead));

        entities.delete(a);

        assert!(!entities.is_alive(dead));

        entities.delete(a);
        entities.delete(b);
        entities.delete(dead);

        assert!(!entities.is_alive(dead));

        entities.maintain();

        assert!(!entities.is_alive(dead));
    }

    #[test]
    fn double_delete_spawn() {
        let mut entities = EntitiesRes::default();

        let a = entities.create();
        assert_eq!(a.id(), 0);
        let b = entities.create();
        assert_eq!(b.id(), 1);

        entities.delete(a);
        entities.delete(a);

        // Remains alive until maintain
        assert!(entities.is_alive(a));

        entities.maintain();

        assert!(!entities.is_alive(a));

        let c = entities.create();

        assert!(entities.is_alive(c));
    }
}
