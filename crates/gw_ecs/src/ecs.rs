use crate::borrow::{
    BorrowMut, BorrowRef, Comp, CompMut, Global, GlobalMut, LevelMut, LevelRef, LevelsMut,
    LevelsRef, ReadOnly, Unique, UniqueMut,
};
use crate::component::{Component, ComponentSet};
use crate::levels::Levels;
use crate::resource::{Resource, Resources};
use crate::Entity;

pub struct Ecs {
    pub(crate) resources: Resources,
}

impl Ecs {
    pub fn new() -> Self {
        let mut res = Resources::default();
        res.insert(Levels::new());

        Ecs { resources: res }
    }

    pub fn insert_global<R: Resource>(&mut self, res: R) {
        self.resources.insert(res);
    }

    pub fn get_global<R: Resource>(&self) -> Option<Global<R>> {
        match self.resources.get::<R>() {
            None => None,
            Some(b) => Some(Global::new(b)),
        }
    }

    pub fn get_global_mut<R: Resource>(&self) -> Option<GlobalMut<R>> {
        match self.resources.get_mut::<R>() {
            None => None,
            Some(b) => Some(GlobalMut::new(b)),
        }
    }

    pub fn levels(&self) -> LevelsRef {
        self.get_global::<Levels>().unwrap()
    }

    pub fn levels_mut(&self) -> LevelsMut {
        self.get_global_mut::<Levels>().unwrap()
    }

    pub fn level(&self) -> LevelRef {
        let levels = self.levels();
        let (levels, parent) = levels.destructure();
        let borrow = levels.current();
        LevelRef::new(parent, borrow)
    }

    pub fn level_mut(&self) -> LevelMut {
        let levels = self.levels();
        let (levels, parent) = levels.destructure();
        let borrow = levels.current_mut();
        LevelMut::new(parent, borrow)
    }

    // spawn
    pub fn spawn<'a, S: ComponentSet<'a>>(&mut self, comps: S) -> Entity {
        let mut level = self.level_mut();
        level.spawn(comps)
    }

    pub fn get_unique<U: Resource>(&self) -> Option<Unique<U>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_unique::<U>()?;
        Some(Unique::new(root, parent, borrow))
    }

    pub fn get_unique_mut<U: Resource>(&self) -> Option<UniqueMut<U>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_unique_mut::<U>()?;
        Some(UniqueMut::new(root, parent, borrow))
    }

    // // entities
    // // entities_mut -- ???
    pub fn register_component<C: Component>(&mut self) {
        // Store in registry
        // Add to every level
        let mut levels = self.levels_mut();
        levels.register_component::<C>();
    }

    pub fn get_component<C: Component>(&self) -> Option<Comp<C>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_component::<C>()?;
        Some(Comp::new(root, parent, borrow))
    }

    pub fn get_component_mut<C: Component>(&self) -> Option<CompMut<C>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_component_mut::<C>()?;
        Some(CompMut::new(root, parent, borrow))
    }

    pub fn fetch<'a, B>(&'a self) -> B
    where
        B: BorrowRef<'a> + ReadOnly,
    {
        B::borrow(&self)
    }

    pub fn fetch_mut<'a, B>(&'a self) -> B
    where
        B: BorrowMut<'a>,
    {
        B::borrow_mut(&self)
    }
}
