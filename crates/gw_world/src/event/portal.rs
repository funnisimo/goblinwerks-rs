/////////////////////////

use super::{EventResult, TileEvent};
use crate::{
    camera::Camera,
    hero::Hero,
    level::Levels,
    map::{Map, PortalInfo},
    position::Position,
};
use gw_app::{ecs::Entity, log, Ecs};
use gw_util::point::Point;

pub struct Portal(PortalInfo);

impl TileEvent for Portal {
    fn fire(&self, ecs: &mut Ecs, entity: Entity, _pos: Point) -> EventResult {
        match try_move_hero_world(ecs, entity, &self.0) {
            true => EventResult::Fail,
            false => EventResult::Success,
        }
    }
}

fn try_move_hero_world(ecs: &mut Ecs, entity: Entity, info: &PortalInfo) -> bool {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();

    let (new_map_id, location) = { (info.map_id().to_string(), info.location().to_string()) };

    if !levels.has_map(&new_map_id) {
        log(format!("UNKNOWN MAP - {}", new_map_id));
        return false;
    }

    let level = levels.current_mut();

    let hero_entity = level.resources.get::<Hero>().unwrap().entity;
    let is_hero = hero_entity == entity;

    let map = level.resources.get_mut::<Map>().unwrap();

    let current_pt = level
        .world
        .entry(hero_entity)
        .unwrap()
        .get_component::<Position>()
        .unwrap()
        .point();

    drop(map);
    drop(level);

    let level = levels.current_mut();
    let mut map = level.resources.get_mut::<Map>().unwrap();
    let index = map.get_wrapped_index(current_pt.x, current_pt.y).unwrap();
    let map_size = map.get_size();

    map.remove_actor(index, hero_entity);

    drop(map);
    drop(level);

    log("Moving hero to new world");
    let new_entity = levels.move_current_entity(hero_entity, &new_map_id);
    log("Changing current world");
    levels.set_current(&new_map_id).unwrap(); // We checked to make sure the map was there earlier

    let level = levels.current_mut();
    if is_hero {
        level.resources.insert(Hero::new(hero_entity));
    }

    {
        let mut camera = level
            .resources
            .get_mut_or_insert_with(|| Camera::new(map_size.0, map_size.1));
        if is_hero {
            camera.set_follows(new_entity);
        }
    }

    let new_pt = {
        let mut map = level.resources.get_mut::<Map>().unwrap();
        let pt = map.locations.get(&location).unwrap().clone();
        map.add_actor(pt, new_entity, true);
        map.to_point(pt)
    };
    {
        let mut entry = level.world.entry(new_entity).unwrap();
        let pos = entry.get_component_mut::<Position>().unwrap();
        pos.set(new_pt.x, new_pt.y);
    }
    {
        let map = level.resources.get::<Map>().unwrap();
        if let Some(ref welcome) = map.welcome {
            level.logger.log(welcome);
        }
    }
    true
}
