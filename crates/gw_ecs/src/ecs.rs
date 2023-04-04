use crate::levels::Levels;
use crate::refcell::{AtomicRef, AtomicRefMut};
use crate::resource::{Resource, ResourceSet, Resources};

pub struct Ecs {
    resources: Resources,
}

impl Ecs {
    pub fn new() -> Self {
        let mut res = Resources::default();
        res.insert(Levels::default());

        Ecs { resources: res }
    }

    pub fn insert_res<R: Resource>(&mut self, res: R) {
        self.resources.insert(res);
    }

    pub fn res<R: Resource>(&self) -> Option<AtomicRef<R>> {
        self.resources.get::<R>()
    }

    pub fn res_mut<R: Resource>(&self) -> Option<AtomicRefMut<R>> {
        self.resources.get_mut::<R>()
    }

    pub fn fetch<S>(&self) -> <S as ResourceSet<'_>>::Result
    where
        for<'a> S: ResourceSet<'a>,
    {
        S::fetch(&self.resources)
    }
}
