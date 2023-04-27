use super::Fetch;
use super::ReadOnly;
use crate::refcell::{AtomicBorrowRef, AtomicRef, AtomicRefMut};
use crate::storage::SparseEntry;
use crate::storage::SparseSet;
use crate::Entity;
use crate::{Component, Ecs};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a level's unique value
pub struct Comp<'b, T>
where
    T: Component,
{
    _levels: AtomicBorrowRef<'b>,
    _level: AtomicBorrowRef<'b>,
    pub(crate) borrow: AtomicRef<'b, SparseSet<T>>,
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

    pub fn iter(&self) -> CompIter<'_, T> {
        CompIter::new(self)
    }
}

pub struct CompIter<'b, T>
where
    T: Component,
{
    inner: Option<&'b [SparseEntry<T>]>,
}

impl<'b, T> CompIter<'b, T>
where
    T: Component,
{
    fn new(comp: &'b Comp<'b, T>) -> Self {
        CompIter {
            inner: Some(comp.borrow.as_slice()),
        }
    }
}

impl<'b, T> Iterator for CompIter<'b, T>
where
    T: Component,
{
    type Item = &'b T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.take() {
                Some(slice) => match slice {
                    [] => return None,
                    [_, ..] => {
                        let (head, tail) = slice.split_at(1);
                        self.inner.replace(tail);
                        match &head[0] {
                            SparseEntry::Empty => {}
                            SparseEntry::Used(entity, data) => {
                                if entity.is_alive() {
                                    return Some(data);
                                }
                            }
                        }
                    }
                },
                None => return None,
            }
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

// impl<T> MaybeBorrowed for Comp<'_, T>
// where
//     T: Component,
// {
//     type Output<'a> = Comp<'a, T>;
// }

impl<T> Fetch for Comp<'_, T>
where
    T: Component,
{
    type Output<'a> = Comp<'a, T>;
    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        ecs.get_component::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

// impl<T> MaybeBorrowed for Option<Comp<'_, T>>
// where
//     T: Component,
// {
//     type Output<'a> = Option<Comp<'a, T>>;
// }

// impl<T> Fetch for Option<Comp<'_, T>>
// where
//     T: Component,
// {
//     type Output<'a> = Option<Comp<'a, T>>;
//     fn fetch(ecs: &Ecs) -> Self::Output<'_> {
//         ecs.get_component::<T>()
//     }
// }

// pub type TryComp<'a, T> = Option<Comp<'a, T>>;

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

/// Reference to a global value

pub struct CompMut<'b, T>
where
    T: Component,
{
    _levels: AtomicBorrowRef<'b>,
    _level: AtomicBorrowRef<'b>,
    pub(crate) borrow: AtomicRefMut<'b, SparseSet<T>>,
}

impl<'b, T> CompMut<'b, T>
where
    T: Component,
{
    pub(crate) fn new(
        levels: AtomicBorrowRef<'b>,
        level: AtomicBorrowRef<'b>,
        borrow: AtomicRefMut<'b, SparseSet<T>>,
    ) -> Self {
        CompMut {
            _levels: levels,
            _level: level,
            borrow,
        }
    }

    // pub fn iter_mut(&mut self) -> CompIterMut<'_, T> {
    //     CompIterMut::new(self)
    // }
}

pub struct CompIterMut<'b, T>
where
    T: Component,
{
    inner: Option<&'b mut [SparseEntry<T>]>,
}

impl<'b, T> CompIterMut<'b, T>
where
    T: Component,
{
    fn new(comp: &'b mut CompMut<'b, T>) -> Self {
        CompIterMut {
            inner: Some(comp.borrow.as_mut_slice()),
        }
    }
}

impl<'b, T> Iterator for CompIterMut<'b, T>
where
    T: Component,
{
    type Item = &'b mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.take() {
                Some(slice) => match slice {
                    [] => return None,
                    [_, ..] => {
                        let (head, tail) = slice.split_at_mut(1);
                        self.inner.replace(tail);
                        match &mut head[0] {
                            SparseEntry::Empty => {}
                            SparseEntry::Used(entity, data) => {
                                if entity.is_alive() {
                                    return Some(data);
                                }
                            }
                        }
                    }
                },
                None => return None,
            }
        }
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

impl<'b, T> Deref for CompMut<'b, T>
where
    T: Component,
{
    type Target = SparseSet<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.borrow.deref()
    }
}

impl<'b, T> DerefMut for CompMut<'b, T>
where
    T: Component,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut SparseSet<T> {
        self.borrow.deref_mut()
    }
}

impl<'b, T> AsRef<SparseSet<T>> for CompMut<'b, T>
where
    T: Component,
{
    #[inline]
    fn as_ref(&self) -> &SparseSet<T> {
        self.borrow.as_ref()
    }
}

impl<'b, T> AsMut<SparseSet<T>> for CompMut<'b, T>
where
    T: Component,
{
    fn as_mut(&mut self) -> &mut SparseSet<T> {
        self.borrow.as_mut()
    }
}

// impl<T> MaybeBorrowed for CompMut<'_, T>
// where
//     T: Component,
// {
//     type Output<'a> = CompMut<'a, T>;
// }

impl<T> Fetch for CompMut<'_, T>
where
    T: Component,
{
    type Output<'a> = CompMut<'a, T>;
    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        ecs.get_component_mut::<T>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////

// impl<T> MaybeBorrowed for Option<CompMut<'_, T>>
// where
//     T: Component,
// {
//     type Output<'a> = Option<CompMut<'a, T>>;
// }

// impl<T> Fetch for Option<CompMut<'_, T>>
// where
//     T: Component,
// {
//     type Output<'a> = Option<CompMut<'a, T>>;

//     fn fetch(ecs: &Ecs) -> Self::Output<'_> {
//         ecs.get_component_mut::<T>()
//     }
// }

// pub type TryCompMut<'a, T> = Option<CompMut<'a, T>>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::Ecs;

    struct Age(u32);
    struct Rings(u32);

    #[test]
    fn comp_mut_basic() {
        let mut ecs = Ecs::new();

        ecs.register_component::<Age>();

        let a = ecs.spawn((Age(20),));
        let b = ecs.spawn((Age(21),));
        let c = ecs.spawn_empty();

        let mut ages = <CompMut<Age>>::fetch(&ecs);

        for age in ages.iter_mut() {
            age.0 += 1;
        }

        let mut count = 0;
        for age in ages.iter() {
            println!("age - {}", age.0);
            count += 1;
        }
        assert_eq!(count, 2);

        let age_a = ages.get(a).unwrap();
        assert_eq!(age_a.0, 21);

        let age_b = ages.get(b).unwrap();
        assert_eq!(age_b.0, 22);

        ages.remove(a);
        assert!(ages.get(a).is_none());

        ages.insert(c, Age(50));
        let age_c = ages.get(c).unwrap();
        assert_eq!(age_c.0, 50);
    }

    #[test]
    fn multi_mut() {
        let mut ecs = Ecs::new();

        ecs.register_component::<Age>();
        ecs.register_component::<Rings>();

        let _a = ecs.spawn((Age(20), Rings(1)));
        let _b = ecs.spawn((Age(21), Rings(2)));
        let _c = ecs.spawn_empty();

        let (ages, rings) = <(CompMut<Age>, CompMut<Rings>)>::fetch(&ecs);

        assert_eq!(ages.iter().count(), 2);
        assert_eq!(rings.iter().count(), 2);
    }
}
