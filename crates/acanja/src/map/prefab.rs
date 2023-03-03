use gw_app::{
    loader::{LoadError, LoadHandler},
    log, Ecs,
};
use gw_world::{
    map::Builder,
    tile::{Tile, TileKind},
};
use std::fmt::{Debug, Display};
use std::{cmp::max, sync::Arc};
use std::{collections::HashMap, str::FromStr};
use toml_edit::{self, TableLike};

#[derive(Clone, Debug)]
pub enum PrefabCell {
    Tile(String),

    MatchAny,
    MatchKind(TileKind),
    MatchTile(String),

    ReplaceKind(TileKind, String),
    ReplaceTile(String, String),
}

impl PrefabCell {
    pub fn matches(&self, tile: &Arc<Tile>) -> bool {
        match self {
            PrefabCell::Tile(_) => true,
            PrefabCell::MatchAny => true,
            PrefabCell::MatchKind(kind) => tile.kind.contains(*kind),
            PrefabCell::MatchTile(id) => &tile.id == id,
            PrefabCell::ReplaceKind(kind, _) => tile.kind.contains(*kind),
            PrefabCell::ReplaceTile(id, _) => &tile.id == id,
        }
    }

    pub fn get_tile(&self) -> Option<&String> {
        match self {
            PrefabCell::Tile(t) => Some(t),
            PrefabCell::ReplaceKind(_, t) => Some(t),
            _ => None,
        }
    }
}

impl FromStr for PrefabCell {
    type Err = LoadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 || s == "*" {
            return Ok(PrefabCell::MatchAny);
        }

        let parts: Vec<&str> = s.split(":").collect();
        if parts.len() == 1 {
            return Ok(PrefabCell::Tile(s.to_string()));
        }

        if parts[0] == "TILE" {
            if parts.len() == 2 {
                return Ok(PrefabCell::MatchTile(parts[1].to_string()));
            } else if parts.len() == 3 {
                return Ok(PrefabCell::ReplaceTile(
                    parts[1].to_string(),
                    parts[2].to_string(),
                ));
            }
        } else if parts[0] == "KIND" {
            if parts.len() < 2 {
                return Err(LoadError::ParseError(format!("Invalid PrefabCell - {}", s)));
            }

            let kind = match parts[1].parse() {
                Err(_) => {
                    return Err(LoadError::ParseError(format!(
                        "Received invalid ItemKind - {}",
                        parts[1]
                    )))
                }
                Ok(v) => v,
            };

            if parts.len() == 2 {
                return Ok(PrefabCell::MatchKind(kind));
            } else if parts.len() == 3 {
                return Ok(PrefabCell::ReplaceKind(kind, parts[2].to_string()));
            }
        }

        Err(LoadError::ParseError(format!("Invalid PrefabCell - {}", s)))
    }
}

impl Display for PrefabCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefabCell::Tile(v) => write!(f, "Tile({})", v),
            PrefabCell::MatchAny => write!(f, "*"),
            PrefabCell::MatchKind(v) => write!(f, "MatchKind({})", v),
            PrefabCell::MatchTile(v) => write!(f, "MatchTile({})", v),
            PrefabCell::ReplaceKind(k, t) => write!(f, "ReplaceKind({},{})", k, t),
            PrefabCell::ReplaceTile(k, t) => write!(f, "ReplaceTile({},{})", k, t),
        }
    }
}

pub struct Prefab {
    width: u32,
    height: u32,
    cells: Vec<PrefabCell>,
    tags: Vec<String>,
}

impl Prefab {
    pub fn new(width: u32, height: u32) -> Self {
        Prefab {
            width,
            height,
            cells: vec![PrefabCell::MatchAny; (width * height) as usize],
            tags: Vec::new(),
        }
    }

    fn to_idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            log(format!(
                "Invalid xy: {}, {} for {},{}",
                x, y, self.width, self.height
            ));
            return None;
        }
        Some((x + y * self.width as i32) as usize)
    }

    pub fn set(&mut self, x: i32, y: i32, cell: PrefabCell) {
        if let Some(index) = self.to_idx(x, y) {
            self.cells[index] = cell;
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    pub fn add_tag(&mut self, tag: &str) {
        if !self.has_tag(tag) {
            self.tags.push(tag.to_string());
        }
    }

    pub fn fits_at(&self, x0: i32, y0: i32, builder: &Builder) -> bool {
        for dy in 0..self.height as i32 {
            for dx in 0..self.width as i32 {
                let x = x0 + dx;
                let y = y0 + dy;
                match self.to_idx(dx, dy) {
                    None => {
                        log(format!("- cell out of bounds @ {},{}", x, y));
                        return false;
                    }
                    Some(idx) => {
                        let cell = self.cells.get(idx).unwrap();
                        let tile = builder.get_tile(x, y);
                        if !cell.matches(&tile) {
                            log(format!(
                                "- match failed for {},{} with cell: {:?} and tile: {:?}",
                                x, y, cell, tile
                            ));
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    pub fn build_at(&self, x0: i32, y0: i32, builder: &mut Builder) {
        for dy in 0..self.height as i32 {
            for dx in 0..self.width as i32 {
                let x = x0 + dx;
                let y = y0 + dy;
                match self.to_idx(dx, dy) {
                    None => {}
                    Some(idx) => {
                        let cell = self.cells.get(idx).unwrap();
                        if let Some(name) = cell.get_tile() {
                            builder.set_tile(x, y, name);
                        }
                    }
                }
            }
        }
    }
}

impl Debug for Prefab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = Vec::new();

        let mut line = "".to_string();
        let mut x = 0;
        for item in self.cells.iter() {
            let ch = match item {
                PrefabCell::MatchAny => '*',
                PrefabCell::MatchKind(_) => '?',
                PrefabCell::MatchTile(_) => '#',
                PrefabCell::Tile(_) => '.',
                PrefabCell::ReplaceKind(_, _) => '%',
                PrefabCell::ReplaceTile(_, _) => '$',
            };

            line.push(ch);

            x += 1;
            if x == self.width {
                lines.push(line);
                line = "".to_string();
                x = 0;
            }
        }

        let mut s = f.debug_struct("Prefab");
        s.field("width", &self.width);
        s.field("height", &self.height);
        s.field("cells", &lines.join("\n"));
        s.field("tags", &self.tags);
        s.finish()
    }
}

////////////////////////////////////////////////

pub struct Prefabs(HashMap<String, Prefab>);

impl Prefabs {
    pub fn new() -> Self {
        Prefabs(HashMap::new())
    }

    pub fn get(&self, id: &str) -> Option<&Prefab> {
        self.0.get(id)
    }

    pub fn insert(&mut self, id: &str, prefab: Prefab) {
        self.0.insert(id.to_string(), prefab);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Prefab)> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Default for Prefabs {
    fn default() -> Self {
        Prefabs::new()
    }
}

////////////////////////////////////////////////

pub struct PrefabFileLoader {
    dump: bool,
}

impl PrefabFileLoader {
    pub fn new() -> PrefabFileLoader {
        PrefabFileLoader { dump: false }
    }

    pub fn with_dump(mut self) -> Self {
        self.dump = true;
        self
    }
}

impl LoadHandler for PrefabFileLoader {
    fn file_loaded(&mut self, path: &str, data: Vec<u8>, ecs: &mut Ecs) -> Result<(), LoadError> {
        let mut prefabs = ecs.resources.get_mut_or_insert_with(|| Prefabs::default());

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

        let doc: toml_edit::Document = match string.parse() {
            Ok(d) => d,
            Err(e) => return Err(LoadError::ParseError(e.to_string())),
        };

        match load_prefab_data(&mut prefabs, doc) {
            Err(e) => return Err(e),
            Ok(count) => {
                log(format!("Loaded {} prefabs", count));
                if self.dump {
                    for (key, item) in prefabs.iter() {
                        log(format!("{} : {:?}", key, item));
                    }
                }
            }
        }

        Ok(())
    }
}

fn load_prefab_data(prefabs: &mut Prefabs, doc: toml_edit::Document) -> Result<u32, LoadError> {
    let table = doc.as_table();

    let mut count = 0;
    let mut default_tiles: HashMap<String, PrefabCell> = HashMap::new();

    if let Some(tiles) = table.get("TILES") {
        match tiles.as_table_like() {
            None => {
                return Err(LoadError::ParseError(
                    "Invalid [TILES] section - must be table".to_string(),
                ));
            }
            Some(table) => {
                load_tiles(&mut default_tiles, table)?;
            }
        }
    }

    for (key, item) in table.iter() {
        if key == "TILES" {
            continue;
        }
        match item.as_table_like() {
            None => {
                return Err(LoadError::ParseError(format!(
                    "Invalid [PREFAB] section: {} - must be table",
                    key
                )));
            }
            Some(table) => {
                load_prefab(prefabs, &default_tiles, key, table)?;
                count += 1;
            }
        }
    }

    Ok(count)
}

fn load_tiles(
    tiles: &mut HashMap<String, PrefabCell>,
    table: &dyn TableLike,
) -> Result<u32, LoadError> {
    let mut count = 0;
    for (key, item) in table.iter() {
        match item.as_str() {
            None => {
                return Err(LoadError::ParseError(format!(
                    "Invalid [TILES] entry - {} : must be string",
                    key
                )))
            }
            Some(text) => match text.parse() {
                Err(e) => return Err(e),
                Ok(v) => {
                    tiles.insert(key.to_string(), v);
                    count += 1;
                }
            },
        }
    }
    Ok(count)
}

fn load_prefab(
    prefabs: &mut Prefabs,
    default_tiles: &HashMap<String, PrefabCell>,
    key: &str,
    table: &dyn TableLike,
) -> Result<(), LoadError> {
    let mut my_tiles = default_tiles.clone();

    if let Some(item) = table.get("tiles") {
        match item.as_table_like() {
            None => {
                return Err(LoadError::ParseError(format!(
                    "Prefab tiles entry must be tablelike: {} - found: {}",
                    key,
                    item.type_name()
                )));
            }
            Some(table) => {
                load_tiles(&mut my_tiles, table)?;
            }
        }
    }

    let mut tags: Vec<String> = Vec::new();

    if let Some(item) = table.get("tags") {
        match item.as_array() {
            None => {
                return Err(LoadError::ParseError(format!(
                    "prefab tags mut be array: {} - found {}",
                    key,
                    item.type_name()
                )))
            }
            Some(arr) => {
                for tag in arr.iter() {
                    match tag.as_str() {
                        None => {
                            return Err(LoadError::ParseError(format!(
                                "prefab has invalid tag : {} - found {}",
                                key,
                                tag.type_name()
                            )))
                        }
                        Some(val) => {
                            tags.push(val.to_string());
                        }
                    }
                }
            }
        }
    }

    match table.get("layout") {
        None => {
            return Err(LoadError::ParseError(format!(
                "Missing layout in prefab: {}.",
                key
            )))
        }
        Some(item) => match item.as_str() {
            None => {
                return Err(LoadError::ParseError(format!(
                    "layout must be string: {} - found: {}",
                    key,
                    item.type_name()
                )))
            }
            Some(text) => {
                let lines: Vec<&str> = text.split('\n').collect();
                let width = lines.iter().fold(0, |out, line| max(out, line.len())) as u32;
                let mut prefab = Prefab::new(width, lines.len() as u32);

                for (y, line) in lines.iter().enumerate() {
                    for (x, ch) in line.char_indices() {
                        match my_tiles.get(&ch.to_string()) {
                            None => {
                                return Err(LoadError::ParseError(format!(
                                    "Unknown tile in layout for {} - found: {}",
                                    key, ch
                                )))
                            }
                            Some(cell) => {
                                prefab.set(x as i32, y as i32, cell.clone());
                            }
                        }
                    }
                }

                for tag in tags.iter() {
                    prefab.add_tag(tag);
                }

                prefabs.insert(key, prefab);
            }
        },
    };

    Ok(())
}
