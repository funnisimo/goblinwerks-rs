use crate::level::get_current_level;
use crate::map::Map;

use super::{
    parse_cure, parse_damage, parse_fixture, parse_heal, parse_message, parse_move_entity,
    parse_move_region, parse_poison, parse_portal, parse_restore_items, parse_store_items,
    parse_tile, parse_treasure,
};
use gw_app::{ecs::Entity, Ecs};
use gw_util::point::Point;
use gw_util::value::Value;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub enum EffectResult {
    Nothing, // Effect did not do anything
    Success,
    Fail,
    Stop,
}

pub trait Effect: Send + Sync + std::fmt::Debug + EffectClone {
    fn fire(&self, ecs: &mut Ecs, pos: Point, entity: Option<Entity>) -> EffectResult;
}

pub type BoxedEffect = Box<dyn Effect>;

// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
// paraphrasing answer by: https://stackoverflow.com/users/42353/dk

// Splitting [EffectClone] into its own trait allows us to provide a blanket
// implementation for all compatible types, without having to implement the
// rest of [Effect].  In this case, we implement it for all types that have
// 'static lifetime (*i.e.* they don't contain non-'static pointers), and
// implement both Effect and Clone.  Don't ask me how the compiler resolves
// implementing [EffectClone] for dyn [Effect] when [Effect] requires [EffectClone];
// I have *no* idea why this works.

pub trait EffectClone {
    fn clone_box(&self) -> Box<dyn Effect>;
}

impl<T> EffectClone for T
where
    T: 'static + Effect + Clone,
{
    fn clone_box(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Effect> {
    fn clone(&self) -> Box<dyn Effect> {
        self.clone_box()
    }
}

// End Effect Clone

pub type EffectParseFn = fn(&Value) -> Result<BoxedEffect, String>;

lazy_static! {
    static ref EFFECT_PARSERS: Mutex<HashMap<String, EffectParseFn>> = {
        let mut map: HashMap<String, EffectParseFn> = HashMap::new();
        map.insert("message".to_string(), parse_message);
        map.insert("move_entity".to_string(), parse_move_entity);
        map.insert("move_region".to_string(), parse_move_region);
        map.insert("damage".to_string(), parse_damage);
        map.insert("cure".to_string(), parse_cure);
        map.insert("heal".to_string(), parse_heal);
        map.insert("poison".to_string(), parse_poison);
        map.insert("tile".to_string(), parse_tile);
        map.insert("fixture".to_string(), parse_fixture);
        map.insert("store_items".to_string(), parse_store_items);
        map.insert("restore_items".to_string(), parse_restore_items);
        map.insert("treasure".to_string(), parse_treasure);
        map.insert("portal".to_string(), parse_portal);

        Mutex::new(map)
    };
}

pub fn register_effect_parser(id: &str, parser: EffectParseFn) {
    EFFECT_PARSERS
        .lock()
        .unwrap()
        .insert(id.to_string().to_lowercase(), parser);
}

pub fn parse_effect(id: &str, value: &Value) -> Result<BoxedEffect, String> {
    let parsers = EFFECT_PARSERS.lock().unwrap();
    match parsers.get(id.to_string().to_lowercase().as_str()) {
        None => Err(format!("No parser found for effect: {}", id)),
        Some(parser) => parser(value),
    }
}

pub fn parse_effects(value: &Value) -> Result<Vec<BoxedEffect>, String> {
    if !value.is_map() {
        return Err(format!("Effects must be map.  Found: {:?}", value));
    }
    let parsers = EFFECT_PARSERS.lock().unwrap();
    let map = value.as_map().unwrap();
    let mut output = Vec::new();
    for (key, val) in map.iter() {
        let id = key.to_string().to_lowercase();
        match parsers.get(&id) {
            None => return Err(format!("No parser found for effect: {}", id)),
            Some(parser) => match parser(val) {
                Err(e) => return Err(e),
                Ok(val) => output.push(val),
            },
        }
    }
    Ok(output)
}

pub fn fire_cell_action(
    ecs: &mut Ecs,
    pos: Point,
    action: &str,
    entity: Option<Entity>,
) -> EffectResult {
    // log(format!("Fire cell effects - {}", action));

    let effects = {
        let level = get_current_level(ecs);
        let map = level.resources.get::<Map>().unwrap();
        let idx = map.get_index(pos.x, pos.y).unwrap();
        match map.get_cell_effects(idx, &action.to_uppercase()) {
            None => return EffectResult::Success,
            Some(d) => d,
        }
    };

    // log(format!(" - effects - {:?}", effects));

    for eff in effects.iter() {
        match eff.fire(ecs, pos, entity) {
            EffectResult::Stop => {
                return EffectResult::Success;
            }
            EffectResult::Fail => {
                return EffectResult::Fail;
            }
            _ => {}
        }
    }

    EffectResult::Success
}
