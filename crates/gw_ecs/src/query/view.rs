use crate::{
    fetch::{Comp, CompMut},
    Component, Ecs, Entity,
};

pub struct View<'a, T>
where
    T: ViewTuple,
{
    data: T::Output<'a>,
}

impl<'a, T> View<'a, T>
where
    T: ViewTuple,
{
    fn new(ecs: &'a Ecs) -> Self {
        View {
            data: T::fetch(ecs),
        }
    }

    fn get(
        &mut self,
        entity: Entity,
    ) -> Option<<<T as ViewTuple>::Output<'a> as ViewSource>::Item<'_>> {
        self.data.get(entity)
    }
}

pub struct EntityView;

//////////////////////////////////////////////////////////
// VIEW SOURCE
//////////////////////////////////////////////////////////

pub trait ViewSource {
    type Item<'a>
    where
        Self: 'a;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>>;
}

impl<T> ViewSource for Comp<'_, T>
where
    T: Component,
{
    type Item<'a> = &'a T where Self: 'a;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        self.borrow.get(entity)
    }
}

impl<T> ViewSource for CompMut<'_, T>
where
    T: Component,
{
    type Item<'a> = &'a mut T where Self: 'a;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        self.borrow.get_mut(entity)
    }
}

impl ViewSource for EntityView {
    type Item<'a> = Entity;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        Some(entity)
    }
}

impl ViewSource for () {
    type Item<'a> = ();

    fn get(&mut self, _entity: Entity) -> Option<Self::Item<'_>> {
        None
    }
}

impl<A> ViewSource for (A,)
where
    A: ViewSource,
{
    type Item<'a> = (A::Item<'a>,) where Self: 'a;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        Some((match self.0.get(entity) {
            None => return None,
            Some(v) => v,
        },))
    }
}

impl<A, B> ViewSource for (A, B)
where
    A: ViewSource,
    B: ViewSource,
{
    type Item<'a> = (A::Item<'a>, B::Item<'a>) where Self: 'a;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
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
}

impl<A, B, C> ViewSource for (A, B, C)
where
    A: ViewSource,
    B: ViewSource,
    C: ViewSource,
{
    type Item<'a> = (A::Item<'a>, B::Item<'a>, C::Item<'a>) where Self: 'a;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
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
}
//////////////////////////////////////////////////////////
// VIEW TUPLE
//////////////////////////////////////////////////////////

pub trait ViewTuple {
    // type Output: for<'a> View<'a> + 'static;
    type Output<'a>: ViewSource;

    fn fetch(ecs: &Ecs) -> Self::Output<'_>;
}

impl<T: Component> ViewTuple for &T {
    type Output<'a> = Comp<'a, T>;

    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        ecs.get_component::<T>().unwrap()
    }
}

impl<T: Component> ViewTuple for &mut T {
    type Output<'a> = CompMut<'a, T>;

    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        ecs.get_component_mut::<T>().unwrap()
    }
}

// impl<T: Component> ViewTuple for Option<&T> {
//     type Output<'a> = TryComp<'a, T>;
// }

// impl<T: Component> ViewTuple for Option<&mut T> {
//     type Output<'a> = TryCompMut<'a, T>;
// }

impl<A> ViewTuple for (A,)
where
    A: ViewTuple,
{
    type Output<'a> = (A::Output<'a>,);

    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        (A::fetch(ecs),)
    }
}

impl<A, B> ViewTuple for (A, B)
where
    A: ViewTuple,
    B: ViewTuple,
{
    type Output<'a> = (A::Output<'a>, B::Output<'a>);

    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        (A::fetch(ecs), B::fetch(ecs))
    }
}

impl<A, B, C> ViewTuple for (A, B, C)
where
    A: ViewTuple,
    B: ViewTuple,
    C: ViewTuple,
{
    type Output<'a> = (A::Output<'a>, B::Output<'a>, C::Output<'a>);

    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        (A::fetch(ecs), B::fetch(ecs), C::fetch(ecs))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Ecs;

    #[derive(Debug, PartialEq)]
    struct Age(u32);

    #[test]
    fn basic_comp() {
        let mut ecs = Ecs::new();
        ecs.register_component::<Age>();

        let a = ecs.spawn((Age(20),));
        // let b = ecs.spawn((Age(10),));
        let c = ecs.spawn_empty();

        let mut view = View::<&Age>::new(&ecs);

        assert_eq!(*view.get(a).unwrap(), Age(20));
        assert_eq!(view.get(c), None);
    }

    #[test]
    fn basic_comp_mut() {
        let mut ecs = Ecs::new();
        ecs.register_component::<Age>();

        let a = ecs.spawn((Age(20),));
        // let b = ecs.spawn((Age(10),));
        let c = ecs.spawn_empty();

        let mut view = View::<&mut Age>::new(&ecs);

        view.get(a).unwrap().0 = 21;

        assert_eq!(*view.get(a).unwrap(), Age(21));
        assert_eq!(view.get(c), None);
    }

    #[test]
    fn basic_tuple() {
        let mut ecs = Ecs::new();
        ecs.register_component::<Age>();

        let a = ecs.spawn((Age(20),));
        // let b = ecs.spawn((Age(10),));
        let c = ecs.spawn_empty();

        let mut view = View::<(&Age,)>::new(&ecs);

        assert_eq!(*view.get(a).unwrap().0, Age(20));
        assert_eq!(view.get(c), None);
    }

    #[test]
    fn basic_tuple_2() {
        let mut ecs = Ecs::new();
        ecs.register_component::<Age>();

        let a = ecs.spawn((Age(20),));
        // let b = ecs.spawn((Age(10),));
        let c = ecs.spawn_empty();

        let mut view = View::<(&Age, &Age)>::new(&ecs);

        assert_eq!(*view.get(a).unwrap().0, Age(20));
        assert_eq!(*view.get(a).unwrap().1, Age(20));
        assert_eq!(view.get(c), None);
    }
}
