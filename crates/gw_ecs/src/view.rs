use crate::atomic_refcell::{AtomicBorrowRef, AtomicRef, AtomicRefMut};
use crate::AtomicBorrowRefMut;
use crate::{resource::Resource, Ecs, Level, Levels};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

// pub trait Fetch<'data> {
//     type Item;

//     fn fetch(ecs: &'data Ecs) -> Self::Item;
// }

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct Res<T>(PhantomData<*const T>);

impl<T> Default for Res<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

// impl<'data, T: Resource> Fetch<'data> for Res<T> {
//     type Item = AtomicRef<'data, T>;

//     fn fetch(ecs: &'data Ecs) -> Self::Item {
//         ecs.res::<T>().expect("Failed to fetch resource.")
//     }
// }

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct ResMut<T>(PhantomData<*const T>);

impl<T> Default for ResMut<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

// impl<'data, T: Resource> Fetch<'data> for ResMut<T> {
//     type Item = AtomicRefMut<'data, T>;

//     fn fetch(ecs: &'data Ecs) -> Self::Item {
//         ecs.res_mut::<T>().expect("Failed to find resource!")
//     }
// }

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct LevelsView;

impl Default for LevelsView {
    fn default() -> Self {
        Self
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

// impl<'data> Fetch<'data> for LevelsView {
//     type Item = AtomicRef<'data, Levels>;

//     fn fetch(ecs: &'data Ecs) -> Self::Item {
//         ecs.res::<Levels>().expect("Failed to fetch levels.")
//     }
// }

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct LevelsMut;

impl Default for LevelsMut {
    fn default() -> Self {
        Self
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

// impl<'data> Fetch<'data> for LevelsMut {
//     type Item = AtomicRefMut<'data, Levels>;

//     fn fetch(ecs: &'data Ecs) -> Self::Item {
//         ecs.res_mut::<Levels>()
//             .expect("Failed to fetch Levels as mut.")
//     }
// }

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct LevelRef;

impl Default for LevelRef {
    fn default() -> Self {
        Self
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct LevelMut;

impl Default for LevelMut {
    fn default() -> Self {
        Self
    }
}

// impl<'data> Fetch<'data> for Current {
//     type Item = Ref<'data, Level>;

//     fn fetch(ecs: &'data Ecs) -> Self::Item {
//         let levels_borrow = ecs.res::<Levels>().expect("Failed to fetch Levels as mut.");
//         let (levels, all_borrow) = levels_borrow.destructure();
//         let (level, borrow) = levels.current().destructure();

//         Ref {
//             inner: level,
//             all_borrow,
//             borrow,
//         }
//     }
// }

/// Shared reference to a component.
pub struct Ref<'a, T> {
    inner: &'a T,
    all_borrow: AtomicBorrowRef<'a>,
    borrow: AtomicBorrowRef<'a>,
}

impl<'a, T> Ref<'a, T> {
    pub(crate) fn new(
        inner: &'a T,
        all_borrow: AtomicBorrowRef<'a>,
        borrow: AtomicBorrowRef<'a>,
    ) -> Self {
        Ref {
            inner,
            all_borrow,
            borrow,
        }
    }

    /// Makes a new [`Ref`].
    ///
    /// This is an associated function that needs to be used as `Ref::map(...)`. A method would interfere with methods of the same name used through Deref.
    #[inline]
    pub fn map<U, F: FnOnce(&'a T) -> &'a U>(orig: Self, f: F) -> Ref<'a, U> {
        Ref {
            inner: f(orig.inner),
            all_borrow: orig.all_borrow,
            borrow: orig.borrow,
        }
    }
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> AsRef<T> for Ref<'a, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.inner
    }
}

/// Shared reference to a component.
pub struct RefMut<'a, T> {
    inner: &'a mut T,
    all_borrow: AtomicBorrowRef<'a>,
    borrow: AtomicBorrowRefMut<'a>,
}

impl<'a, T> RefMut<'a, T> {
    pub(crate) fn new(
        inner: &'a mut T,
        all_borrow: AtomicBorrowRef<'a>,
        borrow: AtomicBorrowRefMut<'a>,
    ) -> Self {
        RefMut {
            inner,
            all_borrow,
            borrow,
        }
    }

    /// Makes a new [`Ref`].
    ///
    /// This is an associated function that needs to be used as `Ref::map(...)`. A method would interfere with methods of the same name used through Deref.
    #[inline]
    pub fn map<U, F: FnOnce(&'a mut T) -> &'a mut U>(orig: Self, f: F) -> RefMut<'a, U> {
        RefMut {
            inner: f(orig.inner),
            all_borrow: orig.all_borrow,
            borrow: orig.borrow,
        }
    }
}

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, T> AsRef<T> for RefMut<'a, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.inner
    }
}

impl<'a, T> AsMut<T> for RefMut<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner
    }
}
