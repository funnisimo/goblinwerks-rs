use super::Component;
use crate::{entity::Entity, world::World};

pub trait ComponentSet: Send + Sync + 'static {
    fn insert(self, world: &mut World, entity: Entity);
    fn remove(world: &mut World, entity: Entity);
}

impl<C: Component> ComponentSet for C {
    fn insert(self, world: &mut World, entity: Entity) {
        let _ = world.write_component::<C>().insert(entity, self);
    }

    fn remove(world: &mut World, entity: Entity) {
        world.write_component::<C>().remove(entity);
    }
}

macro_rules! impl_component_set {
    // use variables to indicate the arity of the tuple
    ($($from:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($from: Component,)*> ComponentSet for ($($from),*,)
        {
            fn insert(self, world: &mut World, entity: Entity) {
                let ($($from,)*) = self;
                $(
                    $from.insert(world, entity);
                )*
            }
            fn remove(world: &mut World, entity: Entity) {
                $(
                    world.write_component::<$from>().remove(entity);
                )*
            }
        }
    }
}

impl_component_set! {A}
impl_component_set! {A, B}
impl_component_set! {A, B, C}
impl_component_set! {A, B, C, D}
impl_component_set! {A, B, C, D, E}
impl_component_set! {A, B, C, D, E, F}
impl_component_set! {A, B, C, D, E, F, G}
impl_component_set! {A, B, C, D, E, F, G, H}
impl_component_set! {A, B, C, D, E, F, G, H, I}
impl_component_set! {A, B, C, D, E, F, G, H, I, J}
impl_component_set! {A, B, C, D, E, F, G, H, I, J, K}
impl_component_set! {A, B, C, D, E, F, G, H, I, J, K, L}
impl_component_set! {A, B, C, D, E, F, G, H, I, J, K, L, M}
impl_component_set! {A, B, C, D, E, F, G, H, I, J, K, L, M, N}
impl_component_set! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O}
impl_component_set! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P}
impl_component_set!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_component_set!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);

#[cfg(test)]
mod test {
    use super::*;
    use crate as gw_bevy;
    use crate::entity::Builder;
    use crate::world::World;
    use bevy_ecs_macros::Component;

    #[derive(Component, Default)]
    struct CompA(u32);

    #[derive(Component, Default)]
    struct CompB(u32);

    #[derive(Component, Default)]
    struct CompC(u32);

    #[test]
    fn comp_set_basic() {
        let mut world = World::default();
        world.register::<CompA>();
        world.register::<CompB>();
        world.register::<CompC>();

        let entity = world.create_entity().id();

        let set = (CompA(1), CompB(2), CompC(3));

        set.insert(&mut world, entity);

        assert!(world.read_component::<CompA>().contains(entity));
        assert!(world.read_component::<CompB>().contains(entity));
        assert!(world.read_component::<CompC>().contains(entity));
    }
}
