use crate::{entity::Entities, Ecs};

// pub trait MaybeBorrowed {
//     type Output<'a>;
// }

pub trait Fetch {
    type Output<'a>;
    fn fetch(ecs: &Ecs) -> Self::Output<'_>;
}

// impl MaybeBorrowed for &Ecs {
//     type Output<'a> = &'a Ecs;
// }

impl Fetch for () {
    type Output<'a> = ();
    fn fetch(_ecs: &Ecs) -> Self::Output<'_> {
        ()
    }
}

impl Fetch for &Ecs {
    type Output<'a> = &'a Ecs;
    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        ecs
    }
}

impl Fetch for Entities {
    type Output<'a> = Unique<'a, Entities>;
    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        ecs.get_unique::<Entities>().unwrap()
    }
}

pub trait ReadOnly {}

mod comp;
mod global;
mod level;
mod unique;

pub use comp::*;
pub use global::*;
pub use level::*;
pub use unique::*;

macro_rules! impl_make_fetch {
    ($(($component: ident, $index: tt))+) => {

        // impl<$($component,)+> MaybeBorrowed for ($($component,)+)
        // where
        //     $($component: MaybeBorrowed,)+
        // {
        //     type Output<'a> = ($(<$component as MaybeBorrowed>::Output<'a>,)+);
        // }
        impl<$($component,)+> Fetch for ($($component,)+)
        where
            $($component: Fetch,)+
        {
            type Output<'a> = ($(<$component as Fetch>::Output<'a>,)+);

            fn fetch(ecs: &Ecs) -> Self::Output<'_> {
                 ($(<$component>::fetch(ecs),)+)
            }
        }

    }
}

macro_rules! make_fetch {
    ($(($component: ident, $index: tt))+; ($component1: ident, $index1: tt) $(($queue_component: ident, $queue_index: tt))*) => {
        impl_make_fetch![$(($component, $index))*];
        make_fetch![$(($component, $index))* ($component1, $index1); $(($queue_component, $queue_index))*];
    };
    ($(($component: ident, $index: tt))+;) => {
        impl_make_fetch![$(($component, $index))*];
    }
}

make_fetch![(A, 0); (B, 1) (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)];
