use super::Entity;
use crate::components::{Component, ComponentSet};
use crate::world::World;

// /// An iterator for entity creation.
// /// Please note that you have to consume
// /// it because iterators are lazy.
// ///
// /// Returned from `World::create_iter`.
// pub struct CreateIter<'a>(pub(crate) EntitiesMut<'a>);

// impl<'a> Iterator for CreateIter<'a> {
//     type Item = Entity;

//     fn next(&mut self) -> Option<Entity> {
//         Some(self.0.alloc.allocate())
//     }
// }

/// A common trait for `EntityBuilder` and `LazyBuilder`, allowing either to be
/// used. Entity is definitely alive, but the components may or may not exist
/// before a call to `World::maintain`.
pub trait Builder {
    /// Appends a component and associates it with the entity.
    ///
    /// If a component was already associated with the entity, it should
    /// overwrite the previous component.
    ///
    /// # Panics
    ///
    /// Panics if the component hasn't been `register()`ed in the
    /// `World`.
    #[cfg(feature = "parallel")]
    fn with<C: Component + Send + Sync>(self, c: C) -> Self;

    /// Appends a component and associates it with the entity.
    ///
    /// If a component was already associated with the entity, it should
    /// overwrite the previous component.
    ///
    /// # Panics
    ///
    /// Panics if the component hasn't been `register()`ed in the
    /// `World`.
    #[cfg(not(feature = "parallel"))]
    fn with<C: Component>(self, c: C) -> Self;

    /// Appends all of a set of components onto the entity.
    fn spawn<C: ComponentSet>(self, c: C) -> Self;

    /// Convenience method that calls `self.with(component)` if
    /// `Some(component)` is provided
    ///
    /// # Panics
    ///
    /// Panics if the component hasn't been `register()`ed in the
    /// `World`.
    #[cfg(feature = "parallel")]
    fn maybe_with<C: Component + Send + Sync>(self, c: Option<C>) -> Self
    where
        Self: Sized,
    {
        match c {
            Some(c) => self.with(c),
            None => self,
        }
    }

    /// Convenience method that calls `self.with(component)` if
    /// `Some(component)` is provided
    ///
    /// # Panics
    ///
    /// Panics if the component hasn't been `register()`ed in the
    /// `World`.
    #[cfg(not(feature = "parallel"))]
    fn maybe_with<C: Component>(self, c: Option<C>) -> Self
    where
        Self: Sized,
    {
        match c {
            Some(c) => self.with(c),
            None => self,
        }
    }

    /// Finishes the building and returns the entity.
    fn id(&self) -> Entity;
}

/// The entity builder, allowing to
/// build an entity together with its components.
///
/// ## Examples
///
/// ```
/// use specs::{prelude::*, storage::HashMapStorage};
///
/// struct Health(f32);
///
/// impl Component for Health {
///     type Storage = HashMapStorage<Self>;
/// }
///
/// struct Pos {
///     x: f32,
///     y: f32,
/// }
///
/// impl Component for Pos {
///     type Storage = DenseVecStorage<Self>;
/// }
///
/// let mut world = World::new();
/// world.register::<Health>();
/// world.register::<Pos>();
///
/// let entity = world
///     .create_entity() // This call returns `EntityBuilder`
///     .with(Health(4.0))
///     .with(Pos { x: 1.0, y: 3.0 })
///     .build(); // Returns the `Entity`
/// ```
///
/// ### Distinguishing Mandatory Components from Optional Components
///
/// ```
/// use specs::{prelude::*, storage::HashMapStorage};
///
/// struct MandatoryHealth(f32);
///
/// impl Component for MandatoryHealth {
///     type Storage = HashMapStorage<Self>;
/// }
///
/// struct OptionalPos {
///     x: f32,
///     y: f32,
/// }
///
/// impl Component for OptionalPos {
///     type Storage = DenseVecStorage<Self>;
/// }
///
/// let mut world = World::new();
/// world.register::<MandatoryHealth>();
/// world.register::<OptionalPos>();
///
/// let mut entitybuilder = world.create_entity().with(MandatoryHealth(4.0));
///
/// // something trivial to serve as our conditional
/// let include_optional = true;
///
/// if include_optional == true {
///     entitybuilder = entitybuilder.with(OptionalPos { x: 1.0, y: 3.0 })
/// }
///
/// let entity = entitybuilder.build();
/// ```
#[must_use = "Please call .build() on this to finish building it."]
pub struct EntityBuilder<'a> {
    /// The (already created) entity for which components will be inserted.
    pub entity: Entity,
    /// A reference to the `World` for component insertions.
    pub world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut World, entity: Entity) -> Self {
        EntityBuilder { entity, world }
    }

    /// Inserts a component into the correct storage
    pub fn insert<T: Component>(&mut self, c: T) {
        // let mut storage: WriteComp<T> = SystemData::fetch(&self.world);
        // // This can't fail.  This is guaranteed by the lifetime 'a
        // // in the EntityBuilder.
        // storage.insert(self.entity, c).unwrap();
        let _ = self.world.write_component::<T>().insert(self.entity, c);
    }

    /// Inserts a component into the correct storage
    pub fn maybe_insert<T: Component>(&mut self, c: Option<T>) {
        if let Some(value) = c {
            // let mut storage: WriteComp<T> = SystemData::fetch(&self.world);
            // // This can't fail.  This is guaranteed by the lifetime 'a
            // // in the EntityBuilder.
            // storage.insert(self.entity, value).unwrap();
            let _ = self.world.write_component::<T>().insert(self.entity, value);
        }
    }
}

impl<'a> Builder for EntityBuilder<'a> {
    /// Inserts a component for this entity.
    ///
    /// If a component was already associated with the entity, it will
    /// overwrite the previous component.
    #[inline]
    fn with<T: Component>(mut self, c: T) -> Self {
        self.insert(c);
        self
    }

    fn spawn<C: ComponentSet>(self, c: C) -> Self {
        c.insert(self.world, self.entity);
        self
    }

    /// Finishes the building and returns the entity. As opposed to
    /// `LazyBuilder`, the components are available immediately.
    #[inline]
    fn id(&self) -> Entity {
        self.entity
    }
}

// impl<'a> Drop for EntityBuilder<'a> {
//     fn drop(&mut self) {
//         if !self.built {
//             let entity = self.entity;
//             self.world.entities().delete(entity);
//         }
//     }
// }
