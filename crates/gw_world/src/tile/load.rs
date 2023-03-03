use super::TileBuilder;
use super::Tiles;
use gw_app::ecs::Ecs;
use gw_app::loader::{LoadError, LoadHandler};
use gw_app::log;
use gw_util::toml::StringTable;

pub fn load_tile_data(dest: &mut Tiles, toml: &StringTable) -> Result<u32, String> {
    let mut count: u32 = 0;
    for (name, data) in toml.iter() {
        let mut builder = TileBuilder::new(name);
        for (key, value) in data.iter() {
            if let Err(e) = builder.set(&key.to_lowercase(), value) {
                return Err(format!("Error processing tile[{}] - {}", &name, e));
            }
        }
        dest.insert(builder.build());
        count += 1;
    }

    Ok(count)
}

pub struct TileFileLoader;

impl TileFileLoader {
    pub fn new() -> TileFileLoader {
        TileFileLoader
    }
}

impl LoadHandler for TileFileLoader {
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, ecs: &mut Ecs) -> Result<(), LoadError> {
        let mut tiles = ecs.resources.get_mut_or_insert_with(|| Tiles::default());

        let string = match String::from_utf8(data) {
            Err(e) => {
                return Err(LoadError::ParseError(format!(
                    "Malformed file data '{}' : {}",
                    path,
                    e.to_string()
                )))
            }
            Ok(v) => v,
        };

        let string_table = match gw_util::toml::parse_to_string_table(&string) {
            Err(e) => {
                return Err(LoadError::ParseError(format!(
                    "Failed to parse '{}' => {}",
                    path, e
                )))
            }
            Ok(v) => v,
        };

        match load_tile_data(&mut tiles, &string_table) {
            Err(e) => return Err(LoadError::ProcessError(e)),
            Ok(count) => {
                log(format!("Loaded {} tiles", count));
            }
        }

        Ok(())
    }
}
