use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;
use gw_world::effect::{BoxedEffect, Effect, EffectResult};
use gw_world::level::{get_current_level_mut, Level};
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
    fn fire(&self, ecs: &mut Ecs, _pos: Point, entity: Option<Entity>) -> EffectResult {
        if entity.is_none() {
            return EffectResult::Nothing;
        }

        let dest = {
            let moons = ecs.resources.get::<Moons>().unwrap();
            moons.destination()
        };

        let mut level = get_current_level_mut(ecs);
        level.logger.log("MoongateTravel");

        let new_xy = {
            let map = level.resources.get::<Map>().unwrap();
            let new_index = map.get_location(dest).unwrap();
            map.to_point(new_index)
        };

        drop(level);

        teleport_being(ecs, entity.unwrap(), new_xy);

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

pub fn teleport_being(ecs: &mut Ecs, entity: Entity, point: Point) {
    // TODO - Return something?  Result?

    let mut level = get_current_level_mut(ecs);

    let Level {
        resources,
        world,
        // logger,
        ..
    } = &mut *level;

    let mut map = resources.get_mut::<Map>().unwrap();
    let mut entry = world.entry(entity).unwrap();
    let mut pos = entry.get_component_mut::<Position>().unwrap();

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
