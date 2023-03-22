use super::{Tile, TileBuilder, TileKind, NO_TILE};
use gw_app::{color::named, log, Glyph, RGBA};
use std::{collections::HashMap, sync::Arc};

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
