use atomize::Atom;

use crate::globals::Globals;
use crate::shred::{PanicIfMissing, Resource};
use crate::{
    Component, Entity, EntityBuilder, ReadComp, ReadGlobal, ReadRes, World, WriteComp, WriteRes,
};
use crate::{SystemData, WriteGlobal};

pub struct Ecs {
    pub(crate) worlds: Vec<World>,
    pub(crate) current: usize,
    globals: Globals,
    registry: Vec<Box<dyn Fn(&mut World) -> ()>>,
}

impl Ecs {
    pub fn empty() -> Self {
        Ecs {
            worlds: Vec::new(),
            current: 0,
            globals: Globals::new(),
            registry: Vec::new(),
        }
    }

    pub fn new(world: World) -> Self {
        let globals = Globals::new();
        let mut world = world;
        world.set_globals(globals.clone());

        Ecs {
            worlds: vec![world],
            current: 0,
            globals,
            registry: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.worlds.is_empty()
    }

    pub fn insert_world(&mut self, world: World) {
        let mut world = world;
        world.set_globals(self.globals.clone());
        self.worlds.push(world);
    }

    pub fn create_world<I: Into<Atom>>(&mut self, id: I) -> &mut World {
        let mut world = World::new(id, self.globals.clone());
        for r in self.registry.iter() {
            (r)(&mut world);
        }
        self.worlds.push(world);
        self.worlds.last_mut().unwrap()
    }

    /// Returns the current active world
    pub fn current_world(&self) -> &World {
        &self.worlds[self.current]
    }

    // TODO - Error Type
    pub fn set_current_world<I: Into<Atom>>(&mut self, id: I) -> Result<(), ()> {
        // TODO - Report error?!
        let id: Atom = id.into();
        match self.worlds.iter().position(|w| w.id() == id) {
            None => Err(()),
            Some(index) => {
                self.current = index;
                Ok(())
            }
        }
    }

    pub fn set_current_with<F>(&mut self, func: F) -> Result<Atom, ()>
    where
        F: Fn(&World) -> bool,
    {
        match self.worlds.iter().position(func) {
            None => {
                // TODO - Report Error?!?!
                Err(())
            }
            Some(index) => {
                self.current = index;
                Ok(self.current_world().id())
            }
        }
    }

    /// Returns a mutable reference to the currently active world
    pub fn current_world_mut(&mut self) -> &mut World {
        &mut self.worlds[self.current]
    }

    pub fn fetch<'a, D: SystemData<'a>>(&'a self) -> D {
        D::fetch(self.current_world())
    }

    pub fn get_world<I: Into<Atom>>(&self, id: I) -> Option<&World> {
        let id: Atom = id.into();
        self.worlds.iter().find(|w| w.id() == id)
    }

    pub fn iter_worlds(&self) -> impl Iterator<Item = &World> {
        self.worlds.iter()
    }

    pub fn iter_worlds_mut(&mut self) -> impl Iterator<Item = &mut World> {
        self.worlds.iter_mut()
    }

    pub fn maintain(&mut self) {
        for world in self.worlds.iter_mut() {
            world.maintain();
        }
    }

    // GLOBALS

    pub fn has_global<G: Resource>(&self) -> bool {
        self.current_world().has_global::<G>()
    }

    /// Inserts a global
    pub fn insert_global<G: Resource>(&mut self, global: G) {
        self.globals.insert(global)
    }

    /// Removes a global
    pub fn remove_global<G: Resource>(&mut self) -> Option<G> {
        self.globals.remove::<G>()
    }

    pub fn read_global<G: Resource>(&self) -> ReadGlobal<G, PanicIfMissing> {
        self.current_world().read_global::<G>()
    }

    pub fn try_read_global<G: Resource>(&self) -> Option<ReadGlobal<G, ()>> {
        self.current_world().try_read_global::<G>()
    }

    pub fn write_global<G: Resource>(&self) -> WriteGlobal<G, PanicIfMissing> {
        self.current_world().write_global::<G>()
    }

    pub fn try_write_global<G: Resource>(&self) -> Option<WriteGlobal<G, ()>> {
        self.current_world().try_write_global::<G>()
    }

    // RESOURCES

    pub fn has_unique<G: Resource>(&self) -> bool {
        self.current_world().has_resource::<G>()
    }

    /// Inserts a unique
    pub fn insert_unique<G: Resource>(&mut self, unique: G) {
        self.current_world_mut().insert_resource(unique)
    }

    /// Removes a unique
    pub fn remove_unique<G: Resource>(&mut self) -> Option<G> {
        self.current_world_mut().remove_resource::<G>()
    }

    pub fn read_resource<G: Resource>(&self) -> ReadRes<G, PanicIfMissing> {
        self.current_world().read_resource::<G>()
    }

    pub fn try_read_resource<G: Resource>(&self) -> Option<ReadRes<G, ()>> {
        self.current_world().try_read_resource::<G>()
    }

    pub fn write_resource<G: Resource>(&self) -> WriteRes<G, PanicIfMissing> {
        self.current_world().write_resource::<G>()
    }

    pub fn try_write_resource<G: Resource>(&self) -> Option<WriteRes<G, ()>> {
        self.current_world().try_write_resource::<G>()
    }

    // COMPONENTS

    pub fn create_entity(&mut self) -> EntityBuilder {
        self.current_world_mut().create_entity()
    }

    pub fn move_entity<I: Into<Atom>, J: Into<Atom>>(
        &mut self,
        entity: Entity,
        source: I,
        dest: J,
    ) -> Entity {
        let source_id: Atom = source.into();
        let source_index = match self.worlds.iter().position(|w| w.id() == source_id) {
            None => panic!("Failed to find source world - {}", source_id),
            Some(index) => index,
        };
        let dest_id: Atom = dest.into();
        let dest_index = match self.worlds.iter().position(|w| w.id() == dest_id) {
            None => panic!("Failed to find destination world - {}", dest_id),
            Some(index) => index,
        };

        if source_index == dest_index {
            return entity;
        }

        let (source_world, dest_world) = if source_index < dest_index {
            let (left, right) = self.worlds.split_at_mut(dest_index);
            let (_, late) = left.split_at_mut(source_index);
            (&mut late[0], &mut right[0])
        } else {
            let (left, right) = self.worlds.split_at_mut(source_index);
            let (_, late) = left.split_at_mut(dest_index);
            (&mut right[0], &mut late[0])
        };

        source_world.move_entity_to(entity, dest_world)
    }

    pub fn register<T: Component>(&mut self)
    where
        T::Storage: Default,
    {
        self.registry.push(Box::new(|w| w.register::<T>()));
    }

    pub fn register_with_storage<F, T>(&mut self, storage: F)
    where
        F: Fn() -> T::Storage + 'static,
        T: Component,
    {
        self.registry
            .push(Box::new(move |w| w.register_with_storage::<T>(storage())));
    }

    pub fn read_component<C: Component>(&self) -> ReadComp<C> {
        self.current_world().read_component::<C>()
    }

    pub fn write_component<C: Component>(&self) -> WriteComp<C> {
        self.current_world().write_component::<C>()
    }
}

impl Default for Ecs {
    fn default() -> Self {
        Ecs::new(World::empty("DEFAULT"))
    }
}
