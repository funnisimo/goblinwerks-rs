use super::LevelLoader;
use gw_app::{
    ecs::{ResourceSet, Write},
    loader::{LoadError, LoadHandler, Loader},
    log,
};
use gw_world::{actor::ActorKindsLoader, level::Levels, tile::TilesLoader};

pub struct GameConfigLoader;

impl LoadHandler for GameConfigLoader {
    fn file_loaded(
        &mut self,
        path: &str,
        data: Vec<u8>,
        ecs: &mut gw_app::Ecs,
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

        ecs.resources.get_or_insert_with(|| Levels::default());

        let (mut loader, mut levels) =
            <(Write<Loader>, Write<Levels>)>::fetch_mut(&mut ecs.resources);

        // Load TILES
        if let Some(tiles_value) = table.get(&"tiles".into()) {
            if tiles_value.is_string() {
                let filename = tiles_value.to_string();
                loader
                    .load_file(&filename, Box::new(TilesLoader::new()))
                    .expect("Failed to load tiles!");
            }
        }

        // Load ACTORS
        if let Some(actor_value) = table.get(&"actors".into()) {
            if actor_value.is_string() {
                let filename = actor_value.to_string();
                loader
                    .load_file(&filename, Box::new(ActorKindsLoader::new()))
                    .expect("Failed to load actors file!");
            }
        }

        levels.set_start_map(&start_map);

        let maps = table.get(&"levels".into()).unwrap().as_map().unwrap();

        let dir = maps.get(&"dir".into()).unwrap().to_string();
        let files = maps.get(&"files".into()).unwrap().as_list().unwrap();

        for f_val in files {
            let name = f_val.to_string();
            let full_path = format!("{}/{}", dir, name);
            loader
                .load_file(&full_path, Box::new(LevelLoader::new()))
                .expect("Failed to load map listed in config file.");
        }

        Ok(())
    }
}
