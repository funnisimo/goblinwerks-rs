use std::ops::Deref;

use gw_ecs::resource::{Resource, Resources};
use gw_ecs::storage::SparseSet;
use gw_ecs::Levels;
use gw_ecs::{refcell::*, ReadOnly};
use gw_ecs::{Component, Level};

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

pub trait BorrowRef<'e> {
    type Output: 'e;
    fn borrow(source: &'e TestEcs) -> Self::Output;
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a global value
#[derive(Debug)]
pub struct Global<'a, T>
where
    T: Resource,
{
    borrow: AtomicRef<'a, T>,
}

impl<'a, T> Global<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(borrow: AtomicRef<'a, T>) -> Self {
        Global { borrow }
    }

    pub(crate) fn destructure(self) -> (&'a T, AtomicBorrowRef<'a>) {
        self.borrow.destructure()
    }
}

impl<'a, T> Clone for Global<'a, T>
where
    T: Resource,
{
    fn clone(&self) -> Self {
        Global {
            borrow: AtomicRef::clone(&self.borrow),
        }
    }
}

impl<'a, T> ReadOnly for Global<'a, T> where T: Resource {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, T> Deref for Global<'a, T>
where
    T: Resource,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.borrow.deref()
    }
}

impl<'a, T> AsRef<T> for Global<'a, T>
where
    T: Resource,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.borrow.as_ref()
    }
}

impl<'e, T> BorrowRef<'e> for Global<'e, T>
where
    T: Resource,
{
    type Output = Global<'e, T>;

    fn borrow(ecs: &'e TestEcs) -> Self::Output {
        ecs.get_global::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

pub type LevelsRef<'a> = Global<'a, Levels>;

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reads the current level

pub struct LevelRef<'a> {
    _parent: AtomicBorrowRef<'a>,
    borrow: AtomicRef<'a, Level>,
}

impl<'a> LevelRef<'a> {
    pub(crate) fn new(parent: AtomicBorrowRef<'a>, borrow: AtomicRef<'a, Level>) -> Self {
        LevelRef {
            _parent: parent,
            borrow,
        }
    }
}

impl<'a> ReadOnly for LevelRef<'a> {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a> Deref for LevelRef<'a> {
    type Target = Level;

    #[inline]
    fn deref(&self) -> &Level {
        self.borrow.deref()
    }
}

impl<'a> AsRef<Level> for LevelRef<'a> {
    #[inline]
    fn as_ref(&self) -> &Level {
        self.borrow.as_ref()
    }
}

impl<'e> BorrowRef<'e> for LevelRef<'e> {
    type Output = LevelRef<'e>;

    fn borrow(ecs: &'e TestEcs) -> Self::Output {
        ecs.level()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a level's unique value
pub struct Unique<'a, T>
where
    T: Resource,
{
    _levels: AtomicBorrowRef<'a>,
    _level: AtomicBorrowRef<'a>,
    borrow: AtomicRef<'a, T>,
}

impl<'a, T> Unique<'a, T>
where
    T: Resource,
{
    pub(crate) fn new(
        levels: AtomicBorrowRef<'a>,
        level: AtomicBorrowRef<'a>,
        borrow: AtomicRef<'a, T>,
    ) -> Self {
        Unique {
            _levels: levels,
            _level: level,
            borrow,
        }
    }
}

impl<'a, T> Clone for Unique<'a, T>
where
    T: Resource,
{
    fn clone(&self) -> Self {
        Unique {
            _levels: AtomicBorrowRef::clone(&self._levels),
            _level: AtomicBorrowRef::clone(&self._level),
            borrow: AtomicRef::clone(&self.borrow),
        }
    }
}

impl<'a, T> ReadOnly for Unique<'a, T> where T: Resource {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a, T> Deref for Unique<'a, T>
where
    T: Resource,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.borrow.deref()
    }
}

impl<'a, T> AsRef<T> for Unique<'a, T>
where
    T: Resource,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.borrow.as_ref()
    }
}

impl<'e, T> BorrowRef<'e> for Unique<'e, T>
where
    T: Resource,
{
    type Output = Unique<'e, T>;

    fn borrow(ecs: &'e TestEcs) -> Self::Output {
        let (levels, root) = ecs.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_unique::<T>().unwrap();
        Unique::new(root, parent, borrow)

        // ecs.get_unique::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a level's unique value
pub struct Comp<'b, T>
where
    T: Component,
{
    _levels: AtomicBorrowRef<'b>,
    _level: AtomicBorrowRef<'b>,
    borrow: AtomicRef<'b, SparseSet<T>>,
}

impl<'b, T> Comp<'b, T>
where
    T: Component,
{
    pub(crate) fn new(
        levels: AtomicBorrowRef<'b>,
        level: AtomicBorrowRef<'b>,
        borrow: AtomicRef<'b, SparseSet<T>>,
    ) -> Self {
        Comp {
            _levels: levels,
            _level: level,
            borrow,
        }
    }
}

impl<'b, T> Clone for Comp<'b, T>
where
    T: Component,
{
    fn clone(&self) -> Self {
        Comp {
            _levels: AtomicBorrowRef::clone(&self._levels),
            _level: AtomicBorrowRef::clone(&self._level),
            borrow: AtomicRef::clone(&self.borrow),
        }
    }
}

impl<'b, T> ReadOnly for Comp<'b, T> where T: Component {}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'b, T> Deref for Comp<'b, T>
where
    T: Component,
{
    type Target = SparseSet<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.borrow.deref()
    }
}

impl<'b, T> AsRef<SparseSet<T>> for Comp<'b, T>
where
    T: Component,
{
    #[inline]
    fn as_ref(&self) -> &SparseSet<T> {
        self.borrow.as_ref()
    }
}

impl<'e, T> BorrowRef<'e> for Comp<'e, T>
where
    T: Component,
{
    type Output = Comp<'e, T>;

    fn borrow(ecs: &'e TestEcs) -> Self::Output {
        ecs.get_component::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

macro_rules! impl_make_borrow {
    ($(($component: ident, $index: tt))+) => {

        impl<'e, $($component,)+> BorrowRef<'e> for ($($component,)+)
        where
        $($component: for<'a> BorrowRef<'e>,)+
        {
            type Output = ($(<$component as BorrowRef<'e>>::Output,)+);

            fn borrow(source: &'e TestEcs) -> Self::Output {
                ($(<$component>::borrow(source),)+)
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

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

type SystemFunc = dyn Fn(&mut TestEcs) -> () + 'static;

pub struct System {
    func: Box<SystemFunc>,
}

impl System {
    pub fn run(&self, ecs: &mut TestEcs) -> () {
        (self.func)(ecs);
    }
}

pub trait IntoSystem<D> {
    fn into_system(self) -> System;
}

impl<F> IntoSystem<&mut TestEcs> for F
where
    F: Fn(&mut TestEcs) -> () + 'static,
{
    fn into_system(self) -> System {
        System {
            func: Box::new(move |ecs| (self)(ecs)),
        }
    }
}

impl<A, Func> IntoSystem<(A,)> for Func
where
    A: for<'e> BorrowRef<'e>,
    Func: for<'e> Fn(<A as BorrowRef<'e>>::Output) -> (),
    Func: 'static,
{
    fn into_system(self) -> System {
        System {
            func: Box::new(move |ecs: &mut TestEcs| {
                let data = <A>::borrow(ecs);
                (self)(data);
            }),
        }
    }
}

impl<A, B, Func> IntoSystem<(A, B)> for Func
where
    A: for<'e> BorrowRef<'e>,
    B: for<'e> BorrowRef<'e>,
    Func: for<'e> Fn(<A as BorrowRef<'e>>::Output, <B as BorrowRef<'e>>::Output) -> (),
    Func: 'static,
{
    fn into_system(self) -> System {
        System {
            func: Box::new(move |ecs: &mut TestEcs| {
                let data_0 = <A>::borrow(ecs);
                let data_1 = <B>::borrow(ecs);
                (self)(data_0, data_1)
            }),
        }
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

pub struct TestEcs {
    pub(crate) resources: Resources,
}

impl TestEcs {
    pub fn new() -> Self {
        let mut res = Resources::default();
        res.insert(Levels::new());

        TestEcs { resources: res }
    }

    pub fn insert_global<R: Resource>(&mut self, res: R) {
        self.resources.insert(res);
    }

    pub fn get_global<R: Resource>(&self) -> Option<Global<'_, R>> {
        match self.resources.get::<R>() {
            None => None,
            Some(b) => Some(Global::new(b)),
        }
    }

    pub fn levels(&self) -> LevelsRef<'_> {
        self.get_global::<Levels>().unwrap()
    }

    pub fn level(&self) -> LevelRef<'_> {
        let levels = self.levels();
        let (levels, parent) = levels.destructure();
        let borrow = levels.current();
        LevelRef::new(parent, borrow)
    }

    pub fn get_unique<U: Resource>(&self) -> Option<Unique<'_, U>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_unique::<U>()?;
        Some(Unique::new(root, parent, borrow))
    }

    pub fn get_component<C: Component>(&self) -> Option<Comp<'_, C>> {
        let (levels, root) = self.levels().destructure();
        let (level, parent) = levels.current().destructure();
        let borrow = level.get_component::<C>()?;
        Some(Comp::new(root, parent, borrow))
    }

    pub fn fetch<'e, B>(&'e self) -> <B as BorrowRef<'_>>::Output
    where
        B: BorrowRef<'e>,
    {
        B::borrow(&self)
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

struct Data(u32);

fn main() {
    let mut ecs = TestEcs::new();

    ecs.insert_global(Data(5));

    let system_fn = |_source: &mut TestEcs| {
        println!("Hello from System!");
    };
    let mut system = system_fn.into_system();
    system.run(&mut ecs);

    let system_fn = |entity: Global<Data>| {
        println!("Hello from Global Data System - {}!", entity.0);
    };
    let mut system: System = system_fn.into_system();
    system.run(&mut ecs);

    let system_fn = |entity: Global<Data>, e2: Unique<Data>| {
        println!(
            "Hello from Global + Unique System - {} + {}!",
            entity.0, e2.0
        );
    };
    let mut system: System = system_fn.into_system();
    system.run(&mut ecs);
}
