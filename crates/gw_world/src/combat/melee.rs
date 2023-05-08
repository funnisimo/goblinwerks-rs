use super::DamageInfo;
use crate::effect::{parse_damage, BoxedEffect};
use crate::log::Logger;
use gw_ecs::{Component, DenseVecStorage};
use gw_ecs::{Entity, World};
use gw_util::value::Value;
use serde::{Deserialize, Serialize};

pub enum CombatParseError {
    InvalidValueType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum CombatMessage {
    #[default]
    Hit,
    Miss,
    Verb(String),
    Replace(String),
}

impl CombatMessage {
    pub fn log(&self, world: &mut World, attacker: Entity, target: Entity, damage: DamageInfo) {
        let attack_name = "attacker";
        let target_name = "target";
        let mut logger = world.write_global::<Logger>();
        match self {
            CombatMessage::Hit => {
                logger.log(format!("{} hit {} [{}]", attack_name, target_name, damage))
            }
            CombatMessage::Miss => logger.log(format!("{} miss {}", attack_name, target_name)),
            CombatMessage::Verb(v) => logger.log(format!(
                "{} {} {} [{}]",
                attack_name, v, target_name, damage
            )),
            CombatMessage::Replace(m) => logger.log(m),
        }
    }
}

#[derive(Debug, Default, Clone /* Serialize, Deserialize */)]
pub struct Attack {
    weight: u32, // choice from multiple chance
    chance: u32, // hit chance
    effects: Vec<BoxedEffect>,
    hit_msg: CombatMessage,
    miss_msg: CombatMessage,
}

pub fn parse_attack(value: &Value) -> Result<Attack, CombatParseError> {
    if value.is_string() {
        // This is a damage string - e.g. "1-5" or "1d6+1"
        let effect: BoxedEffect = match parse_damage(value) {
            Err(_) => return Err(CombatParseError::InvalidValueType),
            Ok(v) => v,
        };

        Ok(Attack {
            effects: vec![effect],
            ..Default::default()
        })
    } else if value.is_int() {
        // This is a damage constant value - e.g. 6
        let effect: BoxedEffect = match parse_damage(value) {
            Err(_) => return Err(CombatParseError::InvalidValueType),
            Ok(v) => v,
        };

        Ok(Attack {
            effects: vec![effect],
            ..Default::default()
        })
    } else if value.is_map() {
        let value_map = value.as_map().unwrap();

        let mut attack = Attack::default();

        // {
        //   weight: <INT> // pick weight of attack (to make some more likely than others) [default] = 100
        //   damage: <STRING> | <INT>
        if let Some(damage_value) = value_map.get(&"damage".into()) {
            let effect: BoxedEffect = match parse_damage(damage_value) {
                Err(_) => return Err(CombatParseError::InvalidValueType),
                Ok(v) => v,
            };
            attack.effects.push(effect);
        }

        //   verb: <STRING> // e.g.[default] - "hit~" for "hit" or "hits"
        //   miss_verb: <STRING> // e.g. [default] - "miss~~" for "miss" or "misses"
        //   chance: <INT>  // chance to hit out of 100 - default = 100
        if let Some(chance_value) = value_map.get(&"chance".into()) {
            let chance = chance_value
                .as_int()
                .ok_or(CombatParseError::InvalidValueType)?;
            attack.chance = chance as u32;
        }

        //   mp: <INT> // magic point cost for attack per use
        //   hit_msg: <STRING> // e.g. [default] - "{attacker} {verb} {target} [{damage}]"
        //   miss_msg: <STRING> // e.g. [default] - "{attacker} {verb} {target}"
        // }

        Ok(attack)
    } else {
        Err(CombatParseError::InvalidValueType)
    }
}

#[derive(Debug, Clone, Component /*Serialize, Deserialize */)]
pub struct Melee {
    attacks: Vec<Attack>,
}

impl Melee {
    pub fn new() -> Self {
        Melee {
            attacks: Vec::new(),
        }
    }
}

pub fn parse_melee(value: &Value) -> Result<Melee, CombatParseError> {
    if value.is_list() {
        Err(CombatParseError::InvalidValueType)
    } else if value.is_string() {
        Err(CombatParseError::InvalidValueType)
    } else if value.is_map() {
        let mut melee = Melee::new();

        let attack = match parse_attack(value) {
            Ok(a) => a,
            Err(e) => return Err(e),
        };
        melee.attacks.push(attack);
        Ok(melee)
    } else {
        Err(CombatParseError::InvalidValueType)
    }
}
