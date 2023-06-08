use gw_ecs::prelude::*;
use rand::Rng;
use std::ops::Deref;

// In this example we add a counter resource and increase it's value in one system,
// while a different system prints the current count to the console.
fn main() {
    // Create a world
    let mut world = World::default();

    // Add the counter resource
    world.insert_resource(Counter { value: 0 });

    // Create a schedule
    let mut schedule = Schedule::default();

    // Add systems to increase the counter and to print out the current value
    schedule.add_system(increase_counter);
    schedule.add_system(print_counter.after(increase_counter));

    for iteration in 1..=10 {
        println!("Simulating frame {iteration}/10");
        schedule.run(&mut world);
    }
}

// Counter resource to be increased and read by systems
#[derive(Debug)]
struct Counter {
    pub value: i32,
}

fn increase_counter(mut counter: ResMut<Counter>) {
    if rand::thread_rng().gen_bool(0.5) {
        counter.value += 1;
        println!("    Increased counter value");
    }
}

fn print_counter(counter: ResRef<Counter>) {
    println!("    {:?}", counter.deref());
}
