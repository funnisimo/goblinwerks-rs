#![allow(dead_code, unused_imports)]

use gw_ecs::fetch::{Global, GlobalMut, Unique, UniqueMut};
use gw_ecs::system::System;
use gw_ecs::Ecs;
use gw_ecs::Fetch;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ops::Deref;

struct Data(u32);

fn simple_system(ecs: &mut Ecs) {
    let d = ecs.get_global::<Data>().unwrap();
    println!("Simple System - {}", d.0);
}

fn main() {
    let mut ecs = Ecs::new();

    ecs.insert_global(Data(5));
    ecs.level_mut().insert_unique(Data(10));

    {
        let source_gl = ecs.get_global::<Data>().unwrap();
        println!("source = {:?}", source_gl.0);

        let global_ref = Global::<Data>::fetch(&ecs);
        println!("borrowed = {:?}", global_ref.0);

        let (gl, un) = <(Global<Data>, Unique<Data>)>::fetch(&ecs);
        println!("borrowed = {:?} {:?}", gl.0, un.0);

        let (gl, un) = ecs.fetch::<(Global<Data>, Unique<Data>)>();
        println!("fetched = {:?} {:?}", gl.0, un.0);
    }

    // let system_fn = |_source: &mut Ecs| {
    //     println!("Hello from System!");
    // };
    // let system = system_fn.into_system();
    // system.run(&mut ecs);

    // let system = simple_system.into_system();
    // system.run(&mut ecs);

    // let system_fn = |entity: Global<Data>| {
    //     println!("Hello from Global Data System - {}!", entity.0);
    // };
    // let mut system: System = system_fn.into_system();
    // system.run(&mut ecs);

    // let system_fn = |entity: Global<Data>, e2: Unique<Data>| {
    //     println!(
    //         "Hello from Global + Unique System - {} + {}!",
    //         entity.0, e2.0
    //     );
    // };
    // let mut system: System = system_fn.into_system();
    // system.run(&mut ecs);
}
