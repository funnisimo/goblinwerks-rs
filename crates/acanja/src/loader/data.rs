use super::{make_level, LevelData, MapData};
use gw_app::{
    ecs::{Read, ResourceSet, Write},
    loader::{LoadError, LoadHandler},
    log,
};
use gw_world::{level::Levels, tile::Tiles};

pub struct LevelDataLoader {
    level_data: Option<LevelData>,
}

impl LevelDataLoader {
    pub fn new(level_data: LevelData) -> Self {
        LevelDataLoader {
            level_data: Some(level_data),
        }
    }
}

impl LoadHandler for LevelDataLoader {
    fn file_loaded(
        &mut self,
        path: &str,
        data: Vec<u8>,
        ecs: &mut gw_app::Ecs,
    ) -> Result<(), LoadError> {
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

        let lines: Vec<String> = string.split("\n").map(|s| s.to_string()).collect();

        self.level_data.as_mut().unwrap().map_data = Some(MapData::Data(lines));

        let (tiles, mut levels) = <(Read<Tiles>, Write<Levels>)>::fetch_mut(&mut ecs.resources);

        let level = make_level(self.level_data.take().unwrap(), &tiles);
        log(format!("Adding Level - {}", level.id));

        levels.insert(level);

        Ok(())
    }
}