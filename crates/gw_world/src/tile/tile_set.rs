use gw_app::RGBA;

use crate::sprite::Sprite;

use super::{tile_is_none, Tile, TileFlags, TileMove};
use std::sync::Arc;

pub struct TileSet {
    tiles: [Arc<Tile>; 2],
}

impl TileSet {
    pub(crate) fn new(ground: Arc<Tile>, feature: Arc<Tile>) -> Self {
        TileSet {
            tiles: [ground, feature],
        }
    }

    pub fn ground(&self) -> Arc<Tile> {
        self.tiles[0].clone()
    }

    pub fn feature(&self) -> Arc<Tile> {
        self.tiles[1].clone()
    }

    // Flags

    pub fn has_flag(&self, flag: TileFlags) -> bool {
        self.tiles.iter().any(|t| t.flags.contains(flag))
    }

    pub fn has_any_flag(&self, flag: TileFlags) -> bool {
        self.tiles.iter().any(|t| t.flags.intersects(flag))
    }

    // Move Flags

    pub fn has_move_flag(&self, flag: TileMove) -> bool {
        self.tiles.iter().any(|t| t.move_flags.contains(flag))
    }

    pub fn has_any_move_flag(&self, flag: TileMove) -> bool {
        self.tiles.iter().any(|t| t.move_flags.intersects(flag))
    }

    pub fn blocks(&self) -> bool {
        self.has_move_flag(TileMove::BLOCKS_MOVE)
    }

    pub fn blocks_vision(&self) -> bool {
        self.has_move_flag(TileMove::BLOCKS_VISION)
    }

    pub fn is_obstruction(&self) -> bool {
        self.has_move_flag(TileMove::BLOCKS_DIAGONAL)
    }

    // Sprite
    pub fn sprite(&self) -> Sprite {
        let mut sprite = Sprite::default();
        for tile in self.tiles.iter() {
            if tile.glyph > 0 {
                sprite.glyph = tile.glyph;
            }
            sprite.fg = RGBA::alpha_mix(&sprite.fg, &tile.fg);
            sprite.bg = RGBA::alpha_mix(&sprite.bg, &tile.bg);
        }
        sprite
    }

    // flavor

    pub fn flavor(&self) -> String {
        let ground_null = tile_is_none(&self.tiles[0]);
        let feature_null = tile_is_none(&self.tiles[1]);
        match (ground_null, feature_null) {
            (false, false) => {
                format!("{} on {}", self.tiles[1].flavor, self.tiles[0].flavor)
            }
            (true, false) => self.tiles[1].flavor.to_string(),
            (false, true) => self.tiles[0].flavor.to_string(),
            (true, true) => "nothing".to_string(),
        }
    }
}
