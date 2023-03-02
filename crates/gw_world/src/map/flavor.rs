// use super::Map;
// use gw_app::ecs::World;

// pub fn flavor_at_xy(world: &World, map: &Map, x: i32, y: i32) -> String {
//     let mut actor_name: Option<String> = None;
//     let mut tile_name: Option<&str> = None;
//     let mut item_name: Option<String> = None;

//     let tile = map.get_tile(x, y);

//     if let Some(t) = tile.as_ref() {
//         tile_name = Some(&t.flavor);
//     }

//     let player_entity = world.hero_entity();
//     for (entity, _) in map.actors[idx].iter() {
//         let actor = world.get_actor(*entity).unwrap();
//         // if !actor.borrow().is_at_xy(x, y, map.id) {
//         //     continue;
//         // }

//         if *entity == player_entity {
//             actor_name = Some("yourself".to_owned());
//         } else {
//             actor_name = Some(actor.borrow().name_with_a());
//         }
//     }

//     for (entity, _) in map.items[idx].iter() {
//         let item = world.get_item(*entity).unwrap();
//         // if !item.borrow().is_at_xy(x, y, map.id) {
//         //     continue;
//         // }

//         item_name = Some(item.borrow().name_with_a());
//     }

//     let fov = world.get_fov(world.hero_entity());
//     let verb = {
//         // match fov.to_idx(x, y) {
//         //     None => "sense".to_owned(),
//         //     Some(idx) => fov.flags[idx].to_string(),
//         // }
//         if fov.is_none() {
//             "remember"
//         } else {
//             let fov = fov.unwrap().borrow();
//             if fov.is_visible(x, y) {
//                 "see"
//             } else if fov.is_revealed(x, y) {
//                 "remember"
//             } else {
//                 "sense"
//             }
//         }
//     };

//     match (actor_name, item_name, tile_name) {
//         (Some(act), Some(item), Some(t)) => format!("You {} {} and {} on {}", verb, act, item, t),
//         (Some(actor), None, Some(t)) => format!("You {} {} on {}", verb, actor, t),
//         (None, Some(item), Some(t)) => format!("You {} {} on {}", verb, item, t),
//         (None, None, Some(t)) => format!("You {} {}", verb, t),
//         _ => "An eerie nothingness.".to_owned(),
//     }
// }
