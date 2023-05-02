use gw_ecs::*;

// A component contains data which is associated with an entity.

#[derive(Debug)]
struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SysA;

impl<'a> System<'a> for SysA {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (WriteComp<'a, Pos>, ReadComp<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        // You could also use `par_join()` to get a rayon `ParallelIterator`.
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
        }
    }
}

fn main() {
    // The `World` is our
    // container for components
    // and other resources.

    let mut world = World::default();
    world.register::<Pos>();
    world.register::<Vel>();

    // An entity may or may not contain some component.

    world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
    world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
    world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

    // This entity does not have `Vel`, so it won't be dispatched.
    world.create_entity().with(Pos(2.0)).build();

    // This builds an async dispatcher.
    // The third parameter of `add` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    #[cfg(feature = "parallel")]
    {
        let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();

        // This dispatches all the systems in parallel and async.
        dispatcher.dispatch(&world);

        // Do something on the main thread

        // dispatcher.wait();
    }

    #[cfg(not(feature = "parallel"))]
    {
        eprintln!("The `async` example should be built with the `\"parallel\"` feature enabled.");
        let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();
        dispatcher.setup(&mut world);
        dispatcher.dispatch(&mut world);
    };
}
