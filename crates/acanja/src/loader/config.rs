use super::LevelLoader;
use gw_app::loader::{LoadError, LoadHandler, Loader};
use gw_ecs::{atomize::Atom, Ecs};
use gw_world::{being::BeingKindsLoader, horde::HordesLoader, tile::TilesLoader};

#[derive(Clone, Debug)]
pub struct StartMap {
    pub map_id: Atom,
    pub location: String,
}

impl StartMap {
    pub fn new<I: Into<Atom>>(map_id: I, location: &str) -> Self {
        StartMap {
            map_id: map_id.into(),
            location: location.to_string(),
        }
    }
}

impl Default for StartMap {
    fn default() -> Self {
        StartMap {
            map_id: Atom::from("DEFAULT"),
            location: "START".to_string(),
        }
    }
}

pub struct GameConfigLoader;

impl LoadHandler for GameConfigLoader {
    fn file_loaded(
        &mut self,
        path: &str,
        data: Vec<u8>,
        ecs: &mut Ecs,
    ) -> Result<(), gw_app::loader::LoadError> {
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

        let table_value = match gw_util::json::parse_string(&string) {
            Err(e) => {
                return Err(LoadError::ParseError(format!(
                    "Failed to parse '{}' => {}",
                    path, e
                )))
            }
            Ok(v) => v,
        };
        let table = table_value.as_map().unwrap();

        println!("Loaded file = {}", path);
        // println!("{:?}", table);

        let start_map = table
            .get(&"start".into())
            .expect("Config must have start value.")
            .to_string();

        {
            let mut loader = ecs.write_global::<Loader>();

            // Load TILES
            if let Some(tiles_value) = table.get(&"tiles".into()) {
                if tiles_value.is_string() {
                    let filename = tiles_value.to_string();
                    loader
                        .load_file(&filename, Box::new(TilesLoader::new()))
                        .expect("Failed to load tiles!");
                }
            }

            // Load BEINGS
            if let Some(actor_value) = table.get(&"beings".into()) {
                if actor_value.is_string() {
                    let filename = actor_value.to_string();
                    loader
                        .load_file(&filename, Box::new(BeingKindsLoader::new()))
                        .expect("Failed to load beings file!");
                }
            }

            // Load HORDES
            if let Some(horde_value) = table.get(&"hordes".into()) {
                if horde_value.is_string() {
                    let filename = horde_value.to_string();
                    loader
                        .load_file(&filename, Box::new(HordesLoader::new()))
                        .expect("Failed to load hordes file!");
                }
            }
        }

        ecs.insert_global(StartMap::new(start_map.as_str(), "START"));

        let maps = table.get(&"levels".into()).unwrap().as_map().unwrap();

        let dir = maps.get(&"dir".into()).unwrap().to_string();
        let files = maps.get(&"files".into()).unwrap().as_list().unwrap();

        for f_val in files {
            let name = f_val.to_string();
            let full_path = format!("{}/{}", dir, name);
            ecs.write_global::<Loader>()
                .load_file(&full_path, Box::new(LevelLoader::new()))
                .expect("Failed to load map listed in config file.");
        }

        Ok(())
    }
}
