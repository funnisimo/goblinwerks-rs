use super::{BoxedEffect, Effect, EffectResult};
use crate::level::Levels;
use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct StoreItems;

impl Effect for StoreItems {
    fn fire(&self, ecs: &mut Ecs, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        level.logger.log(format!("StoreItems!"));
        EffectResult::Success
    }
}

pub(super) fn parse_store_items(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(StoreItems))
    } else {
        Err(format!(
            "Store Items effect can only receive bool values.  Received: {:?}",
            value
        ))
    }
}

////////////////////////

#[derive(Debug, Clone)]
pub struct RestoreItems;

impl Effect for RestoreItems {
    fn fire(&self, ecs: &mut Ecs, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut levels = ecs.resources.get_mut::<Levels>().unwrap();
        let level = levels.current_mut();

        level.logger.log(format!("RestoreItems!"));
        EffectResult::Success
    }
}

pub(super) fn parse_restore_items(value: &Value) -> Result<BoxedEffect, String> {
    if value.is_bool() {
        Ok(Box::new(RestoreItems))
    } else {
        Err(format!(
            "Restore Items effect can only receive bool values.  Received: {:?}",
            value
        ))
    }
}
