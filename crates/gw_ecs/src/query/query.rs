// pub trait Join {
//     fn join(&self) -> Query;
// }

use crate::entity::Entities;
use crate::fetch::Fetch;
use crate::storage::SparseSet;
use crate::Comp;
use crate::Component;
use crate::Entity;
use crate::Unique;

pub struct Query<T>
where
    T: JoinSource,
{
    view: T,
}

impl<T> Query<T>
where
    T: JoinSource,
{
    fn new(view: T) -> Self {
        Query { view }
    }

    fn iter(&self) -> impl Iterator<Item = T::Item<'_>> {
        QueryIter::new(self)
    }
}

pub struct QueryIter<'a, T>
where
    T: JoinSource,
{
    view: &'a T,
    entities: Box<dyn Iterator<Item = &'a Entity> + 'a>,
}

impl<'a, T> QueryIter<'a, T>
where
    T: JoinSource,
{
    pub fn new(query: &'a Query<T>) -> Self {
        QueryIter {
            view: &query.view,
            entities: query.view.entities().unwrap(),
        }
    }
}

impl<'a, T> Iterator for QueryIter<'a, T>
where
    T: JoinSource,
{
    type Item = T::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.entities.next() {
            None => None,
            Some(entity) => self.view.get(entity),
        }
    }
}

pub trait JoinSource {
    type Item<'a>
    where
        Self: 'a;

    fn join(self) -> Query<Self>
    where
        Self: Sized,
    {
        Query::new(self)
    }

    fn get(&self, entity: &Entity) -> Option<Self::Item<'_>>;
    fn entities(&self) -> Option<Box<dyn Iterator<Item = &'_ Entity> + '_>>;
}

impl<A, B> JoinSource for (A, B)
where
    A: JoinSource,
    B: JoinSource,
{
    type Item<'a>
         = (A::Item<'a>, B::Item<'a>) where
            Self: 'a;

    fn get(&self, entity: &Entity) -> Option<Self::Item<'_>> {
        Some((
            match self.0.get(entity) {
                None => return None,
                Some(v) => v,
            },
            match self.1.get(entity) {
                None => return None,
                Some(v) => v,
            },
        ))
    }

    fn entities(&self) -> Option<Box<dyn Iterator<Item = &'_ Entity> + '_>> {
        if let Some(a) = self.0.entities() {
            return Some(a);
        }
        self.1.entities()
    }
}

impl<A, B, C> JoinSource for (A, B, C)
where
    A: JoinSource,
    B: JoinSource,
    C: JoinSource,
{
    type Item<'a>
         = (A::Item<'a>, B::Item<'a>, C::Item<'a>) where
            Self: 'a;

    fn get(&self, entity: &Entity) -> Option<Self::Item<'_>> {
        Some((
            match self.0.get(entity) {
                None => return None,
                Some(v) => v,
            },
            match self.1.get(entity) {
                None => return None,
                Some(v) => v,
            },
            match self.2.get(entity) {
                None => return None,
                Some(v) => v,
            },
        ))
    }

    fn entities(&self) -> Option<Box<dyn Iterator<Item = &'_ Entity> + '_>> {
        if let Some(a) = self.0.entities() {
            return Some(a);
        }
        if let Some(a) = self.1.entities() {
            return Some(a);
        }
        self.2.entities()
    }
}

impl JoinSource for &Unique<'_, Entities> {
    type Item<'a> = Entity where Self: 'a;

    fn get(&self, entity: &Entity) -> Option<Self::Item<'_>> {
        match self.as_ref().contains(*entity) {
            true => Some(*entity),
            false => None,
        }
    }
    fn entities<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a Entity> + 'a>> {
        Some(Box::new(self.as_ref().iter()))
    }
}

impl<T> JoinSource for &Comp<'_, T>
where
    T: Component,
{
    type Item<'a> = &'a T where Self: 'a;

    fn get(&self, entity: &Entity) -> Option<Self::Item<'_>> {
        self.borrow.get(*entity)
    }

    fn entities(&self) -> Option<Box<dyn Iterator<Item = &'_ Entity> + '_>> {
        Some(Box::new(self.as_ref().entities()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fetch::Fetch;
    use crate::Comp;
    use crate::Ecs;
    use crate::Entities;

    struct Age(u32);

    #[test]
    fn basic() {
        let mut ecs = Ecs::new();

        ecs.register_component::<Age>();

        {
            let mut level = ecs.level_mut();
            level.spawn((Age(10),));
            level.spawn((Age(20),));
        }

        let (entities, ages) = <(Entities, Comp<Age>)>::fetch(&ecs);

        let query = (&entities, &ages).join();

        let mut total_age = 0;
        for (_entity, age) in query.iter() {
            total_age += age.0;
        }

        assert_eq!(total_age, 30);
    }
}
