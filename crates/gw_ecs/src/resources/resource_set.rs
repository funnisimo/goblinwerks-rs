use super::{Mut, ReadOnly, Ref, ResourceTypeId};
use super::{Resource, Resources, UnsafeResources};
use atomic_refcell::{AtomicRef, AtomicRefMut};

/// Trait which is implemented for tuples of resources and singular resources. This abstracts
/// fetching resources to allow for ergonomic fetching.
///
/// # Example:
/// ```
/// struct TypeA(usize);
/// struct TypeB(usize);
///
/// # use legion::*;
/// # use legion::systems::ResourceSet;
/// let mut resources = Resources::default();
/// resources.insert(TypeA(55));
/// resources.insert(TypeB(12));
///
/// {
///     let (a, mut b) = <(Read<TypeA>, Write<TypeB>)>::fetch_mut(&mut resources);
///     assert_ne!(a.0, b.0);
///     b.0 = a.0;
/// }
///
/// {
///     let (a, b) = <(Read<TypeA>, Read<TypeB>)>::fetch(&resources);
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

    /// Fetches all defined resources.
    fn fetch_mut(resources: &'a mut Resources) -> Self::Result {
        // safe because mutable borrow ensures exclusivity
        unsafe { Self::fetch_unchecked(&resources.internal) }
    }

    /// Fetches all defined resources.
    fn fetch(resources: &'a Resources) -> Self::Result
    where
        Self: ReadOnly,
    {
        unsafe { Self::fetch_unchecked(&resources.internal) }
    }
}

impl<'a> ResourceSet<'a> for () {
    type Result = ();

    unsafe fn fetch_unchecked(_: &UnsafeResources) -> Self::Result {}
}

impl<'a, T: Resource> ResourceSet<'a> for Ref<T> {
    type Result = AtomicRef<'a, T>;

    unsafe fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<T>();
        resources
            .get(&type_id)
            .map(|x| x.get::<T>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
    }
}

impl<'a, T: Resource> ResourceSet<'a> for Mut<T> {
    type Result = AtomicRefMut<'a, T>;

    unsafe fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
        let type_id = &ResourceTypeId::of::<T>();
        resources
            .get(&type_id)
            .map(|x| x.get_mut::<T>())
            .unwrap_or_else(|| panic_nonexistent_resource(type_id))
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
