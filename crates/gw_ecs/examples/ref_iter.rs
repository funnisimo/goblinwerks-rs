#![allow(dead_code, unused_imports)]

use std::cell::{Ref, RefCell};
use std::marker::PhantomData;

struct Foo {
    bar: Vec<u32>,
}

struct Iter<'a, T> {
    inner: Option<&'a [T]>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.take() {
            Some(slice) => match slice {
                [] => None,
                [_, ..] => {
                    let (head, tail) = slice.split_at(1);
                    self.inner.replace(tail);
                    Some(&head[0])
                }
            },
            None => None,
        }
    }
}

struct IterMut<'a, T> {
    inner: Option<&'a mut [T]>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.take() {
            Some(slice) => match slice {
                [] => None,
                [_, ..] => {
                    let (head, tail) = slice.split_at_mut(1);
                    self.inner.replace(tail);
                    Some(&mut head[0])
                }
            },
            None => None,
        }
    }
}

impl Foo {
    pub fn iter(&self) -> Iter<u32> {
        Iter {
            inner: Some(&self.bar[..]),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<u32> {
        IterMut {
            inner: Some(&mut self.bar[..]),
        }
    }
}

fn main() {
    let foo = RefCell::new(Foo {
        bar: vec![0, 1, 2, 3],
    });

    {
        let borrow = foo.borrow();
        assert_eq!(
            borrow.iter().map(|i| *i + 1).collect::<Vec<u32>>(),
            vec![1, 2, 3, 4]
        );
    }

    {
        let mut borrow_mut = foo.borrow_mut();
        for item in borrow_mut.iter_mut() {
            *item += 1;
        }
        assert_eq!(
            borrow_mut.iter().map(|v| *v).collect::<Vec<u32>>(),
            vec![1, 2, 3, 4]
        );
    }
}
