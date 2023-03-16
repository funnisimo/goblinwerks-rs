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
        let data_table = match data.as_map() {
            None => return Err(format!("Bad data format - {}", name.to_string())),
            Some(v) => v,
        };

        if let Some(ch_value) = data_table.get(&"ch".into()) {
            if ch_value.is_list() {
                let chs = ch_value.as_list().unwrap();

                for ch_value in chs {
                    let ch_text = ch_value.to_string();
                    let id = name.to_string().replace("{}", &ch_text);

                    let mut builder = TileBuilder::new(&id);
                    builder
                        .set(&"ch".into(), ch_value)
                        .expect("Failed to set ch");

                    for (key, value) in data_table.iter() {
                        if key == "ch" {
                            continue;
                        }
                        if let Err(e) = builder.set(key, value) {
                            return Err(format!("Error processing tile[{}] - {}", &name, e));
                        }
                    }
                    dest.insert(builder.build());
                    count += 1;
                }

                continue;
            }
        }

        let mut builder = TileBuilder::new(&name.to_string());

        for (key, value) in data_table.iter() {
            if let Err(e) = builder.set(key, value) {
                return Err(format!("Error processing tile[{}] - {}", &name, e));
            }
        }
        dest.insert(builder.build());
        count += 1;
    }

    Ok(count)
}

pub struct TileTomlFileLoader {
    dump: bool,
}

impl TileTomlFileLoader {
    pub fn new() -> TileTomlFileLoader {
        TileTomlFileLoader { dump: false }
    }

    pub fn with_dump(mut self) -> Self {
        self.dump = true;
        self
    }
}

impl LoadHandler for TileTomlFileLoader {
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

pub struct TileJsonFileLoader {
    dump: bool,
}

impl TileJsonFileLoader {
    pub fn new() -> TileJsonFileLoader {
        TileJsonFileLoader { dump: false }
    }

    pub fn with_dump(mut self) -> Self {
        self.dump = true;
        self
    }
}

impl LoadHandler for TileJsonFileLoader {
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

        let string_table = match gw_util::json::parse_string(&string) {
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
