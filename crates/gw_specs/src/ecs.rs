use crate::globals::{GlobalFetch, GlobalFetchMut, Globals};
use crate::shred::{Fetch, FetchMut, Resource, World};

pub struct Ecs {
    pub(crate) worlds: Vec<World>,
    pub(crate) current: usize,
}

impl Ecs {
    pub fn new() -> Self {
        let globals = Globals::new();
        let mut world = World::empty();
        world.insert(globals);

        Ecs {
            worlds: vec![world],
            current: 0,
        }
    }

    pub fn push_world(&mut self, world: World) -> usize {
        let mut world = world;
        world.insert(Globals::empty());
        self.worlds.push(world);
        self.worlds.len() - 1
    }

    /// Returns the current active world
    pub fn current_world(&self) -> &World {
        &self.worlds[self.current]
    }

    pub fn set_current_index(&mut self, index: usize) {
        // TODO - Safety: is index in bounds?
        let globals = self.current_world().fetch_mut::<Globals>().take();
        assert!(globals.is_some());
        self.current = index;
        self.current_world().fetch_mut::<Globals>().replace(globals);
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
        self.current_world().fetch::<Globals>().has_value::<G>()
    }

    /// Inserts a global
    pub fn insert_global<G: Resource>(&self, global: G) {
        self.current_world().fetch_mut::<Globals>().insert(global)
    }

    /// Removes a global
    pub fn remove_global<G: Resource>(&self) -> Option<G> {
        self.current_world().fetch_mut::<Globals>().remove::<G>()
    }

    pub fn fetch_global<G: Resource>(&self) -> GlobalFetch<G> {
        self.try_fetch_global::<G>().unwrap()
    }

    pub fn try_fetch_global<G: Resource>(&self) -> Option<GlobalFetch<G>> {
        let (globals, borrow) = self.current_world().fetch::<Globals>().destructure();
        match globals.try_fetch::<G>() {
            None => None,
            Some(fetch) => Some(GlobalFetch::new(borrow, fetch)),
        }
    }

    pub fn fetch_global_mut<G: Resource>(&self) -> GlobalFetchMut<G> {
        self.try_fetch_global_mut().unwrap()
    }

    pub fn try_fetch_global_mut<G: Resource>(&self) -> Option<GlobalFetchMut<G>> {
        let (globals, borrow) = self.current_world().fetch::<Globals>().destructure();
        match globals.try_fetch_mut::<G>() {
            None => None,
            Some(fetch) => Some(GlobalFetchMut::new(borrow, fetch)),
        }
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
