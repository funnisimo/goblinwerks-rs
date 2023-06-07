use std::fs::read_to_string;

use super::set_field;
use super::BeingKindBuilder;
use super::BeingKinds;
use gw_app::loader::{LoadError, LoadHandler};
use gw_app::log;
use gw_ecs::prelude::{Ecs, World};
use gw_util::value::Value;

/*
   JSON format:
   "ID": {
        "sprite": <SPRITE_CONFIG>,
        --or--
        "glyph" | "ch": <char> || <int>
        "fg": <RGBA_CONFIG>,
        "bg": <RGBA_CONFIG>,

        "name": <STRING>,
        "flavor": <STRING>,
        "description": <STRING>,

        "ai": <STRING>,
   }
*/

pub fn load_being_data(dest: &mut BeingKinds, data: Value) -> Result<u32, String> {
    let map = match data.to_map() {
        None => return Err("Actor Kind data must be a map.".to_string()),
        Some(v) => v,
    };

    let mut count: u32 = 0;

    for (name, data) in map.iter() {
        let data_table = match data.as_map() {
            None => return Err(format!("Bad data format - {}", name.to_string())),
            Some(v) => v,
        };

        let mut builder = BeingKindBuilder::new(&name.to_string());

        for (key, value) in data_table.iter() {
            if let Err(e) = set_field(&mut builder, &key.to_string(), value) {
                return Err(format!("Error processing actor kind[{}] - {:?}", &name, e));
            }
        }
        dest.insert(builder.build());
        count += 1;
    }

    Ok(count)
}

pub struct BeingKindsLoader {
    dump: bool,
}

impl BeingKindsLoader {
    pub fn new() -> BeingKindsLoader {
        BeingKindsLoader { dump: false }
    }

    pub fn with_dump(mut self) -> Self {
        self.dump = true;
        self
    }
}

impl LoadHandler for BeingKindsLoader {
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, ecs: &mut Ecs) -> Result<(), LoadError> {
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
                "Unsupported file format - require '.toml' or '.json' or '.jsonc'".to_string(),
            ));
        };

        ecs.ensure_global::<BeingKinds>();
        let mut being_kinds = ecs.write_global::<BeingKinds>();

        match load_being_data(&mut being_kinds, string_table) {
            Err(e) => return Err(LoadError::ProcessError(e)),
            Ok(count) => {
                log(format!("Loaded {} actor kinds", count));
                being_kinds.dump();
            }
        }

        if self.dump {
            being_kinds.dump();
        }

        Ok(())
    }
}

pub fn load_being_kinds_file(filename: &str) -> BeingKinds {
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

    let mut kinds = BeingKinds::default();

    match load_being_data(&mut kinds, value) {
        Err(e) => panic!("{}", e),
        Ok(count) => {
            log(format!("Loaded {} actors", count));
        }
    }

    kinds
}
