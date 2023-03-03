use gw_app::color::{BLACK, RGBA};
use gw_app::*;

const WIDTH: usize = 80;
const HEIGHT: usize = 50;

struct MainScreen {
    con: Console,
    cells: Vec<bool>,
    mpos: (i32, i32),
}

impl MainScreen {
    fn new() -> Box<Self> {
        let con = Console::new(WIDTH as u32, HEIGHT as u32, "DEFAULT");
        Box::new(MainScreen {
            con,
            cells: vec![false; (WIDTH * HEIGHT) as usize],
            mpos: (0, 0),
        })
    }

    fn neighbor_count(&self, idx: usize) -> u8 {
        let mut count = 0_u8;

        let x = idx % WIDTH;
        let y = idx / WIDTH;

        for cy in y.saturating_sub(1)..=(y + 1) {
            if cy >= HEIGHT {
                continue;
            }
            for cx in x.saturating_sub(1)..=(x + 1) {
                if cx >= WIDTH {
                    continue;
                }
                if cx == x && cy == y {
                    continue;
                }
                count += match self.cells[cx as usize + cy as usize * WIDTH] {
                    false => 0,
                    true => 1,
                };
            }
        }
        count
    }

    fn next_turn(&mut self) {
        println!("turn");
        self.cells = self
            .cells
            .iter()
            .enumerate()
            .map(|(idx, v)| (v, self.neighbor_count(idx)))
            .map(|(v, cnt)| life_value(v, cnt))
            .collect();
    }

    fn flip(&mut self, x: i32, y: i32) {
        println!("flip - {},{}", x, y);
        let idx = x as usize + y as usize * WIDTH;
        match self.cells.get(idx) {
            None => {}
            Some(true) => self.cells[idx] = false,
            Some(false) => self.cells[idx] = true,
        }
    }

    fn set(&mut self, x: i32, y: i32) {
        let idx = x as usize + y as usize * WIDTH;
        self.cells[idx] = true;
    }
}

impl Screen for MainScreen {
    fn input(&mut self, app: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        let input = app.resources.get::<AppInput>().unwrap();

        let screen_pct = input.mouse_pct();
        match self.con.mouse_pos(screen_pct) {
            None => self.mpos = (0, 0),
            Some(con_pos) => {
                let x = con_pos.0 as i32;
                let y = con_pos.1 as i32;
                self.mpos = (x, y);
            }
        }

        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Space => {
                    self.next_turn();
                    ScreenResult::Continue
                }
                VirtualKeyCode::Escape => ScreenResult::Quit,
                _ => ScreenResult::Continue,
            },
            AppEvent::MouseDown(_) => {
                self.flip(self.mpos.0, self.mpos.1);

                ScreenResult::Continue
            }
            AppEvent::MousePos(_) => {
                if input.mouse_button(0) {
                    self.set(self.mpos.0, self.mpos.1);
                }
                ScreenResult::Continue
            }
            _ => ScreenResult::Continue,
        }
    }

    fn render(&mut self, app: &mut Ecs) {
        let buf = self.con.buffer_mut();

        buf.fill(Some(0), None, Some(BLACK));

        for (idx, val) in self.cells.iter().enumerate() {
            let x: i32 = (idx % WIDTH) as i32;
            let y: i32 = (idx / WIDTH) as i32;

            match val {
                true => {
                    buf.back(x, y, RGBA::rgba(255, 64, 64, 255));
                }
                false => {}
            }
        }

        let idx = (self.mpos.0 + self.mpos.1 * WIDTH as i32) as usize;
        match self.cells.get(idx) {
            None => {}
            Some(false) => {
                buf.back(self.mpos.0, self.mpos.1, RGBA::rgba(64, 255, 64, 255));
            }
            Some(true) => {
                buf.back(self.mpos.0, self.mpos.1, RGBA::rgba(64, 64, 255, 255));
            }
        }

        self.con.render(app);
    }
}

fn life_value(v: &bool, cnt: u8) -> bool {
    match v {
        true if cnt == 2 || cnt == 3 => true,
        false if cnt == 3 => true,
        _ => false,
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Game of Life")
        .fps(60)
        .vsync(false)
        .build();
    app.run(MainScreen::new());
}
