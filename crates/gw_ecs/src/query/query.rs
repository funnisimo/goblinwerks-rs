// pub trait Join {
//     fn join(&self) -> Query;
// }

use super::FilterTuple;
use crate::entity::Entities;
use crate::entity::EntityIter;
use crate::Ecs;
use crate::Fetch;
use crate::Unique;

// pub struct Query<'a, C, F>
// where
//     C: JoinSource,
//     F: FilterTuple,
// {
//     comps: <C as Fetch>::Output<'a>,
//     filter: F,
//     entities: Unique<'a, Entities>,
// }

// impl<'a, C, F> Query<'a, C, F>
// where
//     C: JoinSource,
//     F: FilterTuple,
// {
//     fn new(entities: Unique<'a, Entities>, comps: <C as Fetch>::Output<'a>, filter: F) -> Self {
//         Query {
//             comps,
//             filter,
//             entities,
//         }
//     }

//     pub fn iter(&'a self) -> QueryIter<'a, C, F> {
//         QueryIter::new(self)
//     }
// }

// impl<C, F> Fetch for Query<'_, C, F>
// where
//     C: JoinSource,
//     F: FilterTuple,
// {
//     type Output<'a> = Query<'a, C, F>;

//     fn fetch(ecs: &Ecs) -> Self::Output<'_> {
//         let comps = C::fetch(ecs);
//         let filter = F::fetch(ecs);
//         let entities = Unique::<Entities>::fetch(ecs);
//         Query::new(entities, comps, filter)
//     }
// }

// pub struct QueryIter<'a, C, F>
// where
//     C: JoinSource,
//     F: FilterTuple,
// {
//     query: &'a Query<'a, C, F>,
//     iter: EntityIter<'a>,
// }

// impl<'a, C, F> QueryIter<'a, C, F>
// where
//     C: JoinSource,
//     F: FilterTuple,
// {
//     fn new(query: &'a Query<'a, C, F>) -> Self {
//         let iter = query.entities.iter();
//         QueryIter { query, iter }
//     }
// }

// impl<'a, C, F> Iterator for QueryIter<'a, C, F>
// where
//     C: JoinSource,
//     F: FilterTuple,
// {
//     type Item = <C as JoinSource>::Item<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             match self.iter.next() {
//                 None => return None,
//                 Some(entity) => {
//                     if self.query.filter.filter(*entity) {
//                         self.query.comps.get(entity)
//                     }
//                 }
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::Ecs;

//     struct Age(u32);

//     #[test]
//     fn basic() {
//         let mut ecs = Ecs::new();

//         ecs.register_component::<Age>();

//         {
//             let mut level = ecs.level_mut();
//             level.spawn((Age(10),));
//             level.spawn((Age(20),));
//         }

//         let query = Query::<(Entity, &Age), ()>::fetch(&ecs);

//         for (entity, age) in query.iter() {}
//     }
// }
