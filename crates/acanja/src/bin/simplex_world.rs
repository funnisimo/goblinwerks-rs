use acanja::map::prefab::PrefabFileLoader;
use acanja::map::prefab::Prefabs;
use gw_app::ecs::{systems::ResourceSet, Read};
use gw_app::*;
use gw_util::point::Point;
use gw_util::rng::{RandomNumberGenerator, RngCore};
use gw_world::map::dump_map;
use gw_world::map::{Builder, Map};
use gw_world::memory::MapMemory;
use gw_world::tile::TileFileLoader;
use gw_world::tile::Tiles;
use gw_world::widget::Viewport;
use opensimplex_noise_rs::OpenSimplexNoise;

// SOURCE: https://github.com/Mapet13/terrain-generator-2d

// const IMAGE_SIZE: [i32; 2] = [128, 128];

fn sum_octaves(
    num_iterations: i32,
    point: (i32, i32),
    persistence: f64,
    scale: f64,
    low: f64,
    high: f64,
    noise_fn: impl Fn(f64, f64) -> f64,
) -> f64 {
    let mut max_amp = 0.0;
    let mut amp = 1.0;
    let mut freq = scale;
    let mut noise = 0.0;

    for _ in 0..num_iterations {
        noise += noise_fn(point.0 as f64 * freq, point.1 as f64 * freq) * amp;
        max_amp += amp;
        amp *= persistence;
        freq *= 2.0;
    }

    (noise / max_amp) * (high - low) / 2.0 + (high + low) / 2.0
}

fn generate_gradient(width: u32, height: u32) -> Vec<f32> {
    let mut gradient: Vec<f32> = vec![1.0; (width * height) as usize];

    for x in 0..width {
        for y in 0..height {
            let mut color_value: f32;

            let a = if x > (width / 2) { width - x } else { x };

            let b = if y > height / 2 { height - y } else { y };

            let smaller = std::cmp::min(a, b) as f32;
            color_value = smaller / (width as f32 / 2.0);

            color_value = 1.0 - color_value;
            color_value = color_value * color_value;

            gradient[get_id_from_pos(width, height, x as i32, y as i32)] = match color_value - 0.1 {
                x if x > 1.0 => 1.0,
                x if x < 0.0 => 0.0,
                x => x,
            };
        }
    }

    gradient
}

fn generate_maps(width: u32, height: u32, gradient: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mut rng = RandomNumberGenerator::new();

    let mut height_map = generate_noise_map(rng.next_u64() as i64, 0.004, width, height);
    let mut biome_map = generate_noise_map(rng.next_u64() as i64, 0.007, width, height);

    for x in 0..width as i32 {
        for y in 0..height as i32 {
            height_map[get_id_from_pos(width, height, x, y)] =
                height_map[get_id_from_pos(width, height, x, y)] * 1.1
                    - gradient[get_id_from_pos(width, height, x, y)] * 0.8;
            biome_map[get_id_from_pos(width, height, x, y)] = biome_map
                [get_id_from_pos(width, height, x, y)]
                - (0.1 - gradient[get_id_from_pos(width, height, x, y)]) * 0.4;
            if height_map[get_id_from_pos(width, height, x, y)] < 0.0 {
                height_map[get_id_from_pos(width, height, x, y)] = 0.0;
            }
            if biome_map[get_id_from_pos(width, height, x, y)] < 0.0 {
                biome_map[get_id_from_pos(width, height, x, y)] = 0.0;
            }
        }
    }

    (height_map, biome_map)
}

fn get_id_from_pos(width: u32, _height: u32, x: i32, y: i32) -> usize {
    (x + width as i32 * y) as usize
}

fn generate_noise_map(seed: i64, scale: f64, width: u32, height: u32) -> Vec<f32> {
    let noise_generator = OpenSimplexNoise::new(Some(seed));

    let mut map: Vec<f32> = vec![0.0; (width * height) as usize];
    for x in 0..width as i32 {
        for y in 0..height as i32 {
            let val = sum_octaves(16, (x, y), 0.5, scale, 0.0, 1.0, |x, y| {
                noise_generator.eval_2d(x, y)
            });

            map[get_id_from_pos(width, height, x, y)] = val as f32;
        }
    }
    map
}

#[derive(Clone, Debug, Copy)]
enum Biomes {
    None,
    Grass,
    DeepWater,
    Water,
    Dirt,
    Sand,
    WetSand,
    DarkForest,
    HighDarkForest,
    LightForest,
    Mountain,
    HighMountain,
    Snow,
}

// fn get_biome_color(biome: Biomes) -> [u8; 3] {
//     match biome {
//         Biomes::Grass => [120, 157, 80],
//         Biomes::Water => [9, 82, 198],
//         Biomes::DeepWater => [0, 62, 178],
//         Biomes::Dirt => [114, 98, 49],
//         Biomes::Sand => [194, 178, 128],
//         Biomes::WetSand => [164, 148, 99],
//         Biomes::DarkForest => [60, 97, 20],
//         Biomes::HighDarkForest => [40, 77, 0],
//         Biomes::LightForest => [85, 122, 45],
//         Biomes::Mountain => [140, 142, 123],
//         Biomes::HighMountain => [160, 162, 143],
//         Biomes::Snow => [235, 235, 235],
//     }
// }

fn generate_image(width: u32, height: u32, height_map: &[f32], biome_map: &[f32]) -> Vec<Biomes> {
    // let mut image =
    //     ImageBuffer::<Rgb<u8>, Vec<u8>>::new(IMAGE_SIZE[0] as u32, IMAGE_SIZE[1] as u32);

    let mut image: Vec<Biomes> = vec![Biomes::None; (width * height) as usize];

    for x in 0..width as i32 {
        for y in 0..height as i32 {
            let idx = get_id_from_pos(width, height, x, y);
            let height = height_map[idx];
            let moisture = biome_map[idx];

            let biome = match (height, moisture) {
                (a, _) if a < 0.39 => Biomes::DeepWater,
                (a, _) if a < 0.42 => Biomes::Water,
                (a, b) if a < 0.46 && b < 0.57 => Biomes::Sand,
                (a, b) if a < 0.47 && b < 0.6 => Biomes::WetSand,
                (a, b) if a < 0.47 && b >= 0.6 => Biomes::Dirt,
                (a, b) if a > 0.54 && b < 0.43 && a < 0.62 => Biomes::Grass,
                (a, b) if a < 0.62 && b >= 0.58 => Biomes::HighDarkForest,
                (a, b) if a < 0.62 && b >= 0.49 => Biomes::DarkForest,
                (a, _) if a >= 0.79 => Biomes::Snow,
                (a, _) if a >= 0.74 => Biomes::HighMountain,
                (a, b) if a >= 0.68 && b >= 0.10 => Biomes::Mountain,
                _ => Biomes::LightForest,
            };

            image[idx] = biome;
            // let color = get_biome_color(biome);
            // let pixel = image.get_pixel_mut(x as u32, y as u32);
            // *pixel = image::Rgb(color);
        }
    }

    image
}

pub fn build_world_map<'t>(tiles: &'t Tiles, prefabs: &Prefabs, width: u32, height: u32) -> Map {
    let mut builder = Builder::new(tiles, width, height);

    println!("Generating gradient...");
    let gradient = generate_gradient(width, height);
    println!("DONE");

    println!("Generating maps...");
    let (height_map, biome_map) = generate_maps(width, height, &gradient);
    println!("DONE");

    println!("Generating image...");
    let image = generate_image(width, height, &height_map, &biome_map);

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let idx = get_id_from_pos(width, height, x, y);
            let biome = image[idx];

            let tile = match biome {
                Biomes::DarkForest => "FOREST",
                Biomes::DeepWater => "DEEP_OCEAN",
                Biomes::Dirt => "FLOOR",
                Biomes::Grass => "GRASSLAND",
                Biomes::HighDarkForest => "FOREST",
                Biomes::HighMountain => "MOUNTAIN",
                Biomes::LightForest => "FOREST",
                Biomes::Mountain => "MOUNTAIN",
                Biomes::None => "ERROR",
                Biomes::Sand => "BEACH",
                Biomes::Snow => "SNOW",
                Biomes::Water => "LAKE",
                Biomes::WetSand => "BEACH",
            };

            builder.set_tile(x, y, tile);
        }
    }

    builder.build()
}

struct MainScreen {
    viewport: Viewport,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let viewport = Viewport::builder("VIEWPORT").size(128, 128).build();

        Box::new(MainScreen { viewport })
    }

    fn build_new_map(&self, ecs: &mut Ecs) {
        let mut map = {
            let (tiles, prefabs) = <(Read<Tiles>, Read<Prefabs>)>::fetch(&ecs.resources);

            log(format!("- prefabs: {}", prefabs.len()));
            // let mut map = dig_room_level(&tiles, 80, 50);
            build_world_map(&tiles, &prefabs, 128, 128)
        };

        map.reveal_all();
        map.make_fully_visible();

        ecs.resources.insert(map);
        ecs.resources.insert(MapMemory::new(128, 128));
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
