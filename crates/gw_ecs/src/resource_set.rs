use crate::refcell::{AtomicRef, AtomicRef2, AtomicRefMut, AtomicRefMut2};
use crate::resource::{Resource, ResourceTypeId};
use crate::resources::{Resources, UnsafeResources};
use crate::{Level, LevelMut, LevelRef, Levels, Res, ResMut};

/// Trait which is implemented for tuples of resources and singular resources. This abstracts
/// fetching resources to allow for ergonomic fetching.
///
/// # Example:
/// ```
/// struct TypeA(usize);
/// struct TypeB(usize);
///
/// # use gw_ecs::*;
/// # use gw_ecs::ResourceSet;
/// let mut resources = Resources::default();
/// resources.insert(TypeA(55));
/// resources.insert(TypeB(12));
///
/// {
///     let (a, mut b) = <(Res<TypeA>, ResMut<TypeB>)>::fetch(&resources);
///     assert_ne!(a.0, b.0);
///     b.0 = a.0;
/// }
///
/// {
///     let (a, b) = <(Res<TypeA>, Res<TypeB>)>::fetch(&resources);
///     assert_eq!(a.0, b.0);
/// }
/// ```
pub trait ResourceSet<'a> {
    /// The resource reference returned during a fetch.
    type Result: 'a;

    /// Fetches all defined resources, without checking mutability.
    ///
    /// # Safety
    /// It is up to the end user to validate proper mutability rules across the resources being accessed.
    unsafe fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result;

    // /// Fetches all defined resources.
    // fn fetch_mut(resources: &'a mut Resources) -> Self::Result {
    //     // safe because mutable borrow ensures exclusivity
    //     unsafe { Self::fetch_unchecked(&resources.internal) }
    // }

    /// Fetches all defined resources.
    fn fetch(resources: &'a Resources) -> Self::Result
// where
    //     Self: ReadOnly,
    {
        unsafe { Self::fetch_unchecked(&resources.internal) }
    }
}

impl<'a> ResourceSet<'a> for () {
    type Result = ();

    unsafe fn fetch_unchecked(_: &UnsafeResources) -> Self::Result {}
}

impl<'a, T: Resource> ResourceSet<'a> for Res<T> {
    type Result = AtomicRef<'a, T>;

    unsafe fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<T>();
        resources
            .get(&type_id)
            .map(|x| x.get::<T>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
    }
}

impl<'a, T: Resource> ResourceSet<'a> for ResMut<T> {
    type Result = AtomicRefMut<'a, T>;

    unsafe fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<T>();
        resources
            .get(&type_id)
            .map(|x| x.get_mut::<T>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
    }
}

impl<'a> ResourceSet<'a> for LevelRef {
    type Result = AtomicRef2<'a, Level>;

    unsafe fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        let levels_ref = resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id));

        let (levels, all_borrow) = levels_ref.destructure();
        let (inner, borrow) = levels.current().destructure();
        AtomicRef2::new(inner, all_borrow, borrow)
    }
}

impl<'a> ResourceSet<'a> for LevelMut {
    type Result = AtomicRefMut2<'a, Level>;

    unsafe fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<Levels>();
        let levels_ref = resources
            .get(&type_id)
            .map(|x| x.get::<Levels>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id));

        let (levels, all_borrow) = levels_ref.destructure();
        let (inner, borrow) = levels.current_mut().destructure();
        AtomicRefMut2::new(inner, all_borrow, borrow)
    }
}

fn panic_nonexistent_resource(type_id: &ResourceTypeId) -> ! {
    #[cfg(debug_assertions)]
    panic!("resource {} does not exist", type_id.name);
    #[cfg(not(debug_assertions))]
    panic!("some resource does not exist");
}

macro_rules! resource_tuple {
    ($head_ty:ident) => {
        impl_resource_tuple!($head_ty);
    };
    ($head_ty:ident, $( $tail_ty:ident ),*) => (
        impl_resource_tuple!($head_ty, $( $tail_ty ),*);
        resource_tuple!($( $tail_ty ),*);
    );
}

macro_rules! impl_resource_tuple {
    ( $( $ty: ident ),* ) => {
        #[allow(unused_parens, non_snake_case)]
        impl<'a, $( $ty: ResourceSet<'a> ),*> ResourceSet<'a> for ($( $ty, )*)
        {
            type Result = ($( $ty::Result, )*);

            unsafe fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
                ($( $ty::fetch_unchecked(resources), )*)
            }
        }
    };
}

#[cfg(feature = "extended-tuple-impls")]
resource_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

#[cfg(not(feature = "extended-tuple-impls"))]
resource_tuple!(A, B, C, D, E, F, G, H);
