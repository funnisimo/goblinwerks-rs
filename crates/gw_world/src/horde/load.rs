use super::Hordes;
use super::{set_field, HordeBuilder};
use crate::being::{self, BeingKinds};
use gw_app::loader::{LoadError, LoadHandler};
use gw_app::log;
use gw_ecs::Ecs;
use gw_util::value::Value;
use std::fs::read_to_string;

/*
   JSON format:
   {
    leader: <ID> or { "kind": <ID>, ... },  // REQUIRED
    frequency: <INT> or { <INT>: <FORMULA>, "<A>-<B>": <FORMULA>, "C+": <FORMULA> } // defaults to 100
    members: [ <ID> or { "kind": <ID>, ... },* ],   // defaults to none
    counts: [ <INT>,* ],    // if not same len as members, defaults to 1 for missing entries
    spawnTile: <ID>,    // The tile id
    machineId: <ID>,     // the machine that this horde is for
    flags: <STRING>,  // Any horde flags in flags string format
    tags: Vec<String> | String, // Any tags as string or array of string (strings can be multi-value)
   }
*/

pub fn load_horde_data(dest: &mut Hordes, beings: &BeingKinds, data: Value) -> Result<u32, String> {
    let list = match data.to_list() {
        None => return Err("Horde Kind data must be an array.".to_string()),
        Some(v) => v,
    };

    let mut count: u32 = 0;

    for data in list.iter() {
        let data_table = match data.as_map() {
            None => return Err(format!("Bad data format - {}", data.to_string())),
            Some(v) => v,
        };

        let leader = match data_table.get(&"leader".into()) {
            None => {
                return Err(format!(
                    "horde entry missing leader field. - {:?}",
                    data_table
                ))
            }
            Some(leader) => leader,
        };

        let leader_being = match being::from_value(leader, beings, "CUSTOM", None) {
            Err(e) => {
                return Err(format!(
                    "failed to get/create leader being kind - {:?} : {:?}",
                    e, leader
                ))
            }
            Ok(v) => v,
        };

        let mut builder = HordeBuilder::new(leader_being);

        for (key, value) in data_table.iter() {
            if key.as_str().unwrap() == "leader" {
                continue;
            }
            if let Err(e) = set_field(&mut builder, &key.to_string(), value, beings) {
                return Err(format!("Error processing horde - {:?}", e));
            }
        }
        dest.push(builder.build());
        count += 1;
    }

    Ok(count)
}

pub struct HordesLoader {
    dump: bool,
}

impl HordesLoader {
    pub fn new() -> HordesLoader {
        HordesLoader { dump: false }
    }

    pub fn with_dump(mut self) -> Self {
        self.dump = true;
        self
    }
}

impl LoadHandler for HordesLoader {
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
        ecs.ensure_global::<Hordes>();

        let mut hordes = ecs.write_global::<Hordes>();
        let being_kinds = ecs.read_global::<BeingKinds>();

        match load_horde_data(&mut hordes, &being_kinds, string_table) {
            Err(e) => return Err(LoadError::ProcessError(e)),
            Ok(count) => {
                log(format!("Loaded {} hordes", count));
                being_kinds.dump();
            }
        }

        if self.dump {
            hordes.dump();
        }

        Ok(())
    }
}

pub fn load_hordes_file(filename: &str, being_kinds: &BeingKinds) -> Hordes {
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

    let mut hordes = Hordes::default();

    match load_horde_data(&mut hordes, being_kinds, value) {
        Err(e) => panic!("{}", e),
        Ok(count) => {
            log(format!("Loaded {} hordes", count));
        }
    }

    hordes
}
