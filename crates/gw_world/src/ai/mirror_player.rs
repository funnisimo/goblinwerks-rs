use crate::action::{move_step::MoveStepAction, BoxedAction};
use crate::hero::Hero;
use crate::level::Level;
use crate::position::Position;
use gw_app::ecs::Entity;
use gw_util::point::Point;

struct MirrorState {
    last_xy: Point,
}

impl MirrorState {
    fn new(point: &Point) -> Self {
        MirrorState {
            last_xy: point.clone(),
        }
    }
}

pub fn ai_mirror_player(ecs: &mut Level, entity: Entity) -> Option<BoxedAction> {
    let hero_entity = ecs.resources.get::<Hero>().unwrap().entity;

    let hero_point = {
        let hero_entry = ecs.world.entry(hero_entity).unwrap();
        hero_entry.get_component::<Position>().unwrap().point()
    };

    let mut entry = ecs.world.entry(entity).unwrap();
    let last_point = match entry.get_component_mut::<MirrorState>() {
        Err(_) => {
            entry.add_component(MirrorState::new(&hero_point));
            hero_point.clone()
        }
        Ok(state) => {
            let last_xy = state.last_xy.clone();
            state.last_xy = hero_point;
            last_xy
        }
    };

    let delta = hero_point - last_point;
    let unit = delta.as_dir();

    // move in that direction
    Some(Box::new(MoveStepAction::new(entity, unit.x, unit.y)))
}
