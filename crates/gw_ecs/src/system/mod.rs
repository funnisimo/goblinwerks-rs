// use crate::fetch::{Fetch, MaybeBorrowed};
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

// pub trait IntoSystem<D> {
//     fn into_system(self) -> System;
// }

// impl<F> IntoSystem<&mut Ecs> for F
// where
//     F: Fn(&mut Ecs) -> () + 'static,
// {
//     fn into_system(self) -> System {
//         System {
//             func: Box::new(move |ecs| (self)(ecs)),
//         }
//     }
// }

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

/*

FROM RUST LANG POST

// use core::cell::RefMut;
use core::cell::Ref;
use core::cell::RefCell;

/// This is our system
#[derive(Default)]
struct Source { age: RefCell<u32>, }
/// A function that operates on a Source
struct System { func: Box<dyn Fn(&Source) -> ()>, }

/// Converts a type into a system
trait IntoSystem<A> {
    // Needs bikeshed
    fn into_system_via_representative(self, _representative: A) -> System where Self: Sized {
        self.into_system()
    }

    fn into_system(self) -> System;
}

/// A work function
fn sys_source(source: &Source) { println!("source system - {}", source.age.borrow()); }
/// A fetch work function
fn sys_u32(age: u32) { println!("age system - {}", age); }
/// A fetch work function
fn sys_ref(age: Ref<u32>) { println!("ref system - {}", *age); }


/// Make a system for a Source level func
impl<F> IntoSystem<&Source> for F
where
F: Fn(&Source) -> () + 'static
{
    fn into_system(self) -> System {
        System::new(Box::new(move |source| {
            (self)(source);
        }))
    }
}

/// Make a system that takes a single fetch parameter
impl<F,A> IntoSystem<(A,)> for F
where
A: Fetch + for<'x> MaybeBorrowed<'x, Output = A>,
F: Fn(A) -> () + 'static
{
    fn into_system(self) -> System {
        System::new(Box::new(move |source| {
            let data = A::fetch(source);
            (self)(data);
        }))
    }
}

// Using `Ref<'_, u32>` does not resolve the ambiguity errors and also triggers
// a `coherence_leak_check` warning.  https://github.com/rust-lang/rust/issues/56105
// I.e. the code using `Ref<'_, 32>` may or may not continue to be accepted
struct AllRef;
impl<F> IntoSystem<(AllRef,)> for F
where
F: Fn(Ref<'_, u32>) -> () + 'static
{
    fn into_system(self) -> System {
        System::new(Box::new(move |source| {
            let data = AllRef::fetch(source);
            (self)(data);
        }))
    }
}

// ---------------------------------------------

impl System {
    /// Construct a new System with the given work function
    fn new(func: Box<dyn Fn(&Source) -> ()> ) -> Self {
        System { func }
    }

    /// Run the system's work function
    fn run(&self, source: &Source) {
        (self.func)(source);
    }
}

//---------------------------------------------------

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

impl<'a> MaybeBorrowed<'a> for AllRef {
    type Output = Ref<'a, u32>;
}
impl Fetch for AllRef {
    fn fetch(source: &Source) -> <Self as MaybeBorrowed<'_>>::Output {
        source.age.borrow()
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

// ---------------------------------------------

fn main() {
    let  source = Source::default();

    let age = u32::fetch(&source);
    println!("solo u32 = {}", age);

    let (age,age_ref) = <(u32,AllRef)>::fetch(&source);
    println!("fetch + ref = {} : {}", age, age_ref);

    let sys_a: System = sys_source.into_system();   // Works

    ////////////////////////////////
    // Question 1 - What has to change to make the simpler A.into_system() format work?
    ////////////////////////////////

    let sys_b: System = sys_u32.into_system();      // error[E0282]: type annotations needed
    // let sys_b: System = <fn(u32) as IntoSystem<(u32,)>>::into_system(sys_u32); // Works

    ////////////////////////////////
    // Question 2 - What has to change to make the lifetime limited fetches work?
    ////////////////////////////////

    // let sys_c: System = sys_ref.into_system();      // error[E0282]: type annotations needed
    // let sys_c: System = <fn(Ref<u32>) as IntoSystem<(AllRef,)>>::into_system(sys_ref); // error: implementation of `Fetch` is not general enough
    // = note: `Fetch<'0>` would have to be implemented for the type `Ref<'_, u32>`, for any lifetime `'0`...
    // = note: ...but `Fetch<'1>` is actually implemented for the type `Ref<'1, u32>`, for some specific lifetime `'1`
    // let sys_c = sys_ref.into_system_via_representative((AllRef,));
    let sys_c = sys_ref.into_system();

    sys_a.run(&source);
    sys_b.run(&source);
    sys_c.run(&source);
}


 */
