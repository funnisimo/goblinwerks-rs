use super::TileBuilder;
use super::Tiles;
use gw_app::ecs::Ecs;
use gw_app::loader::{LoadError, LoadHandler};
use gw_app::log;
use gw_util::value::Value;

pub fn load_tile_data(dest: &mut Tiles, toml: Value) -> Result<u32, String> {
    let map = match toml.to_map() {
        None => return Err("Wrong data format.".to_string()),
        Some(v) => v,
    };

    let mut count: u32 = 0;

    for (name, data) in map.iter() {
        let mut builder = TileBuilder::new(&name.to_string());

        let data_table = match data.as_map() {
            None => return Err(format!("Bad data format - {}", name.to_string())),
            Some(v) => v,
        };

        for (key, value) in data_table.iter() {
            let key_val = key.to_string().to_lowercase();
            if let Err(e) = builder.set(&key_val, &value.to_string()) {
                return Err(format!("Error processing tile[{}] - {}", &name, e));
            }
        }
        dest.insert(builder.build());
        count += 1;
    }

    Ok(count)
}

pub struct TileFileLoader {
    dump: bool,
}

impl TileFileLoader {
    pub fn new() -> TileFileLoader {
        TileFileLoader { dump: false }
    }

    pub fn with_dump(mut self) -> Self {
        self.dump = true;
        self
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

        let string_table = match gw_util::toml::parse_string(&string) {
            Err(e) => {
                return Err(LoadError::ParseError(format!(
                    "Failed to parse '{}' => {}",
                    path, e
                )))
            }
            Ok(v) => v,
        };

        match load_tile_data(&mut tiles, string_table) {
            Err(e) => return Err(LoadError::ProcessError(e)),
            Ok(count) => {
                log(format!("Loaded {} tiles", count));
            }
        }

        if self.dump {
            tiles.dump();
        }

        Ok(())
    }
}
