use super::{BoxedEffect, Effect, EffectResult, Message};
use crate::log::Logger;
use crate::task::Executor;
use crate::{camera::Camera, hero::Hero, map::Map, position::Position};
use gw_app::log;
use gw_ecs::prelude::Atom;
use gw_ecs::prelude::{Commands, Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;

// #[derive(Debug, Clone)]
// pub struct Portal(Atom, String);

// impl Portal {
//     pub fn new(map_id: String, location: String) -> Self {
//         Portal(
//             Atom::from(map_id.to_uppercase().as_str()),
//             location.to_uppercase(),
//         )
//     }
// }

// impl Effect for Portal {
//     fn fire(&self, world: &mut World, _pos: Point, entity: Option<Entity>) -> EffectResult {
//         match entity {
//             None => try_change_world(world, self.0, &self.1),
//             Some(entity) => try_move_hero_world(world, entity, self.0, &self.1),
//         }
//     }
// }

pub(super) fn parse_portal(value: &Value) -> Result<BoxedEffect, String> {
    Ok(Box::new(Message("TODO - TAKE A PORTAL".to_string())))

    //     if value.is_string() {
    //         Ok(Box::new(Portal::new(
    //             value.to_string(),
    //             "START".to_string(),
    //         )))
    //     } else if value.is_map() {
    //         let map = value.as_map().unwrap();

    //         let id = match map.get(&"map".into()) {
    //             None => return Err(format!("Portal effects require 'map' field")),
    //             Some(val) => val.to_string(),
    //         };

    //         let location = match map.get(&"location".into()) {
    //             None => "START".to_string(),
    //             Some(val) => val.to_string(),
    //         };

    //         Ok(Box::new(Portal::new(id, location)))
    //     } else {
    //         Err(format!(
    //             "Portal effects can only receive string values or objects.  Received: {:?}",
    //             value
    //         ))
    //     }
}

// fn try_move_hero_world(
//     world: &mut World,
//     entity: Entity,
//     new_map_id: Atom,
//     location: &String,
// ) -> EffectResult {
//     // TODO - Change this to an event with a change world system...
//     let lazy_update = world.read_resource::<Commands>();
//     let current_map_id = world.id();
//     let location = location.clone();

//     log("EXEC MOVE WORLD");

//     lazy_update.exec_ecs(move |ecs| {
//         if !ecs.has_world(new_map_id) {
//             log(format!("UNKNOWN DEST MAP - {}", new_map_id));
//             return;
//         }

//         #[allow(unused_assignments)]
//         let mut is_hero = false;

//         let hero_entity = match ecs.get_world(current_map_id) {
//             None => {
//                 log(format!("UNKNOWN SOURCE MAP - {}", current_map_id));
//                 return;
//             }
//             Some(current_world) => {
//                 let hero_entity = current_world.read_resource::<Hero>().entity;
//                 let mut map = current_world.write_resource::<Map>();
//                 let positions = current_world.read_component::<Position>();

//                 is_hero = hero_entity == entity;

//                 let current_pt = positions.get(hero_entity).unwrap().point();

//                 let index = map.get_wrapped_index(current_pt.x, current_pt.y).unwrap();

//                 map.remove_being(index, hero_entity);

//                 hero_entity
//             }
//         };

//         log("Moving hero to new world");
//         let new_entity = ecs.move_entity(hero_entity, current_map_id, new_map_id);

//         log("Changing current world");
//         ecs.set_current_world(new_map_id); // We checked to make sure the map was there earlier

//         let new_world = ecs.current_world_mut();
//         if is_hero {
//             new_world.ensure_resource::<Hero>();
//             let mut hero = new_world.write_resource::<Hero>();
//             hero.entity = new_entity;
//         }

//         let map_size = new_world.read_resource::<Map>().size();

//         {
//             let mut camera =
//                 new_world.write_resource_or_insert_with(|| Camera::new(map_size.0, map_size.1));
//             if is_hero {
//                 camera.set_follows(new_entity);
//             }
//         }

//         let mut map = new_world.write_resource::<Map>();

//         let new_pt = {
//             let pt = map.locations.get(&location).unwrap().clone();
//             map.add_being(pt, new_entity, true);
//             map.to_point(pt)
//         };
//         {
//             let mut positions = new_world.write_component::<Position>();
//             let mut pos = positions.get_mut(new_entity).unwrap();
//             pos.set(new_pt.x, new_pt.y);
//         }
//         {
//             if let Some(ref welcome) = map.welcome {
//                 let mut logger = new_world.write_global::<Logger>();
//                 logger.log(welcome);
//             }
//         }

//         let mut executor = new_world.write_resource::<Executor>();
//         executor.remove(new_entity); // just in case
//         executor.insert(new_entity, 0);
//     });

//     EffectResult::Success
// }

// fn try_change_world(world: &mut World, new_map_id: Atom, _location: &String) -> EffectResult {
//     // let mut levels = ecs.resources.get_mut::<Levels>().unwrap();

//     let lazy_update = world.read_resource::<Commands>();

//     lazy_update.exec_ecs(move |ecs| {
//         if !ecs.has_world(new_map_id) {
//             log(format!("UNKNOWN DEST MAP - {}", new_map_id));
//             return;
//         }

//         ecs.set_current_world(new_map_id); // We checked to make sure the map was there earlier

//         let world = ecs.current_world_mut();

//         let map_size = {
//             let map = world.read_resource::<Map>();
//             if let Some(ref welcome) = map.welcome {
//                 let mut logger = world.write_global::<Logger>();
//                 logger.log(welcome);
//             }
//             map.size()
//         };

//         world.ensure_resource_with(|| Camera::new(map_size.0, map_size.1));
//     });

//     EffectResult::Success
// }
