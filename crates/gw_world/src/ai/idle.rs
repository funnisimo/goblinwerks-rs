use crate::action::{idle::IdleAction, BoxedAction};
use crate::actor::Actor;
use crate::level::Levels;
use gw_app::ecs::Entity;
use gw_app::Ecs;

pub fn ai_idle(ecs: &mut Ecs, entity: Entity) -> Option<BoxedAction> {
    let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
    let level = levels.current_mut();
    let entry = level.world.entry(entity).unwrap();
    let actor = entry.get_component::<Actor>().unwrap();
    let time = actor.act_time;

    Some(Box::new(IdleAction::new(entity, time)))
}
