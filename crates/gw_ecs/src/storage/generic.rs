use super::{AccessMutReturn, InsertResult, ReadComp, WriteComp};
use crate::specs::world::{Component, Entity};

#[cfg(feature = "nightly")]
use super::UnprotectedStorage;
#[cfg(feature = "nightly")]
use std::ops::DerefMut;

pub struct Seal;

/// Provides generic read access to both `ReadStorage` and `WriteStorage`
pub trait GenericReadComp {
    /// The component type of the storage
    type Component: Component;

    /// Get immutable access to an `Entity`s component
    fn get(&self, entity: Entity) -> Option<&Self::Component>;

    /// Private function to seal the trait
    fn _private() -> Seal;
}

impl<'a, T> GenericReadComp for ReadComp<'a, T>
where
    T: Component,
{
    type Component = T;

    fn get(&self, entity: Entity) -> Option<&Self::Component> {
        ReadComp::get(self, entity)
    }

    fn _private() -> Seal {
        Seal
    }
}

impl<'a: 'b, 'b, T> GenericReadComp for &'b ReadComp<'a, T>
where
    T: Component,
{
    type Component = T;

    fn get(&self, entity: Entity) -> Option<&Self::Component> {
        ReadComp::get(*self, entity)
    }

    fn _private() -> Seal {
        Seal
    }
}

impl<'a, T> GenericReadComp for WriteComp<'a, T>
where
    T: Component,
{
    type Component = T;

    fn get(&self, entity: Entity) -> Option<&Self::Component> {
        WriteComp::get(self, entity)
    }

    fn _private() -> Seal {
        Seal
    }
}

impl<'a: 'b, 'b, T> GenericReadComp for &'b WriteComp<'a, T>
where
    T: Component,
{
    type Component = T;

    fn get(&self, entity: Entity) -> Option<&Self::Component> {
        WriteComp::get(*self, entity)
    }

    fn _private() -> Seal {
        Seal
    }
}

/// Provides generic write access to `WriteStorage`, both as a value and a
/// mutable reference.
pub trait GenericWriteComp {
    /// The component type of the storage
    type Component: Component;
    /// The wrapper through with mutable access of a component is performed.
    #[cfg(feature = "nightly")]
    type AccessMut<'a>: DerefMut<Target = Self::Component>
    where
        Self: 'a;

    /// Get mutable access to an `Entity`s component
    fn get_mut(&mut self, entity: Entity) -> Option<AccessMutReturn<'_, Self::Component>>;

    /// Get mutable access to an `Entity`s component. If the component does not
    /// exist, it is automatically created using `Default::default()`.
    ///
    /// Returns None if the entity is dead.
    fn get_mut_or_default(
        &mut self,
        entity: Entity,
    ) -> Option<AccessMutReturn<'_, Self::Component>>
    where
        Self::Component: Default;

    /// Insert a component for an `Entity`
    fn insert(&mut self, entity: Entity, comp: Self::Component) -> InsertResult<Self::Component>;

    /// Remove the component for an `Entity`
    fn remove(&mut self, entity: Entity);

    /// Private function to seal the trait
    fn _private() -> Seal;
}

impl<'a, T> GenericWriteComp for WriteComp<'a, T>
where
    T: Component,
{
    #[cfg(feature = "nightly")]
    type AccessMut<'b>
    where
        Self: 'b,
    = <<T as Component>::Storage as UnprotectedStorage<T>>::AccessMut<'b>;
    type Component = T;

    fn get_mut(&mut self, entity: Entity) -> Option<AccessMutReturn<'_, T>> {
        WriteComp::get_mut(self, entity)
    }

    fn get_mut_or_default(&mut self, entity: Entity) -> Option<AccessMutReturn<'_, T>>
    where
        Self::Component: Default,
    {
        if !self.contains(entity) {
            self.insert(entity, Default::default())
                .ok()
                .and_then(move |_| self.get_mut(entity))
        } else {
            self.get_mut(entity)
        }
    }

    fn insert(&mut self, entity: Entity, comp: Self::Component) -> InsertResult<Self::Component> {
        WriteComp::insert(self, entity, comp)
    }

    fn remove(&mut self, entity: Entity) {
        WriteComp::remove(self, entity);
    }

    fn _private() -> Seal {
        Seal
    }
}

impl<'a: 'b, 'b, T> GenericWriteComp for &'b mut WriteComp<'a, T>
where
    T: Component,
{
    #[cfg(feature = "nightly")]
    type AccessMut<'c>
    where
        Self: 'c,
    = <<T as Component>::Storage as UnprotectedStorage<T>>::AccessMut<'c>;
    type Component = T;

    fn get_mut(&mut self, entity: Entity) -> Option<AccessMutReturn<'_, T>> {
        WriteComp::get_mut(*self, entity)
    }

    fn get_mut_or_default(&mut self, entity: Entity) -> Option<AccessMutReturn<'_, T>>
    where
        Self::Component: Default,
    {
        if !self.contains(entity) {
            self.insert(entity, Default::default())
                .ok()
                .and_then(move |_| self.get_mut(entity))
        } else {
            self.get_mut(entity)
        }
    }

    fn insert(&mut self, entity: Entity, comp: Self::Component) -> InsertResult<Self::Component> {
        WriteComp::insert(*self, entity, comp)
    }

    fn remove(&mut self, entity: Entity) {
        WriteComp::remove(*self, entity);
    }

    fn _private() -> Seal {
        Seal
    }
}
