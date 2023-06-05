use gw_bevy::prelude::*;

// A component contains data which is associated with an entity.

#[derive(Debug, Component)]
struct Vel(f32);

#[derive(Debug, Component)]
struct Pos(f32);

fn system_a(mut pos: WriteComp<Pos>, vel: ReadComp<Vel>) {
    // The `.join()` combines multiple components,
    // so we only access those entities which have
    // both of them.
    // You could also use `par_join()` to get a rayon `ParallelIterator`.

    println!("System - Updating positions");
    for (mut pos, vel) in (&mut pos, &vel).join() {
        pos.0 += vel.0;
        println!(
            "vel: {:?} = {} - {}, pos: {:?} = {} - {}",
            vel,
            vel.is_added(),
            vel.is_changed(),
            pos,
            pos.is_added(),
            pos.is_changed()
        );
    }
}

fn main() {
    // The `World` is our
    // container for components
    // and other resources.

    let mut world = World::empty("123");
    world.register::<Pos>();
    world.register::<Vel>();

    // This builds a dispatcher.
    // The third parameter of `add` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    let mut dispatcher = Schedule::new();
    dispatcher.add_system(system_a);

    // An entity may or may not contain some component.

    world.create_entity().with(Vel(2.0)).with(Pos(0.0)).id();
    world.create_entity().with(Vel(4.0)).with(Pos(1.6)).id();
    world.create_entity().with(Vel(1.5)).with(Pos(5.4)).id();

    // This entity does not have `Vel`, so it won't be dispatched.
    world.create_entity().with(Pos(2.0)).id();

    // This dispatches all the systems in parallel (but blocking).
    dispatcher.run(&mut world);
    world.maintain();
    dispatcher.run(&mut world);
    world.maintain();
    dispatcher.run(&mut world);
}
