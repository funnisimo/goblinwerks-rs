use super::{BoxedEffect, Effect, EffectResult};
use crate::log::Logger;
use gw_ecs::prelude::{Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct Message(pub(super) String);

impl Message {
    pub fn new(text: &str) -> Self {
        Message(text.to_string())
    }
}

impl Effect for Message {
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut logger = world.write_global::<Logger>();

        logger.log(self.0.clone());
        EffectResult::Success
    }
}

pub(super) fn parse_message(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_string() {
        Ok(Box::new(Message(value.to_string())))
    } else {
        Err(format!(
            "Message tile events can only receive string values.  Received: {:?}",
            value
        ))
    }
}
