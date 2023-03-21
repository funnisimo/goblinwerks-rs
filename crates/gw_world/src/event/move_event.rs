use super::{EventResult, TileEvent};
use crate::{
    level::{Level, Levels},
    map::Map,
    position::Position,
};
use gw_app::{
    ecs::{Entity, EntityStore},
    Ecs,
};
use gw_util::point::Point;

///////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct MoveEntity(i32, i32);

impl TileEvent for MoveEntity {
    fn fire(&self, ecs: &mut Ecs, entity: Entity, _pos: Point) -> EventResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        let Level {
            resources, world, ..
        } = level;

        let mut map = resources.get_mut::<Map>().unwrap();

        let mut entry = world.entry_mut(entity).unwrap();

        let pos = entry.get_component_mut::<Position>().unwrap();
        let orig_pt = pos.point();

        let old_idx = map.get_wrapped_index(orig_pt.x, orig_pt.y).unwrap();
        map.remove_actor(old_idx, entity);
        // println!("changed : {}", old_idx);

        let (new_x, new_y) = map.try_wrap_xy(pos.x + self.0, pos.y + self.1).unwrap();
        pos.set(new_x, new_y);

        let new_idx = map.get_wrapped_index(pos.x, pos.y).unwrap();
        map.add_actor(new_idx, entity, pos.blocks_move);

        // if let Some(mut fov) = entry.get_component_mut::<FOV>() {
        //     fov.set_needs_update();
        // }

        // let hero = resources.get::<Hero>().unwrap();

        // if not  - need to check to see if we became visible to hero
        // if self.entity != hero.entity {
        // let hero_entry = world.entry(hero.entity).unwrap();

        // if let Some(fov) = hero_entry.get_component::<FOV>() {
        //     if !fov.is_or_was_visible(orig_pt.x, orig_pt.y) && fov.is_visible(new_x, new_y) {
        //         world.push_with_id(self.entity, Interrupt::new());
        //     }
        // }
        // }

        EventResult::Success
    }
}

///////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct MoveRegion(i32, i32);

impl TileEvent for MoveRegion {
    fn fire(&self, ecs: &mut Ecs, _entity: Entity, _pos: Point) -> EventResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        let mut map = level.resources.get_mut::<Map>().unwrap();

        map.move_region_pos(self.0, self.1);

        EventResult::Success
    }
}
