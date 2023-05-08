use crate::{
    specs::{prelude::*, world::EntitiesRes},
    Ecs,
};
use crossbeam_queue::SegQueue;
use std::sync::Arc;

pub(crate) struct Queue<T>(SegQueue<T>);

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self(SegQueue::default())
    }
}

#[cfg(feature = "parallel")]
pub trait LazyUpdateInternal: Send + Sync {
    fn update(self: Box<Self>, world: &mut World);
}

#[cfg(not(feature = "parallel"))]
pub trait LazyUpdateInternal {
    fn update(self: Box<Self>, world: &mut World);
}

/// An updte that works on the whole Ecs
#[cfg(feature = "parallel")]
pub trait LazyUpdateEcs: Send + Sync {
    /// Do the update
    fn update(self: Box<Self>, ecs: &mut Ecs);
}

#[cfg(not(feature = "parallel"))]
pub trait LazyUpdateEcs {
    fn update(self: Box<Self>, ecs: &mut Ecs);
}

/// Generates two versions of functions within the macro call:
///
/// * One with `Send + Sync` bounds when the `"parallel"` feature is enabled.
/// * One without `Send + Sync` bounds when the `"parallel"` feature is
///   disabled.
///
/// TODO: When trait aliases land on stable we can remove this macro.
/// See <https://github.com/rust-lang/rust/issues/41517>.
///
/// ```rust,ignore
/// #![cfg(feature = "parallel")]
/// trait ComponentBound = Component + Send + Sync;
/// #![cfg(not(feature = "parallel"))]
/// trait ComponentBound = Component;
/// ```
///
/// Alternative solutions are listed in:
/// <https://github.com/amethyst/specs/pull/674#issuecomment-585013726>
macro_rules! parallel_feature {
    (
        $(
            $(#[$attrs:meta])* $($words:ident)+<$($ty_params:ident),+> $args:tt $(-> $return_ty:ty)?
            where
                $($ty_param:ident:
                    $bound_first:ident $(< $ty_type:ident = $ty_bound:tt >)? $(($($fn_bound:tt)*))?
                    $(+ $bound:tt $($bound_2:ident)?)*,)+
            $body:block
        )+
    ) =>
    {
        $(
            $(#[$attrs])*
            #[cfg(feature = "parallel")]
            $($words)+<$($ty_params),+> $args $(-> $return_ty)?
            where
                $($ty_param:
                    Send + Sync +
                    $bound_first $(< $ty_type = $ty_bound >)? $(($($fn_bound)*))?
                    $(+ $bound $($bound_2)?)*,)+
            $body

            $(#[$attrs])*
            #[cfg(not(feature = "parallel"))]
            $($words)+<$($ty_params),+> $args $(-> $return_ty)?
            where
                $($ty_param:
                    $bound_first $(< $ty_type = $ty_bound >)? $(($($fn_bound)*))?
                    $(+ $bound $($bound_2)?)*,)+
            $body
        )+
    };
}

/// Like `EntityBuilder`, but inserts the component
/// lazily, meaning on `maintain`.
/// If you need those components to exist immediately,
/// you have to insert them into the storages yourself.
#[must_use = "Please call .build() on this to finish building it."]
pub struct LazyBuilder<'a> {
    /// The entity that we're inserting components for.
    pub entity: Entity,
    /// The lazy update reference.
    pub lazy: &'a LazyUpdate,
}

impl<'a> Builder for LazyBuilder<'a> {
    parallel_feature! {
        /// Inserts a component using [LazyUpdate].
        ///
        /// If a component was already associated with the entity, it will
        /// overwrite the previous component.
        fn with<C>(self, component: C) -> Self
        where
            C: Component,
        {
            let entity = self.entity;
            self.lazy.exec(move |world| {
                let mut storage: WriteComp<C> = SystemData::fetch(world);
                if storage.insert(entity, component).is_err() {
                    log::warn!(
                        "Lazy insert of component failed because {:?} was dead.",
                        entity
                    );
                }
            });

            self
        }
    }

    /// Finishes the building and returns the built entity.
    /// Please note that no component is associated to this
    /// entity until you call [`World::maintain`].
    fn build(self) -> Entity {
        self.entity
    }
}

#[cfg(feature = "parallel")]
impl<F> LazyUpdateInternal for F
where
    F: FnOnce(&mut World) + Send + Sync + 'static,
{
    fn update(self: Box<Self>, world: &mut World) {
        self(world);
    }
}

#[cfg(not(feature = "parallel"))]
impl<F> LazyUpdateInternal for F
where
    F: FnOnce(&mut World) + 'static,
{
    fn update(self: Box<Self>, world: &mut World) {
        self(world);
    }
}

#[cfg(feature = "parallel")]
impl<F> LazyUpdateEcs for F
where
    F: FnOnce(&mut Ecs) + Send + Sync + 'static,
{
    fn update(self: Box<Self>, ecs: &mut Ecs) {
        self(ecs);
    }
}

#[cfg(not(feature = "parallel"))]
impl<F> LazyUpdateEcs for F
where
    F: FnOnce(&mut Ecs) + 'static,
{
    fn update(self: Box<Self>, ecs: &mut Ecs) {
        self(ecs);
    }
}

/// Lazy updates can be used for world updates
/// that need to borrow a lot of resources
/// and as such should better be done at the end.
/// They work lazily in the sense that they are
/// dispatched when calling `world.maintain()`.
///
/// Lazy updates are dispatched in the order that they
/// are requested. Multiple updates sent from one system
/// may be overridden by updates sent from other systems.
///
/// Please note that the provided methods take `&self`
/// so there's no need to get `LazyUpdate` mutably.
/// This resource is added to the world by default.
#[derive(Default)]
pub struct LazyUpdate {
    queue: Arc<Queue<Box<dyn LazyUpdateInternal>>>,
    pub(crate) queue_ecs: Arc<Queue<Box<dyn LazyUpdateEcs>>>,
}

impl LazyUpdate {
    parallel_feature! {

        /// Lazily inserts a global.
        ///
        /// ## Examples
        ///
        /// ```
        /// # use specs::prelude::*;
        /// #
        /// struct Pos(f32, f32);
        ///
        /// struct InsertPos;
        ///
        /// impl<'a> System<'a> for InsertPos {
        ///     type SystemData = (ReadRes<'a, LazyUpdate>,);
        ///
        ///     fn run(&mut self, (lazy,): Self::SystemData) {
        ///         lazy.insert_global(Pos(1.0, 1.0));
        ///     }
        /// }
        /// ```
        pub fn insert_global<G>(&self, global: G)
        where
            G: Resource,
        {
            self.exec(move |world| {
                world.insert_global(global);
            });
        }

        /// Lazily removes a global.
        ///
        /// ## Examples
        ///
        /// ```
        /// # use specs::prelude::*;
        /// #
        /// struct Pos(f32, f32);
        ///
        /// struct InsertPos;
        ///
        /// impl<'a> System<'a> for InsertPos {
        ///     type SystemData = (ReadRes<'a, LazyUpdate>,);
        ///
        ///     fn run(&mut self, (lazy,): Self::SystemData) {
        ///         lazy.remove_global::<Pos>();
        ///     }
        /// }
        /// ```
        pub fn remove_global<G>(&self)
        where
            G: Resource,
        {
            self.exec(move |world| {
                world.remove_global::<G>();
            });
        }


        /// Lazily inserts a resource.
        ///
        /// ## Examples
        ///
        /// ```
        /// # use specs::prelude::*;
        /// #
        /// struct Pos(f32, f32);
        ///
        /// struct InsertPos;
        ///
        /// impl<'a> System<'a> for InsertPos {
        ///     type SystemData = (ReadRes<'a, LazyUpdate>,);
        ///
        ///     fn run(&mut self, (lazy,): Self::SystemData) {
        ///         lazy.insert_resource(Pos(1.0, 1.0));
        ///     }
        /// }
        /// ```
        pub fn insert_resource<G>(&self, global: G)
        where
            G: Resource,
        {
            self.exec(move |world| {
                world.insert_resource(global);
            });
        }

        /// Lazily removes a resource.
        ///
        /// ## Examples
        ///
        /// ```
        /// # use specs::prelude::*;
        /// #
        /// struct Pos(f32, f32);
        ///
        /// struct InsertPos;
        ///
        /// impl<'a> System<'a> for InsertPos {
        ///     type SystemData = (ReadRes<'a, LazyUpdate>,);
        ///
        ///     fn run(&mut self, (lazy,): Self::SystemData) {
        ///         lazy.remove_global::<Pos>();
        ///     }
        /// }
        /// ```
        pub fn remove_resource<G>(&self)
        where
            G: Resource,
        {
            self.exec(move |world| {
                world.remove_resource::<G>();
            });
        }

        /// Lazily inserts a component for an entity.
        ///
        /// ## Examples
        ///
        /// ```
        /// # use specs::prelude::*;
        /// #
        /// struct Pos(f32, f32);
        ///
        /// impl Component for Pos {
        ///     type Storage = VecStorage<Self>;
        /// }
        ///
        /// struct InsertPos;
        ///
        /// impl<'a> System<'a> for InsertPos {
        ///     type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);
        ///
        ///     fn run(&mut self, (ent, lazy): Self::SystemData) {
        ///         let a = ent.create();
        ///         lazy.insert(a, Pos(1.0, 1.0));
        ///     }
        /// }
        /// ```
        pub fn insert_component<C>(&self, e: Entity, c: C)
        where
            C: Component,
        {
            self.exec(move |world| {
                let mut storage: WriteComp<C> = SystemData::fetch(world);
                if storage.insert(e, c).is_err() {
                    log::warn!("Lazy insert of component failed because {:?} was dead.", e);
                }
            });
        }

        /// Lazily inserts components for entities.
        ///
        /// ## Examples
        ///
        /// ```
        /// # use specs::prelude::*;
        /// #
        /// struct Pos(f32, f32);
        ///
        /// impl Component for Pos {
        ///     type Storage = VecStorage<Self>;
        /// }
        ///
        /// struct InsertPos;
        ///
        /// impl<'a> System<'a> for InsertPos {
        ///     type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);
        ///
        ///     fn run(&mut self, (ent, lazy): Self::SystemData) {
        ///         let a = ent.create();
        ///         let b = ent.create();
        ///
        ///         lazy.insert_all(vec![(a, Pos(3.0, 1.0)), (b, Pos(0.0, 4.0))]);
        ///     }
        /// }
        /// ```
        pub fn insert_all_components<C, I>(&self, iter: I)
        where
            C: Component,
            I: IntoIterator<Item = (Entity, C)> + 'static,
        {
            self.exec(move |world| {
                let mut storage: WriteComp<C> = SystemData::fetch(world);
                for (e, c) in iter {
                    if storage.insert(e, c).is_err() {
                        log::warn!("Lazy insert of component failed because {:?} was dead.", e);
                    }
                }
            });
        }

        /// Lazily removes a component.
        ///
        /// ## Examples
        ///
        /// ```
        /// # use specs::prelude::*;
        /// #
        /// struct Pos;
        ///
        /// impl Component for Pos {
        ///     type Storage = VecStorage<Self>;
        /// }
        ///
        /// struct RemovePos;
        ///
        /// impl<'a> System<'a> for RemovePos {
        ///     type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);
        ///
        ///     fn run(&mut self, (ent, lazy): Self::SystemData) {
        ///         for entity in ent.join() {
        ///             lazy.remove::<Pos>(entity);
        ///         }
        ///     }
        /// }
        /// ```
        pub fn remove_component<C>(&self, e: Entity)
        where
            C: Component,
        {
            self.exec(move |world| {
                let mut storage: WriteComp<C> = SystemData::fetch(world);
                storage.remove(e);
            });
        }

        /// Lazily executes a closure with world access.
        ///
        /// ## Examples
        ///
        /// ```
        /// # use specs::prelude::*;
        /// #
        /// struct Pos;
        ///
        /// impl Component for Pos {
        ///     type Storage = VecStorage<Self>;
        /// }
        ///
        /// struct Execution;
        ///
        /// impl<'a> System<'a> for Execution {
        ///     type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);
        ///
        ///     fn run(&mut self, (ent, lazy): Self::SystemData) {
        ///         for entity in ent.join() {
        ///             lazy.exec(move |world| {
        ///                 if world.is_alive(entity) {
        ///                     println!("Entity {:?} is alive.", entity);
        ///                 }
        ///             });
        ///         }
        ///     }
        /// }
        /// ```
        pub fn exec<F>(&self, f: F)
        where
            F: FnOnce(&mut World) + 'static,
        {
            self.queue
                .0
                .push(Box::new(f));
        }

        /// Lazily executes a closure with mutable Ecs access.
        ///
        /// This can be used to change the current world or move an entity.
        ///
        /// ## Examples
        ///
        /// ```
        /// # use gw_ecs::*;
        /// #
        ///
        /// struct Sys;
        ///
        /// impl<'a> System<'a> for Sys {
        ///     type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);
        ///
        ///     fn run(&mut self, (ent, lazy): Self::SystemData) {
        ///         for entity in ent.join() {
        ///             lazy.exec_ecs(move |ecs| {
        ///                 // complete extermination!
        ///                 ecs.set_current_world(a!(MARS))
        ///             });
        ///         }
        ///     }
        /// }
        /// ```
        pub fn exec_ecs<F>(&self, f: F)
        where
            F: FnOnce(&mut Ecs) + 'static,
        {
            self.queue_ecs.0.push(Box::new(f));
        }
    }

    /// Creates a new `LazyBuilder` which inserts components
    /// using `LazyUpdate`. This means that the components won't
    /// be available immediately, but only after a `maintain`
    /// on `World` is performed.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use specs::prelude::*;
    /// # let mut world = World::new();
    /// struct Pos(f32, f32);
    ///
    /// impl Component for Pos {
    ///     type Storage = VecStorage<Self>;
    /// }
    ///
    /// # let lazy = world.read_resource::<LazyUpdate>();
    /// # let entities = world.entities();
    /// let my_entity = lazy.create_entity(&entities).with(Pos(1.0, 3.0)).build();
    /// ```
    pub fn create_entity(&self, ent: &EntitiesRes) -> LazyBuilder {
        let entity = ent.create();

        LazyBuilder { entity, lazy: self }
    }

    /// Deletes an entity
    pub fn delete_entity(&self, e: Entity) {
        self.exec(move |world| {
            let _ = world.entities().delete(e);
        });
    }

    pub(crate) fn clone(&self) -> Self {
        Self {
            queue: self.queue.clone(),
            queue_ecs: self.queue_ecs.clone(),
        }
    }

    pub(crate) fn maintain(&self, world: &mut World) {
        while let Some(l) = self.queue.0.pop() {
            l.update(world);
        }
    }

    pub(crate) fn take_ecs_funcs(&self) -> Vec<Box<dyn LazyUpdateEcs>> {
        let mut result = Vec::<Box<dyn LazyUpdateEcs>>::new();
        while let Some(l) = self.queue_ecs.0.pop() {
            result.push(l);
        }
        result
    }
}

impl Drop for LazyUpdate {
    fn drop(&mut self) {
        // TODO: remove as soon as leak is fixed in crossbeam
        while self.queue.0.pop().is_some() {}
    }
}
