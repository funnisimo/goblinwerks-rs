use crate::prelude::DetectChanges;
use crate::tick::ComponentTicks;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Read<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct CompRef<'b, T> {
    pub(crate) data: &'b T,
    pub(crate) ticks: &'b ComponentTicks,
    pub(crate) last_system_tick: u32,
    pub(crate) world_tick: u32,
}

impl<'b, T> CompRef<'b, T> {
    pub fn new(
        data: &'b T,
        ticks: &'b ComponentTicks,
        last_system_tick: u32,
        world_tick: u32,
    ) -> Self {
        CompRef {
            data,
            ticks,
            world_tick,
            last_system_tick,
        }
    }
}

impl<'b, T> DetectChanges for CompRef<'b, T> {
    /// Returns `true` if this value was added after the system last ran.
    fn is_added(&self) -> bool {
        self.ticks.is_added(self.last_system_tick, self.world_tick)
    }

    /// Returns `true` if this value was added or mutably dereferenced after the system last ran.
    fn is_changed(&self) -> bool {
        self.ticks
            .is_changed(self.last_system_tick, self.world_tick)
    }

    /// Returns the change tick recording the time this data was most recently changed.
    ///
    /// Note that components and resources are also marked as changed upon insertion.
    ///
    /// For comparison, the previous change tick of a system can be read using the
    /// [`SystemChangeTick`](crate::system::SystemChangeTick)
    /// [`SystemParam`](crate::system::SystemParam).
    fn last_changed(&self) -> u32 {
        self.ticks.changed.tick
    }
}

impl<'b, T> Deref for CompRef<'b, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<'b, T> Debug for CompRef<'b, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

/// Allows to fetch a resource in a system mutably.
///
/// If the resource isn't strictly required, you should use `Option<Write<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
/// * `F`: The setup handler (default: `DefaultProvider`)
pub struct CompMut<'b, T> {
    pub(crate) data: &'b mut T,
    pub(crate) ticks: &'b mut ComponentTicks,
    pub(crate) last_system_tick: u32,
    pub(crate) world_tick: u32,
}

impl<'b, T> CompMut<'b, T> {
    pub fn new(
        data: &'b mut T,
        ticks: &'b mut ComponentTicks,
        last_system_tick: u32,
        world_tick: u32,
    ) -> Self {
        CompMut {
            data,
            ticks,
            last_system_tick,
            world_tick,
        }
    }
}

impl<'b, T> DetectChanges for CompMut<'b, T> {
    /// Returns `true` if this value was added after the system last ran.
    fn is_added(&self) -> bool {
        self.ticks.is_added(self.last_system_tick, self.world_tick)
    }

    /// Returns `true` if this value was added or mutably dereferenced after the system last ran.
    fn is_changed(&self) -> bool {
        self.ticks
            .is_changed(self.last_system_tick, self.world_tick)
    }

    /// Returns the change tick recording the time this data was most recently changed.
    ///
    /// Note that components and resources are also marked as changed upon insertion.
    ///
    /// For comparison, the previous change tick of a system can be read using the
    /// [`SystemChangeTick`](crate::system::SystemChangeTick)
    /// [`SystemParam`](crate::system::SystemParam).
    fn last_changed(&self) -> u32 {
        self.ticks.changed.tick
    }
}

impl<'b, T> Deref for CompMut<'b, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<'b, T> DerefMut for CompMut<'b, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.ticks.set_changed(self.world_tick);
        self.data
    }
}

impl<'b, T> Debug for CompMut<'b, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use crate as gw_bevy;
    use crate::prelude::*;

    #[derive(Component, Debug)]
    struct CompA(u32);

    fn system_a(mut comp: WriteComp<CompA>) {
        println!(
            "write_comp - {}, {}",
            comp.last_system_tick, comp.world_tick
        );
        for mut a in (&mut comp).join() {
            println!(
                "pre mut - {:?} > {}, {}",
                a.ticks, a.last_system_tick, a.world_tick
            );
            a.0 += 1;
            println!(
                "post mut - {:?} > {}, {}",
                a.ticks, a.last_system_tick, a.world_tick
            );
        }
    }

    #[test]
    fn is_added() {
        let mut world = World::default();

        world.register::<CompA>();

        let mut schedule = Schedule::new();
        schedule.add_system(system_a);

        println!("world tick = {}", world.current_tick());

        let e = world.spawn(CompA(0));

        {
            let comp = world.read_component::<CompA>();
            assert!(comp.get(e).unwrap().is_added());
            assert!(comp.get(e).unwrap().is_changed());
        }

        world.maintain();

        {
            let comp = world.read_component::<CompA>();
            assert!(!comp.get(e).unwrap().is_added());
            assert!(!comp.get(e).unwrap().is_changed());
        }

        println!("world tick = {}", world.current_tick());

        schedule.run(&mut world);

        {
            let comp = world.read_component::<CompA>();
            assert!(!comp.get(e).unwrap().is_added());
            assert!(comp.get(e).unwrap().is_changed());
        }
    }
}
