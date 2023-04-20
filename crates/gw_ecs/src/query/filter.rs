use crate::{Comp, Component, Ecs, Entity};

pub trait Filterable {
    fn filter(&self, entity: Entity) -> bool;
}

pub trait FilterTuple {
    type Output<'a>: Filterable;
    fn fetch<'a>(ecs: &'a Ecs) -> Self::Output<'a>;
}

impl FilterTuple for () {
    type Output<'a> = ();
    fn fetch(_ecs: &Ecs) -> Self::Output<'_> {
        ()
    }
}

impl Filterable for () {
    fn filter(&self, _entity: Entity) -> bool {
        false
    }
}

impl<A> FilterTuple for (A,)
where
    A: FilterTuple,
{
    type Output<'a> = (A::Output<'a>,);
    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        (A::fetch(ecs),)
    }
}

impl<A> Filterable for (A,)
where
    A: Filterable,
{
    fn filter(&self, entity: Entity) -> bool {
        self.0.filter(entity)
    }
}

impl<A, B> FilterTuple for (A, B)
where
    A: FilterTuple,
    B: FilterTuple,
{
    type Output<'a> = (A::Output<'a>, B::Output<'a>);
    fn fetch(ecs: &Ecs) -> Self::Output<'_> {
        (A::fetch(ecs), B::fetch(ecs))
    }
}

impl<A, B> Filterable for (A, B)
where
    A: Filterable,
    B: Filterable,
{
    fn filter(&self, entity: Entity) -> bool {
        self.0.filter(entity) && self.1.filter(entity)
    }
}

impl<C> FilterTuple for With<'_, C>
where
    C: Component,
{
    type Output<'a> = With<'a, C>;

    fn fetch<'a>(ecs: &'a Ecs) -> Self::Output<'a> {
        With::new(ecs)
    }
}

impl<C> FilterTuple for Without<'_, C>
where
    C: Component,
{
    type Output<'a> = Without<'a, C>;

    fn fetch<'a>(ecs: &'a Ecs) -> Self::Output<'a> {
        Without::new(ecs)
    }
}

pub struct Filter<'a, T>
where
    T: FilterTuple,
{
    tuple: T::Output<'a>,
}

impl<'a, T> Filter<'a, T>
where
    T: FilterTuple,
{
    pub(super) fn new(ecs: &'a Ecs) -> Self {
        Filter {
            tuple: T::fetch(ecs),
        }
    }

    pub fn filter(&self, entity: Entity) -> bool {
        self.tuple.filter(entity)
    }
}

pub struct With<'a, C>
where
    C: Component,
{
    data: Comp<'a, C>,
}

impl<'a, C> With<'a, C>
where
    C: Component,
{
    fn new(ecs: &'a Ecs) -> Self {
        With {
            data: ecs.get_component::<C>().unwrap(),
        }
    }
}

impl<C> Filterable for With<'_, C>
where
    C: Component,
{
    fn filter(&self, entity: Entity) -> bool {
        self.data.contains(entity)
    }
}

pub struct Without<'a, C>
where
    C: Component,
{
    data: Comp<'a, C>,
}

impl<'a, C> Without<'a, C>
where
    C: Component,
{
    fn new(ecs: &'a Ecs) -> Self {
        Without {
            data: ecs.get_component::<C>().unwrap(),
        }
    }
}

impl<C> Filterable for Without<'_, C>
where
    C: Component,
{
    fn filter(&self, entity: Entity) -> bool {
        !self.data.contains(entity)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Age(u32);

    #[test]
    fn basic_with() {
        let mut ecs = Ecs::new();
        ecs.register_component::<Age>();

        let without_entity = ecs.spawn_empty();
        let with_entity = ecs.spawn((Age(20),));

        let filter = <Filter<With<Age>>>::new(&ecs);

        assert_eq!(filter.filter(without_entity), false);
        assert_eq!(filter.filter(with_entity), true);
    }

    #[test]
    fn basic_without() {
        let mut ecs = Ecs::new();
        ecs.register_component::<Age>();

        let without_entity = ecs.spawn_empty();
        let with_entity = ecs.spawn((Age(20),));

        let filter = <Filter<Without<Age>>>::new(&ecs);

        assert_eq!(filter.filter(without_entity), true);
        assert_eq!(filter.filter(with_entity), false);
    }

    #[test]
    fn basic_tuple() {
        let mut ecs = Ecs::new();
        ecs.register_component::<Age>();

        let without_entity = ecs.spawn_empty();
        let with_entity = ecs.spawn((Age(20),));

        let filter = <Filter<(With<Age>, With<Age>)>>::new(&ecs);

        assert_eq!(filter.filter(without_entity), false);
        assert_eq!(filter.filter(with_entity), true);
    }

    #[test]
    fn fail_tuple() {
        let mut ecs = Ecs::new();
        ecs.register_component::<Age>();

        let without_entity = ecs.spawn_empty();
        let with_entity = ecs.spawn((Age(20),));

        let filter = <Filter<(With<Age>, Without<Age>)>>::new(&ecs);

        assert_eq!(filter.filter(without_entity), false);
        assert_eq!(filter.filter(with_entity), false);
    }
}
