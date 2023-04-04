use std::marker::PhantomData;

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct Res<T>(PhantomData<*const T>);

impl<T> Default for Res<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct ResMut<T>(PhantomData<*const T>);

impl<T> Default for ResMut<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct LevelRef;

impl Default for LevelRef {
    fn default() -> Self {
        Self
    }
}

// unsafe impl<T> Send for Res<T> {}
// unsafe impl<T: Sync> Sync for Res<T> {}

/// Reads a single entity data component type from a chunk.
#[derive(Debug, Copy, Clone)]
pub struct LevelMut;

impl Default for LevelMut {
    fn default() -> Self {
        Self
    }
}
