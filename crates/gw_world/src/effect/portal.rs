use super::{BoxedEffect, Effect, EffectResult};
use crate::{camera::Camera, hero::Hero, level::Levels, map::Map, position::Position};
use gw_app::{ecs::Entity, log, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;

#[derive(Debug, Clone)]
pub struct Portal(String, String);

impl Portal {
    pub fn new(map_id: String, location: String) -> Self {
        Portal(map_id.to_uppercase(), location.to_uppercase())
    }
}

impl Effect for Portal {
    fn fire(&self, ecs: &mut Ecs, _pos: Point, entity: Option<Entity>) -> EffectResult {
        match entity {
            None => try_change_world(ecs, &self.0, &self.1),
            Some(entity) => try_move_hero_world(ecs, entity, &self.0, &self.1),
        }
    }
}

pub(super) fn parse_portal(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_string() {
        Ok(Box::new(Portal::new(
            value.to_string(),
            "START".to_string(),
        )))
    } else if value.is_map() {
        let map = value.as_map().unwrap();

        let id = match map.get(&"map".into()) {
            None => return Err(format!("Portal effects require 'map' field")),
            Some(val) => val.to_string(),
        };

        let location = match map.get(&"location".into()) {
            None => "START".to_string(),
            Some(val) => val.to_string(),
        };

        Ok(Box::new(Portal::new(id, location)))
    } else {
        Err(format!(
            "Portal effects can only receive string values or objects.  Received: {:?}",
            value
        ))
    }
}

fn try_move_hero_world(
    ecs: &mut Ecs,
    entity: Entity,
    new_map_id: &String,
    location: &String,
) -> EffectResult {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
    if !levels.has_map(&new_map_id) {
        log(format!("UNKNOWN MAP - {}", new_map_id));
        return EffectResult::Fail;
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

    let map_size = level.resources.get::<Map>().unwrap().get_size();

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
        let pt = map.locations.get(location).unwrap().clone();
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
    EffectResult::Success
}

fn try_change_world(ecs: &mut Ecs, new_map_id: &String, _location: &String) -> EffectResult {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();

    if !levels.has_map(&new_map_id) {
        log(format!("UNKNOWN MAP - {}", new_map_id));
        return EffectResult::Fail;
    }

    levels.set_current(&new_map_id).unwrap(); // We checked to make sure the map was there earlier

    let level = levels.current_mut();

    let map_size = {
        let map = level.resources.get::<Map>().unwrap();
        if let Some(ref welcome) = map.welcome {
            level.logger.log(welcome);
        }
        map.get_size()
    };

    level
        .resources
        .get_or_insert_with(|| Camera::new(map_size.0, map_size.1));

    EffectResult::Success
}
