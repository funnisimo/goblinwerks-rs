use crate::Ecs;

pub trait BorrowRef<'a> {
    fn borrow(source: &'a Ecs) -> Self;
}

// This may be unnecessary -
pub trait BorrowMut<'a> {
    fn borrow_mut(source: &'a Ecs) -> Self;
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

macro_rules! impl_make_borrow {
    ($(($component: ident, $index: tt))+) => {

        impl<'a, $($component: BorrowRef<'a>,)+> BorrowRef<'a> for ($($component,)+)
        {
            fn borrow(source: &'a Ecs) -> Self {
                ($(<$component>::borrow(source),)+)
            }
        }

        impl<'a, $($component: BorrowRef<'a>,)+> ReadOnly for ($($component,)+) {}

        impl<'a, $($component: BorrowMut<'a>,)+> BorrowMut<'a> for ($($component,)+)
        {
            fn borrow_mut(source: &'a Ecs) -> Self {
                ($(<$component>::borrow_mut(source),)+)
            }
        }

    }
}

macro_rules! make_borrow {
    ($(($component: ident, $index: tt))+; ($component1: ident, $index1: tt) $(($queue_component: ident, $queue_index: tt))*) => {
        impl_make_borrow![$(($component, $index))*];
        make_borrow![$(($component, $index))* ($component1, $index1); $(($queue_component, $queue_index))*];
    };
    ($(($component: ident, $index: tt))+;) => {
        impl_make_borrow![$(($component, $index))*];
    }
}

make_borrow![(A, 0); (B, 1) (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)];
