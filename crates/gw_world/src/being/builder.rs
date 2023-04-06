use super::{Being, BeingKind, BeingKindFlags};
use crate::{
    ai::Actor,
    sprite::{Sprite, SpriteParseError},
};
use gw_app::{Glyph, RGBA};
use gw_util::value::Value;
use std::sync::Arc;

pub struct BeingKindBuilder {
    pub(super) id: String,
    pub(super) sprite: Sprite,
    pub(super) info: Being,
    pub(super) flags: BeingKindFlags,
    pub(super) actor: Actor,
}

impl BeingKindBuilder {
    pub(super) fn new(id: &str) -> Self {
        BeingKindBuilder {
            id: id.to_string(),
            sprite: Sprite::default(),
            info: Being::new(id.to_string()),
            flags: BeingKindFlags::empty(),
            actor: Actor::new("IDLE".to_string()),
        }
    }

    /// need to call this first
    pub fn extend(&mut self, kind: &Arc<BeingKind>) -> &mut Self {
        self.sprite = kind.sprite.clone();
        self.info = kind.being.clone();
        self.flags = kind.flags.clone();
        self.actor = kind.actor.clone();
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
        self.actor.ai.reset(ai);
        self
    }

    pub fn hero(&mut self) -> &mut Self {
        self.flags.insert(BeingKindFlags::HERO);
        self
    }

    pub fn talk(&mut self, talk: &str) -> &mut Self {
        self.info.talk = Some(talk.to_string());
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.info.name = Some(name.to_string());
        self
    }

    pub fn flavor(&mut self, flavor: &str) -> &mut Self {
        self.info.flavor = Some(flavor.to_string());
        self
    }

    pub fn description(&mut self, description: &str) -> &mut Self {
        self.info.description = Some(description.to_string());
        self
    }

    pub fn move_flags(&mut self, flag_string: &str) -> &mut Self {
        self.info.move_flags.apply(flag_string);
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
        "hero" => {
            // {"hero": true}
            builder.hero();
            Ok(())
        }
        "move" => {
            builder.move_flags(&value.to_string());
            Ok(())
        }
        _ => Err(BuilderError::UnknownField(field.to_string())),
    }
}
