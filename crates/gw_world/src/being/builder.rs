use super::{Being, BeingKind, BeingKindFlags, BeingKinds, Stat, Stats};
use crate::{
    combat::{parse_melee, Melee},
    sprite::{Sprite, SpriteParseError},
};
use gw_app::{log, Glyph, RGBA};
use gw_util::value::Value;
use std::{collections::HashMap, sync::Arc};

pub struct BeingKindBuilder {
    pub(super) id: String,
    pub(super) sprite: Sprite,
    pub(super) being: Being,
    pub(super) task: String,
    pub(super) melee: Option<Melee>,
    pub(super) stats: Stats,
}

impl BeingKindBuilder {
    pub(super) fn new(id: &str) -> Self {
        BeingKindBuilder {
            id: id.to_string(),
            sprite: Sprite::default(),
            being: Being::new(id.to_string()),
            task: "IDLE".to_string(),
            melee: None,
            stats: Stats::new(),
        }
    }

    /// need to call this first
    pub fn extend(&mut self, kind: &Arc<BeingKind>) -> &mut Self {
        self.sprite = kind.sprite.clone();
        self.being = kind.being.clone();
        self.task = kind.task.clone();
        self.melee = kind.melee.clone();
        self.stats = kind.stats.clone();
        self
    }

    pub fn glyph(&mut self, glyph: Glyph) -> &mut Self {
        self.sprite.glyph = glyph;
        self
    }

    pub fn fg(&mut self, fg: RGBA) -> &mut Self {
        self.sprite.fg = fg;
        self
    }

    pub fn bg(&mut self, bg: RGBA) -> &mut Self {
        self.sprite.bg = bg;
        self
    }

    pub fn sprite(&mut self, sprite: Sprite) -> &mut Self {
        self.sprite = sprite;
        self
    }

    pub fn ai(&mut self, ai: &str) -> &mut Self {
        self.task = ai.to_string();
        self
    }

    pub fn hero(&mut self) -> &mut Self {
        self.being.kind_flags.insert(BeingKindFlags::HERO);
        self
    }

    pub fn talk(&mut self, talk: &str) -> &mut Self {
        self.being.talk = Some(talk.to_string());
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.being.name = Some(name.to_string());
        self
    }

    pub fn flavor(&mut self, flavor: &str) -> &mut Self {
        self.being.flavor = Some(flavor.to_string());
        self
    }

    pub fn description(&mut self, description: &str) -> &mut Self {
        self.being.description = Some(description.to_string());
        self
    }

    pub fn move_flags(&mut self, flag_string: &str) -> &mut Self {
        self.being.move_flags.apply(flag_string);
        self
    }

    pub fn apply_flags(&mut self, flag_string: &str) -> &mut Self {
        self.being.kind_flags.apply(flag_string);
        self.being.move_flags.apply(flag_string);
        self.being.ai_flags.apply(flag_string);
        self
    }

    pub fn xp(&mut self, xp: u32) -> &mut Self {
        self.being.xp = xp;
        self
    }

    pub fn melee(&mut self, melee: Melee) -> &mut Self {
        self.melee = Some(melee);
        self
    }

    pub fn no_melee(&mut self) -> &mut Self {
        self.melee = None;
        self
    }

    pub fn build(self) -> Arc<BeingKind> {
        Arc::new(BeingKind::new(self))
    }
}

/*
   JSON format:
   "ID": {
       "sprite": "<SPRITE_CONFIG>",
       --or--
       "glyph" | "ch": "ch" || ###,
       "fg": "<RGBA_CONFIG>",
       "bg": "<RGBA_CONFIG>",

       "flavor": <STRING>,
       "description": <STRING>
   }
*/

#[derive(Debug, Clone)]
pub enum BuilderError {
    BadSprite(SpriteParseError),
    UnknownField(String),
    BadField(String, Value),
}

pub fn set_field(
    builder: &mut BeingKindBuilder,
    field: &str,
    value: &Value,
) -> Result<(), BuilderError> {
    match field {
        "sprite" => {
            let sprite: Sprite = match value.try_into() {
                Err(e) => return Err(BuilderError::BadSprite(e)),
                Ok(s) => s,
            };
            builder.sprite(sprite);
            Ok(())
        }
        "glyph" | "ch" => {
            if value.is_int() {
                builder.glyph(value.as_int().unwrap() as Glyph);
                Ok(())
            } else {
                let text = value.to_string();
                if text.len() == 0 {
                    return Err(BuilderError::BadSprite(SpriteParseError::BadGlyph(text)));
                }
                let ch = text.chars().next().unwrap();
                builder.glyph(ch as Glyph);
                Ok(())
            }
        }
        "fg" => match value.try_into() {
            Err(e) => Err(BuilderError::BadSprite(SpriteParseError::BadForeColor(e))),
            Ok(c) => {
                builder.fg(c);
                Ok(())
            }
        },
        "bg" => match value.try_into() {
            Err(e) => Err(BuilderError::BadSprite(SpriteParseError::BadBackColor(e))),
            Ok(c) => {
                builder.bg(c);
                Ok(())
            }
        },
        "flavor" => {
            builder.flavor(&value.to_string());
            Ok(())
        }
        "description" => {
            builder.description(&value.to_string());
            Ok(())
        }
        "ai" => {
            // {"ai": <STRING>}
            builder.ai(&value.to_string().to_uppercase());
            Ok(())
        }
        "task" => {
            // {"task": <STRING>}
            builder.ai(&value.to_string().to_uppercase());
            Ok(())
        }
        "hero" => {
            // {"hero": true}
            builder.hero();
            Ok(())
        }
        "move" => {
            builder.move_flags(&value.to_string());
            Ok(())
        }
        "flags" => {
            builder.apply_flags(&value.to_string());
            Ok(())
        }
        "xp" => match value.as_int() {
            None => Err(BuilderError::BadField("xp".to_string(), value.clone())),
            Some(c) => {
                builder.xp(c as u32);
                Ok(())
            }
        },
        "ranged" => Ok(()),
        "melee" => {
            if value.is_bool() {
                if value.as_bool().unwrap() == false {
                    builder.no_melee();
                }
                Ok(())
            } else {
                match parse_melee(value) {
                    Err(_) => Err(BuilderError::BadField("melee".to_string(), value.clone())),
                    Ok(melee) => {
                        builder.melee(melee);
                        Ok(())
                    }
                }
            }
        }
        "health" => {
            if value.is_int() {
                let value = value.as_int().unwrap();
                builder.stats.set(Stat::HEALTH, value as i32);
                Ok(())
            } else {
                Err(BuilderError::BadField("health".to_string(), value.clone()))
            }
        }
        "mp" => {
            if value.is_int() {
                let value = value.as_int().unwrap();
                builder.stats.set(Stat::MAGIC, value as i32);
                Ok(())
            } else {
                Err(BuilderError::BadField("mp".to_string(), value.clone()))
            }
        }
        _ => Err(BuilderError::UnknownField(field.to_string())),
    }
}

#[derive(Debug)]
pub enum BeingKindError {
    UnknownId(String),
    MissingKind,
    InvalidValueType(Value),
}

pub fn from_value(
    value: &Value,
    being_kinds: &BeingKinds,
    suffix: &str,
    group_info: Option<&HashMap<String, HashMap<String, Value>>>,
) -> Result<Arc<BeingKind>, BeingKindError> {
    log(format!("Being - {:?}", value));
    if value.is_string() {
        match being_kinds.get(&value.to_string().to_uppercase()) {
            None => return Err(BeingKindError::UnknownId(value.to_string())),
            Some(k) => return Ok(k),
        };
    } else if value.is_map() {
        let map = value.as_map().unwrap();

        if let Some(kind_value) = map.get(&"kind".into()) {
            let id = format!("{}_{}", kind_value.to_string().to_uppercase(), suffix);
            let mut builder = BeingKind::builder(&id);

            match being_kinds.get(&kind_value.to_string().to_uppercase()) {
                None => panic!(
                    "Being kind extends missing being - {}",
                    kind_value.to_string()
                ),
                Some(base) => {
                    builder.extend(&base);
                }
            }

            if let Some(group_info) = group_info {
                // // ALL BEINGS HAVE THESE VALUES
                if let Some(group_values) = group_info.get("BEING") {
                    for (k, v) in group_values.iter() {
                        set_field(&mut builder, k, v).unwrap();
                    }
                }

                // ANY CUSTOM GROUP VALUES
                if let Some(groups) = map.get(&"groups".into()) {
                    for group in groups.to_string().split(&['|', ',', ':']).map(|v| v.trim()) {
                        if let Some(group_values) = group_info.get(group) {
                            for (k, v) in group_values.iter() {
                                set_field(&mut builder, k, v).unwrap();
                            }
                        }
                    }
                } else if let Some(group) = map.get(&"group".into()) {
                    let group = group.to_string();
                    if let Some(group_values) = group_info.get(group.trim()) {
                        for (k, v) in group_values.iter() {
                            set_field(&mut builder, k, v).unwrap();
                        }
                    }
                }
            }

            if let Some(talk) = map.get(&"talk".into()) {
                builder.talk(&talk.to_string());
            }

            if let Some(name) = map.get(&"name".into()) {
                builder.name(&name.to_string());
            }

            if let Some(ai) = map.get(&"ai".into()) {
                if ai.is_string() {
                    builder.ai(&ai.to_string());
                } else if ai.is_bool() {
                    if !ai.as_bool().unwrap() {
                        builder.ai("IDLE");
                    }
                }
            }

            let new_being = builder.build();
            println!("CUSTOM BEING - {:?}", new_being);
            Ok(new_being)
        } else {
            Err(BeingKindError::MissingKind)
        }
    } else {
        Err(BeingKindError::InvalidValueType(value.clone()))
    }
}
