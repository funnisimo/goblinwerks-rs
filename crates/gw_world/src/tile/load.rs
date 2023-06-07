use super::TileBuilder;
use super::Tiles;
use gw_app::loader::{LoadError, LoadHandler};
use gw_app::log;
use gw_ecs::prelude::Ecs;
use gw_util::value::Value;
use std::fs::read_to_string;

pub fn load_tile_data(dest: &mut Tiles, data: Value) -> Result<u32, String> {
    let map = match data.to_map() {
        None => return Err("Tile data must be a map.".to_string()),
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

pub struct TilesLoader {
    dump: bool,
}

impl TilesLoader {
    pub fn new() -> TilesLoader {
        TilesLoader { dump: false }
    }

    pub fn with_dump(mut self) -> Self {
        self.dump = true;
        self
    }
}

impl LoadHandler for TilesLoader {
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, ecs: &mut Ecs) -> Result<(), LoadError> {
        ecs.ensure_global::<Tiles>();
        let mut tiles = ecs.write_global::<Tiles>();

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

        let string_table = if path.ends_with(".toml") {
            match gw_util::toml::parse_string(&string) {
                Err(e) => {
                    return Err(LoadError::ParseError(format!(
                        "Failed to parse '{}' => {}",
                        path, e
                    )))
                }
                Ok(v) => v,
            }
        } else if path.ends_with(".json") || path.ends_with(".jsonc") {
            match gw_util::json::parse_string(&string) {
                Err(e) => {
                    return Err(LoadError::ParseError(format!(
                        "Failed to parse '{}' => {}",
                        path, e
                    )))
                }
                Ok(v) => v,
            }
        } else {
            return Err(LoadError::ParseError(
                "Unsupported file extension - require '.toml' or '.json' or '.jsonc'".to_string(),
            ));
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

pub fn load_tiles_file(filename: &str) -> Tiles {
    let file_text = read_to_string(filename).expect(&format!("Failed to open {filename}"));

    let value = if filename.ends_with(".toml") {
        match gw_util::toml::parse_string(&file_text) {
            Err(e) => {
                panic!("Failed to parse '{}' => {}", filename, e);
            }
            Ok(v) => v,
        }
    } else if filename.ends_with(".json") || filename.ends_with(".jsonc") {
        match gw_util::json::parse_string(&file_text) {
            Err(e) => {
                panic!("Failed to parse '{}' => {}", filename, e);
            }
            Ok(v) => v,
        }
    } else {
        panic!(
                "Unsupported file extension - require '.toml' or '.json' or '.jsonc'.  found: {filename}"
            );
    };

    let mut tiles = Tiles::default();

    match load_tile_data(&mut tiles, value) {
        Err(e) => panic!("{}", e),
        Ok(count) => {
            log(format!("Loaded {} tiles", count));
        }
    }

    tiles
}
