use crate::component::{Component, ComponentSet};
use crate::levels::Levels;
use crate::refcell::{
    AtomicRef, AtomicRef2, AtomicRef3, AtomicRefMut, AtomicRefMut2, AtomicRefMut3,
};
use crate::resource::{Resource, ResourceSet, Resources};
use crate::storage::DenseStorage;
use crate::{Entity, Level, ReadOnly, Unique};

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

    pub fn get_global<R: Resource>(&self) -> Option<AtomicRef<R>> {
        self.resources.get::<R>()
    }

    pub fn get_global_mut<R: Resource>(&self) -> Option<AtomicRefMut<R>> {
        self.resources.get_mut::<R>()
    }

    pub fn fetch<S>(&self) -> <S as ResourceSet<'_>>::Result
    where
        for<'a> S: ResourceSet<'a> + ReadOnly,
    {
        S::fetch_from(&self.resources)
    }

    pub fn fetch_mut<S>(&mut self) -> <S as ResourceSet<'_>>::Result
    where
        for<'a> S: ResourceSet<'a>,
    {
        S::fetch_mut_from(&mut self.resources)
    }

    pub fn levels(&self) -> AtomicRef<Levels> {
        self.get_global::<Levels>().unwrap()
    }

    pub fn levels_mut(&mut self) -> AtomicRefMut<Levels> {
        self.get_global_mut::<Levels>().unwrap()
    }

    pub fn level(&self) -> AtomicRef2<Level> {
        let levels = self.levels();
        let (levels, parent) = levels.destructure();
        let borrow = levels.current();
        AtomicRef2::new(parent, borrow)
    }

    pub fn level_mut(&mut self) -> AtomicRefMut2<Level> {
        let levels = self.levels();
        let (levels, parent) = levels.destructure();
        let borrow = levels.current_mut();
        AtomicRefMut2::new(parent, borrow)
    }

    // spawn
    pub fn spawn<'a, S: ComponentSet<'a>>(&mut self, comps: S) -> Entity {
        let mut level = self.level_mut();
        level.spawn(comps)
    }

    pub fn get_unique<U: Unique>(&self) -> Option<AtomicRef3<U>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_unique::<U>()?;
        Some(AtomicRef3::new(root, parent, borrow))
    }

    pub fn get_unique_mut<U: Unique>(&mut self) -> Option<AtomicRefMut3<U>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_unique_mut::<U>()?;
        Some(AtomicRefMut3::new(root, parent, borrow))
    }

    // entities
    // entities_mut -- ???
    pub fn register_component<C: Component>(&mut self) {
        // Store in registry
        // Add to every level
        let mut levels = self.levels_mut();
        levels.register_component::<C>();
    }

    pub fn get_components<C: Component>(&self) -> Option<AtomicRef3<DenseStorage<C>>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_component::<C>()?;
        Some(AtomicRef3::new(root, parent, borrow))
    }

    pub fn get_components_mut<C: Component>(&mut self) -> Option<AtomicRefMut3<DenseStorage<C>>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_component_mut::<C>()?;
        Some(AtomicRefMut3::new(root, parent, borrow))
    }
}
