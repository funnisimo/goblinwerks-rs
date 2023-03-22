use std::sync::Arc;

use crate::{
    sprite::Sprite,
    tile::{Tile, TileFlags, TileMove, TileSet},
};

use super::{CellFlags, Map};

pub trait Cell {
    fn ground(&self) -> &Arc<Tile>;
    fn fixture(&self) -> &Arc<Tile>;
    fn index(&self) -> usize;
    fn map(&self) -> &Map;

    fn get_tiles(&self) -> TileSet {
        TileSet::new(self.ground().clone(), self.fixture().clone())
    }

    // Flags

    fn has_tile_flag(&self, flag: TileFlags) -> bool {
        self.ground().flags.contains(flag) || self.fixture().flags.contains(flag)
    }

    fn has_any_tile_flag(&self, flag: TileFlags) -> bool {
        self.ground().flags.intersects(flag) || self.fixture().flags.intersects(flag)
    }

    fn has_cell_flag(&self, flag: CellFlags) -> bool {
        self.map().has_flag(self.index(), flag)
    }

    fn has_any_cell_flag(&self, flag: CellFlags) -> bool {
        self.map().has_any_flag_idx(self.index(), flag)
    }

    // Move Flags

    fn has_move_flag(&self, flag: TileMove) -> bool {
        self.ground().move_flags.contains(flag) || self.fixture().move_flags.contains(flag)
    }

    fn has_any_move_flag(&self, flag: TileMove) -> bool {
        self.ground().move_flags.intersects(flag) || self.fixture().move_flags.intersects(flag)
    }

    fn blocks(&self) -> bool {
        self.has_move_flag(TileMove::BLOCKS_MOVE)
    }

    fn blocks_vision(&self) -> bool {
        self.has_move_flag(TileMove::BLOCKS_VISION)
    }

    fn is_obstruction(&self) -> bool {
        self.has_move_flag(TileMove::BLOCKS_DIAGONAL)
    }

    fn is_opaque(&self) -> bool {
        let tile = self.ground();
        tile.blocks_vision() || self.blocks_vision()
    }

    // Sprite
    fn sprite(&self) -> Sprite {
        let mut sprite = Sprite::default();
        let ground = self.ground();
        sprite.mix(ground.glyph, ground.fg, ground.bg);
        let feature = self.fixture();
        sprite.mix(feature.glyph, feature.fg, feature.bg);

        // for tile in self.tiles.iter() {
        //     if tile.glyph > 0 {
        //         sprite.glyph = tile.glyph;
        //     }
        //     sprite.fg = RGBA::alpha_mix(&sprite.fg, &tile.fg);
        //     sprite.bg = RGBA::alpha_mix(&sprite.bg, &tile.bg);
        // }
        sprite
    }

    // flavor

    fn flavor(&self) -> String {
        if let Some(flavor) = self.map().flavors.get(&self.index()) {
            return flavor.clone();
        }

        let ground = self.ground();
        let feature = self.fixture();

        let ground_null = ground.is_null();
        let feature_null = feature.is_null();
        match (ground_null, feature_null) {
            (false, false) => {
                format!("{} on {}", feature.flavor, ground.flavor)
            }
            (true, false) => feature.flavor.clone(),
            (false, true) => ground.flavor.clone(),
            (true, true) => "nothing".to_string(),
        }
    }
}

///////////////////

pub struct CellRef<'m> {
    map: &'m Map,
    idx: usize,
}

impl<'m> CellRef<'m> {
    pub(crate) fn new(map: &'m Map, idx: usize) -> Self {
        CellRef { map, idx }
    }
}

impl<'m> Cell for CellRef<'m> {
    fn ground(&self) -> &'m Arc<Tile> {
        &self.map.ground[self.idx]
    }

    fn fixture(&self) -> &'m Arc<Tile> {
        &self.map.fixture[self.idx]
    }

    fn index(&self) -> usize {
        self.idx
    }
    fn map(&self) -> &Map {
        self.map
    }
}

///////////////////

pub struct CellMut<'m> {
    map: &'m mut Map,
    idx: usize,
}

impl<'m> CellMut<'m> {
    pub(crate) fn new(map: &'m mut Map, idx: usize) -> Self {
        CellMut { map, idx }
    }
}

impl<'m> Cell for CellMut<'m> {
    fn ground(&self) -> &Arc<Tile> {
        &self.map.ground[self.idx]
    }

    fn fixture(&self) -> &Arc<Tile> {
        &self.map.fixture[self.idx]
    }

    fn index(&self) -> usize {
        self.idx
    }
    fn map(&self) -> &Map {
        self.map
    }
}
