use crate::{Resource, World};

macro_rules! fetch_panic {
    () => {{
        panic!(
            "\
            Tried to fetch resource of type `{resource_name_simple}`[^1] from the `World`, but \
            the resource does not exist.\n\
\n\
            You may ensure the resource exists through one of the following methods:\n\
\n\
            * Inserting it when the world is created: `world.insert(..)`.\n\
            * If the resource implements `Default`, include it in a system's `SystemData`, \
              and ensure the system is registered in the dispatcher.\n\
            * If the resource does not implement `Default`, insert it in the world during \
              `System::setup`.\n\
\n\
            [^1]: Full type name: `{resource_name_full}`\
            ",
            resource_name_simple = tynm::type_name::<T>(),
            resource_name_full = std::any::type_name::<T>(),
        )
    }};
}

/// A trait for doing the setup for SystemData.
pub trait SetupHandler<T>: Sized {
    /// Sets up `World` for fetching `T`.
    fn setup(world: &mut World);
}

/// A `SetupHandler` that simply uses the default implementation.
pub struct DefaultIfMissing;

impl<T> SetupHandler<T> for DefaultIfMissing
where
    T: Default + Resource,
{
    fn setup(world: &mut World) {
        world.resources.entry().or_insert_with(T::default);
    }
}

/// A setup handler that simply does nothing and thus will cause a panic on
/// fetching.
///
/// A typedef called `ReadExpect` exists, so you usually don't use this type
/// directly.
pub struct PanicIfMissing;

impl<T> SetupHandler<T> for PanicIfMissing
where
    T: Resource,
{
    fn setup(_: &mut World) {}
}

/// A `SetupHandler` that simply does nothing.
impl<T> SetupHandler<T> for ()
where
    T: Resource,
{
    fn setup(_world: &mut World) {}
}
