use crate::globals::{GlobalFetch, GlobalFetchMut, Globals};
use crate::shred::{Resource, World};

pub struct Ecs {
    globals: Globals,
    pub(crate) worlds: Vec<World>,
}

impl Ecs {
    pub fn new() -> Self {
        let globals = Globals::new();
        let mut world = World::empty();
        world.insert(globals.clone());

        Ecs {
            worlds: vec![world],
            globals,
        }
    }

    pub fn insert_world(&mut self, world: World) {
        let mut world = world;
        world.insert(self.globals.clone());

        self.worlds.push(world);
    }

    /// Returns the current active world
    pub fn current_world(&self) -> &World {
        &self.worlds[0]
    }

    /// Returns a mutable reference to the currently active world
    pub fn current_world_mut(&mut self) -> &mut World {
        &mut self.worlds[0]
    }

    pub fn has_global<G: Resource>(&self) -> bool {
        self.globals.has_value::<G>()
    }

    /// Inserts a global
    pub fn insert_global<G: Resource>(&mut self, global: G) {
        self.globals.insert(global)
    }

    /// Removes a global
    pub fn remove_global<G: Resource>(&mut self) -> Option<G> {
        self.globals.remove()
    }

    pub fn get_global<G: Resource>(&self) -> GlobalFetch<G> {
        self.globals.fetch()
    }

    pub fn try_get_global<G: Resource>(&self) -> Option<GlobalFetch<G>> {
        self.globals.try_fetch()
    }

    pub fn get_global_mut<G: Resource>(&self) -> GlobalFetchMut<G> {
        self.globals.fetch_mut()
    }

    pub fn try_get_global_mut<G: Resource>(&self) -> Option<GlobalFetchMut<G>> {
        self.globals.try_fetch_mut()
    }
}
