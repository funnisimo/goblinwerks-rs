use gw_ecs::{Ecs, World};
use gw_util::rng::RandomNumberGenerator;

pub mod action;
pub mod being;
pub mod camera;
pub mod combat;
pub mod effect;
pub mod fov;
pub mod hero;
pub mod horde;
pub mod level;
pub mod log;
pub mod map;
pub mod memory;
pub mod position;
pub mod sprite;
pub mod task;
pub mod tile;
pub mod treasure;
pub mod widget;

/// Register all the standard components for gw_world
pub fn register_components(ecs: &mut Ecs) {
    ecs.register::<position::Position>();
    ecs.register::<sprite::Sprite>();
    ecs.register::<task::Task>();
    ecs.register::<being::Being>();
    ecs.register::<combat::Melee>();
    ecs.register::<being::Stats>();
    ecs.register::<horde::HordeRef>();
}

pub fn setup_ecs(ecs: &mut Ecs) {
    ecs.ensure_global::<tile::Tiles>();
    ecs.ensure_global::<being::BeingKinds>();
    ecs.ensure_global::<horde::Hordes>();
    ecs.ensure_global::<log::Logger>();
}

/// Ensure all the standard resources for gw_world
pub fn setup_world(world: &mut World) {
    world.ensure_resource::<camera::Camera>();
    world.ensure_resource::<task::Executor>();
    world.ensure_resource::<level::NeedsDraw>();
    world.ensure_resource::<task::UserAction>();
    world.ensure_resource::<hero::Hero>();
    world.ensure_resource::<RandomNumberGenerator>();
}
