#![allow(dead_code, unused_imports)]

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ops::Deref;

pub trait Borrowable {
    fn borrow(source: &Source) -> Self;
}

#[derive(Debug, Default)]
pub struct Entity(u32);

pub struct Source {}

impl Source {
    pub fn new() -> Self {
        Source {}
    }

    pub fn borrow<T: Borrowable>(&self) -> T {
        T::borrow(self)
    }

    pub fn get_global<T>(&self) -> Global<T>
    where
        T: Default,
    {
        Global {
            borrow: T::default(),
        }
    }

    pub fn get_unique<T>(&self) -> Unique<T>
    where
        T: Default,
    {
        Unique {
            borrow: T::default(),
        }
    }
}

#[derive(Debug)]
pub struct Global<T>
where
    T: Default,
{
    borrow: T,
}

impl<T> Deref for Global<T>
where
    T: Default,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.borrow
    }
}

impl<T> AsRef<T> for Global<T>
where
    T: Default,
{
    #[inline]
    fn as_ref(&self) -> &T {
        &self.borrow
    }
}

impl<T> Borrowable for Global<T>
where
    T: Default,
{
    fn borrow(source: &Source) -> Self {
        source.get_global::<T>()
    }
}

#[derive(Debug)]
pub struct Unique<T>
where
    T: Default,
{
    borrow: T,
}

impl<T> Borrowable for Unique<T>
where
    T: Default,
{
    fn borrow(source: &Source) -> Self {
        source.get_unique::<T>()
    }
}

impl<T> Deref for Unique<T>
where
    T: Default,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.borrow
    }
}

impl<T> AsRef<T> for Unique<T>
where
    T: Default,
{
    #[inline]
    fn as_ref(&self) -> &T {
        &self.borrow
    }
}

// impl<A, B> Borrowable for (A, B)
// where
//     A: Borrowable,
//     B: Borrowable,
// {
//     fn borrow(source: &Source) -> Self {
//         (<A>::borrow(source), <B>::borrow(source))
//     }
// }

type SystemFunc = dyn FnMut(&Source) -> ();

pub struct System {
    func: Box<SystemFunc>,
}

impl System {
    fn run(&mut self, source: &Source) -> () {
        (self.func)(source);
    }
}

pub trait IntoSystem<D> {
    fn into_system(self) -> System;
}

impl<F> IntoSystem<&Source> for F
where
    F: FnMut(&Source) -> () + 'static,
{
    fn into_system(mut self) -> System {
        System {
            func: Box::new(move |source| (self)(source)),
        }
    }
}

// impl<Func, A> IntoSystem<(A,)> for Func
// where
//     A: Borrowable,
//     Func: FnMut(A) -> () + 'static,
// {
//     fn into_system(mut self) -> System {
//         System {
//             func: Box::new(move |source| {
//                 let data = A::borrow(source);
//                 (self)(data)
//             }),
//         }
//     }
// }

// impl<Func, A, B> IntoSystem<(A, B)> for Func
// where
//     A: Borrowable,
//     B: Borrowable,
//     Func: FnMut(A, B) -> () + 'static,
// {
//     fn into_system(mut self) -> System {
//         System {
//             func: Box::new(move |source| {
//                 let data_a = A::borrow(source);
//                 let data_b = B::borrow(source);
//                 (self)(data_a, data_b)
//             }),
//         }
//     }
// }

macro_rules! impl_make_system_fn {
    ($(($component: ident, $index: tt))+) => {

        impl<$($component: Borrowable,)+> Borrowable for ($($component,)+)
        {
            fn borrow(source: &Source) -> Self {
                ($(<$component>::borrow(source),)+)
            }
        }

        impl< $($component: Borrowable,)+ Func> IntoSystem<($($component,)+)> for Func
        where
            Func: FnMut($($component,)+) -> () + 'static,
        {
            fn into_system(mut self) -> System {
                System {
                    func: Box::new(move |source| {
                        let data = source.borrow::<($($component,)+)>();
                        (self)($(data.$index,)+)
                    }),
                }
            }
        }
    }
}

macro_rules! make_system_fn {
    ($(($component: ident, $index: tt))+; ($component1: ident, $index1: tt) $(($queue_component: ident, $queue_index: tt))*) => {
        impl_make_system_fn![$(($component, $index))*];
        make_system_fn![$(($component, $index))* ($component1, $index1); $(($queue_component, $queue_index))*];
    };
    ($(($component: ident, $index: tt))+;) => {
        impl_make_system_fn![$(($component, $index))*];
    }
}

make_system_fn![(A, 0); (B, 1) (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)];

fn main() {
    let source = Source {};

    let source_gl = source.get_global::<Entity>();

    println!("source = {:?}", source_gl);

    let global_ref = Global::<Entity>::borrow(&source);

    println!("borrowed = {:?}", global_ref.0);

    let (gl, un) = <(Global<Entity>, Unique<Entity>)>::borrow(&source);

    println!("borrowed = {:?} {:?}", gl.0, un);

    let (gl, un) = source.borrow::<(Global<Entity>, Unique<Entity>)>();

    println!("fetched = {:?} {:?}", gl.0, un);

    let system_fn = |_source: &Source| {
        println!("Hello from System!");
    };
    let mut system = system_fn.into_system();
    system.run(&source);

    let system_fn = |entity: Global<Entity>| {
        println!("Hello from Global Entity System - {}!", entity.0);
    };
    let mut system = system_fn.into_system();
    system.run(&source);

    let system_fn = |entity: Global<Entity>, e2: Unique<Entity>| {
        println!(
            "Hello from Global + Unique System - {} + {}!",
            entity.0, e2.0
        );
    };
    let mut system = system_fn.into_system();
    system.run(&source);
}
