use super::{BoxedEffect, Effect, EffectResult};
use crate::{map::Map, position::Position};
use gw_app::log;
use gw_ecs::prelude::{Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;

///////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct MoveEntity(i32, i32);

impl Effect for MoveEntity {
    fn fire(&self, world: &mut World, _pos: Point, entity: Option<Entity>) -> EffectResult {
        let entity = match entity {
            None => return EffectResult::Fail,
            Some(entity) => entity,
        };

        let mut map = world.write_resource::<Map>();

        let mut positions = world.write_component::<Position>();
        let mut pos = positions.get_mut(entity).unwrap();
        let orig_pt = pos.point();

        let old_idx = map.get_index(orig_pt.x, orig_pt.y).unwrap();
        map.remove_being(old_idx, entity);
        // println!("changed : {}", old_idx);

        // let (new_x, new_y) = map.try_wrap_xy(pos.x + self.0, pos.y + self.1).unwrap();
        let (new_x, new_y) = (pos.x + self.0, pos.y + self.1);
        pos.set(new_x, new_y);

        log(format!(
            "Move entity({},{}) from: {:?}, to: {:?}",
            self.0, self.1, orig_pt, *pos
        ));

        // let new_idx = map.get_wrapped_index(pos.x, pos.y).unwrap();
        let new_idx = map.get_index(pos.x, pos.y).unwrap();
        map.add_being(new_idx, entity, pos.blocks_move);

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

        EffectResult::Success
    }
}

pub(super) fn parse_move_entity(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_list() {
        let list = value.as_list().unwrap();
        if list.len() != 2 {
            return Err(format!(
                "move entity tile event requires 2 int array: e.g. [1,0].  Found: {:?}",
                list
            ));
        }
        let val_0 = list.get(0).unwrap();
        if !val_0.is_int() {
            return Err(format!(
                "move entity tile event requires 2 int array: e.g. [1,0].  Found: {:?}",
                list
            ));
        }
        let val_1 = list.get(1).unwrap();
        if !val_1.is_int() {
            return Err(format!(
                "move entity tile event requires 2 int array: e.g. [1,0].  Found: {:?}",
                list
            ));
        }
        Ok(Box::new(MoveEntity(
            val_0.as_int().unwrap() as i32,
            val_1.as_int().unwrap() as i32,
        )))
    } else {
        Err(format!(
            "Move Entity tile events can only receive a list of 2 ints.  Received: {:?}",
            value
        ))
    }
}

///////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct MoveRegion(i32, i32);

impl Effect for MoveRegion {
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut map = world.write_resource::<Map>();

        let orig = map.region().clone();
        map.move_region_pos(self.0, self.1);
        let updated = map.region();

        log(format!(
            "Move Region({},{}) from: {},{}, to: {},{}",
            self.0, self.1, orig.x1, orig.y1, updated.x1, updated.y1,
        ));

        EffectResult::Success
    }
}

pub(super) fn parse_move_region(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_list() {
        let list = value.as_list().unwrap();
        if list.len() != 2 {
            return Err(format!(
                "move region tile event requires 2 int array: e.g. [1,0].  Found: {:?}",
                list
            ));
        }
        let val_0 = list.get(0).unwrap();
        if !val_0.is_int() {
            return Err(format!(
                "move region tile event requires 2 int array: e.g. [1,0].  Found: {:?}",
                list
            ));
        }
        let val_1 = list.get(1).unwrap();
        if !val_1.is_int() {
            return Err(format!(
                "move region tile event requires 2 int array: e.g. [1,0].  Found: {:?}",
                list
            ));
        }
        Ok(Box::new(MoveRegion(
            val_0.as_int().unwrap() as i32,
            val_1.as_int().unwrap() as i32,
        )))
    } else {
        Err(format!(
            "Move Region tile events can only receive a list of 2 ints.  Received: {:?}",
            value
        ))
    }
}
