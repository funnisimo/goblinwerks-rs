use gw_app::{
    color::named::{self, BLACK},
    *,
};
use gw_util::rng::{RandomNumberGenerator, RngCore};
use opensimplex_noise_rs::OpenSimplexNoise;

struct MainScreen {
    con: Panel,
    rng: RandomNumberGenerator,
    // values: Vec<f64>,
    // scale: (f64, f64),
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let con = Panel::new(160, 100, "DEFAULT");
        let rng = RandomNumberGenerator::new();
        Box::new(MainScreen {
            con,
            rng,
            // values: vec![0.0; 160 * 100], // scale: (1.0, 1.0),
        })
    }

    pub fn draw(&mut self) {
        let seed = self.rng.next_u64();

        let width = self.con.width() as usize;
        let height = self.con.height() as usize;

        let mut values = get_noise(
            NoiseConfig {
                size: (width, height),
                pcts: (1.0, 1.0, 1.0),
                ..Default::default()
            },
            seed,
        );

        // let mut values = vec![0.5; width * height];
        print_histogram(&values);

        square_bump((width, height), &mut values, 1.0);
        print_histogram(&values);

        let buf = self.con.buffer_mut();
        let black = BLACK.into();

        for y in 0..height {
            for x in 0..width {
                let idx = (x + width * y) as usize;
                let v = values[idx];

                let alpha = (255.0 * v) as u8;
                if alpha < 128 {
                    buf.draw(
                        x as i32,
                        y as i32,
                        0,
                        black,
                        RGBA::rgba(0, 0, 255, 255 - alpha),
                    );
                } else if alpha > 215 {
                    buf.draw(x as i32, y as i32, 0, black, named::WHITE.into());
                } else if alpha > 205 {
                    buf.draw(x as i32, y as i32, 0, black, named::SADDLEBROWN.into());
                } else if alpha > 190 {
                    buf.draw(x as i32, y as i32, 0, black, named::GRAY14.into());
                } else if alpha > 175 {
                    buf.draw(x as i32, y as i32, 0, black, named::DARKGREEN.into());
                } else if alpha > 140 {
                    buf.draw(x as i32, y as i32, 0, black, named::GREEN.into());
                } else {
                    buf.draw(x as i32, y as i32, 0, black, named::SANDYBROWN.into());
                }
            }
        }
    }
}

impl Screen for MainScreen {
    fn setup(&mut self, _ecs: &mut Ecs) {
        self.draw();
    }

    fn input(&mut self, _ecs: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::KeyDown(key) => match key.key_code {
                VirtualKeyCode::Escape => return ScreenResult::Quit,
                // VirtualKeyCode::X => {
                //     if key.shift {
                //         self.scale.0 = self.scale.0 * 2.0;
                //     } else {
                //         self.scale.0 = self.scale.0 / 2.0;
                //     }
                //     self.draw();
                // }
                // VirtualKeyCode::Y => {
                //     if key.shift {
                //         self.scale.1 = self.scale.1 * 2.0;
                //     } else {
                //         self.scale.1 = self.scale.1 / 2.0;
                //     }
                //     self.draw();
                // }
                _ => self.draw(),
            },
            AppEvent::MouseDown(_) => self.draw(),
            _ => {}
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut Ecs) {
        self.con.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768).title("Minimal Example").build();
    app.run(MainScreen::new());
}

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
