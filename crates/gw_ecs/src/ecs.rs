use crate::globals::Globals;
use crate::shred::{PanicIfMissing, Resource};
use crate::WriteGlobal;
use crate::{ReadGlobal, ReadRes, World, WriteRes};

pub struct Ecs {
    pub(crate) worlds: Vec<World>,
    pub(crate) current: usize,
    globals: Globals,
}

impl Ecs {
    pub fn new(world: World) -> Self {
        let globals = Globals::default();
        let mut world = world;
        world.set_globals(globals.clone());

        Ecs {
            worlds: vec![world],
            current: 0,
            globals,
        }
    }

    pub fn push_world(&mut self, world: World) -> usize {
        let mut world = world;
        world.set_globals(self.globals.clone());
        self.worlds.push(world);
        self.worlds.len() - 1
    }

    /// Returns the current active world
    pub fn current_world(&self) -> &World {
        &self.worlds[self.current]
    }

    pub fn set_current_index(&mut self, index: usize) {
        // TODO - Safety: is index in bounds?
        self.current = index;
    }

    pub fn set_current_with<F>(&mut self, func: F) -> Option<usize>
    where
        F: Fn(&World) -> bool,
    {
        match self.worlds.iter().position(func) {
            None => None,
            Some(index) => {
                self.set_current_index(index);
                Some(index)
            }
        }
    }

    /// Returns a mutable reference to the currently active world
    pub fn current_world_mut(&mut self) -> &mut World {
        &mut self.worlds[self.current]
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
}

impl Default for Ecs {
    fn default() -> Self {
        Ecs::new(World::empty())
    }
}
