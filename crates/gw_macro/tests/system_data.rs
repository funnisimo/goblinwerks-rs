use gw_ecs::{Ecs, ReadGlobal, ResourceId, SystemData, World, Write};

#[derive(Default, PartialEq, Debug)]
pub struct Clock(u32);
#[derive(Default, PartialEq, Debug)]
pub struct Timer(u32);

// This will implement `SystemData` for `MySystemData`.
// Please note that this will only work if `SystemData`, `World` and `ResourceId` are included.
#[derive(SystemData)]
pub struct MySystemData<'a> {
    pub clock: ReadGlobal<'a, Clock>,
    pub timer: Write<'a, Timer>,
}

#[test]
fn basic() {
    let mut ecs = Ecs::default();

    ecs.current_world_mut().insert_global(Clock(5));
    ecs.current_world_mut().insert(Timer(0));

    let mut data = ecs.current_world().system_data::<MySystemData>();
    assert_eq!(data.clock.0, 5);
    assert_eq!(data.timer.0, 0);

    data.timer.0 = 1;
}
