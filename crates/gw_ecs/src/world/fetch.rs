use super::World;
use crate::{
    components::{Component, ReadComp, WriteComp},
    entity::Entities,
    prelude::{GlobalMut, GlobalRef},
    resources::{ResMut, ResRef},
    system::Resource,
};

pub trait Fetch {
    type Item<'a>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a>;
}

// RESOURCES

impl<'w, T> Fetch for ResRef<'w, T>
where
    T: Resource,
{
    type Item<'a> = ResRef<'a, T>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.read_resource::<T>()
    }
}

impl<'w, T> Fetch for ResMut<'w, T>
where
    T: Resource,
{
    type Item<'a> = ResMut<'a, T>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.write_resource::<T>()
    }
}

// GLOBALS

impl<'w, T> Fetch for GlobalRef<'w, T>
where
    T: Resource,
{
    type Item<'a> = GlobalRef<'a, T>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.read_global::<T>()
    }
}

impl<'w, T> Fetch for GlobalMut<'w, T>
where
    T: Resource,
{
    type Item<'a> = GlobalMut<'a, T>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.write_global::<T>()
    }
}

// COMPONENTS

impl<'w, T> Fetch for ReadComp<'w, T>
where
    T: Component,
{
    type Item<'a> = ReadComp<'a, T>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.read_component::<T>()
    }
}

impl<'w, T> Fetch for WriteComp<'w, T>
where
    T: Component,
{
    type Item<'a> = WriteComp<'a, T>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.write_component::<T>()
    }
}

// ENTITIES

impl<'w> Fetch for Entities<'w> {
    type Item<'a> = Entities<'a>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.entities()
    }
}

macro_rules! impl_fetch {
    // use variables to indicate the arity of the tuple
    ($($from:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($from: Fetch,)*> Fetch for ($($from),*,)
        {
            type Item<'a> = ($(<$from as Fetch>::Item<'a>,)*);

            fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
                ($(
                    $from::fetch(world),
                )*)
            }
        }
    }
}

impl_fetch! {A}
impl_fetch! {A, B}
impl_fetch! {A, B, C}
impl_fetch! {A, B, C, D}
impl_fetch! {A, B, C, D, E}
impl_fetch! {A, B, C, D, E, F}
impl_fetch! {A, B, C, D, E, F, G}
impl_fetch! {A, B, C, D, E, F, G, H}
impl_fetch! {A, B, C, D, E, F, G, H, I}
impl_fetch! {A, B, C, D, E, F, G, H, I, J}
impl_fetch! {A, B, C, D, E, F, G, H, I, J, K}
impl_fetch! {A, B, C, D, E, F, G, H, I, J, K, L}
impl_fetch! {A, B, C, D, E, F, G, H, I, J, K, L, M}
impl_fetch! {A, B, C, D, E, F, G, H, I, J, K, L, M, N}
impl_fetch! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O}
impl_fetch! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P}
impl_fetch!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_fetch!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
