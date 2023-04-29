use gw_specs::{
    specs::storage::{DenseVecStorage, VecStorage},
    Component, Ecs,
};

#[derive(Default, PartialEq, Debug, Component)]
pub struct Clock(u32);

#[derive(Default, PartialEq, Debug, Component)]
#[storage(VecStorage)]
pub struct Timer(u32);

#[test]
fn basic() {
    let mut _ecs = Ecs::default();
}
