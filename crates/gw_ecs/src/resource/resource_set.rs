use super::{ResourceTypeId, Resources, UnsafeResources};
use crate::{view::ReadOnly, Ecs};

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
    fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result;

    /// Fetches all defined resources.
    fn fetch_from(resources: &'a Resources) -> Self::Result
    where
        Self: ReadOnly,
    {
        Self::fetch_unchecked(&resources.internal)
    }

    /// Fetches all defined resources.
    fn fetch_mut_from(resources: &'a Resources) -> Self::Result {
        Self::fetch_unchecked(&resources.internal)
    }
}

impl<'a> ResourceSet<'a> for () {
    type Result = ();

    fn fetch_unchecked(_: &UnsafeResources) -> Self::Result {}
}

pub(crate) fn panic_nonexistent_resource(type_id: &ResourceTypeId) -> ! {
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

            fn fetch_unchecked(resources: &'a UnsafeResources) -> Self::Result {
                ($( $ty::fetch_unchecked(resources), )*)
            }
        }
    };
}

#[cfg(feature = "extended-tuple-impls")]
resource_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

#[cfg(not(feature = "extended-tuple-impls"))]
resource_tuple!(A, B, C, D, E, F, G, H);

macro_rules! view_tuple {
    ($head_ty:ident) => {
        impl_view_tuple!($head_ty);
    };
    ($head_ty:ident, $( $tail_ty:ident ),*) => (
        impl_view_tuple!($head_ty, $( $tail_ty ),*);
        view_tuple!($( $tail_ty ),*);
    );
}

macro_rules! impl_view_tuple {
    ( $( $ty: ident ),* ) => {
        impl<$( $ty: ReadOnly ),*> ReadOnly for ($( $ty, )*) {}

    //     impl<$( $ty: DefaultFilter ),*> DefaultFilter for ($( $ty, )*) {
    //         type Filter = EntityFilterTuple<
    //             And<($( <$ty::Filter as EntityFilter>::Layout, )*)>,
    //             And<($( <$ty::Filter as EntityFilter>::Dynamic, )*)>
    //         >;
    //     }

    //     impl<$( $ty: IntoView ),*> IntoView for ($( $ty, )*) {
    //         type View = ($( $ty::View, )*);
    //     }

    //     impl<'a, $( $ty: View<'a> + 'a ),*> View<'a> for ($( $ty, )*) {
    //         type Element = <Self::Fetch as IntoIndexableIter>::Item;
    //         type Fetch = MultiFetch<'a, ($( $ty::Fetch, )*)>;
    //         type Iter = MapInto<Zip<($( $ty::Iter, )*)>, Option<MultiFetch<'a, ($( $ty::Fetch, )*)>>>;
    //         type Read = Vec<ComponentTypeId>;
    //         type Write = Vec<ComponentTypeId>;

    //         unsafe fn fetch(
    //             components: &'a Components,
    //             archetypes: &'a [Archetype],
    //             query: QueryResult<'a>,
    //         ) -> Self::Iter {
    //             MapInto::new(
    //                 multizip(
    //                     (
    //                         $( $ty::fetch(components, archetypes, query.clone()), )*
    //                     )
    //                 )
    //             )
    //         }

    //         paste::item! {
    //             fn validate() {
    //                 #![allow(non_snake_case)]
    //                 $( let [<$ty _reads>] = $ty::reads_types(); )*
    //                 $( let [<$ty _writes>] = $ty::writes_types(); )*
    //                 let reads = [$( [<$ty _reads>].as_ref(), )*];
    //                 let writes = [$( [<$ty _writes>].as_ref(), )*];

    //                 for (i, writes) in writes.iter().enumerate() {
    //                     for (j, other_reads) in reads.iter().enumerate() {
    //                         if i == j { continue; }
    //                         for w in writes.iter() {
    //                             assert!(!other_reads.iter().any(|x| x == w));
    //                         }
    //                     }
    //                 }
    //             }

    //             fn validate_access(access: &ComponentAccess) -> bool {
    //                 $( $ty::validate_access(access) )&&*
    //             }

    //             fn reads_types() -> Self::Read {
    //                 #![allow(non_snake_case)]
    //                 let types = std::iter::empty();
    //                 $( let [<$ty _reads>] = $ty::reads_types(); )*
    //                 $( let types = types.chain([<$ty _reads>].as_ref().iter()); )*
    //                 types.copied().collect()
    //             }

    //             fn writes_types() -> Self::Write {
    //                 #![allow(non_snake_case)]
    //                 let types = std::iter::empty();
    //                 $( let [<$ty _writes>] = $ty::writes_types(); )*
    //                 $( let types = types.chain([<$ty _writes>].as_ref().iter()); )*
    //                 types.copied().collect()
    //             }

    //             fn requires_permissions() -> Permissions<ComponentTypeId> {
    //                 let mut permissions = Permissions::new();
    //                 $( permissions.add($ty::requires_permissions()); )*
    //                 permissions
    //             }
    //         }

    //         fn reads<Comp: Component>() -> bool {
    //             $(
    //                 $ty::reads::<Comp>()
    //             )||*
    //         }

    //         fn writes<Comp: Component>() -> bool {
    //             $(
    //                 $ty::writes::<Comp>()
    //             )||*
    //         }
    //     }

    //     impl<'a, $( $ty: Fetch ),*> crate::internals::iter::map::From<($( Option<$ty>, )*)>
    //         for Option<MultiFetch<'a, ($( $ty, )*)>>
    //     {
    //         fn from(value: ($( Option<$ty>, )*)) -> Self {
    //             #[allow(non_snake_case)]
    //             let ($( $ty, )*) = value;
    //             let valid = $( $ty.is_some() )&*;
    //             if valid {
    //                 Some(MultiFetch {
    //                     fetches: ($( $ty.unwrap(), )*),
    //                     _phantom: PhantomData
    //                 })
    //             } else {
    //                 None
    //             }
    //         }
    //     }

    //     impl<'a, $( $ty: Fetch ),*> IntoIndexableIter for MultiFetch<'a, ($( $ty, )*)> {
    //         type IntoIter = IndexedIter<($( $ty::IntoIter, )*)>;
    //         type Item = <Self::IntoIter as Iterator>::Item;

    //         fn into_indexable_iter(self) -> Self::IntoIter {
    //             #[allow(non_snake_case)]
    //             let ($( $ty, )*) = self.fetches;
    //             IndexedIter::new(($( $ty.into_indexable_iter(), )*))
    //         }
    //     }

    //     impl<'a, $( $ty: Fetch ),*> IntoIterator for MultiFetch<'a, ($( $ty, )*)> {
    //         type IntoIter = <Self as IntoIndexableIter>::IntoIter;
    //         type Item = <Self as IntoIndexableIter>::Item;

    //         fn into_iter(self) -> Self::IntoIter {
    //             self.into_indexable_iter()
    //         }
    //     }

    //     unsafe impl<'a, $( $ty: ReadOnlyFetch),*> ReadOnlyFetch for MultiFetch<'a, ($( $ty, )*)>
    //     {
    //         fn get_components(&self) -> Self::Data {
    //             #[allow(non_snake_case)]
    //             let ($( $ty, )*) = &self.fetches;
    //             (($( $ty.get_components(), )*))
    //         }
    //     }

    //     impl<'a, $( $ty: Fetch ),*> Fetch for MultiFetch<'a, ($( $ty, )*)> {
    //         type Data = ($( $ty::Data, )*);

    //         #[inline]
    //         fn into_components(self) -> Self::Data {
    //             #[allow(non_snake_case)]
    //             let ($( $ty, )*) = self.fetches;
    //             ($( $ty.into_components(), )*)
    //         }

    //         #[inline]
    //         fn find<Comp: 'static>(&self) -> Option<&[Comp]> {
    //             #[allow(non_snake_case)]
    //             let ($( $ty, )*) = &self.fetches;
    //             let mut result = None;
    //             $(
    //                 result = result.or_else(|| $ty.find());
    //             )*
    //             result
    //         }

    //         #[inline]
    //         fn find_mut<Comp: 'static>(&mut self) -> Option<&mut [Comp]> {
    //             #[allow(non_snake_case)]
    //             let ($( $ty, )*) = &mut self.fetches;
    //             let mut result = None;
    //             $(
    //                 result = result.or_else(move || $ty.find_mut());
    //             )*
    //             result
    //         }

    //         #[inline]
    //         fn version<Comp: Component>(&self) -> Option<u64> {
    //             #[allow(non_snake_case)]
    //             let ($( $ty, )*) = &self.fetches;
    //             let mut result = None;
    //             $(
    //                 result = result.or_else(|| $ty.version::<Comp>());
    //             )*
    //             result
    //         }

    //         #[inline]
    //         fn accepted(&mut self) {
    //             #[allow(non_snake_case)]
    //             let ($( $ty, )*) = &mut self.fetches;
    //             $( $ty.accepted(); )*
    //         }
    //     }
    };
}

#[cfg(feature = "extended-tuple-impls")]
view_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

#[cfg(not(feature = "extended-tuple-impls"))]
view_tuple!(A, B, C, D, E, F, G, H);
