use gw_app::*;

mod entity;
mod level;
mod light;
mod noise;
mod player;

use entity::Entity;
use level::Level;
use player::Player;

const FONT: &str = "resources/terminal_8x8.png";
const LEVEL_PREFIX: &str = "resources/demo_level";

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;
const PLAYER_SPEED: f32 = 0.2;
const PLAYER_FOV_RADIUS: usize = 40;

struct DoryenDemo {
    con: Console,
    player: Player,
    entities: Vec<Entity>,
    mouse_pos: (f32, f32),
    level: Level,
    loaded: bool,
}

impl  DoryenDemo {
    fn new(app: &mut AppContext) -> Box<Self> {
        let con = Console::new(CONSOLE_WIDTH, CONSOLE_HEIGHT, FONT);

        Box::new(Self {
            con,
            player: Player::new(PLAYER_SPEED),
            mouse_pos: (0.0, 0.0),
            level: Level::new(app, LEVEL_PREFIX),
            entities: Vec::new(),
            loaded: false,
        })
    }
}

impl DoryenDemo {
    fn clear_con(&mut self) {
        self.con.buffer_mut()
            .fill(Some(' ' as u32), Some(BLACK), Some(BLACK), );
    }

    fn render_entities(&mut self) {
        let buffer = self.con.buffer_mut();
        for entity in self.entities.iter() {
            if self.level.is_in_fov(entity.pos) {
                entity.render(buffer, &self.level);
            }
        }
        let player_pos = self.player.pos();
        let player_light = self.level.light_at(player_pos);
        self.player.render(buffer, player_light);
    }
}

impl Screen for DoryenDemo {
    fn update(&mut self, app: &mut AppContext, _ms: f64) -> ScreenResult {
        if !self.loaded {
            if let Some(entities) = self.level.try_load() {
                self.loaded = true;
                self.player.move_to(self.level.start_pos());
                self.level.compute_fov(self.player.pos(), PLAYER_FOV_RADIUS);
                self.entities = entities;
            }
        }
        if self.loaded {
            let mut coef = 1.0 / std::f32::consts::SQRT_2;
            let mut mov = self.player.move_from_input(app.input());
            if self.level.is_wall(self.player.next_pos((mov.0, 0))) {
                mov.0 = 0;
                coef = 1.0;
            }
            if self.level.is_wall(self.player.next_pos((0, mov.1))) {
                mov.1 = 0;
                coef = 1.0;
            }
            if self.player.move_by(mov, coef) {
                self.level.compute_fov(self.player.pos(), PLAYER_FOV_RADIUS);
            }
            self.mouse_pos = app.input().mouse_pct();
            self.level.update();
        }
        ScreenResult::Continue
    }
    fn render(&mut self, app: &mut AppContext) {
        if self.loaded {
            self.clear_con();
            self.level.render(self.con.buffer_mut(), self.player.pos());
            self.render_entities();
            let fps = app.current_fps();

            draw::colored(self.con.buffer_mut()).align(TextAlign::Center).print(
            
                (CONSOLE_WIDTH / 2) as i32,
                (CONSOLE_HEIGHT - 2) as i32,
                &format!("#[white]Move with #[red]arrows or WSAD #[white]Fire with #[red]mouse   {:4} fps",fps),
            
            );
        } else {
            draw::colored(self.con.buffer_mut()).align(TextAlign::Center).print(
                (CONSOLE_WIDTH / 2) as i32,
                (CONSOLE_HEIGHT / 2) as i32,
                "#[white]Loading#[red]...",
            );
        }

        self.con.render(app);
    }
}


fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Close App Example")
        .font(FONT)
        .image(&(LEVEL_PREFIX.to_owned() + ".png"))
        .image(&(LEVEL_PREFIX.to_owned() + "_color.png"))
        .vsync(false)
        .build();

    app.run_with(Box::new(|app| DoryenDemo::new(app)));
}
