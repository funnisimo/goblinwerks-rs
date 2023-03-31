use crate::action::{idle::IdleAction, BoxedAction};
use crate::actor::Actor;
use crate::level::get_current_level_mut;
use gw_app::ecs::Entity;
use gw_app::Ecs;

pub fn ai_idle(ecs: &mut Ecs, entity: Entity) -> Option<BoxedAction> {
    let mut level = get_current_level_mut(ecs);
    let entry = level.world.entry(entity).unwrap();
    let actor = entry.get_component::<Actor>().unwrap();
    let time = actor.act_time;

    Some(Box::new(IdleAction::new(entity, time)))
}
