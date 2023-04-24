// pub trait Join {
//     fn join(&self) -> Query;
// }

use std::marker::PhantomData;

use super::View;
use crate::{entity::EntityIter, Ecs, Entities, Unique};

pub struct Query<T>
where
    T: View,
{
    _phantom: PhantomData<T>,
}

impl<T> Query<T>
where
    T: View,
{
    pub(super) fn new() -> Self {
        Query {
            _phantom: PhantomData::default(),
        }
    }

    pub fn iter<'a>(&self, ecs: &'a Ecs) -> QueryIter<'a, T> {
        let entities = ecs.get_unique::<Entities>().unwrap();
        let data = T::data(ecs);
        QueryIter::new(entities, data)
    }
}

// ITER

pub struct QueryIter<'a, T>
where
    T: View,
{
    data: <T as View>::Data<'a>,
    entities: Unique<'a, Entities>,
    index: usize,
}

impl<'a, T> QueryIter<'a, T>
where
    T: View,
{
    pub fn new(entities: Unique<'a, Entities>, data: <T as View>::Data<'a>) -> Self {
        QueryIter {
            data,
            entities,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for QueryIter<'a, T>
where
    T: View,
{
    type Item = T::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.entities.len() {
                return None;
            }
            match self.entities.data.get(self.index) {
                None => return None,
                Some(e) => {
                    self.index += 1;
                    if e.is_alive() {
                        return T::get(&self.data, *e);
                    }
                }
            }
        }
    }
}

//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////

// pub struct QueryMut<'a, T>
// where
//     T: ViewMut<'a>,
// {
//     view: T,
//     _phantom: PhantomData<&'a T>,
// }

// impl<'a, T> QueryMut<'a, T>
// where
//     T: ViewMut<'a>,
// {
//     pub(super) fn new(view: T) -> Self {
//         QueryMut {
//             view,
//             _phantom: PhantomData::default(),
//         }
//     }

//     pub fn iter(&mut self, entities: &'a Unique<Entities>) -> QueryMutIter<'a, T> {
//         QueryMutIter::new(&mut self.view, entities)
//     }
// }

// pub struct QueryMutIter<'a, T>
// where
//     T: ViewMut<'a>,
// {
//     view: &'a mut T,
//     entities: &'a Unique<'a, Entities>,
//     iter: EntityIter<'a>,
// }

// impl<'a, T> QueryMutIter<'a, T>
// where
//     T: ViewMut<'a>,
// {
//     pub fn new(view: &'a mut T, entities: &'a Unique<'a, Entities>) -> Self {
//         let iter = entities.iter();
//         QueryMutIter {
//             view,
//             entities,
//             iter,
//         }
//     }
// }

// impl<'a, T> Iterator for QueryMutIter<'a, T>
// where
//     T: ViewMut<'a>,
// {
//     type Item = T::Item<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.iter.next() {
//             None => None,
//             Some(entity) => self.view.get_mut(entity),
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::fetch::Fetch;
    use crate::Ecs;
    use crate::Entities;
    use crate::{Comp, CompMut};

    struct Age(u32);

    // #[test]
    // fn basic() {
    //     let mut ecs = Ecs::new();

    //     ecs.register_component::<Age>();

    //     {
    //         let mut level = ecs.level_mut();
    //         level.spawn((Age(10),));
    //         level.spawn((Age(20),));
    //     }

    //     let (entities, ages) = <(Entities, Comp<Age>)>::fetch(&ecs);

    //     let query = (&entities, &ages).join();

    //     let mut total_age = 0;
    //     for (_entity, age) in query.iter(&entities) {
    //         total_age += age.0;
    //     }

    //     assert_eq!(total_age, 30);
    // }

    #[test]
    fn basic() {
        let mut ecs = Ecs::new();

        ecs.register_component::<Age>();

        ecs.spawn((Age(10),));
        ecs.spawn((Age(20),));

        let query = <(Entities, Comp<Age>)>::join();

        let mut total_age = 0;
        for (_entity, age) in query.iter(&ecs) {
            total_age += age.0;
        }

        assert_eq!(total_age, 30);
    }

    // #[test]
    // fn basic_mut() {
    //     let mut ecs = Ecs::new();

    //     ecs.register_component::<Age>();

    //     ecs.spawn((Age(10),));
    //     ecs.spawn((Age(20),));

    //     let mut query = <(Entities, CompMut<Age>)>::join(&ecs);
    //     // let filter = <(WithComp<Address>,)>::filter(&ecs);

    //     let mut total_age = 0;
    //     for (_entity, mut age) in query.iter() {
    //         age.0 += 1;
    //         total_age += age.0;
    //     }

    //     assert_eq!(total_age, 32);
    // }
}
