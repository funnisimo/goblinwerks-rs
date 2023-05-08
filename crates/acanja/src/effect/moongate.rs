use gw_ecs::{Entity, SystemData, World, WriteComp, WriteRes};
use gw_util::point::Point;
use gw_util::value::Value;
use gw_world::effect::{BoxedEffect, Effect, EffectResult};
use gw_world::log::Logger;
use gw_world::map::Map;
use gw_world::position::Position;

use crate::tasks::Moons;

////////////////////////

#[derive(Debug, Clone)]
pub struct MoongateTravel;

impl MoongateTravel {
    pub fn new() -> Self {
        MoongateTravel
    }
}

impl Effect for MoongateTravel {
    fn fire(&self, world: &mut World, _pos: Point, entity: Option<Entity>) -> EffectResult {
        if entity.is_none() {
            return EffectResult::Nothing;
        }

        let dest = {
            let moons = world.read_resource::<Moons>();
            moons.destination()
        };

        world.with_resource_mut(|logger: &mut Logger| logger.log("MoongateTravel"));

        let new_xy = {
            let map = world.read_resource::<Map>();
            let new_index = map.get_location(dest).unwrap();
            map.to_point(new_index)
        };

        teleport_being(world, entity.unwrap(), new_xy);

        EffectResult::Success
    }
}

pub fn parse_moongate_travel(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(MoongateTravel))
    } else {
        Err(format!(
            "MoongateTravel effects can only receive bool values.  Received: {:?}",
            value
        ))
    }
}

pub fn teleport_being(world: &mut World, entity: Entity, point: Point) {
    // TODO - Return something?  Result?

    let (mut map, mut positions) = <(WriteRes<Map>, WriteComp<Position>)>::fetch(world);

    let mut pos = positions.get_mut(entity).unwrap();

    println!("teleport being - {} -> {}", pos.point(), point);

    // remove actor at old location...
    let old_idx = map.get_index(pos.x, pos.y).unwrap();
    map.remove_being(old_idx, entity);

    // add actor at new location...
    let new_idx = map.get_index(point.x, point.y).unwrap();
    map.add_being(new_idx, entity, true);

    pos.x = point.x;
    pos.y = point.y;

    // DO NOT FIRE EFFECTS
    // TODO - Is this the right final answer?
    //      - It does help avoid some looping issues.
}
