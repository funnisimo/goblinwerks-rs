use gw_app::color::RGBA;
use gw_app::*;

const FONT: &str = "resources/terminal_8x8.png";
const BIG_FONT: &str = "resources/ProjectUtumno_full_32x32.png";

const _BLACK: RGBA = RGBA::rgb(0, 0, 0);
const _GRAY: RGBA = RGBA::rgb(128, 128, 128);
const _RED: RGBA = RGBA::rgb(192, 32, 32);
const _YELLOW: RGBA = RGBA::rgb(192, 192, 32);

struct RNG {
    seed: u64,
}

impl RNG {
    pub fn new() -> Self {
        RNG { seed: 0xdead_beef }
    }

    fn next_u64(&mut self) -> u64 {
        self.seed = 214_013u64.wrapping_mul(self.seed).wrapping_add(2_531_011);
        self.seed
    }
}

// struct LoadingScreen {
//     con: Console,
// }
// impl LoadingScreen {
//     fn new() -> Box<Self> {
//         let con = Console::new(80, 50, "DEFAULT");
//         Box::new(LoadingScreen { con })
//     }
// }

// impl Screen for LoadingScreen {
//     fn update(&mut self, app: &mut AppContext, _frame_time_ms: f64) -> ScreenResult {
//         if app.get_font(BIG_FONT).is_some() {
//             return ScreenResult::Replace(MainScreen::new());
//         }
//         ScreenResult::Continue
//     }

//     fn render(&mut self, app: &mut AppContext) {
//         let buf = self.con.buffer_mut();
//         buf.clear(true, true, true);

//         buf.fill(Some('.' as u32), Some(YELLOW), Some(BLACK));

//         draw::plain(buf).print(1, 1, "Hello Rust World");
//         draw::plain(buf).print(1, 2, "Loading a bigger font...");

//         self.con.render(app);
//     }
// }

struct MainScreen {
    con: Console,
    len: u32,
    rng: RNG,
}

impl MainScreen {
    pub fn new() -> Box<MainScreen> {
        let con = Console::new(80, 40, BIG_FONT);

        Box::new(MainScreen {
            con,
            len: 0,
            rng: RNG::new(),
        })
    }
}

impl Screen for MainScreen {
    fn update(&mut self, app: &mut AppContext, _ms: f64) -> ScreenResult {
        if self.len == 0 {
            match app.get_font(BIG_FONT) {
                None => {}
                Some(font) => self.len = font.count(),
            }
        }
        ScreenResult::Continue
    }

    fn input(&mut self, _ctx: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::MouseDown(_) => ScreenResult::Pop,
            _ => ScreenResult::Continue,
        }
    }

    fn render(&mut self, app: &mut AppContext) {
        // let screen_pct = app.input().mouse_pct();
        // let cell_pct = self.con.cell_pos(screen_pct);

        if self.len == 0 {
            return;
        }

        let buf = self.con.buffer_mut();

        // buf.clear(true, true, true);

        for y in 0..buf.height() as i32 {
            for x in 0..buf.width() as i32 {
                if self.rng.next_u64() % 10_u64 == 0 {
                    let glyph = self.rng.next_u64() as u32 % self.len;
                    buf.draw_opt(x, y, Some(glyph), Some(RGBA::rgb(255, 255, 255)), None)
                }
            }
        }

        self.con.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Loading Screen Example")
        .font(FONT)
        .font(BIG_FONT)
        .vsync(false)
        .build();
    app.run_screen(MainScreen::new());
}
