use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;
use gw_world::effect::{BoxedEffect, Effect, EffectResult};
use gw_world::level::Levels;

////////////////////////

#[derive(Debug, Clone)]
pub struct Gremlins;

impl Gremlins {
    pub fn new() -> Self {
        Gremlins
    }
}

impl Effect for Gremlins {
    fn fire(&self, ecs: &mut Ecs, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        level.logger.log("Gremlins");
        EffectResult::Success
    }
}

pub fn parse_gremlins(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(Gremlins))
    } else {
        Err(format!(
            "Gremlins effects can only receive bool values.  Received: {:?}",
            value
        ))
    }
}
