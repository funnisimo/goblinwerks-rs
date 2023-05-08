use gw_ecs::{Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;
use gw_world::{
    effect::{BoxedEffect, Effect, EffectResult},
    log::Logger,
};

////////////////////////

#[derive(Debug, Clone)]
pub struct Mark(String);

impl Mark {
    pub fn new(name: String) -> Self {
        Mark(name)
    }
}

impl Effect for Mark {
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut logger = world.write_global::<Logger>();
        logger.log(format!("Mark = {}", self.0));
        EffectResult::Success
    }
}

pub fn parse_mark(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_string() {
        Ok(Box::new(Mark::new(value.to_string())))
    } else {
        Err(format!(
            "Mark effects can only receive string values.  Received: {:?}",
            value
        ))
    }
}
