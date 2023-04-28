use crate::globals::{GlobalFetch, GlobalFetchMut, Globals};
use crate::shred::{Fetch, FetchMut, Resource};
use crate::World;

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
        self.current_world_mut().insert_global(global)
    }

    /// Removes a global
    pub fn remove_global<G: Resource>(&mut self) -> Option<G> {
        self.current_world_mut().remove_global::<G>()
    }

    pub fn fetch_global<G: Resource>(&self) -> GlobalFetch<G> {
        self.current_world().fetch_global::<G>()
    }

    pub fn try_fetch_global<G: Resource>(&self) -> Option<GlobalFetch<G>> {
        self.current_world().try_fetch_global::<G>()
    }

    pub fn fetch_global_mut<G: Resource>(&self) -> GlobalFetchMut<G> {
        self.current_world().fetch_global_mut::<G>()
    }

    pub fn try_fetch_global_mut<G: Resource>(&self) -> Option<GlobalFetchMut<G>> {
        self.current_world().try_fetch_global_mut::<G>()
    }

    // UNIQUES

    pub fn has_unique<G: Resource>(&self) -> bool {
        self.current_world().has_value::<G>()
    }

    /// Inserts a unique
    pub fn insert_unique<G: Resource>(&mut self, unique: G) {
        self.current_world_mut().insert(unique)
    }

    /// Removes a unique
    pub fn remove_unique<G: Resource>(&mut self) -> Option<G> {
        self.current_world_mut().remove::<G>()
    }

    pub fn fetch_unique<G: Resource>(&self) -> Fetch<G> {
        self.try_fetch_unique::<G>().unwrap()
    }

    pub fn try_fetch_unique<G: Resource>(&self) -> Option<Fetch<G>> {
        self.current_world().try_fetch::<G>()
    }

    pub fn fetch_unique_mut<G: Resource>(&self) -> FetchMut<G> {
        self.try_fetch_unique_mut().unwrap()
    }

    pub fn try_fetch_unique_mut<G: Resource>(&self) -> Option<FetchMut<G>> {
        self.current_world().try_fetch_mut::<G>()
    }
}

impl Default for Ecs {
    fn default() -> Self {
        Ecs::new(World::empty())
    }
}
