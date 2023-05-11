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

use gw_ecs::{
    schedule::{run_stage_once, RunControl, Schedule},
    System, World, WriteRes,
};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut dispatcher = Schedule::new().with("PRE_UPDATE", SayHelloSystem);

    dispatcher
        .new_stage_after("UPDATE", "MY_BATCH")
        .control(CustomBatchControllerSystem)
        .add_system(BuyTomatoSystem)
        .add_system(BuyPotatoSystem);

    let mut world = World::empty(1);

    dispatcher.setup(&mut world);

    // Running phase
    for i in 0..10 {
        println!("Dispatching {} ", i);

        dispatcher.run(&mut world);
        sleep(Duration::new(0, 100000000));
    }

    // Done
    println!("Execution finished");
}

// Resources

#[derive(Default)]
pub struct PotatoStore(i32);

#[derive(Default)]
pub struct TomatoStore(f32);

/// System that says "Hello!"

pub struct SayHelloSystem;

impl<'a> System<'a> for SayHelloSystem {
    type SystemData = (WriteRes<'a, PotatoStore>, WriteRes<'a, TomatoStore>);

    fn run(&mut self, _data: Self::SystemData) {
        println!("Hello!")
    }
}

/// System that says "Buy Potato"

pub struct BuyPotatoSystem;

impl<'a> System<'a> for BuyPotatoSystem {
    type SystemData = WriteRes<'a, PotatoStore>;

    fn run(&mut self, _data: Self::SystemData) {
        println!("Buy Potato")
    }
}

/// System that says "Buy Tomato"

pub struct BuyTomatoSystem;

impl<'a> System<'a> for BuyTomatoSystem {
    type SystemData = WriteRes<'a, TomatoStore>;

    fn run(&mut self, _data: Self::SystemData) {
        println!("Buy Tomato")
    }
}

/// Batch controller that customizes how inner systems are executed
pub struct CustomBatchControllerSystem;

impl RunControl for CustomBatchControllerSystem {
    fn run_stage(&mut self, stage: &mut gw_ecs::schedule::StageData, world: &mut World) {
        {
            // The scope is used to unload the resource before dispatching inner systems.
            let _ts = world.read_resource::<TomatoStore>();
        }

        println!("Batch execution");
        for _i in 0..3 {
            run_stage_once(stage, world);
        }
    }
}
