use crate::Ecs;

pub trait System {
    fn run(&mut self, ecs: &mut Ecs) -> ();
}

impl<F> System for F
where
    F: FnMut(&mut Ecs) -> (),
{
    fn run(&mut self, ecs: &mut Ecs) -> () {
        (self)(ecs)
    }
}

// /// Allows a type to be borrowed by [`World::borrow`], [`World::run`] and workloads.
// pub trait WorldBorrow<'a> {
//     #[allow(missing_docs)]
//     type Borrow: 'a;

//     /// This function is where the actual borrowing happens.
//     fn world_borrow(world: &'a mut Ecs) -> Option<Self::Borrow>;
// }

// impl<'a, T> WorldBorrow<'a> for Global<T>
// where
//     T: Resource,
// {
//     type Borrow = AtomicRef<'a, T>;

//     fn world_borrow(world: &'a mut Ecs) -> Option<Self::Borrow> {
//         world.get_global::<T>()
//     }
// }

// pub struct WorkloadSystem {
//     pub(crate) system_fn: Box<dyn Fn(&mut Ecs) -> Result<(), ()> + Send + Sync + 'static>,
// }

// impl WorkloadSystem {
//     pub fn call(&mut self, ecs: &mut Ecs) {
//         (self.system_fn)(ecs);
//     }
// }

// pub trait IntoSystem {
//     /// Wraps a function in a struct containing all information required by a workload.
//     fn into_system(self) -> Result<WorkloadSystem, ()>;
// }

// impl<Func, A> IntoSystem for Func
// where
//     for<'a> A: WorldBorrow<'a>,
//     for<'a> Func: Fn(A) -> () + 'static + Send + Sync,
// {
//     fn into_system(self) -> Result<WorkloadSystem, ()> {
//         Ok(WorkloadSystem {
//             system_fn: Box::new(|ecs| {
//                 let mut r = A::world_borrow(ecs).unwrap();
//                 Ok((self)(r))
//             }),
//         })
//     }
// }
