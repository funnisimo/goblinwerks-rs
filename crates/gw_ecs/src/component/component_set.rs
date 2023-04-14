use crate::{Component, Entity, Level};

/// Trait which is implemented for tuples of components and singular components. This allows
/// easy spawning of component bundles.
///
/// # Example:
/// ```
/// struct TypeA(usize);
/// struct TypeB(usize);
///
/// # use gw_ecs::*;
/// # use gw_ecs::component::ComponentSet;
/// let mut ecs = Ecs::new();
/// ecs.register_component<TypeA>();
/// ecs.register_component<TypeB>();
/// ecs.spawn((TypeA(1), TypeB(2)));
/// ```
pub trait ComponentSet<'a> {
    /// Fetches all defined resources.
    fn spawn(self, level: &mut Level, entity: Entity);
}

impl<'a> ComponentSet<'a> for () {
    fn spawn(self, _: &mut Level, _: Entity) {}
}

// impl<'a, C> ComponentSet<'a> for C
// where
//     C: Component,
// {
//     fn spawn(self, level: &mut Level, entity: Entity) {
//         level.add_component(entity, self);
//     }
// }

macro_rules! impl_add_component {
    ($(($component: ident, $index: tt))+) => {
        impl<'a, $($component: Component,)+> ComponentSet<'a> for ($($component,)+) {
            #[inline]
            #[track_caller]
            fn spawn(self, level: &mut Level, entity: Entity) {
                $(
                    level.add_component(entity, self.$index);
                )+
            }
        }
    }
}

macro_rules! add_component {
    ($(($component: ident, $index: tt))+; ($component1: ident, $index1: tt) $(($queue_component: ident, $queue_index: tt))*) => {
        impl_add_component![$(($component, $index))*];
        add_component![$(($component, $index))* ($component1, $index1); $(($queue_component, $queue_index))*];
    };
    ($(($component: ident, $index: tt))+;) => {
        impl_add_component![$(($component, $index))*];
    }
}

add_component![(A, 0); (B, 1) (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)];
