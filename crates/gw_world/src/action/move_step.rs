use crate::action::idle::IdleAction;
use crate::action::{Action, ActionResult};
use crate::actor::Actor;
use crate::hero::Hero;
use crate::level::Level;
use crate::map::Cell;
use crate::map::Map;
use crate::position::Position;
use gw_app::ecs::systems::ResourceSet;
use gw_app::ecs::{Entity, EntityStore, Read};
use gw_app::log;

#[derive(Copy, Clone, Debug)]
pub struct MoveStepAction {
    pub entity: Entity,
    pub dx: i32,
    pub dy: i32,
}

impl MoveStepAction {
    pub fn new(entity: Entity, dx: i32, dy: i32) -> MoveStepAction {
        MoveStepAction { entity, dx, dy }
    }

    fn validate(&mut self, level: &mut Level) -> Option<ActionResult> {
        let Level {
            resources, world, ..
        } = level;

        let (map, hero) = <(Read<Map>, Read<Hero>)>::fetch(resources);

        let entry = world.entry_mut(self.entity).unwrap();

        let actor = match entry.get_component::<Actor>() {
            Err(_) => return Some(ActionResult::Dead(self.entity)),
            Ok(a) => a,
        };

        if self.dx == 0 && self.dy == 0 {
            return Some(ActionResult::Replace(Box::new(IdleAction::new(
                self.entity,
                actor.act_time,
            ))));
        }

        let pos = match entry.get_component::<Position>() {
            Err(_) => return Some(ActionResult::Dead(self.entity)),
            Ok(pos) => pos,
        };

        let orig_pt = pos.point();

        let idx = match map.get_index(orig_pt.x + self.dx, orig_pt.y + self.dy) {
            None => {
                log("Bump edge of world");
                return Some(ActionResult::Replace(Box::new(IdleAction::new(
                    self.entity,
                    actor.act_time,
                ))));
            }
            Some(idx) => idx,
        };

        // bump
        // let new_idx = match map.to_idx(new_x, new_y) {
        //     None => return Some(ActionResult::Fail("Not a valid location.".to_string())),
        //     Some(idx) => idx,
        // };

        // for (potential_target, blocks) in map.actors[new_idx].iter() {
        //     if *blocks {
        //         return Some(ActionResult::Replace(Box::new(BumpAction::new(
        //             self.entity,
        //             *potential_target,
        //         ))));
        //     }
        // }

        let actor_is_hero = self.entity == hero.entity;

        if map.is_blocked(idx) {
            let flavor = map.get_cell(idx).unwrap().flavor();
            if actor_is_hero {
                level.logger.log(format!("Blocked by {}", flavor));
            }
            return Some(ActionResult::Replace(Box::new(IdleAction::new(
                self.entity,
                actor.act_time,
            ))));
        }

        None
    }

    fn do_action(&mut self, level: &mut Level) -> ActionResult {
        let Level {
            resources, world, ..
        } = level;

        let mut map = resources.get_mut::<Map>().unwrap();

        let mut entry = world.entry_mut(self.entity).unwrap();

        let pos = entry.get_component_mut::<Position>().unwrap();
        let orig_pt = pos.point();

        let old_idx = map.get_index(orig_pt.x, orig_pt.y).unwrap();
        map.remove_actor(old_idx, self.entity);
        // println!("changed : {}", old_idx);

        let (new_x, new_y) = map.try_wrap_xy(pos.x + self.dx, pos.y + self.dy).unwrap();
        pos.set(new_x, new_y);

        let new_idx = map.get_index(pos.x, pos.y).unwrap();
        map.add_actor(new_idx, self.entity, pos.blocks_move);

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

        let actor = entry.get_component::<Actor>().unwrap();

        let mut move_time = actor.act_time;
        if self.dx != 0 && self.dy != 0 {
            move_time = (1.42 * move_time as f32) as u32;
        }
        ActionResult::Done(move_time) // move time
    }
}

impl Action for MoveStepAction {
    fn execute(&mut self, level: &mut Level) -> ActionResult {
        match self.validate(level) {
            Some(res) => res,
            None => self.do_action(level),
        }
    }
}
