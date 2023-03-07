use gw_app::{color::named::BLACK, *};
use gw_util::noise;
use gw_util::rng::{RandomNumberGenerator, RngCore};

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

        let values = noise::get_noise(
            noise::NoiseConfig {
                size: (width, height),
                pcts: (1.0, 1.0, 1.0),
                ..Default::default()
            },
            seed,
        );

        // let mut values = vec![0.5; width * height];
        noise::print_histogram(&values);

        let buf = self.con.buffer_mut();
        let black = BLACK.into();

        for y in 0..height {
            for x in 0..width {
                let idx = (x + width * y) as usize;
                let v = values[idx];

                let alpha = (255.0 * v) as u8;
                buf.draw(x as i32, y as i32, 0, black, RGBA::rgba(255, 0, 0, alpha));
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
