use gw_ecs::prelude::*;
use rand::RngCore;

// This example shows a basic component pipeline.
// It looks at added components and removed components in various systems.

#[derive(Component, Default, Debug)]
struct Compute(u32);

fn cycle_logger() {
    println!("Started cycle...");
}

fn spawn_system(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let chance = rng.next_u32() % 100;

    // 10% chance to
    if chance < 10 {
        let entity = commands.spawn(Compute::default()).id();
        println!("Creating compute job: {}", entity.id());
    }
}

fn init_system(entities: Entities, mut computes: WriteComp<Compute>) {
    // TODO - ... in computes.join() should give us a CompMut instead of a CompRef
    let mut rng = rand::thread_rng();

    for (entity, mut compute) in (&entities, computes.added()).join() {
        let count = rng.next_u32() % 10;
        compute.0 = count;
        println!("init compute - {} : {}", entity.id(), count);
    }
}

fn compute_system(entities: Entities, mut computes: WriteComp<Compute>, mut commands: Commands) {
    for (entity, mut compute) in (&entities, &mut computes).join() {
        println!("- processing {}: {}", entity.id(), compute.0);
        compute.0 = compute.0.saturating_sub(1);

        if compute.0 == 0 {
            println!("--- job finished: {}", entity.id());
            commands.remove_component::<Compute>(entity);
        }
    }
}

fn cleanup_system(mut removed_computes: Removed<Compute>, mut commands: Commands) {
    for entity in removed_computes.iter() {
        println!("!! CLEANUP compute - {}", entity.id());
        commands.entity(*entity).despawn();
    }
}

fn main() {
    // The `World` is our
    // container for components
    // and other resources.

    let mut world = World::default();
    world.register::<Compute>();

    // This builds a dispatcher.
    // The third parameter of `add` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    let mut dispatcher = Schedule::new();
    dispatcher.add_systems(
        (
            cycle_logger,
            spawn_system,
            init_system,
            compute_system,
            cleanup_system,
        )
            .chain(),
    );

    // Just run the simulation 10 times...
    for _ in 0..100 {
        dispatcher.run(&mut world);
        world.maintain();
    }

    {
        let entities = world.entities();
        let computes = world.read_component::<Compute>();

        println!("Remaining entities & jobs");
        for (entity, compute) in (&entities, computes.maybe()).join() {
            match compute {
                None => println!(" - {}: NO COMPUTE", entity.id()),
                Some(c) => println!(" - {}: {}", entity.id(), c.0),
            }
        }
    }
}
