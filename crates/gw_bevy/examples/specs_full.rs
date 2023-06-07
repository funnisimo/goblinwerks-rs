use gw_bevy::prelude::*;
use rayon::iter::ParallelIterator;

// -- Components --
// A component exists for 0..n
// entities.

#[derive(Clone, Debug, Component)]
struct CompInt(i32);

#[derive(Clone, Debug, Component)]
struct CompBool(bool);

#[derive(Clone, Debug, Component)]
struct CompFloat(f32);

// -- Resources --
// Resources are unique and can be accessed
// from systems using the same sync strategy
// as component storages

#[derive(Clone, Debug, Default)]
struct Sum(usize);

// -- System Data --
// Each system has an associated
// data type.

#[derive(SystemParam)]
struct IntAndBoolData<'w> {
    comp_int: ReadComp<'w, CompInt>,
    comp_bool: WriteComp<'w, CompBool>,
}

#[derive(SystemParam)]
struct SpawnData<'w> {
    comp_int: WriteComp<'w, CompInt>,
    entities: Entities<'w>,
}

#[derive(SystemParam)]
struct StoreMaxData<'w> {
    comp_float: ReadComp<'w, CompFloat>,
    comp_int: ReadComp<'w, CompInt>,
    entities: Entities<'w>,
}

// -- Systems --

fn sys_print_bool(data: ReadComp<CompBool>) {
    println!("[sys_print_bool]");
    for b in (&data).join() {
        println!("Bool: {:?}", b);
    }
}

fn sys_check_positive(mut data: IntAndBoolData) {
    println!("[sys_check_positive]");
    // Join merges the two component storages,
    // so you get all (CompInt, CompBool) pairs.
    for (ci, mut cb) in (&data.comp_int, &mut data.comp_bool).join() {
        cb.0 = ci.0 > 0;
    }
}

#[derive(Default)]
struct SysSpawn {
    counter: i32,
}

fn sys_spawn(mut local: Local<SysSpawn>, mut data: SpawnData) {
    println!("[sys_spawn]");
    if local.counter == 0 {
        let entity = data.entities.join().next().unwrap();
        println!(" - delete entity: {}", entity.id());
        let _ = data.entities.delete(entity);
    }

    let entity = data.entities.create();
    // This line can't fail because we just made the entity.
    data.comp_int
        .insert(entity, CompInt(local.counter))
        .unwrap();

    println!(" - create entity: {} -> {}", entity.id(), local.counter);

    local.counter += 1;

    if local.counter > 100 {
        local.counter = 0;
    }
    println!(" - counter: {}", local.counter);
}

/// Stores the entity with
/// the greatest int.
#[derive(Default)]
struct SysStoreMax(Option<Entity>);

fn sys_store_max(mut local: Local<SysStoreMax>, data: StoreMaxData) {
    use std::i32::MIN;

    println!("[sys_store_max]");

    let mut max_entity = None;
    let mut max = MIN;

    for (entity, value) in (&data.entities, &data.comp_int).join() {
        if value.0 >= max {
            max = value.0;
            max_entity = Some(entity);
        }
    }

    (*local).0 = max_entity;

    // Let's print information about our entity
    if let Some(e) = (*local).0 {
        if let Some(f) = data.comp_float.get(e) {
            println!("Entity {} with biggest int has float value {:?}", e.id(), f);
        } else {
            println!("Entity {} with biggest int has no float value", e.id());
        }
    }
}

fn join_parallel(
    comp_bool: ReadComp<CompBool>,
    comp_int: ReadComp<CompInt>,
    mut comp_float: WriteComp<CompFloat>,
) {
    println!("[join_parallel]");
    // use gw_ecs::specs::rayon::prelude::*;
    (&comp_bool, &comp_int, &mut comp_float)
        .par_join()
        // only iterate over entities with a `CompBool(true)`
        .filter(|(b, _b, ref _a)| b.0)
        // set the `CompFloat` value to the float repr of `CompInt`
        .for_each(|(_, i, mut f)| f.0 += i.0 as f32);
}

/// Takes every `CompFloat` and tries to add `CompInt` if it exists.
fn add_int_to_float(comp_int: ReadComp<CompInt>, comp_float: ReadComp<CompFloat>) {
    println!("[add_int_to_float]");
    // This system demonstrates the use of `.maybe()`.
    // As the name implies, it doesn't filter any entities; it yields an
    // `Option<CompInt>`. So the `join` will yield all entities that have a
    // `CompFloat`, just returning a `CompInt` if the entity happens to have
    // one.
    for (f, i) in (&comp_float, comp_int.maybe()).join() {
        let f = f.0;
        let i = i.map(|i| i.0 as f32).unwrap_or(0.0);
        let sum = f + i;
        println!("Result: {} + {} = {}", f, i, sum);
    }

    // An alternative way to write this out:
    // (note that `entities` is just another system data of type
    // `Entities<'a>`)
    //
    // ```
    // for (entity, f) in (&entities, &comp_float).join() {
    //     let i = comp_int.get(e); // retrieves the component for the current entity
    //
    //     let sum = f.0 + i.map(|i| i.0 as f32).unwrap_or(0.0);
    //     println!("Result: sum = {}", sum);
    // }
    // ```
}

fn main() {
    let mut w = World::default();

    w.register::<CompBool>();
    w.register::<CompInt>();
    w.register::<CompFloat>();
    w.ensure_resource::<Sum>();

    // This builds our dispatcher, which contains the systems.
    // Every system has a name and can depend on other systems.
    // "check_positive" depends on  "print_bool" for example,
    // because we want to print the components before executing
    // `SysCheckPositive`.
    #[allow(unused_mut)]
    let mut schedule = Schedule::new();
    schedule
        .add_system(sys_print_bool)
        .add_system(sys_check_positive)
        .add_system(sys_store_max)
        .add_system(sys_spawn)
        .add_system(sys_print_bool);

    #[cfg(feature = "parallel")]
    {
        schedule.add_system(join_parallel);
    }

    schedule.add_system(add_int_to_float);

    // create_entity() of World provides with an EntityBuilder to add components to
    // an Entity
    w.create_entity()
        .with(CompInt(4))
        .with(CompBool(false))
        .id();
    // build() returns an entity, we will use it later to perform a deletion
    let e = w.create_entity().with(CompInt(9)).with(CompBool(true)).id();
    w.create_entity()
        .with(CompInt(-1))
        .with(CompBool(false))
        .id();
    w.create_entity().with(CompInt(127)).id();
    w.create_entity().with(CompBool(false)).id();
    w.create_entity().with(CompFloat(0.1)).id();

    schedule.run(&mut w);
    w.maintain();

    // Insert a component, associated with `e`.
    if let Err(err) = w.write_component().insert(e, CompFloat(4.0)) {
        eprintln!("Failed to insert component! {:?}", err);
    }

    schedule.run(&mut w);
    w.maintain();
}
