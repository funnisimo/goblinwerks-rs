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

impl<'e, 'b> Ecs
where
    'e: 'b,
{
    pub fn new() -> Self {
        let mut res = Resources::default();
        res.insert(Levels::new());

        Ecs { resources: res }
    }

    pub fn insert_global<R: Resource>(&mut self, res: R) {
        self.resources.insert(res);
    }

    pub fn get_global<R: Resource>(&'e self) -> Option<Global<'b, R>> {
        match self.resources.get::<R>() {
            None => None,
            Some(b) => Some(Global::new(b)),
        }
    }

    pub fn get_global_mut<R: Resource>(&'e self) -> Option<GlobalMut<'b, R>> {
        match self.resources.get_mut::<R>() {
            None => None,
            Some(b) => Some(GlobalMut::new(b)),
        }
    }

    pub fn levels(&'e self) -> LevelsRef<'b> {
        self.get_global::<Levels>().unwrap()
    }

    pub fn levels_mut(&'e self) -> LevelsMut<'b> {
        self.get_global_mut::<Levels>().unwrap()
    }

    pub fn level(&'e self) -> LevelRef<'b> {
        let levels = self.levels();
        let (levels, parent) = levels.destructure();
        let borrow = levels.current();
        LevelRef::new(parent, borrow)
    }

    pub fn level_mut(&'e self) -> LevelMut<'b> {
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

    pub fn get_unique<U: Resource>(&'e self) -> Option<Unique<'b, U>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_unique::<U>()?;
        Some(Unique::new(root, parent, borrow))
    }

    pub fn get_unique_mut<U: Resource>(&'e self) -> Option<UniqueMut<'b, U>> {
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

    pub fn get_component<C: Component>(&'e self) -> Option<Comp<'b, C>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_component::<C>()?;
        Some(Comp::new(root, parent, borrow))
    }

    pub fn get_component_mut<C: Component>(&'e self) -> Option<CompMut<'b, C>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_component_mut::<C>()?;
        Some(CompMut::new(root, parent, borrow))
    }

    pub fn fetch<B>(&'e self) -> <B as BorrowRef<'e>>::Output
    where
        B: BorrowRef<'e> + ReadOnly,
    {
        B::borrow(&self)
    }

    pub fn fetch_mut<B>(&'e self) -> <B as BorrowMut<'e>>::Output
    where
        B: BorrowMut<'e>,
    {
        B::borrow_mut(&self)
    }
}
