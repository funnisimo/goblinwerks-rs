use super::{Tile, TileFlags, TileKind, TileLayer, TileMove};
use crate::effect::{parse_effect, BoxedEffect, Portal};
use crate::sprite::parse_glyph;
use crate::sprite::Sprite;
use gw_app::color::get_color;
use gw_app::{log, Glyph, RGBA};
use gw_util::value::{Key, Value};
use std::sync::Arc;

pub struct TileBuilder {
    tile: Tile,
    layer_set: bool,
}

impl TileBuilder {
    pub fn new(id: &str) -> Self {
        TileBuilder {
            tile: Tile::new(id),
            layer_set: false,
        }
    }

    pub fn kind(mut self, kind: TileKind) -> Self {
        self.tile.kind.insert(kind);
        self
    }

    pub fn layer(mut self, layer: TileLayer) -> Self {
        self.tile.layer = layer;
        self
    }

    pub fn sprite(mut self, glyph: Glyph, fg: RGBA, bg: RGBA) -> Self {
        self.tile.glyph = glyph;
        self.tile.fg = fg;
        self.tile.bg = bg;
        self
    }

    pub fn glyph(mut self, glyph: Glyph) -> Self {
        self.tile.glyph = glyph;
        self
    }

    pub fn fg(mut self, fg: RGBA) -> Self {
        self.tile.fg = fg;
        self
    }

    pub fn bg(mut self, bg: RGBA) -> Self {
        self.tile.bg = bg;
        self
    }

    pub fn flavor(mut self, text: &str) -> Self {
        self.tile.flavor = text.to_string();
        self
    }

    pub fn description(mut self, text: &str) -> Self {
        self.tile.description = text.to_string();
        self
    }

    pub fn blocks(mut self) -> Self {
        self.tile.move_flags |= TileMove::BLOCKS_MOVE;
        self
    }

    pub fn blocks_vision(mut self) -> Self {
        self.tile.move_flags |= TileMove::BLOCKS_VISION;
        self
    }

    pub fn flags<T>(mut self, flags: T) -> Self
    where
        T: Into<TileFlags>,
    {
        self.tile.flags |= flags.into();
        self
    }

    pub fn move_flags<T>(mut self, flags: T) -> Self
    where
        T: Into<TileMove>,
    {
        self.tile.move_flags |= flags.into();
        self
    }

    pub fn action(mut self, action: String, tile_event: BoxedEffect) -> Self {
        self.add_effect(action.to_uppercase(), tile_event);
        self
    }

    fn add_effect<S: ToString>(&mut self, action: S, tile_event: BoxedEffect) {
        let action = action.to_string().to_uppercase();
        match self.tile.effects.get_mut(&action) {
            None => {
                self.tile.effects.insert(action, vec![tile_event]);
            }
            Some(data) => data.push(tile_event),
        }
    }

    fn add_effects<S: ToString>(&mut self, action: S, value: &Value) -> Result<(), String> {
        let action = action.to_string().to_uppercase();
        if value.is_map() {
            let map = value.as_map().unwrap();
            for (key, val) in map.iter() {
                let event = key.to_string();

                match parse_effect(&event, val) {
                    Ok(ev) => self.add_effect(&action, ev),
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        } else {
            Err(format!("Actions must be an object, found: {:?}", value))
        }
    }

    pub fn set(&mut self, field: &Key, value: &Value) -> Result<(), String> {
        let field_str = field.to_string().to_lowercase();
        match field_str.as_str() {
            "sprite" => {
                let text = value.to_string();
                // log(format!("parse sprite for tile - {}", value));
                let sprite: Sprite = match text.parse() {
                    Err(e) => {
                        log(format!(
                            "Failed to parse sprite for tile - {} - {:?}",
                            text,
                            text.chars().collect::<Vec<char>>()
                        ));
                        return Err(format!("Failed to parse sprite : {} - {}", value, e));
                    }
                    Ok(sprite) => sprite,
                };
                self.tile.glyph = sprite.glyph;
                self.tile.fg = sprite.fg;
                self.tile.bg = sprite.bg;
            }
            "ch" | "glyph" => {
                if value.is_int() {
                    self.tile.glyph = value.as_int().unwrap() as Glyph;
                } else {
                    self.tile.glyph = parse_glyph(&value.to_string()).expect("Unknown glyph");
                }
            }
            "fg" => {
                self.tile.fg = get_color(&value.to_string()).expect("Unknown fg color");
            }
            "bg" => {
                self.tile.bg = get_color(&value.to_string()).expect("Unknown bg color");
            }
            "kind" => {
                let text = value.to_string();
                let kind: TileKind = match text.parse() {
                    Err(e) => return Err(format!("Failed to parse kind : {} - {}", value, e)),
                    Ok(kind) => kind,
                };
                if kind == TileKind::FIXTURE && !self.layer_set {
                    self.tile.layer = TileLayer::FIXTURE;
                }
                self.tile.kind = kind;
            }
            "flavor" => {
                self.tile.flavor = value.to_string();
            }
            "flags" => {
                self.tile.flags.apply(&value.to_string());
            }
            "move" => {
                self.tile.move_flags.apply(&value.to_string());
            }
            "blocks" => match value.to_string().as_str() {
                "true" => {
                    self.tile.move_flags.insert(TileMove::BLOCKS_ALL);
                }
                "move" => {
                    self.tile.move_flags.insert(TileMove::BLOCKS_MOVE);
                }
                "vision" | "sight" => {
                    self.tile.move_flags.insert(TileMove::BLOCKS_VISION);
                }
                _ => panic!("Unknown 'blocks' value for tile - {}", value),
            },
            "layer" => {
                self.tile.layer = value.to_string().parse().unwrap();
                self.layer_set = true;
            }

            // Tile Actions
            "use" => return self.add_effects("use", value),
            "descend" => {
                if value.is_string() {
                    let map_id = value.to_string();
                    let ev = Box::new(Portal::new(map_id, "START".to_string()));
                    self.add_effect("descend", ev);
                } else {
                    return self.add_effects("descend", value);
                }
            }
            "climb" => {
                if value.is_string() {
                    let map_id = value.to_string();
                    let ev = Box::new(Portal::new(map_id, "START".to_string()));
                    self.add_effect("climb", ev);
                } else {
                    return self.add_effects("climb", value);
                }
            }
            "enter" => return self.add_effects("enter", value),
            "exit" => return self.add_effects("exit", value),
            "lock" => return self.add_effects("lock", value),
            "unlock" => return self.add_effects("unlock", value),
            "open" => return self.add_effects("open", value),
            "close" => return self.add_effects("close", value),
            "drop" => return self.add_effects("drop", value),

            _ => log(format!("Ignoring tile field - {}", field)),
        }
        Ok(())
    }

    pub fn build(self) -> Arc<Tile> {
        Arc::new(self.tile)
    }
}

#[cfg(test)]
mod test {
    use super::TileBuilder;
    use crate::effect::Message;
    use gw_app::{color::init_colors, Glyph, RGBA};
    use gw_util::{json, value::Value};

    #[test]
    fn basic() {
        let builder = TileBuilder::new("ID").glyph('@' as Glyph);

        let tile = builder.build();

        assert_eq!(tile.id.as_str(), "ID");
        assert_eq!(tile.glyph, '@' as Glyph);
    }

    #[test]
    fn basic_json() {
        init_colors();

        let mut builder = TileBuilder::new("ID");

        builder.set(&"glyph".into(), &"@".into()).unwrap();
        builder.set(&"fg".into(), &"red".into()).unwrap();
        builder.set(&"bg".into(), &"#00F".into()).unwrap();

        let tile = builder.build();

        assert_eq!(tile.id.as_str(), "ID");
        assert_eq!(tile.glyph, '@' as Glyph);
        assert_eq!(tile.fg, RGBA::rgb(255, 0, 0));
        assert_eq!(tile.bg, RGBA::rgb(0, 0, 255));
    }

    #[test]
    fn basic_action() {
        let builder = TileBuilder::new("ID").action("use".into(), Box::new(Message::new("Test")));

        let tile = builder.build();

        assert_eq!(tile.effects.get("USE").unwrap().len(), 1);
    }

    #[test]
    fn json_actions() {
        let mut builder = TileBuilder::new("ID");

        let json = r#"{
            "message": "Tacos Everyday!" 
        }"#;

        let json_value = json::parse_string(&json).unwrap();

        builder.set(&"use".into(), &json_value).unwrap();

        let tile = builder.build();

        assert_eq!(tile.id.as_str(), "ID");
        assert_eq!(tile.effects.get("USE").unwrap().len(), 1);
    }

    #[test]
    fn json_actions_simple() {
        let mut builder = TileBuilder::new("ID");

        let json_value: Value = "MAP_ID".into();

        builder.set(&"descend".into(), &json_value).unwrap();

        let tile = builder.build();

        assert_eq!(tile.id.as_str(), "ID");
        assert_eq!(tile.effects.get("DESCEND").unwrap().len(), 1);
    }
}
