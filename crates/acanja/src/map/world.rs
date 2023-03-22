use super::prefab::*;
use gw_app::log;
use gw_util::blob::{Blob, BlobConfig};
use gw_util::grid::{spread_replace, Grid};
use gw_util::noise::{get_noise, print_histogram, square_bump, NoiseConfig};
use gw_util::rng::{RandomNumberGenerator, RngCore};
use gw_world::effect::Portal;
use gw_world::map::{closest_points_matching, find_random_point, Builder, Cell, Map};
use gw_world::tile::{TileKind, Tiles};
use rand::prelude::SliceRandom;

pub fn build_world_map(tiles: &Tiles, prefabs: &Prefabs, width: u32, height: u32) -> Map {
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
        // log(format!(
        //     "spread forest @ {},{} => {} - pct land covered = {:.2}",
        //     x, y, count, pct
        // ));
        if pct > 0.3 {
            break;
        }
    }

    let grassland = tiles.get("GRASSLAND").unwrap();
    let forest = tiles.get("FOREST").unwrap();
    for (x, y, v) in grid.iter() {
        let index = map.get_wrapped_index(x, y).unwrap();
        if *v == 1 {
            map.reset_tiles(index, grassland.clone());
        } else if *v == 2 {
            map.reset_tiles(index, forest.clone());
        }
    }

    // ADD TOWNS...
    for i in 1..=4 {
        let town_loc = find_random_point(&mut map, &mut rng, |_x, _y, tiles| {
            tiles.ground().kind == TileKind::FLOOR && tiles.fixture().is_null()
        })
        .expect("Failed to find town location");
        let index = map.get_wrapped_index(town_loc.x, town_loc.y).unwrap();

        map.place_fixture(index, tiles.get("TOWN").unwrap());

        // For when you come out of the town
        let loc_name = format!("TOWN{}", i);
        map.set_location(&loc_name, index);

        // For going into the town
        let portal = Box::new(Portal::new(loc_name, "START".to_string()));
        map.add_effect(index, "descend", portal);
    }

    // ADD STARTING LOCATION
    let start_loc = {
        let choices =
            closest_points_matching(&map, width as i32 / 2, height as i32 / 2, |_x, _y, cell| {
                !cell.blocks()
            });
        if choices.len() == 0 {
            panic!("Failed to find starting location!");
        }
        choices.choose(&mut rng).unwrap().clone()
    };
    let index = map.get_wrapped_index(start_loc.x, start_loc.y).unwrap();

    map.locations.insert("START".to_string(), index);

    map
}

#[allow(dead_code)]
pub fn build_world_map_with_noise<'t>(
    tiles: &'t Tiles,
    prefabs: &Prefabs,
    width: u32,
    height: u32,
) -> Map {
    let mut builder = Builder::new(tiles, width, height);

    loop {
        let width = width as usize;
        let height = height as usize;

        let mut elevation = get_noise(
            NoiseConfig {
                size: (width, height),
                pcts: (1.0, 1.0, 1.0),
                ..Default::default()
            },
            builder.rng_mut().next_u64(),
        );

        // let mut values = vec![0.5; width * height];
        print_histogram(&elevation);

        square_bump((width, height), &mut elevation, 1.0);
        print_histogram(&elevation);

        let moisture = get_noise(
            NoiseConfig {
                size: (width, height),
                pcts: (1.0, 1.0, 1.0),
                ..Default::default()
            },
            builder.rng_mut().next_u64(),
        );

        for y in 0..height {
            for x in 0..width {
                let idx = (x + width * y) as usize;
                let e = elevation[idx];
                let m = moisture[idx];

                let tile = biome_tile(e, m);
                builder.set_tile(x as i32, y as i32, tile);
            }
        }

        // do sanity checks?

        break;
    }

    builder.build()
}

#[allow(dead_code)]
fn biome_tile(e: f64, m: f64) -> &'static str {
    // these thresholds will need tuning to match your generator
    if e < 0.3 {
        return "DEEP_OCEAN"; // DEEP OCEAN;
    }
    if e < 0.4 {
        return "OCEAN"; // OCEAN;
    }
    if e < 0.5 {
        return "SHALLOW_OCEAN"; // SHALLOW OCEAN;
    }
    if e < 0.52 {
        return "BEACH"; // BEACH;
    }

    if e > 0.8 {
        // if m < 0.1 {
        //     return RGBA::rgb(200, 200, 200); // SCORCHED;
        // }
        if m < 0.2 {
            return "MOUNTAIN"; // BARE;
        }
        if m < 0.5 {
            return "FOREST_MOUNTAIN"; // TUNDRA;
        }
        return "SNOW_MOUNTAIN"; // SNOW;
    }

    if e > 0.7 {
        if m < 0.33 {
            return "HILL"; // TEMPERATE_DESERT;
        }
        if m < 0.66 {
            return "SHRUB_HILL"; // SHRUBLAND;
        }
        return "FOREST_HILL"; // TAIGA;
    }

    if e > 0.6 {
        if m < 0.16 {
            return "SHRUB_DESERT"; // TEMPERATE_DESERT;
        }
        if m < 0.50 {
            return "GRASSLAND"; // GRASSLAND;
        }
        if m < 0.83 {
            return "FOREST"; // TEMPERATE_DECIDUOUS_FOREST;
        }
        return "RAINFOREST"; // TEMPERATE_RAIN_FOREST;
    }

    if m < 0.16 {
        return "DESERT"; // SUBTROPICAL_DESERT;
    }
    if m < 0.33 {
        return "GRASSLAND"; // GRASSLAND;
    }
    if m < 0.66 {
        return "FOREST"; // TROPICAL_SEASONAL_FOREST;
    }
    return "RAINFOREST"; // TROPICAL_RAIN_FOREST;
}
