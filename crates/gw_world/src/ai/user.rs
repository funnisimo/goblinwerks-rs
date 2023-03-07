// #[derive(Clone, Copy, Debug)]
// pub struct PlayerAI {}

// impl PlayerAI {
//     pub fn new() -> AI {
//         AI::Player(PlayerAI {})
//     }

//     pub fn next_action(&mut self, entity: Entity, _level: &World) -> Action {
//         NeedInputAction::new(entity)
//     }
// }

use crate::action::need_input::NeedInputAction;
use crate::action::BoxedAction;
use crate::level::Level;
use gw_app::ecs::Entity;

pub fn ai_user_control(_ecs: &mut Level, entity: Entity) -> Option<BoxedAction> {
    Some(Box::new(NeedInputAction::new(entity)))
}
