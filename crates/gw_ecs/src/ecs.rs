use atomic_refcell::{AtomicRef, AtomicRefMut};
use legion::{query::ReadOnly, Resources};

pub use legion::systems::{Resource, ResourceSet};
pub use legion::Read as Res;
pub use legion::Write as ResMut;

#[derive(Default)]
pub struct Ecs {
    resources: Resources,
}

impl Ecs {
    pub fn new() -> Self {
        Ecs {
            resources: Resources::default(),
        }
    }

    pub fn insert_res<R: Resource>(&mut self, res: R) {
        self.resources.insert(res);
    }

    pub fn res<R: Resource>(&self) -> Option<AtomicRef<R>> {
        self.resources.get::<R>()
    }

    pub fn res_mut<R: Resource>(&mut self) -> Option<AtomicRefMut<R>> {
        self.resources.get_mut::<R>()
    }

    pub fn fetch<S>(&self) -> <S as ResourceSet<'_>>::Result
    where
        for<'a> S: ResourceSet<'a> + ReadOnly,
    {
        S::fetch(&self.resources)
    }

    pub fn fetch_mut<S>(&mut self) -> <S as ResourceSet<'_>>::Result
    where
        for<'a> S: ResourceSet<'a>,
    {
        S::fetch_mut(&mut self.resources)
    }
}
