use crate::action::idle::IdleAction;
use crate::action::{Action, ActionResult};
use crate::actor::Actor;
use crate::effect::fire_cell_action;
use crate::hero::Hero;
use crate::level::{get_current_level, get_current_level_mut, Level};
use crate::map::Map;
use crate::map::{cell_flavor, Cell};
use crate::position::Position;
use gw_app::ecs::systems::ResourceSet;
use gw_app::ecs::{Entity, EntityStore, Read};
use gw_app::{log, Ecs};
use gw_util::point::Point;

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

    fn validate(&mut self, ecs: &mut Ecs) -> Option<ActionResult> {
        let mut level = get_current_level_mut(ecs);
        let Level {
            resources,
            world,
            logger,
            ..
        } = &mut *level;

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

        let idx = match map.get_wrapped_index(orig_pt.x + self.dx, orig_pt.y + self.dy) {
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
        let act_time = actor.act_time;
        drop(actor);

        if map.is_blocked(idx) {
            // Check for actor at location...
            if let Some(entity) = map.iter_actors(idx).next() {
                if let Some(entity) = world.entry(entity) {
                    // Check for shopkeeper
                    // Check for talk...
                    if let Ok(other_actor) = entity.get_component::<Actor>() {
                        if let Some(ref talk) = other_actor.talk {
                            log(format!("{} says: '{}'", other_actor.name(), talk));
                            return Some(ActionResult::Replace(Box::new(IdleAction::new(
                                self.entity,
                                act_time,
                            ))));
                        }
                        // Check for combat?

                        // Should this be a different thing?
                        log(format!("{} says: 'Hello'", other_actor.name()));
                        return Some(ActionResult::Replace(Box::new(IdleAction::new(
                            self.entity,
                            act_time,
                        ))));
                    }
                }
            }

            let flavor = cell_flavor(&*map, world, idx);
            if actor_is_hero {
                logger.log(format!("Blocked by {}", flavor));
            }

            return Some(ActionResult::Replace(Box::new(IdleAction::new(
                self.entity,
                act_time,
            ))));
        }

        None
    }

    fn do_action(&mut self, ecs: &mut Ecs) -> ActionResult {
        // let Level {
        //     resources, world, ..
        // } = levels.current_mut();

        let orig_pt = {
            let mut level = get_current_level_mut(ecs);
            let entry = level.world.entry(self.entity).unwrap();
            let pos = entry.get_component::<Position>().unwrap();
            pos.point()
        };

        let (old_idx, new_idx, new_pt) = {
            let level = get_current_level(ecs);
            let map = level.resources.get::<Map>().unwrap();
            let (new_x, new_y) = map
                .try_wrap_xy(orig_pt.x + self.dx, orig_pt.y + self.dy)
                .unwrap();

            (
                map.get_wrapped_index(orig_pt.x, orig_pt.y).unwrap(),
                map.get_wrapped_index(new_x, new_y).unwrap(),
                Point::new(new_x, new_y),
            )
        };

        // TODO - How to check for permission to exit?
        fire_cell_action(ecs, orig_pt, "exit", Some(self.entity));

        // println!("changed : {}", old_idx);
        let blocks_move = {
            let mut level = get_current_level_mut(ecs);
            let mut entry = level.world.entry_mut(self.entity).unwrap();
            let pos = entry.get_component_mut::<Position>().unwrap();

            pos.set(new_pt.x, new_pt.y);
            pos.blocks_move
        };

        {
            let level = get_current_level_mut(ecs);
            let mut map = level.resources.get_mut::<Map>().unwrap();
            map.remove_actor(old_idx, self.entity);
            map.add_actor(new_idx, self.entity, blocks_move);
        }

        // TODO - How to check for permission to enter?
        fire_cell_action(ecs, new_pt, "enter", Some(self.entity));

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

        let mut level = get_current_level_mut(ecs);
        let entry = level.world.entry(self.entity).unwrap();
        let actor = entry.get_component::<Actor>().unwrap();

        let mut move_time = actor.act_time;
        if self.dx != 0 && self.dy != 0 {
            move_time = (1.42 * move_time as f32) as u32;
        }
        ActionResult::Done(move_time) // move time
    }
}

impl Action for MoveStepAction {
    fn execute(&mut self, ecs: &mut Ecs) -> ActionResult {
        if let Some(res) = self.validate(ecs) {
            return res;
        }

        self.do_action(ecs)
    }
}
