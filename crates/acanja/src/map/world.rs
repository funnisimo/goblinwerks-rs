use super::prefab::*;
use gw_app::log;
use gw_util::rng::RngCore;
use gw_world::map::{Builder, Map};
use gw_world::tile::{Tile, Tiles};
use opensimplex_noise_rs::OpenSimplexNoise;
use std::cmp::max;
use std::f32::consts::PI;
use std::sync::Arc;

pub fn build_world_map<'t>(tiles: &'t Tiles, prefabs: &Prefabs, width: u32, height: u32) -> Map {
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

const TAU: f64 = PI as f64 * 2.0;

fn cylindernoise(noise: &mut OpenSimplexNoise, nx: f64, ny: f64) -> f64 {
    let angle_x = TAU * nx;

    /* In "noise parameter space", we need nx and ny to travel the
    same distance. The circle created from nx needs to have
    circumference=1 to match the length=1 line created from ny,
    which means the circle's radius is 1/2π, or 1/tau */
    noise.eval_3d(angle_x.cos() / TAU, (angle_x.sin()) / TAU, ny)
}

// fn torusnoise(noise: &mut FastNoise, nx: f32, ny: f32) -> f32 {
//     let angle_x = TAU * nx;
//     let angle_y = TAU * ny;
//     noise.get_noise4D(
//         angle_x.cos() / TAU,
//         angle_x.sin() / TAU,
//         angle_y.cos() / TAU,
//         angle_y.sin() / TAU,
//     )
// }

struct NoiseConfig {
    pub size: (usize, usize),
    pub offsets: (f64, f64, f64),
    pub mults: (f64, f64, f64),
    pub pcts: (f64, f64, f64),
    pub pows: (f64, f64, f64),
    pub all_pow: f64,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        NoiseConfig {
            size: (160, 100),
            offsets: (0.0, 0.0, 0.0),
            mults: (0.5, 0.05, 0.015),
            pcts: (0.5, 0.5, 0.5),
            pows: (1.0, 1.0, 1.0),
            all_pow: 1.0,
        }
    }
}

fn get_noise(config: NoiseConfig, seed: u64) -> Vec<f64> {
    let size = config.size;
    let offsets = config.offsets;
    let mults = config.mults;
    let pcts = config.pcts;
    let pows = config.pows;
    let all_pow = config.all_pow;

    let noise = OpenSimplexNoise::new(Some(seed as i64));
    let mut values = vec![0.0; size.0 * size.1];

    for y in 0..size.1 {
        for x in 0..size.0 {
            let nx: f64 = x as f64; // * self.scale.0;
            let ny: f64 = y as f64; // * self.scale.0;

            // let mut val = noise.eval_2d(nx, ny);

            let e = pcts.0
                * noise
                    .eval_2d(mults.0 * (nx + offsets.0), mults.0 * (ny + offsets.0))
                    .abs()
                    .powf(pows.0)
                + pcts.1
                    * noise
                        .eval_2d(mults.1 * (nx + offsets.1), mults.1 * (ny + offsets.1))
                        .abs()
                        .powf(pows.1)
                + pcts.2
                    * noise
                        .eval_2d(mults.2 * (nx + offsets.2), mults.2 * (ny + offsets.2))
                        .abs()
                        .powf(pows.2);

            let norm = e / (pcts.0 + pcts.1 + pcts.2);
            let val = norm.powf(all_pow);

            let idx = (x + size.0 * y) as usize;
            values[idx] = val;
        }
    }

    normalize(&mut values);

    // log(format!("- scale={:?}", self.scale));
    // log(format!("- neg count={}/{}", neg_count, size.0 * size.1));

    values
}

fn print_histogram(values: &Vec<f64>) {
    let mut histogram: Vec<u32> = vec![0; 10];

    for v in values.iter() {
        let hist_idx = (v / 0.1).clamp(0.0, 9.0) as usize;
        histogram[hist_idx] = histogram[hist_idx] + 1;
    }

    println!("HISTOGRAM");
    let count = values.len() as f32;
    for i in 0..10 {
        let num = match histogram[i] {
            0 => 0,
            x => 1 + (100.0 * x as f32 / count) as usize,
        };
        let chars = "#".repeat(num);
        println!("{:2} |{}", i, chars);
    }
}

fn square_bump(size: (usize, usize), values: &mut Vec<f64>, pow: f64) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            let idx = x + size.0 * y;
            let val = values[idx];

            let nx = 2.0 * (x as f64 / size.0 as f64) - 1.0; // -1.0 to 1.0
            let ny = 2.0 * (y as f64 / size.1 as f64) - 1.0; // -1.0 to 1.0

            let d = 1.0 - ((1.0 - nx.powi(2)) * (1.0 - ny.powi(2))); // 0.0 to 1.0 (close to 0.0 in middle)
            let e = (val + (1.0 - d.powf(pow))) / 2.0;

            values[idx] = e;
        }
    }
}

fn normalize(values: &mut Vec<f64>) {
    let mut lo: f64 = f64::MAX;
    let mut hi: f64 = f64::MIN;

    for v in values.iter() {
        if *v < lo {
            lo = *v;
        }
        if *v > hi {
            hi = *v;
        }
    }

    let range = hi - lo;

    for idx in 0..values.len() {
        let v = values[idx];

        let norm = ((v - lo) / range).clamp(0.0, 1.0);
        values[idx] = norm;
    }
}

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
