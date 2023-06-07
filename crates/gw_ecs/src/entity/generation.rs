use std::fmt;
use std::num::NonZeroI32;

/// Index generation. When a new entity is placed at an old index,
/// it bumps the `Generation` by 1. This allows to avoid using components
/// from the entities that were deleted.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Generation(pub(crate) NonZeroI32);

// Show the inner value as i32 instead of u32.
impl fmt::Debug for Generation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Generation").field(&self.id()).finish()
    }
}

impl Generation {
    pub(crate) fn one() -> Self {
        Generation(unsafe { NonZeroI32::new_unchecked(1) })
    }

    pub(crate) fn zero() -> Self {
        Generation(unsafe { NonZeroI32::new_unchecked(0) })
    }

    #[cfg(test)]
    pub fn new(v: i32) -> Self {
        Generation(NonZeroI32::new(v).expect("generation id must be non-zero"))
    }

    /// Returns the id of the generation.
    #[inline]
    pub fn id(self) -> i32 {
        self.0.get()
    }

    /// Returns `true` if entities of this `Generation` are alive.
    #[inline]
    pub fn is_alive(self) -> bool {
        self.id() > 0
    }

    /// Revives and increments a dead `Generation`.
    ///
    /// # Panics
    ///
    /// Panics if it is alive.
    pub(crate) fn raised(self) -> Generation {
        assert!(!self.is_alive());
        unsafe { Generation(NonZeroI32::new_unchecked(1 - self.id())) }
    }
}

/// Convenience wrapper around Option<Generation>
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct ZeroableGeneration(pub(crate) Option<Generation>);

impl ZeroableGeneration {
    /// Returns the id of the generation.
    #[inline]
    pub fn id(self) -> i32 {
        // should optimise to a noop.
        self.0.map(|gen| gen.id()).unwrap_or(0)
    }

    /// Returns `true` if entities of this `Generation` are alive.
    #[inline]
    pub(crate) fn is_alive(self) -> bool {
        self.id() > 0
    }

    /// Kills this `Generation`.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if it's not alive.
    pub(crate) fn die(&mut self) {
        debug_assert!(self.is_alive());
        self.0 = NonZeroI32::new(-self.id()).map(Generation);
    }

    /// Revives and increments a dead `Generation`.
    ///
    /// # Panics
    ///
    /// Panics if it is alive.
    pub(crate) fn raised(self) -> Generation {
        assert!(!self.is_alive());
        let gen = 1i32.checked_sub(self.id()).expect("generation overflow");
        Generation(unsafe { NonZeroI32::new_unchecked(gen) })
    }

    /// Revives and increments a dead `ZeroableGeneration`.
    ///
    /// # Panics
    ///
    /// Panics if it is alive.
    pub(crate) fn raise(&mut self) -> Generation {
        let gen = self.raised();
        self.0 = Some(gen);
        gen
    }
}
