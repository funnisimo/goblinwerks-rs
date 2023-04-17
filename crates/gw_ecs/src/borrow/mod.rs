use crate::Ecs;

pub trait BorrowRef<'e> {
    type Output: 'e;
    fn borrow(source: &'e Ecs) -> Self::Output;
}

// This may be unnecessary -
pub trait BorrowMut<'e> {
    type Output: 'e;
    fn borrow_mut(source: &'e Ecs) -> Self::Output;
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

        impl<'e, $($component,)+> BorrowRef<'e> for ($($component,)+)
        where
        $($component: for<'a> BorrowRef<'e>,)+
        {
            type Output = ($(<$component as BorrowRef<'e>>::Output,)+);

            fn borrow(source: &'e Ecs) -> Self::Output {
                ($(<$component>::borrow(source),)+)
            }
        }

        impl<'e, $($component: BorrowRef<'e>,)+> ReadOnly for ($($component,)+) {}

        impl<'e, $($component,)+> BorrowMut<'e> for ($($component,)+)
        where
        $($component: for<'a> BorrowMut<'e>,)+
        {
            type Output = ($(<$component as BorrowMut<'e>>::Output,)+);

            fn borrow_mut(source: &'e Ecs) -> Self::Output {
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
