//! This example shows how to use and define a batch dispatcher.
//!
//! The batch feature allows to control the dispatching of a group of
//! systems.
//!
//! Specifically here we have three Systems
//! - `SayHelloSystem`: Which is directly registered under the main dispatcher.
//! - `BuyTomatoSystem` and `BuyPotatoSystem` are registered to the batch.
//!
//! Notice that none of these systems are directly depending on others.
//! The `SayHelloSystem` is requesting the resources `TomatoStore` and
//! `PotatoStore`, which are also requested by the other two systems inside
//! the batch and by the batch controller itself.
//!
//! This example demonstrates that the batch dispatcher is able to affect on how
//! the systems inside the batch are executed
//!
//! This is done by defining `CustomBatchControllerSystem` which executes its
//! inner `System`s three times.

use gw_ecs::prelude::*;
use std::{thread::sleep, time::Duration};

fn main() {
    let mut schedule = Schedule::new();

    schedule
        .add_system(say_hello)
        .add_system(buy_potato)
        .add_system(custom_run_control);

    let mut world = World::default();
    world.ensure_resource::<TomatoStore>();
    world.ensure_resource::<PotatoStore>();

    // Running phase
    for i in 0..10 {
        println!("Dispatching {} ", i);

        schedule.run(&mut world);
        sleep(Duration::new(0, 100000000));
    }

    // Done
    println!("Execution finished");
}

// Resources

#[derive(Default)]
pub struct PotatoStore(u32);

#[derive(Default)]
pub struct TomatoStore(u32);

/// System that says "Hello!"
fn say_hello(pot: ResRef<PotatoStore>, tom: ResRef<TomatoStore>) {
    println!("Hello! - potatos: {}, tomatoes: {}", pot.0, tom.0);
}

/// System that says "Buy Potato"
fn buy_potato(mut pot: ResMut<PotatoStore>) {
    pot.0 += 1;
    println!("Buy Potato - {}", pot.0);
}

/// System that says "Buy Tomato"
fn buy_tomato(mut tom: ResMut<TomatoStore>) {
    tom.0 += 1;
    println!("Buy Tomato - {}", tom.0);
}

/// Batch controller that customizes how inner systems are executed
fn custom_run_control(world: &mut World) {
    println!("Batch buy tomato");
    for _i in 0..3 {
        world.exec(buy_tomato);
    }
}
