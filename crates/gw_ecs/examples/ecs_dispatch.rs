use gw_ecs::prelude::*;

#[derive(Debug, Default)]
struct UniqueA;

// A resource usually has a `Default` implementation
// which will be used if the resource has not been added.
#[derive(Debug, Default)]
struct UniqueB;

fn print_system(a: ResRef<UniqueA>, mut b: ResMut<UniqueB>) {
    println!("PrintSystem = {:?} + {:?}", &*a, &*b);

    *b = UniqueB; // We can mutate UniqueB here
                  // because it's `Write`.
}

fn main() {
    let mut ecs = Ecs::default();
    let world = ecs.current_world_mut();
    world.insert_resource(UniqueA::default());
    world.ensure_resource::<UniqueB>();

    let mut schedule = Schedule::new();
    schedule.add_system(print_system);

    // Dispatch as often as you want to
    schedule.run(ecs.current_world_mut());
    schedule.run(ecs.current_world_mut());
    // ...
}
