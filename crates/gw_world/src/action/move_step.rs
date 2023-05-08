use crate::action::idle::IdleAction;
use crate::action::{Action, ActionResult};
use crate::being::Being;
use crate::effect::fire_cell_action;
use crate::hero::Hero;
use crate::log::Logger;
use crate::map::cell_flavor;
use crate::map::Map;
use crate::position::Position;
use gw_app::log;
use gw_ecs::{Entity, ReadComp, ReadRes, SystemData, World, WriteRes};
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

    fn validate(&mut self, world: &mut World) -> Option<ActionResult> {
        let (map, hero, beings, positions, mut logger) = <(
            ReadRes<Map>,
            ReadRes<Hero>,
            ReadComp<Being>,
            ReadComp<Position>,
            WriteRes<Logger>,
        )>::fetch(world);

        let being = match beings.get(self.entity) {
            None => return Some(ActionResult::Dead(self.entity)),
            Some(a) => a,
        };

        if self.dx == 0 && self.dy == 0 {
            return Some(ActionResult::Replace(Box::new(IdleAction::new(
                self.entity,
                being.act_time,
            ))));
        }

        let pos = match positions.get(self.entity) {
            None => return Some(ActionResult::Dead(self.entity)),
            Some(pos) => pos,
        };

        let orig_pt = pos.point();

        let idx = match map.get_wrapped_index(orig_pt.x + self.dx, orig_pt.y + self.dy) {
            None => {
                log("Bump edge of world");
                return Some(ActionResult::Replace(Box::new(IdleAction::new(
                    self.entity,
                    being.act_time,
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
        let act_time = being.act_time;
        drop(being);

        if map.is_blocked(idx) {
            // Check for actor at location...
            if let Some(other) = map.iter_beings(idx).next() {
                // Check for shopkeeper (Really only possible for horses)

                // Check for talk...
                if let Some(other_being) = beings.get(other) {
                    if let Some(ref talk) = other_being.talk {
                        if actor_is_hero {
                            log(format!("{} says: '{}'", other_being.name(), talk));
                            return Some(ActionResult::Replace(Box::new(IdleAction::new(
                                self.entity,
                                act_time,
                            ))));
                        }
                    }
                    // Check for combat?
                    // if hero and will make other hostile ask for

                    // Should this be a different thing?
                    if actor_is_hero {
                        log(format!("{} says: 'Hello'", other_being.name()));
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

    fn do_action(&mut self, world: &mut World) -> ActionResult {
        // let Level {
        //     resources, world, ..
        // } = levels.current_mut();

        let orig_pt = world
            .read_component::<Position>()
            .get(self.entity)
            .unwrap()
            .point();

        let (old_idx, new_idx, new_pt) = {
            let map = world.read_resource::<Map>();
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
        fire_cell_action(world, orig_pt, "exit", Some(self.entity));

        // println!("changed : {}", old_idx);
        let blocks_move = {
            let mut positions = world.write_component::<Position>();
            let pos = positions.get_mut(self.entity).unwrap();
            pos.set(new_pt.x, new_pt.y);
            pos.blocks_move
        };

        {
            let mut map = world.write_resource::<Map>();
            map.remove_being(old_idx, self.entity);
            map.add_being(new_idx, self.entity, blocks_move);
        }

        // TODO - How to check for permission to enter?
        fire_cell_action(world, new_pt, "enter", Some(self.entity));

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

        let beings = world.read_component::<Being>();
        let actor = beings.get(self.entity).unwrap();

        let mut move_time = actor.act_time;
        if self.dx != 0 && self.dy != 0 {
            move_time = (1.42 * move_time as f32) as u32;
        }
        ActionResult::Done(move_time) // move time
    }
}

impl Action for MoveStepAction {
    fn execute(&mut self, world: &mut World) -> ActionResult {
        if let Some(res) = self.validate(world) {
            return res;
        }

        self.do_action(world)
    }
}
