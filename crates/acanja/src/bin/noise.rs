use gw_util::rng::{RandomNumberGenerator, RngCore};
use noise::{NoiseFn, Perlin, Simplex};

fn main() {
    let mut rng = RandomNumberGenerator::new();

    let seed = rng.next_u32();

    test_noise(seed, 100.0, 0.0);
    test_noise(seed, 100.0, 0.5);
    test_noise(seed, 1.0, 0.0);
    test_noise(seed, 200.0, -10.0);
    test_noise(seed, 50.0, 1.0);
    test_noise(seed, 5.0, 0.0);
    test_noise(seed, 500.0, -10.0);
    test_noise(seed, 1.0, 0.0);

    println!("seed = {}", seed);
}

fn test_noise(seed: u32, div: f64, offset: f64) {
    let noise = Perlin::new(seed);

    let mut lo: f64 = f64::MAX;
    let mut hi: f64 = f64::MIN;

    let mut values: Vec<f64> = vec![0.0; 100 * 100];

    for x in 0..100 {
        for y in 0..100 {
            let nx = x as f64 / div - offset;
            let ny = y as f64 / div - offset;

            let v = noise.get([nx, ny]);
            if v < lo {
                lo = v;
            }
            if v > hi {
                hi = v;
            }

            let idx = x + y * 100;
            values[idx] = v;
        }
    }

    let range_lo = 0.0 / div - offset;
    let range_hi = 100.0 / div - offset;

    println!(
        "Noise: div={}, offset={}, range={}-{}",
        div, offset, range_lo, range_hi
    );

    println!("   raw - min={}, max={}", lo, hi);

    let range = hi - lo;

    let normalized: Vec<f64> = values
        .iter()
        .map(|v| (100.0 * (v - lo) / range).trunc())
        .collect();

    let mut counts: Vec<u32> = vec![0; 10];

    for v in normalized.iter() {
        let iv = *v as usize;
        let idx = iv % 10;
        counts[idx] += 1;
    }

    for (i, v) in counts.iter().enumerate() {
        let dot_len = v / 100 + 1;
        let dots = "#".repeat(dot_len as usize);

        println!("{}: {:4} |{}", i, v, dots);
    }
}
