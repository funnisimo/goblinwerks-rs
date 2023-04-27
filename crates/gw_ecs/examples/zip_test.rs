#![allow(dead_code, unused_imports, unused_mut)]

use std::cell::{Ref, RefCell, RefMut};
use std::convert::{AsMut, AsRef};
use std::ops::{Deref, DerefMut};
use std::slice::Iter;

struct Data<T> {
    data: Vec<T>,
}

impl<T> Data<T> {
    fn new() -> Self {
        Data { data: Vec::new() }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    // fn get_mut(&mut self, index: usize) -> Option<&mut T> {
    //     self.data.get_mut(index)
    // }

    fn iter(&self) -> DataIter<'_, T> {
        DataIter {
            inner: Some(self.data.as_slice()),
        }
    }

    fn iter_mut(&mut self) -> DataIterMut<'_, T> {
        DataIterMut {
            inner: Some(self.data.as_mut_slice()),
        }
    }
}

struct DataIter<'a, T> {
    inner: Option<&'a [T]>,
}

impl<'a, T> Iterator for DataIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.take() {
            Some(slice) => match slice {
                [] => None,
                [_, ..] => {
                    let (head, tail) = (&slice[0], &slice[1..]);
                    self.inner.replace(tail);
                    Some(head)
                }
            },
            None => None,
        }
    }
}

struct DataIterMut<'a, T> {
    inner: Option<&'a mut [T]>,
}

impl<'a, T> Iterator for DataIterMut<'a, T> {
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

struct Zip<'a, I, T, U>
where
    I: Iterator<Item = usize> + Clone,
{
    range: I,
    ref_a: &'a Data<T>,
    ref_b: &'a Data<U>,
}

impl<'a, I, T, U> Zip<'a, I, T, U>
where
    I: Iterator<Item = usize> + Clone,
{
    fn new(range: I, ref_a: &'a Ref<'a, Data<T>>, ref_b: &'a Ref<'a, Data<U>>) -> Self {
        Zip {
            range,
            ref_a: ref_a.deref(),
            ref_b: ref_b.deref(),
        }
    }

    fn iter(self) -> impl Iterator<Item = (&'a T, &'a U)> {
        self.range
            .clone()
            .filter_map(|idx| match self.ref_a.get(idx) {
                None => None,
                Some(a) => match self.ref_b.get(idx) {
                    None => None,
                    Some(b) => Some((a, b)),
                },
            })
    }
}

/////////////////////////////////////

trait DataMut<'a> {
    type Item<'b>
    where
        Self: 'a,
        Self: 'b;

    type Source<'b>
    where
        Self: 'a,
        Self: 'b;

    fn get_mut(source: Self::Source<'a>, index: usize) -> Option<Self::Item<'_>>;
}

impl<'a, T> DataMut<'a> for Ref<'a, Data<T>>
where
    T: 'static,
{
    type Source<'b> = &'b Self where Self: 'a, Self: 'b;
    type Item<'b> = &'b T where
        Self: 'a,
        Self: 'b;

    fn get_mut(source: Self::Source<'a>, index: usize) -> Option<Self::Item<'_>> {
        source.data.get(index)
    }
}

impl<'a, T> DataMut<'a> for RefMut<'a, Data<T>>
where
    T: 'static,
{
    type Source<'b> = &'b mut Self where Self: 'a, Self: 'b;
    type Item<'b> = &'b mut T where
        Self: 'a,
        Self: 'b;

    fn get_mut(source: Self::Source<'a>, index: usize) -> Option<Self::Item<'_>> {
        source.data.get_mut(index)
    }
}

/////////////////////////////////////

struct ZipMut<'a, I, T, U>
where
    I: Iterator<Item = usize> + Clone,
{
    range: I,
    ref_a: &'a Data<T>,
    ref_b: &'a Data<U>,
}

impl<'a, I, T, U> ZipMut<'a, I, T, U>
where
    I: Iterator<Item = usize> + Clone,
{
    fn new(range: I, ref_a: &'a Data<T>, ref_b: &'a mut Data<U>) -> Self {
        ZipMut {
            range,
            ref_a: ref_a,
            ref_b: ref_b,
        }
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&'a T, &'a U)> {
        self.range
            .clone()
            .filter_map(|idx| match self.ref_a.get(idx) {
                None => None,
                Some(a) => match self.ref_b.get(idx) {
                    None => None,
                    Some(b) => Some((a, b)),
                },
            })
    }
}

struct World {
    a: RefCell<Data<u32>>,
    b: RefCell<Data<i16>>,
    c: RefCell<Data<f32>>,
}

impl World {
    fn new() -> Self {
        World {
            a: RefCell::new(Data::new()),
            b: RefCell::new(Data::new()),
            c: RefCell::new(Data::new()),
        }
    }

    fn get_a(&self) -> Ref<Data<u32>> {
        self.a.borrow()
    }

    fn get_mut_a(&self) -> RefMut<Data<u32>> {
        self.a.borrow_mut()
    }

    fn get_b(&self) -> Ref<Data<i16>> {
        self.b.borrow()
    }

    fn get_mut_b(&self) -> RefMut<Data<i16>> {
        self.b.borrow_mut()
    }

    fn get_c(&self) -> Ref<Data<f32>> {
        self.c.borrow()
    }

    fn get_mut_c(&self) -> RefMut<Data<f32>> {
        self.c.borrow_mut()
    }
}

fn main() {
    let world = World::new();

    {
        let mut data_mut = world.get_mut_a();
        data_mut.data.push(1);
        data_mut.data.push(2);
        data_mut.data.push(3);
        data_mut.data.push(4);
    }

    {
        let mut data_mut = world.get_mut_b();
        data_mut.data.push(1);
        data_mut.data.push(2);
    }

    {
        let mut data_ref = world.get_mut_a();

        for v in data_ref.iter_mut() {
            *v += 1;
        }

        println!("{:?}", data_ref.iter().map(|v| *v).collect::<Vec<u32>>());
    }

    let data_a = world.get_a();
    let data_b = world.get_b();
    // let ref_a = data_a.deref();
    // let ref_b = data_b.deref();

    let z = Zip::new(0..4, &data_a, &data_b);

    for (a, b) in z.iter() {
        println!("zip - {}, {}", a, b);
    }

    drop(data_b);

    let mut mut_b = world.get_mut_b();

    let mut z = ZipMut::new(0..4, data_a.deref(), mut_b.deref_mut());

    for (a, b) in z.iter_mut() {
        println!("zip mut - {}, {}", a, b);
    }
    drop(z);

    // let res: Option<&mut i16> = mut_b.get_mut(0);
    // println!("res = {}", res.unwrap());

    // let res: Option<&mut i16> = do_data_mut(&data_a, 1);
    // println!("res = {}", res.unwrap());
}

fn do_data_mut<'a, D: DataMut<'a>>(
    data: <D as DataMut<'a>>::Source<'a>,
    index: usize,
) -> Option<D::Item<'_>> {
    D::get_mut(data, index)
}
