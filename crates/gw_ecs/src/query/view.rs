use crate::{
    entity::Entities,
    fetch::{Comp, CompMut, Unique},
    Component, Entity, Fetch, TryComp, TryCompMut,
};

pub trait View {
    type Item<'a>
    where
        Self: 'a;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>>;
}

impl<'a, C> View for Comp<'a, C>
where
    C: Component,
{
    type Item<'b> = &'b C where Self: 'b;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        let comp = self.borrow.as_ref();
        comp.get(entity)
    }
}

impl<'a, C> View for CompMut<'a, C>
where
    C: Component,
{
    type Item<'b> = &'b mut C where Self: 'b;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        let comp = self.borrow.as_mut();
        comp.get_mut(entity)
    }
}

impl<'a, C> View for TryComp<'a, C>
where
    C: Component,
{
    type Item<'b> = Option<&'b C> where Self: 'b;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        match self.as_mut() {
            None => None,
            Some(comp) => Some(comp.get(entity)),
        }
    }
}

impl<'a, C> View for TryCompMut<'a, C>
where
    C: Component,
{
    type Item<'b> = Option<&'b mut C> where Self: 'b;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        match self.as_mut() {
            None => None,
            Some(comp) => Some(comp.get_mut(entity)),
        }
    }
}

impl<A> View for (A,)
where
    A: View,
{
    type Item<'a> = (A::Item<'a>,) where Self: 'a;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        match self.0.get(entity) {
            None => None,
            Some(d) => Some((d,)),
        }
    }
}

impl<A, B> View for (A, B)
where
    A: View,
    B: View,
{
    type Item<'a> = (A::Item<'a>, B::Item<'a>) where Self: 'a;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        Some((
            match self.0.get(entity) {
                None => return None,
                Some(d) => d,
            },
            match self.1.get(entity) {
                None => return None,
                Some(d) => d,
            },
        ))
    }
}

impl<A, B, C> View for (A, B, C)
where
    A: View,
    B: View,
    C: View,
{
    type Item<'a> = (A::Item<'a>, B::Item<'a>, C::Item<'a>) where Self:'a;

    fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
        Some((
            match self.0.get(entity) {
                None => return None,
                Some(d) => d,
            },
            match self.1.get(entity) {
                None => return None,
                Some(d) => d,
            },
            match self.2.get(entity) {
                None => return None,
                Some(d) => d,
            },
        ))
    }
}

pub trait IntoView {
    // type View: for<'a> View<'a> + 'static;
    type View<'a>: View;
}

impl<T: Component> IntoView for &T {
    type View<'a> = Comp<'a, T>;
}

impl<T: Component> IntoView for &mut T {
    type View<'a> = CompMut<'a, T>;
}

impl<T: Component> IntoView for Option<&T> {
    type View<'a> = TryComp<'a, T>;
}

impl<T: Component> IntoView for Option<&mut T> {
    type View<'a> = TryCompMut<'a, T>;
}

impl<A> IntoView for (A,)
where
    A: IntoView,
    A: Fetch,
{
    type View<'a> = (A::View<'a>,);
}

impl<A, B> IntoView for (A, B)
where
    A: IntoView,
    A: Fetch,
    B: IntoView,
    B: Fetch,
{
    type View<'a> = (A::View<'a>, B::View<'a>);
}

impl<A, B, C> IntoView for (A, B, C)
where
    A: IntoView,
    A: Fetch,
    B: IntoView,
    B: Fetch,
    C: IntoView,
    C: Fetch,
{
    type View<'a> = (A::View<'a>, B::View<'a>, C::View<'a>);
}

// pub trait JoinSource: Fetch {
//     type Item<'a>
//     where
//         Self: 'a;

//     fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>>;
// }

// impl Fetch for Entity {
//     type Output<'a> = Unique<'a, Entities>;

//     fn fetch(ecs: &crate::Ecs) -> Self::Output<'_> {
//         ecs.get_unique::<Entities>().unwrap()
//     }
// }

// impl JoinSource for Entity {
//     type Item<'a> = Entity;

//     fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
//         Some(entity)
//     }
// }

// impl<C> JoinSource for &C
// where
//     C: Component,
// {
//     type Item<'a> = &'a C where Self: 'a;

//     fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
//         self.borrow.get(entity)
//     }
// }

// impl<C> JoinSource for CompMut<'_, C>
// where
//     C: Component,
// {
//     type Item<'a> = &'a mut C where Self: 'a;

//     fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
//         self.borrow.get_mut(entity)
//     }
// }

// impl<A> JoinSource for (A,)
// where
//     A: JoinSource,
// {
//     type Item<'a> = (A::Item<'a>,) where
//         A: 'a;

//     fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
//         Some((match self.0.get(entity) {
//             None => return None,
//             Some(v) => v,
//         },))
//     }
// }

// impl<A, B> JoinSource for (A, B)
// where
//     A: JoinSource,
//     B: JoinSource,
// {
//     type Item<'a> = (A::Item<'a>,B::Item<'a>) where
//         A: 'a, B: 'a;

//     fn get(&mut self, entity: Entity) -> Option<Self::Item<'_>> {
//         Some((
//             match self.0.get(entity) {
//                 None => return None,
//                 Some(v) => v,
//             },
//             match self.1.get(entity) {
//                 None => return None,
//                 Some(v) => v,
//             },
//         ))
//     }
// }
