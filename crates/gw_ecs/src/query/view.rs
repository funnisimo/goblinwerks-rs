use crate::CompMut;
use crate::Component;
use crate::Ecs;
use crate::Entity;
use crate::EntityStore;
use crate::Fetch;
use crate::{Comp, Unique};

use super::Query;

pub trait View: Fetch {
    type Data<'a>;
    type Item<'a>;

    fn join() -> Query<Self>
    where
        Self: Sized,
    {
        Query::new()
    }

    fn data<'a>(ecs: &'a Ecs) -> Self::Data<'a>;

    fn get<'a, 'b>(data: &'a Self::Data<'b>, entity: Entity) -> Option<Self::Item<'a>>;
}

impl View for EntityStore {
    type Data<'a> = Unique<'a, EntityStore>;
    type Item<'a> = Entity;

    fn data<'a>(ecs: &'a Ecs) -> Self::Data<'a> {
        ecs.get_unique::<EntityStore>().unwrap()
    }

    fn get<'a, 'b>(data: &'a Self::Data<'b>, entity: Entity) -> Option<Self::Item<'a>> {
        match data.contains(entity) {
            true => Some(entity),
            false => None,
        }
    }
}

// COMP

impl<'c, T> View for Comp<'c, T>
where
    T: Component,
{
    type Data<'b> = Comp<'b, T>;
    type Item<'a> = &'a T;

    fn data<'a>(ecs: &'a Ecs) -> Self::Data<'a> {
        ecs.get_component::<T>().unwrap()
    }

    fn get<'a, 'b>(data: &'a Self::Data<'b>, entity: Entity) -> Option<Self::Item<'a>> {
        data.as_ref().get(entity)
    }
}

// impl<'a, T> View for Option<Comp<'a, T>>
// where
//     T: Component,
// {
//     type Data<'b> = Comp<'b, T>;
//     type Item = Option<T>;

//     fn join(ecs: &Ecs) -> Self::Data<'_> {
//         ecs.get_component::<T>().unwrap()
//     }

//     fn get(data: &Self::Data<'a>, entity: Entity) -> Option<&'a Self::Item> {
//         Some(data.borrow.get(entity))
//     }
// }

// COMP MUT

impl<'c, T> View for CompMut<'c, T>
where
    T: Component,
{
    type Data<'b> = CompMut<'b, T>;
    type Item<'b> = &'b T;

    fn data<'a>(ecs: &'a Ecs) -> Self::Data<'a> {
        ecs.get_component_mut::<T>().unwrap()
    }

    fn get<'a, 'b>(data: &'a Self::Data<'b>, entity: Entity) -> Option<Self::Item<'a>> {
        data.as_ref().get(entity)
    }
}

// impl<T> View for Option<CompMut<'_, T>>
// where
//     T: Component,
// {
//     type Data<'a> = CompMut<'a, T>;
//     type Item<'a> = Option<<Comp<'a, T> as Fetch>::Output<'a>>;

//     fn join(ecs: &Ecs) -> Self::Data<'_> {
//         ecs.get_component_mut::<T>().unwrap()
//     }

//     fn get(&self, entity: Entity) -> Option<Self::Item<'_>> {
//         self.borrow.get(entity)
//     }
// }

// TUPLES

// TODO - Just don't have this - why is it here anyway?
impl View for () {
    type Data<'a> = ();
    type Item<'a> = ();

    fn data<'a>(_ecs: &'a Ecs) -> Self::Data<'a> {
        ()
    }

    fn get<'a, 'b>(_data: &'a Self::Data<'b>, _entity: Entity) -> Option<Self::Item<'a>> {
        None
    }
}

macro_rules! impl_make_view {
    ($(($component: ident, $index: tt))+) => {

        // impl<$($component,)+> MaybeBorrowed for ($($component,)+)
        // where
        //     $($component: MaybeBorrowed,)+
        // {
        //     type Output<'a> = ($(<$component as MaybeBorrowed>::Output<'a>,)+);
        // }
        impl<$($component,)+> View for ($($component,)+)
        where
            $($component: View,)+
        {
            type Data<'a> = ($(<$component as View>::Data<'a>,)+);
            type Item<'a> = ($(<$component as View>::Item<'a>,)+);

            fn data<'a>(ecs: &'a Ecs) -> Self::Data<'a> {
                ($(<$component>::data(ecs),)+)
            }

            fn get<'a, 'b>(data: &'a Self::Data<'b>, entity: Entity) -> Option<Self::Item<'a>>
            {
                Some((
                    $(
                        match $component::get(&data.$index, entity) {
                            None => return None,
                            Some(d) => d,
                        },
                    )+
                ))
            }

        }

    }
}

macro_rules! make_view {
    ($(($component: ident, $index: tt))+; ($component1: ident, $index1: tt) $(($queue_component: ident, $queue_index: tt))*) => {
        impl_make_view![$(($component, $index))*];
        make_view![$(($component, $index))* ($component1, $index1); $(($queue_component, $queue_index))*];
    };
    ($(($component: ident, $index: tt))+;) => {
        impl_make_view![$(($component, $index))*];
    }
}

make_view![(A, 0); (B, 1) (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)];
