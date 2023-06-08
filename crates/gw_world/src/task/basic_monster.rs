use super::{execute_actor_action, get_hero_entity, TaskResult};
use crate::{
    action::{idle::IdleAction, move_step::MoveStepAction, BoxedAction},
    being::Being,
    fov::FOV,
    map::Map,
    position::Position,
};
use gw_ecs::prelude::{Entity, Fetch, ReadComp, World};
use gw_util::path::a_star_search;

pub fn basic_monster_ai(world: &mut World, entity: Entity) -> TaskResult {
    let hero_entity = get_hero_entity(world);

    let (beings, positions) = <(ReadComp<Being>, ReadComp<Position>)>::fetch(world);

    let being = beings.get(entity).unwrap();
    let pos = positions.get(entity).unwrap();

    let (act_time, _ai_flags) = (being.act_time, being.ai_flags.clone());
    let mons_pt = pos.point();

    // Can player see me?  If so, I can see player...
    let can_see_player = match world.try_read_resource::<FOV>() {
        None => {
            // TODO - do a line test?
            false
        }
        Some(fov) => fov.is_visible(mons_pt.x, mons_pt.y),
    };

    let action: BoxedAction = if !can_see_player {
        Box::new(IdleAction::new(entity, act_time as u32))
    } else {
        // world.set_update_sidebar(); // something is probably going to change
        let hero_pos = positions.get(hero_entity).unwrap();
        // let (_hero_being, hero_pos) = query.get(&level.world, hero_entity).unwrap();

        let map = world.read_resource::<Map>();
        match a_star_search(mons_pt, hero_pos.point(), &*map, false) {
            Some(path) => {
                if path.len() > 1 {
                    let step = path[1] - path[0];
                    Box::new(MoveStepAction::new(entity, step.x, step.y))
                // } else if path.len() == 1 {
                //     // next to player
                //     // return TalkAction::new(entity, "Tag, you are it");
                //     Box::new(MeleeAction::new(entity, hero_entity))
                } else {
                    Box::new(IdleAction::new(entity, act_time as u32))
                }
            }
            None => {
                // Box::new(TalkAction::new(entity, "I see you")),
                Box::new(IdleAction::new(entity, act_time as u32))
            }
        }
    };

    drop(beings);
    drop(positions);

    return execute_actor_action(action, world, entity);
}
