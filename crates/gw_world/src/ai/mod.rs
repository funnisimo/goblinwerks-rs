use crate::action::BoxedAction;
use crate::ai::idle::ai_idle;
use crate::ai::user::ai_user_control;
use crate::level::Level;
use gw_app::ecs::Entity;
use gw_app::{log, Ecs};
// use mirror_entity::MirrorEntity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// mod mirror_player;
// pub use mirror_player::MirrorPlayerAI;

// mod move_randomly;
// pub use move_randomly::MoveRandomly;

// mod player;
// pub use player::PlayerAI;

mod ai_flags;
pub mod idle;
// pub mod mirror_entity;
pub mod user;

pub use ai_flags::AIFlags;

// mod basic_monster;
// pub use basic_monster::BasicMonster;

pub type AiFn = fn(&mut Ecs, Entity) -> Option<BoxedAction>;

// #[allow(unused_variables)]
// pub trait AiHandler: Send + Sync {
//     fn on_enter(&self, ecs: &mut Ecs, entity: Entity) -> () {}
//     fn next_action(&self, ecs: &mut Ecs, entity: Entity) -> Option<BoxedAction>;
//     fn on_exit(&self, ecs: &mut Ecs, entity: Entity) -> () {}
// }

// impl<F> AiHandler for F
// where
//     F: Fn(&mut Ecs, Entity) -> Option<BoxedAction> + Send + Sync,
// {
//     fn next_action(&self, ecs: &mut Ecs, entity: Entity) -> Option<BoxedAction> {
//         (self)(ecs, entity)
//     }
// }

// pub type BoxedAiHandler = Box<dyn AiHandler>;

lazy_static::lazy_static! {
    // pub static ref AI_HANDLERS: Mutex<HashMap<String,Arc<dyn AiHandler>>> = {
    //     let mut handlers: HashMap<String,Arc<dyn AiHandler>> = HashMap::new();
    //     handlers.insert("IDLE".to_string(), Arc::new(ai_idle));
    //     handlers.insert("USER_CONTROL".to_string(), Arc::new(ai_user_control));
    //     handlers.insert("MIRROR_ENTITY".to_string(), Arc::new(MirrorEntity));
    //     Mutex::new(handlers)
    // };

    // pub static ref DEFAULT_AI: Arc<dyn AiHandler> = Arc::new(ai_idle);
    pub static ref AI_HANDLERS: Mutex<HashMap<String,AiFn>> = {
        let mut handlers: HashMap<String,AiFn> = HashMap::new();
        handlers.insert("IDLE".to_string(), ai_idle);
        handlers.insert("USER_CONTROL".to_string(), ai_user_control);
        // handlers.insert("MIRROR_ENTITY".to_string(), MirrorEntity);
        Mutex::new(handlers)
    };

    pub static ref DEFAULT_AI: AiFn = ai_idle;
}

pub fn register_ai(name: &str, handler: AiFn) {
    AI_HANDLERS
        .lock()
        .unwrap()
        .insert(name.to_string(), handler);
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct AI {
    stack: Vec<String>,
}

impl AI {
    pub fn new() -> Self {
        AI {
            stack: vec!["IDLE".to_string()],
        }
    }

    pub fn reset(&mut self, ai: &str) {
        self.stack = vec![ai.to_string()];
    }

    pub fn push(&mut self, name: &str) {
        // TODO - Validate name
        self.stack.push(name.to_string());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn replace(&mut self, name: &str) {
        self.stack.pop();
        self.stack.push(name.to_string());
    }

    pub fn current(&self) -> AiFn {
        let handlers = AI_HANDLERS.lock().unwrap();

        let name = match self.stack.last() {
            None => {
                log("NO AI");
                return DEFAULT_AI.clone();
            }
            Some(name) => name,
        };

        match handlers.get(name) {
            None => {
                log(format!(
                    "Could not find AI - {}, choices = {:?}",
                    name,
                    handlers.keys()
                ));
                DEFAULT_AI.clone()
            }
            Some(handler) => handler.clone(),
        }
    }
}

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
