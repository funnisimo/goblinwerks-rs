use super::*;
use crate::effect::BoxedEffect;
use crate::treasure::Treasure;
use gw_app::Glyph;
use gw_app::RGBA;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

lazy_static! {
    pub static ref NO_TILE: Arc<Tile> = TileBuilder::new("NONE")
        // .sprite(0 as Glyph, RGBA::new(), RGBA::new())
        .build();
}

#[derive(Default, Debug)]
pub struct Tile {
    pub id: String,
    pub glyph: Glyph,
    pub fg: RGBA,
    pub bg: RGBA,
    pub flags: TileFlags,
    pub move_flags: TileMove,
    pub liquid: TileLiquid,
    pub layer: TileLayer,
    pub effects: HashMap<String, Vec<BoxedEffect>>,
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
    pub(super) fn new(id: &str) -> Self {
        Tile {
            id: id.to_owned(),

            glyph: 0,
            fg: RGBA::rgba(0, 0, 0, 0),
            bg: RGBA::rgba(0, 0, 0, 0),

            flags: TileFlags::empty(),
            move_flags: TileMove::empty(),
            treasure: Treasure::empty(),
            kind: TileKind::empty(),
            effects: HashMap::new(),
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
