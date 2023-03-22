use super::{BoxedEffect, Effect, EffectResult};
use crate::level::Levels;
use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct Message(String);

impl Message {
    pub fn new(text: &str) -> Self {
        Message(text.to_string())
    }
}

impl Effect for Message {
    fn fire(&self, ecs: &mut Ecs, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        level.logger.log(self.0.clone());
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
