use super::*;
use crate::sprite::Sprite;
use crate::treasure::Treasure;
use gw_app::color::named;
use gw_app::log;
use gw_app::Glyph;
use gw_app::RGBA;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;

lazy_static! {
    pub static ref NO_TILE: Arc<Tile> = TileBuilder::new("NONE")
        // .sprite(0 as Glyph, RGBA::new(), RGBA::new())
        .build();
}

#[derive(Debug, Default)]
pub struct Tile {
    pub id: String,
    pub glyph: Glyph,
    pub fg: RGBA,
    pub bg: RGBA,
    pub flags: TileFlags,
    pub move_flags: TileMove,
    pub liquid: TileLiquid,
    pub layer: TileLayer,
    pub actions: HashMap<TileAction, String>,
    pub kind: TileKind,
    pub treasure: Treasure,
    pub mimic: Option<Arc<Tile>>,
    pub object: usize, // TODO - Option<ItemKind>

    pub level: usize,
    pub rarity: u32,
    pub priority: u32,

    // pub edge: Option<Arc<Tile>>,    // What is this?
    pub flavor: String,
    pub description: String,
}

impl Tile {
    pub fn new(id: &str) -> Self {
        Tile {
            id: id.to_owned(),

            glyph: 0,
            fg: RGBA::rgba(0, 0, 0, 0),
            bg: RGBA::rgba(0, 0, 0, 0),

            flags: TileFlags::empty(),
            move_flags: TileMove::empty(),
            treasure: Treasure::empty(),
            kind: TileKind::empty(),
            actions: HashMap::new(),
            liquid: TileLiquid::NONE,
            layer: TileLayer::GROUND,

            mimic: None,
            object: 0,

            level: 0,
            rarity: 0,
            priority: 0,

            // edge: None,
            flavor: "".to_owned(),
            description: "".to_owned(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.kind.is_empty()
    }

    pub fn blocks(&self) -> bool {
        self.move_flags.intersects(TileMove::BLOCKS_MOVE)
    }

    pub fn blocks_vision(&self) -> bool {
        self.move_flags.intersects(TileMove::BLOCKS_VISION)
    }

    pub fn is_obstruction(&self) -> bool {
        self.move_flags.contains(TileMove::BLOCKS_DIAGONAL)
    }
}

pub struct TileBuilder {
    tile: Tile,
}

impl TileBuilder {
    pub fn new(id: &str) -> Self {
        TileBuilder {
            tile: Tile::new(id),
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

    pub fn set(&mut self, field: &str, value: &str) -> Result<(), String> {
        match field {
            "sprite" => {
                // log(format!("parse sprite for tile - {}", value));
                let sprite: Sprite = match value.parse() {
                    Err(e) => {
                        log(format!(
                            "Failed to parse sprite for tile - {} - {:?}",
                            value,
                            value.chars().collect::<Vec<char>>()
                        ));
                        return Err(format!("Failed to parse sprite : {} - {}", value, e));
                    }
                    Ok(sprite) => sprite,
                };
                self.tile.glyph = sprite.glyph;
                self.tile.fg = sprite.fg;
                self.tile.bg = sprite.bg;
            }
            "kind" => {
                let kind: TileKind = match value.parse() {
                    Err(e) => return Err(format!("Failed to parse kind : {} - {}", value, e)),
                    Ok(kind) => kind,
                };
                self.tile.kind = kind;
            }
            "flavor" => {
                self.tile.flavor = value.to_owned();
            }
            "flags" => {
                self.tile.flags.apply(value);
            }
            "move" => {
                self.tile.move_flags.apply(value);
            }
            "layer" => {
                self.tile.layer = value.parse().unwrap();
            }
            _ => return Err(format!("Unknown tile field - {}", field)),
        }
        Ok(())
    }

    pub fn build(self) -> Arc<Tile> {
        Arc::new(self.tile)
    }
}

// pub fn get_tile<'a>(idx: usize) -> Option<&'a Tile> {
//     match global_world().tiles().get(idx) {
//         None => None,
//         Some(tile) => Some(tile),
//     }
// }

// pub fn get_tile_with_name(name: &str) -> Option<&Tile> {
//     match global_world().tiles().get_index(&name.to_owned()) {
//         None => None,
//         Some(tile) => global_world().tiles().get(tile),
//     }
// }

// pub fn get_tile_idx(name: &str) -> usize {
//     match global_world().tiles().get_index(&name.to_owned()) {
//         None => 0,
//         Some(tile) => tile,
//     }
// }

pub fn load_default_tiles(cache: &mut Tiles) {
    cache.insert(NO_TILE.clone());
    cache.insert(
        TileBuilder::new("ERROR")
            .sprite('!' as Glyph, named::RED.into(), named::BLACK.into())
            .build(),
    );
    cache.insert(
        TileBuilder::new("WALL")
            .kind(TileKind::WALL)
            .sprite('#' as Glyph, RGBA::rgb(32, 32, 32), named::BLACK.into())
            .move_flags("BLOCKS_ALL")
            .flavor("a solid granite wall")
            .build(),
    );
    cache.insert(
        TileBuilder::new("FLOOR")
            .kind(TileKind::FLOOR)
            .sprite(
                '.' as Glyph,
                named::DARK_GREEN.into(),
                RGBA::rgb(20, 32, 20),
            )
            .flavor("the stone floor")
            .build(),
    );
    cache.insert(
        TileBuilder::new("HALL")
            .kind(TileKind::HALL)
            .sprite('.' as Glyph, named::LIGHTBLUE.into(), RGBA::rgb(20, 20, 32))
            .flavor("the stone hallway")
            .build(),
    );
}

#[derive(Debug)]
pub struct Tiles {
    tiles: HashMap<String, Arc<Tile>>,
}

impl Tiles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty() -> Self {
        Tiles {
            tiles: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Arc<Tile>> {
        match self.tiles.get(name) {
            None => None,
            Some(tile) => Some(tile.clone()),
        }
    }

    pub fn insert(&mut self, tile: Arc<Tile>) {
        self.tiles.insert(tile.id.clone(), tile);
    }

    // pub fn load(&mut self, toml: &StringTable) -> Result<(), String> {
    //     match load_tile_data(self, toml) {
    //         Err(e) => Err(e),
    //         Ok(count) => {
    //             log(format!("Loaded tiles :: count={}", count));
    //             Ok(())
    //         }
    //     }
    // }

    pub fn dump(&self) {
        log("TILES");
        for (id, tile) in self.tiles.iter() {
            log(format!("{} : {:?}", id, tile));
        }
    }
}

impl Default for Tiles {
    fn default() -> Self {
        let mut cache = Tiles {
            tiles: HashMap::new(),
        };

        load_default_tiles(&mut cache);
        cache
    }
}
