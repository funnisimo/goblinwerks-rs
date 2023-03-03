use super::prefab::*;
use bracket_noise::prelude::*;
use gw_app::log;
use gw_util::rng::RngCore;
use gw_world::map::{Builder, Map};
use gw_world::tile::Tiles;
use std::cmp::max;

pub fn build_world_map<'t>(tiles: &'t Tiles, prefabs: &Prefabs, width: u32, height: u32) -> Map {
    let mut builder = Builder::new(tiles, width, height);

    loop {
        builder.fill("LAKE");

        let seed = builder.rng_mut().next_u64();
        let mut noise = FastNoise::seeded(seed);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);

        let mut elevation: Vec<f32> = vec![0.0; (width * height) as usize];

        let scale: (f32, f32) = (width as f32 * 1.0, height as f32 * 1.0);
        let mut lo = f32::MAX;
        let mut hi = f32::MIN;

        for x in 0..width {
            for y in 0..height {
                let dx = x as f32 / scale.0 - 0.5;
                let dy = y as f32 / scale.1 - 0.5;
                let v = noise.get_noise(dx, dy);

                let idx = (x + y * width) as usize;
                elevation[idx] = v;

                if v < lo {
                    lo = v;
                }
                if v > hi {
                    hi = v;
                }
            }
        }

        let range = hi - lo;

        for x in 0..width {
            for y in 0..height {
                let idx = (x + y * width) as usize;
                let v = elevation[idx];
                let norm = (100.0 * (v - lo) / range).trunc();
                elevation[idx] = norm;
            }
        }

        for x in 0..width as i32 {
            for y in 0..height as i32 {
                let idx = (x + y * width as i32) as usize;
                let v = elevation[idx];
                if v < 10.0 {
                    builder.set_tile(x, y, "DEEP_OCEAN");
                } else if v < 20.0 {
                    builder.set_tile(x, y, "OCEAN");
                } else if v < 30.0 {
                    builder.set_tile(x, y, "SHALLOW_OCEAN");
                } else if v < 40.0 {
                    builder.set_tile(x, y, "BEACH");
                } else if v < 40.0 {
                    builder.set_tile(x, y, "BEACH");
                } else if v < 70.0 {
                    builder.set_tile(x, y, "GRASSLAND");
                } else if v < 80.0 {
                    builder.set_tile(x, y, "FOREST");
                } else if v < 90.0 {
                    builder.set_tile(x, y, "HILLS");
                } else {
                    builder.set_tile(x, y, "MOUNTAINS");
                }
            }
        }

        break;
    }

    builder.build()
}
