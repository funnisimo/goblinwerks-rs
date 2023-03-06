use crate::action::BoxedAction;
use gw_app::ecs::{Ecs, Entity};

// mod mirror_player;
// pub use mirror_player::MirrorPlayerAI;

// mod move_randomly;
// pub use move_randomly::MoveRandomly;

// mod player;
// pub use player::PlayerAI;

pub mod idle;
pub mod mirror_player;

// mod basic_monster;
// pub use basic_monster::BasicMonster;

pub type AiFn = fn(&mut Ecs, Entity) -> Option<BoxedAction>;

// pub trait AI: Send + Sync {
//     fn next_action(&self, ecs: &mut Ecs, entity: Entity) -> Option<BoxedAction>;
// }

// pub type BoxedAI = Box<dyn AI>;

// pub type AiFn = dyn Fn(&mut Ecs, Entity) -> Option<BoxedAction> + 'static + Send + Sync;
// pub type BoxedAiFn = Box<AiFn>;

// impl<F> AI for F
// where
//     F: Fn(&mut Ecs, Entity) -> Option<BoxedAction> + 'static + Send + Sync,
// {
//     fn next_action(&self, ecs: &mut Ecs, entity: Entity) -> Option<BoxedAction> {
//         (self)(ecs, entity)
//     }
// }

// impl AI {
//     pub fn next_action(&mut self, entity: EntityId, world: &World) -> Action {
//         match world.get_actor(entity) {
//             Some(actor) => {
//                 if actor.borrow().is_dead() {
//                     return DeadAction::new(entity);
//                 }
//                 match actor.borrow_mut().take_action() {
//                     Some(action) => {
//                         return action;
//                     }
//                     _ => (),
//                 }
//             }
//             _ => (),
//         }

//         match self {
//             AI::Player(ai) => ai.next_action(entity, world),
//             AI::Idle(ai) => ai.next_action(entity, world),
//             AI::MirrorPlayer(ai) => ai.next_action(entity, world),
//             AI::MoveRandomly(ai) => ai.next_action(entity, world),
//             AI::BasicMonster(ai) => ai.next_action(entity, world),
//         }
//     }
// }
