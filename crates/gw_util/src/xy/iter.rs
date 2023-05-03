// use std::slice::Iter;

#[cfg(feature = "try_trait_v2")]
use std::ops::Try;
use std::slice::Iter;
#[cfg(feature = "trusted_random_access")]
use std::{iter::TrustedRandomAccessNoCoerce, ops::Try};

use crate::point::Point;

/// An iterator that yields the current count and the element during iteration.
///
/// This `struct` is created by the [`enumerate`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`enumerate`]: Iterator::enumerate
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EnumerateXY<I> {
    iter: I,
    width: usize,
    count: usize,
}
impl<I> EnumerateXY<I> {
    pub fn new(iter: I, width: usize, height: usize) -> EnumerateXY<I> {
        let _ = height;
        EnumerateXY {
            iter,
            count: 0,
            width,
        }
    }
}

impl<I> Iterator for EnumerateXY<I>
where
    I: Iterator,
{
    type Item = (Point, <I as Iterator>::Item);

    /// # Overflow Behavior
    ///
    /// The method does no guarding against overflows, so enumerating more than
    /// `usize::MAX` elements either produces the wrong result or panics. If
    /// debug assertions are enabled, a panic is guaranteed.
    ///
    /// # Panics
    ///
    /// Might panic if the index of the element overflows a `usize`.
    #[inline]
    fn next(&mut self) -> Option<(Point, <I as Iterator>::Item)> {
        let a = self.iter.next()?;
        let x = self.count % self.width;
        let y = self.count / self.width;
        self.count += 1;
        Some((Point::new(x as i32, y as i32), a))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<(Point, I::Item)> {
        let a = self.iter.nth(n)?;
        let i = self.count + n;
        let x = i % self.width;
        let y = i / self.width;
        self.count = i + 1;
        Some((Point::new(x as i32, y as i32), a))
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[cfg(feature = "try_trait_v2")]
    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R
    where
        Self: Sized,
        Fold: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        #[inline]
        fn enumerate_xy<'a, T, Acc, R>(
            count: &'a mut usize,
            width: &'a usize,
            mut fold: impl FnMut(Acc, (usize, T)) -> R + 'a,
        ) -> impl FnMut(Acc, T) -> R + 'a {
            // #[rustc_inherit_overflow_checks]
            move |acc, item| {
                let x = *count % width;
                let y = *count / width;
                let point = Point::new(x, y);
                let acc = fold(acc, (point, item));
                *count += 1;
                acc
            }
        }

        self.iter
            .try_fold(init, enumerate_xy(&mut self.count, &self.width, fold))
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        #[inline]
        fn enumerate_xy<T, Acc>(
            mut count: usize,
            width: usize,
            mut fold: impl FnMut(Acc, (Point, T)) -> Acc,
        ) -> impl FnMut(Acc, T) -> Acc {
            // #[rustc_inherit_overflow_checks]
            move |acc, item| {
                let x = count % width;
                let y = count / width;
                let point = Point::new(x as i32, y as i32);
                let acc = fold(acc, (point, item));
                count += 1;
                acc
            }
        }

        self.iter
            .fold(init, enumerate_xy(self.count, self.width, fold))
    }

    #[cfg(feature = "iter_advance_by")]
    #[inline]
    // #[rustc_inherit_overflow_checks]
    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        match self.iter.advance_by(n) {
            ret @ Ok(_) => {
                self.count += n;
                ret
            }
            ret @ Err(advanced) => {
                self.count += advanced;
                ret
            }
        }
    }

    #[cfg(feature = "trusted_random_access")]
    // #[rustc_inherit_overflow_checks]
    #[inline]
    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> <Self as Iterator>::Item
    where
        Self: TrustedRandomAccessNoCoerce,
    {
        // SAFETY: the caller must uphold the contract for
        // `Iterator::__iterator_get_unchecked`.
        let value = unsafe { try_get_unchecked(&mut self.iter, idx) };

        let count = self.count + idx;
        let x = count % self.width;
        let y = count % self.height;
        let point = Point::new(x as i32, y as i32);
        (point, value)
    }
}

pub trait IterXY<T> {
    fn iter_xy(&self, width: usize, height: usize) -> EnumerateXY<Iter<T>>;
}

impl<T> IterXY<T> for Vec<T> {
    fn iter_xy(&self, width: usize, height: usize) -> EnumerateXY<Iter<T>> {
        EnumerateXY::new(self.iter(), width, height)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut iter = data.iter_xy(3, 3);

        assert_eq!(iter.next().unwrap(), (Point::new(0, 0), &1));
        assert_eq!(iter.next().unwrap(), (Point::new(1, 0), &2));
        assert_eq!(iter.next().unwrap(), (Point::new(2, 0), &3));
        assert_eq!(iter.next().unwrap(), (Point::new(0, 1), &4));
        assert_eq!(iter.next().unwrap(), (Point::new(1, 1), &5));
        assert_eq!(iter.next().unwrap(), (Point::new(2, 1), &6));
        assert_eq!(iter.next().unwrap(), (Point::new(0, 2), &7));
        assert_eq!(iter.next().unwrap(), (Point::new(1, 2), &8));
        assert_eq!(iter.next().unwrap(), (Point::new(2, 2), &9));
        assert_eq!(iter.next(), None);
    }
}
