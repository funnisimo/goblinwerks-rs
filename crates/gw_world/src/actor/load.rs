use super::set_field;
use super::ActorKindBuilder;
use super::ActorKinds;
use gw_app::ecs::Ecs;
use gw_app::loader::{LoadError, LoadHandler};
use gw_app::log;
use gw_util::value::Value;

/*
   JSON format:
   "ID": {
       "sprite": "<SPRITE_CONFIG>",
       --or--
       "glyph" | "ch": "ch" || ###,
       "fg": "<RGBA_CONFIG>",
       "bg": "<RGBA_CONFIG>",

       "flavor": <STRING>,
       "description": <STRING>
   }
*/

pub fn load_actor_data(dest: &mut ActorKinds, data: Value) -> Result<u32, String> {
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

        let mut builder = ActorKindBuilder::new(&name.to_string());

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

pub struct ActorKindTomlFileLoader {
    dump: bool,
}

impl ActorKindTomlFileLoader {
    pub fn new() -> ActorKindTomlFileLoader {
        ActorKindTomlFileLoader { dump: false }
    }

    pub fn with_dump(mut self) -> Self {
        self.dump = true;
        self
    }
}

impl LoadHandler for ActorKindTomlFileLoader {
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, ecs: &mut Ecs) -> Result<(), LoadError> {
        let mut tiles = ecs
            .resources
            .get_mut_or_insert_with(|| ActorKinds::default());

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

        match load_actor_data(&mut tiles, string_table) {
            Err(e) => return Err(LoadError::ProcessError(e)),
            Ok(count) => {
                log(format!("Loaded {} actor kinds", count));
            }
        }

        if self.dump {
            tiles.dump();
        }

        Ok(())
    }
}

pub struct ActorKindJsonFileLoader {
    dump: bool,
}

impl ActorKindJsonFileLoader {
    pub fn new() -> ActorKindJsonFileLoader {
        ActorKindJsonFileLoader { dump: false }
    }

    pub fn with_dump(mut self) -> Self {
        self.dump = true;
        self
    }
}

impl LoadHandler for ActorKindJsonFileLoader {
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, ecs: &mut Ecs) -> Result<(), LoadError> {
        let mut tiles = ecs
            .resources
            .get_mut_or_insert_with(|| ActorKinds::default());

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

        match load_actor_data(&mut tiles, string_table) {
            Err(e) => return Err(LoadError::ProcessError(e)),
            Ok(count) => {
                log(format!("Loaded {} actor kinds", count));
            }
        }

        if self.dump {
            tiles.dump();
        }

        Ok(())
    }
}
