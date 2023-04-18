#![allow(dead_code, unused_imports)]

use gw_ecs::fetch::{Global, GlobalMut, Unique, UniqueMut};
use gw_ecs::system::System;
use gw_ecs::Ecs;
use gw_ecs::Fetch;
use gw_macro::system;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ops::Deref;

struct Data(u32);

#[system]
fn simple_fn(ecs: &Ecs) {
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

    simple_fn_system(&ecs);
}
