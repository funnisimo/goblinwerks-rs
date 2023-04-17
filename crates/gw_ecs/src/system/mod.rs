use crate::borrow::BorrowRef;
use crate::Ecs;

type SystemFunc = dyn Fn(&mut Ecs) -> () + 'static;

pub struct System {
    func: Box<SystemFunc>,
}

impl System {
    pub fn run(&self, ecs: &mut Ecs) -> () {
        (self.func)(ecs);
    }
}

pub trait IntoSystem<D> {
    fn into_system(self) -> System;
}

impl<F> IntoSystem<&mut Ecs> for F
where
    F: Fn(&mut Ecs) -> () + 'static,
{
    fn into_system(self) -> System {
        System {
            func: Box::new(move |ecs| (self)(ecs)),
        }
    }
}

// impl<A, Func> IntoSystem<(A,)> for Func
// where
//     A: for<'e> BorrowRef<'e>,
//     Func: for<'e> Fn(<A as BorrowRef<'e>>::Output) -> (),
//     Func: 'static,
// {
//     fn into_system(self) -> System {
//         System {
//             func: Box::new(move |ecs: &mut Ecs| {
//                 let data = <A>::borrow(ecs);
//                 (self)(data);
//             }),
//         }
//     }
// }

// impl<A, B, Func> IntoSystem<(A, B)> for Func
// where
//     A: for<'e> BorrowRef<'e>,
//     B: for<'e> BorrowRef<'e>,
//     Func: for<'e> Fn(<A as BorrowRef<'e>>::Output, <B as BorrowRef<'e>>::Output) -> (),
//     Func: 'static,
// {
//     fn into_system(self) -> System {
//         System {
//             func: Box::new(move |ecs: &mut Ecs| {
//                 let data_0 = <A>::borrow(ecs);
//                 let data_1 = <B>::borrow(ecs);
//                 (self)(data_0, data_1)
//             }),
//         }
//     }
// }

// macro_rules! impl_make_system_fn {
//     ($(($component: ident, $index: tt))+) => {

//         impl< $($component,)+ Func> IntoSystem<($($component,)+)> for Func
//         where
//             $(for<'a> $component: BorrowRef<'a>,)+
//             Func: FnMut($($component,)+) -> () + 'static,
//         {
//             fn into_system(mut self) -> System {
//                 System {
//                     func: Box::new(move |ecs: &mut Ecs| {
//                         let data = <($($component,)+)>::borrow(ecs);
//                         (self)($(data.$index,)+)
//                     }),
//                 }
//             }
//         }

//     }
// }

// macro_rules! make_system_fn {
//     ($(($component: ident, $index: tt))+; ($component1: ident, $index1: tt) $(($queue_component: ident, $queue_index: tt))*) => {
//         impl_make_system_fn![$(($component, $index))*];
//         make_system_fn![$(($component, $index))* ($component1, $index1); $(($queue_component, $queue_index))*];
//     };
//     ($(($component: ident, $index: tt))+;) => {
//         impl_make_system_fn![$(($component, $index))*];
//     }
// }

// make_system_fn![(A, 0); (B, 1) (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)];
