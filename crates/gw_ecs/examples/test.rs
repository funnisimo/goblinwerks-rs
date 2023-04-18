#![allow(dead_code, unused_imports, unused_variables)]

use gw_ecs::{Ecs, Fetch, Global, GlobalMut};

use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
};

//////////////////////////////////////////////////////
// Got MaybeBorrowed from:
// https://users.rust-lang.org/t/problems-matching-up-lifetimes-between-various-traits-and-closure-parameters/71994
// https://users.rust-lang.org/t/into-like-trait-cant-infer-types-even-with-result-type/92672/5

// trait MaybeBorrowed {
//     type Output<'a>;
// }

// trait Fetch: MaybeBorrowed {
//     fn fetch(source: &Ecs) -> <Self as MaybeBorrowed>::Output<'_>;
// }

// impl MaybeBorrowed for &Ecs {
//     type Output<'a> = &'a Ecs;
// }
// impl Fetch for &Ecs {
//     fn fetch(source: &Ecs) -> <Self as MaybeBorrowed>::Output<'_> {
//         source
//     }
// }

// impl MaybeBorrowed for u32 {
//     type Output<'a> = u32;
// }
// impl Fetch for u32 {
//     fn fetch(source: &Ecs) -> <Self as MaybeBorrowed>::Output<'_> {
//         *source.age.borrow()
//     }
// }

// impl MaybeBorrowed for Ref<'_, u32> {
//     type Output<'a> = Ref<'a, u32>;
// }
// impl Fetch for Ref<'_, u32> {
//     fn fetch(source: &Ecs) -> <Self as MaybeBorrowed>::Output<'_> {
//         source.age.borrow()
//     }
// }

// impl MaybeBorrowed for RefMut<'_, u32> {
//     type Output<'a> = RefMut<'a, u32>;
// }
// impl Fetch for RefMut<'_, u32> {
//     fn fetch(source: &Ecs) -> <Self as MaybeBorrowed>::Output<'_> {
//         source.age.borrow_mut()
//     }
// }

// impl<A> MaybeBorrowed for (A,)
// where
//     A: MaybeBorrowed,
// {
//     type Output<'a> = (<A as MaybeBorrowed>::Output<'a>,);
// }
// impl<A> Fetch for (A,)
// where
//     A: Fetch,
// {
//     fn fetch(source: &Ecs) -> <Self as MaybeBorrowed>::Output<'_> {
//         (A::fetch(source),)
//     }
// }

// impl<A, B> MaybeBorrowed for (A, B)
// where
//     A: MaybeBorrowed,
//     B: MaybeBorrowed,
// {
//     type Output<'a> = (
//         <A as MaybeBorrowed>::Output<'a>,
//         <B as MaybeBorrowed>::Output<'a>,
//     );
// }
// impl<A, B> Fetch for (A, B)
// where
//     A: Fetch,
//     B: Fetch,
// {
//     fn fetch(source: &Ecs) -> <Self as MaybeBorrowed>::Output<'_> {
//         (A::fetch(source), B::fetch(source))
//     }
// }

/// A function that operates on a Ecs
struct System {
    func: Box<dyn Fn(&Ecs) -> ()>,
}

impl System {
    /// Construct a new System with the given work function
    fn new(func: Box<dyn Fn(&Ecs) -> ()>) -> Self {
        System { func }
    }

    /// Run the system's work function
    fn run(&self, source: &Ecs) {
        (self.func)(source);
    }
}

impl<F> From<F> for System
where
    F: Fn(&Ecs) -> () + 'static,
{
    fn from(value: F) -> Self {
        System::new(Box::new(value))
    }
}

fn test_sys(source: &Ecs) {
    let a = <Global<u32>>::fetch(source);
    println!("sys = {}", *a);
}

fn test_sys_system(source: &Ecs) {
    let data = <(&Ecs,)>::fetch(source);
    test_sys(data.0);
}

fn val_sys(val: Global<u32>) {
    println!("u32 sys = {}", *val);
}

fn val_sys_system(source: &Ecs) {
    let data = <(Global<u32>,)>::fetch(source);
    val_sys(data.0);
}

fn ref_sys(val: Global<u32>) {
    println!("u32 sys = {}", *val);
}

fn ref_sys_system(source: &Ecs) {
    let data = <(Global<u32>,)>::fetch(source);
    ref_sys(data.0);
}

fn main() {
    let mut source = Ecs::new();
    source.insert_global(4u32);

    {
        let a = <Global<u32>>::fetch(&source);
        println!("u32 = {}", *a);
    }
    {
        let mut mut_a = <GlobalMut<u32>>::fetch(&source);
        *mut_a = 10;
        println!("mut_u32 = {}", *mut_a);
    }

    let (a,) = <(Global<u32>,)>::fetch(&source);
    println!("u32 = {}", *a);

    let (a, b) = <(Global<u32>, Global<u32>)>::fetch(&source);
    println!("dual fetch = {}, {}", *a, *b);

    let system: System = val_sys_system.into();
    system.run(&source);

    let system: System = ref_sys_system.into();
    system.run(&source);

    let system: System = test_sys_system.into();
    system.run(&source);
}
