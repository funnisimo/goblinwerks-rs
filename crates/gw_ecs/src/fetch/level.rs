use super::ReadOnly;
use super::{Fetch, Global, GlobalMut, MaybeBorrowed};
use crate::{
    refcell::{AtomicBorrowRef, AtomicRef, AtomicRefMut},
    Ecs, Level, Levels,
};
use std::ops::{Deref, DerefMut};

pub type LevelsRef<'a> = Global<'a, Levels>;
pub type LevelsMut<'a> = GlobalMut<'a, Levels>;

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

impl MaybeBorrowed for LevelRef<'_> {
    type Output<'a> = LevelRef<'a>;
}

impl Fetch for LevelRef<'_> {
    fn fetch(ecs: &Ecs) -> LevelRef<'_> {
        ecs.level()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

pub struct LevelMut<'a> {
    _parent: AtomicBorrowRef<'a>,
    borrow: AtomicRefMut<'a, Level>,
}

impl<'a> LevelMut<'a> {
    pub(crate) fn new(parent: AtomicBorrowRef<'a>, borrow: AtomicRefMut<'a, Level>) -> Self {
        LevelMut {
            _parent: parent,
            borrow,
        }
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'a> Deref for LevelMut<'a> {
    type Target = Level;

    #[inline]
    fn deref(&self) -> &Level {
        self.borrow.deref()
    }
}

impl<'a> DerefMut for LevelMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow.deref_mut()
    }
}

impl<'a> AsRef<Level> for LevelMut<'a> {
    #[inline]
    fn as_ref(&self) -> &Level {
        self.borrow.as_ref()
    }
}

impl<'a> AsMut<Level> for LevelMut<'a> {
    fn as_mut(&mut self) -> &mut Level {
        self.borrow.as_mut()
    }
}

impl MaybeBorrowed for LevelMut<'_> {
    type Output<'a> = LevelMut<'a>;
}

impl Fetch for LevelMut<'_> {
    fn fetch(ecs: &Ecs) -> LevelMut<'_> {
        ecs.level_mut()
    }
}
