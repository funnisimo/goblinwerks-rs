use acanja::map::prefab::{PrefabFileLoader, Prefabs};
// use acanja::map::world::build_world_map;
use gw_app::ecs::{systems::ResourceSet, Read};
use gw_app::*;
use gw_util::blob::{Blob, BlobConfig};
use gw_util::grid::{spread_replace, Grid};
use gw_util::point::Point;
use gw_util::rng::{RandomNumberGenerator, RngCore};
use gw_world::map::dump_map;
use gw_world::map::Map;
use gw_world::memory::MapMemory;
use gw_world::tile::{TileFileLoader, Tiles};
use gw_world::widget::{Camera, Viewport};

const MAP_WIDTH: u32 = 80;
const MAP_HEIGHT: u32 = 50;

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT")
            .size(MAP_WIDTH, MAP_HEIGHT)
            .build();

        Box::new(MainScreen { viewport })
    }

    fn build_new_map(&self, ecs: &mut Ecs) {
        let mut map = {
            let (tiles, prefabs) = <(Read<Tiles>, Read<Prefabs>)>::fetch(&ecs.resources);

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_world_map(&tiles, &prefabs, MAP_WIDTH, MAP_HEIGHT)
        };

        map.reveal_all();
        map.make_fully_visible();

        ecs.resources.insert(map);
        ecs.resources.insert(MapMemory::new(160, 100));
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, ecs: &mut Ecs) {
        let resources = &mut ecs.resources;
        resources.get_or_insert_with(|| Tiles::default());
        resources.get_or_insert_with(|| Prefabs::default());

        self.build_new_map(ecs);
    }

    fn input(&mut self, ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        if let Some(result) = self.viewport.input(ecs, ev) {
            return result;
        }

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Space => {
                    self.build_new_map(ecs);
                }
                VirtualKeyCode::Escape => {
                    return ScreenResult::Quit;
                }
                VirtualKeyCode::Down => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        log("Camera down");
                        camera.pos.y = camera.pos.y + 1;
                    }
                }
                VirtualKeyCode::Left => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        camera.pos.x = camera.pos.x - 1;
                    }
                }
                VirtualKeyCode::Up => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        camera.pos.y = camera.pos.y - 1;
                    }
                }
                VirtualKeyCode::Right => {
                    if let Some(mut camera) = ecs.resources.get_mut::<Camera>() {
                        camera.pos.x = camera.pos.x + 1;
                    }
                }
                VirtualKeyCode::Equals => {
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 - 8).max(20), (size.1 - 5).max(10));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                VirtualKeyCode::Minus => {
                    let map_size = ecs.resources.get::<Map>().unwrap().get_size();
                    let size = self.viewport.size();
                    self.viewport
                        .resize((size.0 + 8).min(map_size.0), (size.1 + 5).min(map_size.1));
                    log(format!("Viewport size={:?}", self.viewport.size()));
                }
                _ => {}
            },
            _ => {}
        }

        ScreenResult::Continue
    }

    fn message(&mut self, _app: &mut Ecs, id: String, value: Option<Value>) -> ScreenResult {
        match id.as_str() {
            "VIEWPORT_MOVE" => {
                // let pt: Point = value.unwrap().try_into().unwrap();
                // log(format!("Mouse Pos = {}", pt));
            }
            "VIEWPORT_CLICK" => {
                let pt: Point = value.unwrap().try_into().unwrap();
                log(format!("CLICK = {}", pt));
            }
            _ => {}
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut Ecs) {
        self.viewport.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Acanja - World Viewer")
        .file(
            "assets/tiles.toml",
            Box::new(TileFileLoader::new().with_dump()),
        )
        .file(
            "assets/store_prefab.toml",
            Box::new(PrefabFileLoader::new().with_dump()),
        )
        .vsync(false)
        .build();

    app.run(MainScreen::new());
}

fn build_world_map(tiles: &Tiles, prefabs: &Prefabs, width: u32, height: u32) -> Map {
    let mut map = Map::new(width, height);

    let mut rng = RandomNumberGenerator::new();

    let ocean = tiles.get("OCEAN").unwrap();
    map.fill(ocean);

    let mut land_blob = Blob::new(BlobConfig {
        rng: RandomNumberGenerator::seeded(rng.next_u64()),
        min_width: (width as f32 * 0.3) as u32,
        min_height: (height as f32 * 0.3) as u32,
        max_width: (width as f32 * 0.8) as u32,
        max_height: (height as f32 * 0.8) as u32,
        percent_seeded: 53,
        ..BlobConfig::default()
    });

    let mut grid = Grid::new(width as usize, height as usize, 0);
    let mut land_count;
    let total_count = width * height;
    loop {
        land_blob.carve(width, height, |x, y| {
            grid.set(x, y, 1);
        });
        land_count = grid.count(1);
        let pct = land_count as f32 / total_count as f32;
        log(format!("carve land blob - pct covered={:.2}", pct));
        if pct > 0.3 {
            break;
        }
    }

    let mut forest_count = 0;
    loop {
        let x = rng.rand(width);
        let y = rng.rand(height);

        let count = spread_replace(&mut grid, x as i32, y as i32, 1, 2, &mut rng, 30);
        forest_count = forest_count + count;

        let pct = forest_count as f32 / land_count as f32;
        log(format!(
            "spread forest @ {},{} => {} - pct land covered = {:.2}",
            x, y, count, pct
        ));
        if pct > 0.3 {
            break;
        }
    }

    let grassland = tiles.get("GRASSLAND").unwrap();
    let forest = tiles.get("FOREST").unwrap();
    for (x, y, v) in grid.iter() {
        if *v == 1 {
            map.set_tile(x, y, grassland.clone());
        } else if *v == 2 {
            map.set_tile(x, y, forest.clone());
        }
    }

    map
}
