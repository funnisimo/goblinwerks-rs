#![allow(dead_code, unused_imports, unused_variables)]

use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
};

/// This is our system
#[derive(Default)]
struct Source {
    age: RefCell<u32>,
}

//////////////////////////////////////////////////////
// Got MaybeBorrowed from:
// https://users.rust-lang.org/t/problems-matching-up-lifetimes-between-various-traits-and-closure-parameters/71994

trait MaybeBorrowed<'a> {
    type Output: 'a + Sized;
}

/// Retrieve/borrow value from container
trait Fetch: for<'a> MaybeBorrowed<'a> {
    fn fetch(source: &Source) -> <Self as MaybeBorrowed<'_>>::Output;
}

impl<'a> MaybeBorrowed<'a> for u32 {
    type Output = u32;
}
impl Fetch for u32 {
    fn fetch(source: &Source) -> <Self as MaybeBorrowed<'_>>::Output {
        *source.age.borrow()
    }
}

impl<'a> MaybeBorrowed<'a> for Ref<'_, u32> {
    type Output = Ref<'a, u32>;
}
impl Fetch for Ref<'_, u32> {
    fn fetch(source: &Source) -> <Self as MaybeBorrowed<'_>>::Output {
        source.age.borrow()
    }
}

impl<'a> MaybeBorrowed<'a> for RefMut<'_, u32> {
    type Output = RefMut<'a, u32>;
}
impl Fetch for RefMut<'_, u32> {
    fn fetch(source: &Source) -> <Self as MaybeBorrowed<'_>>::Output {
        source.age.borrow_mut()
    }
}

impl<'a, A> MaybeBorrowed<'a> for (A,)
where
    A: MaybeBorrowed<'a>,
{
    type Output = (<A as MaybeBorrowed<'a>>::Output,);
}
impl<A> Fetch for (A,)
where
    A: Fetch,
{
    fn fetch(source: &Source) -> <Self as MaybeBorrowed<'_>>::Output {
        (A::fetch(source),)
    }
}

impl<'a, A, B> MaybeBorrowed<'a> for (A, B)
where
    A: MaybeBorrowed<'a>,
    B: MaybeBorrowed<'a>,
{
    type Output = (
        <A as MaybeBorrowed<'a>>::Output,
        <B as MaybeBorrowed<'a>>::Output,
    );
}
impl<A, B> Fetch for (A, B)
where
    A: Fetch,
    B: Fetch,
{
    fn fetch(source: &Source) -> <Self as MaybeBorrowed<'_>>::Output {
        (A::fetch(source), B::fetch(source))
    }
}

/// A function that operates on a Source
struct System {
    func: Box<dyn Fn(&Source) -> ()>,
}

impl System {
    /// Construct a new System with the given work function
    fn new(func: Box<dyn Fn(&Source) -> ()>) -> Self {
        System { func }
    }

    /// Run the system's work function
    fn run(&self, source: &Source) {
        (self.func)(source);
    }
}

/// Converts a type into a system
trait IntoSystem {
    fn into_system(self) -> System;
}

/// Make a system for a Source level func
impl<F> IntoSystem for F
where
    F: Fn(&Source) -> () + 'static,
{
    fn into_system(self) -> System {
        System::new(Box::new(move |source| {
            (self)(source);
        }))
    }
}

fn test_sys(source: &Source) {
    let a = u32::fetch(source);
    println!("sys = {}", a);
}

fn main() {
    let source = Source::default();
    *source.age.borrow_mut() = 4;

    let a = u32::fetch(&source);
    println!("u32 = {}", a);

    {
        let ref_a = Ref::<u32>::fetch(&source);
        println!("ref_u32 = {}", *ref_a);
    }
    {
        let mut mut_a = RefMut::<u32>::fetch(&source);
        *mut_a = 10;
        println!("mut_u32 = {}", *mut_a);
    }

    let (a,) = <(u32,)>::fetch(&source);
    println!("u32 = {}", a);

    let (a, b) = <(u32, u32)>::fetch(&source);
    println!("dual fetch = {}, {}", a, b);

    let system: System = test_sys.into_system();
    system.run(&source);
}
