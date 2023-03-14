use std::marker::PhantomData;

/// A marker trait which marks types which only perform data reads.
#[doc(hidden)]
pub unsafe trait ReadOnly {}

unsafe impl<T> ReadOnly for &T {}
unsafe impl<T> ReadOnly for Option<&T> {}

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
        unsafe impl<$( $ty: ReadOnly ),*> ReadOnly for ($( $ty, )*) {}
    }
}

#[cfg(feature = "extended-tuple-impls")]
view_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

#[cfg(not(feature = "extended-tuple-impls"))]
view_tuple!(A, B, C, D, E, F, G, H);

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct Ref<T>(PhantomData<*const T>);

impl<T> Default for Ref<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

unsafe impl<T> Send for Ref<T> {}
unsafe impl<T: Sync> Sync for Ref<T> {}
unsafe impl<T> ReadOnly for Ref<T> {}

/// Reads a mutable single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct Mut<T>(PhantomData<*const T>);

impl<T> Default for Mut<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

unsafe impl<T> Send for Mut<T> {}
unsafe impl<T: Sync> Sync for Mut<T> {}
