use super::{BoxedEffect, Effect, EffectResult};
use crate::log::Logger;
use gw_ecs::prelude::{Entity, World};
use gw_util::point::Point;
use gw_util::value::Value;

////////////////////////

#[derive(Debug, Clone)]
pub struct StoreItems;

impl Effect for StoreItems {
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut logger = world.write_global::<Logger>();
        logger.log(format!("StoreItems!"));
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
    fn fire(&self, world: &mut World, _pos: Point, _entity: Option<Entity>) -> EffectResult {
        let mut logger = world.write_global::<Logger>();

        logger.log(format!("RestoreItems!"));
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
