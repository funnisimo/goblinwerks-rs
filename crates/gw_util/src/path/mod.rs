// use crate::map::closest_points_matching;

mod search_grid;
pub use search_grid::*;

mod astar;
pub use astar::*;

mod cost_map;
pub use cost_map::*;

mod source;
pub use source::*;

// pub fn path_for_entity_to_xy(
//     world: &World,
//     entity: EntityId,
//     x: i32,
//     y: i32,
// ) -> Option<Vec<Point>> {
//     let start = match world.entity_point(entity) {
//         None => return None,
//         Some(pt) => pt,
//     };
//     let end = Point::new(x, y);

//     let entity_source = EntitySource::new(entity);
//     a_star_search(start, end, entity_source, true)
// }

// pub fn path_for_hero_to_xy(world: &World, x: i32, y: i32) -> Option<Vec<Point>> {
//     let map = world.map().borrow();
//     let fov = world.get_fov(world.hero_entity()).unwrap().borrow();

//     let closest_points = closest_points_matching(&map, x, y, |x1, y1, tile| {
//         (fov.is_mapped(x1, y1) || fov.is_revealed(x1, y1)) && !tile.blocks()
//     });
//     if closest_points.len() == 0 {
//         println!("No Revealed points close to {},{}", x, y);
//         return None;
//     }

//     let end = closest_points[0];
//     let start = world.hero_point();

//     let hero_source = HeroSource::new();
//     a_star_search(start, end, hero_source, true)
// }
