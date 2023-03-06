use crate::action::{idle::IdleAction, BoxedAction};
use crate::actor::Actor;
use gw_app::ecs::{Ecs, Entity};

pub fn ai_idle(ecs: &mut Ecs, entity: Entity) -> Option<BoxedAction> {
    let entry = ecs.world.entry(entity).unwrap();
    let actor = entry.get_component::<Actor>().unwrap();
    let time = actor.act_time;

    Some(Box::new(IdleAction::new(entity, time)))
}
