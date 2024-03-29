use std::{
    any::{Any, TypeId},
    cmp::Ordering,
    fmt::Debug,
    hash::Hash,
    // marker::PhantomData,
};

// SOURCE - SHRED + LEGION

/// The id of a [`Resource`], which simply wraps a type id and a "dynamic ID".
/// The "dynamic ID" is usually just left `0`, and, unless such documentation
/// says otherwise, other libraries will assume that it is always `0`; non-zero
/// IDs are only used for special resource types that are specifically defined
/// in a more dynamic way, such that resource types can essentially be created
/// at run time, without having different static types.
///
/// [`Resource`]: trait.Resource.html
#[derive(Clone, Eq, Ord)]
pub struct ResourceId {
    type_id: TypeId,
    dynamic_id: u64,
    name: &'static str,
}

impl ResourceId {
    /// Creates a new resource id from a given type.
    #[inline]
    pub fn new<T: 'static>() -> Self {
        ResourceId::new_with_dynamic_id::<T>(0)
    }

    #[inline]
    pub fn of<T: 'static>() -> Self {
        ResourceId::new_with_dynamic_id::<T>(0)
    }

    /// Create a new resource id from a raw type ID.
    #[inline]
    pub fn from_type_id(type_id: TypeId) -> Self {
        ResourceId::from_type_id_and_dynamic_id(type_id, 0, "Unknown")
    }

    /// Creates a new resource id from a given type and a `dynamic_id`.
    ///
    /// This is usually not what you want (unless you're implementing scripting
    /// with `shred` or some similar mechanism to define resources at run-time).
    ///
    /// Creating resource IDs with a `dynamic_id` unequal to `0` is only
    /// recommended for special types that are specifically defined for
    /// scripting; most libraries will just assume that resources are
    /// identified only by their type.
    #[inline]
    pub fn new_with_dynamic_id<T: 'static>(dynamic_id: u64) -> Self {
        ResourceId::from_type_id_and_dynamic_id(
            TypeId::of::<T>(),
            dynamic_id,
            std::any::type_name::<T>(),
        )
    }

    /// Create a new resource id from a raw type ID and a "dynamic ID" (see type
    /// documentation).
    #[inline]
    pub(crate) fn from_type_id_and_dynamic_id(
        type_id: TypeId,
        dynamic_id: u64,
        name: &'static str,
    ) -> Self {
        ResourceId {
            type_id,
            dynamic_id,
            name,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn assert_same_type_id<R: Resource>(&self) {
        let res_id0 = ResourceId::new::<R>();
        assert_eq!(
            res_id0.type_id, self.type_id,
            "Passed a `ResourceId` with a wrong type ID"
        );
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

// #[derive(Clone, Eq, Ord, Hash, PartialEq, PartialOrd)]

impl Hash for ResourceId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        state.write_u64(self.dynamic_id);
    }
}

impl PartialEq for ResourceId {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.dynamic_id == other.dynamic_id
    }
}

impl PartialOrd for ResourceId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.type_id.partial_cmp(&other.type_id) {
            None => None,
            Some(Ordering::Equal) => self.dynamic_id.partial_cmp(&other.dynamic_id),
            x => x,
        }
    }
}

impl Debug for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dynamic_id != 0 {
            write!(f, "<{}[{}]>", self.name, self.dynamic_id)
        } else {
            write!(f, "<{}>", self.name)
        }
    }
}

// pub struct ResourceIdOf<T> {
//     pub(crate) id: ResourceId,
//     phantom: PhantomData<T>,
// }

// impl<T> Default for ResourceIdOf<T>
// where
//     T: 'static,
// {
//     fn default() -> Self {
//         ResourceIdOf {
//             id: ResourceId::of::<T>(),
//             phantom: PhantomData,
//         }
//     }
// }

// /// A resource is a data slot which lives in the `World` can only be accessed
// /// according to Rust's typical borrowing model (one writer xor multiple
// /// readers).
// #[cfg(feature = "parallel")]
// pub trait Resource: Any + Send + Sync + 'static {}

/// A resource is a data slot which lives in the `World` can only be accessed
/// according to Rust's typical borrowing model (one writer xor multiple
/// readers).
// #[cfg(not(feature = "parallel"))]
pub trait Resource: Any + 'static {}

// #[cfg(feature = "parallel")]
// impl<T> Resource for T where T: Any  {}
// #[cfg(not(feature = "parallel"))]
impl<T> Resource for T where T: Any + 'static {}

// /// A resource is a data slot which lives in the `World` can only be accessed
// /// according to Rust's typical borrowing model (one writer xor multiple
// /// readers).
// // #[cfg(not(feature = "parallel"))]
// pub trait NonSend: Any + 'static {}

// // #[cfg(feature = "parallel")]
// // impl<T> NonSend for T where T: Any + Send + Sync {}
// // #[cfg(not(feature = "parallel"))]
// impl<T> NonSend for T where T: Any + 'static {}

// Code is based on https://github.com/chris-morgan/mopa
// with the macro inlined for `Resource`. License files can be found in the
// directory of this source file, see COPYRIGHT, LICENSE-APACHE and
// LICENSE-MIT.
impl dyn Resource {
    /// Returns the boxed value if it is of type `T`, or `Err(Self)` if it
    /// isn't.
    #[inline]
    pub fn downcast<T: Resource>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        if self.is::<T>() {
            unsafe { Ok(self.downcast_unchecked()) }
        } else {
            Err(self)
        }
    }

    /// Returns the boxed value, blindly assuming it to be of type `T`.
    ///
    /// # Safety
    ///
    /// If you are not *absolutely certain* of `T`, you *must not* call this.
    /// Using anything other than the correct type `T` for this `Resource`
    /// will result in UB.
    #[inline]
    pub unsafe fn downcast_unchecked<T: Resource>(self: Box<Self>) -> Box<T> {
        Box::from_raw(Box::into_raw(self) as *mut T)
    }

    /// Returns true if the boxed type is the same as `T`
    #[inline]
    pub fn is<T: Resource>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    /// Returns some reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    #[inline]
    pub fn downcast_ref<T: Resource>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(self.downcast_ref_unchecked()) }
        } else {
            Option::None
        }
    }

    /// Returns a reference to the boxed value, blindly assuming it to be of
    /// type `T`.
    ///
    /// # Safety
    ///
    /// If you are not *absolutely certain* of `T`, you *must not* call this.
    /// Using anything other than the correct type `T` for this `Resource`
    /// will result in UB.
    #[inline]
    pub unsafe fn downcast_ref_unchecked<T: Resource>(&self) -> &T {
        &*(self as *const Self as *const T)
    }

    /// Returns some mutable reference to the boxed value if it is of type `T`,
    /// or `None` if it isn't.
    #[inline]
    pub fn downcast_mut<T: Resource>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(self.downcast_mut_unchecked()) }
        } else {
            Option::None
        }
    }

    /// Returns a mutable reference to the boxed value, blindly assuming it to
    /// be of type `T`.
    ///
    /// # Safety
    ///
    /// If you are not *absolutely certain* of `T`, you *must not* call this.
    /// Using anything other than the correct type `T` for this `Resource`
    /// will result in UB.
    #[inline]
    pub unsafe fn downcast_mut_unchecked<T: Resource>(&mut self) -> &mut T {
        &mut *(self as *mut Self as *mut T)
    }
}
