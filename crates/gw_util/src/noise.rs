use opensimplex_noise_rs::OpenSimplexNoise;
use std::f64::consts::PI;

const TAU: f64 = PI as f64 * 2.0;

pub fn cylindernoise(noise: &mut OpenSimplexNoise, nx: f64, ny: f64) -> f64 {
    let angle_x = TAU * nx;

    /* In "noise parameter space", we need nx and ny to travel the
    same distance. The circle created from nx needs to have
    circumference=1 to match the length=1 line created from ny,
    which means the circle's radius is 1/2π, or 1/tau */
    noise.eval_3d(angle_x.cos() / TAU, (angle_x.sin()) / TAU, ny)
}

pub fn torusnoise(noise: &mut OpenSimplexNoise, nx: f64, ny: f64) -> f64 {
    let angle_x = TAU * nx;
    let angle_y = TAU * ny;
    noise.eval_4d(
        angle_x.cos() / TAU,
        angle_x.sin() / TAU,
        angle_y.cos() / TAU,
        angle_y.sin() / TAU,
    )
}

pub struct NoiseConfig {
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

pub fn get_noise(config: NoiseConfig, seed: u64) -> Vec<f64> {
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

pub fn print_histogram(values: &Vec<f64>) {
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

pub fn square_bump(size: (usize, usize), values: &mut Vec<f64>, pow: f64) {
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

pub fn normalize(values: &mut Vec<f64>) {
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
