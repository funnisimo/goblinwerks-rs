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

pub(crate) use fetch_panic;

/// A trait for doing the setup for SystemData.
pub trait SetupHandler<T>: Sized {
    /// Sets up `World` for fetching `T`.
    fn setup(world: &mut World);
}

/// A `SetupHandler` that simply does nothing.
pub struct NoSetup;

/// A `SetupHandler` that simply does nothing.
impl<T> SetupHandler<T> for NoSetup
where
    T: Resource,
{
    fn setup(_world: &mut World) {}
}

/// A `SetupHandler` that adds a default constructor.
pub struct SetupDefault;

impl<T> SetupHandler<T> for SetupDefault
where
    T: Default + Resource,
{
    fn setup(world: &mut World) {
        world.resources.ensure(T::default);
    }
}

/// A `SetupHandler` that adds a default constructor.
/// This is the most common usage for setup
impl<T> SetupHandler<T> for ()
where
    T: Resource + Default,
{
    fn setup(world: &mut World) {
        world.resources.ensure(T::default);
    }
}
