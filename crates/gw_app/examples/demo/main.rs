use gw_app::*;
use gw_app::ecs::{Read, Write, systems::ResourceSet};

mod entity;
mod level;
mod light;
mod noise;
mod player;

use entity::Entity;
use gw_app::fps::Fps;
use level::{Level, load_level};
use player::Player;

const FONT: &str = "resources/terminal_8x8.png";
const LEVEL_PREFIX: &str = "resources/demo_level";

pub const CONSOLE_WIDTH: u32 = 80;
pub const CONSOLE_HEIGHT: u32 = 45;
pub const PLAYER_SPEED: f32 = 0.2;
pub const PLAYER_FOV_RADIUS: usize = 40;

pub struct Entities(pub Vec<Entity>);

struct DoryenDemo {
    con: Panel,
    map_con: Panel,
    // player: Player,
    // entities: Vec<Entity>,
    // mouse_pos: (f32, f32),
    // level: Level,
    loaded: bool,
}

impl  DoryenDemo {
    fn new() -> Box<Self> {
        let con = Panel::new(CONSOLE_WIDTH, CONSOLE_HEIGHT, FONT);
        let map_con = Panel::new(CONSOLE_WIDTH, CONSOLE_HEIGHT, "SUBCELL");

        Box::new(Self {
            con,
            map_con,
            // player: Player::new(PLAYER_SPEED),
            // mouse_pos: (0.0, 0.0),
            // level: Level::new(app, LEVEL_PREFIX),
            // entities: Vec::new(),
            loaded: false,
        })
    }
}

impl DoryenDemo {
    fn clear_con(&mut self) {
        self.con.buffer_mut()
            .fill(Some(0), Some(RGBA::new()), Some(RGBA::new()) );
    }

    fn render_entities(&mut self, ecs: &mut Ecs) {

        let (entities, level, player) = <(Read<Entities>, Read<Level>, Read<Player>)>::fetch(&ecs.resources);

        let buffer = self.con.buffer_mut();
        for entity in entities.0.iter() {
            if level.is_in_fov(entity.pos) {
                entity.render(buffer, &*level);
            }
        }

        let player_pos = player.pos();
        let player_light = level.light_at(player_pos);
        player.render(buffer, player_light);
    }
}

impl Screen for DoryenDemo {
    fn update(&mut self, app: &mut Ecs) -> ScreenResult {
        if !self.loaded {
            self.loaded = load_level(app, LEVEL_PREFIX);
        }
        if self.loaded {
            let (input, mut player, mut level) = <(Read<AppInput>, Write<Player>, Write<Level>)>::fetch_mut(&mut app.resources);

            let mut coef = 1.0 / std::f32::consts::SQRT_2;
            let mut mov = player.move_from_input(&*input);
            if level.is_wall(player.next_pos((mov.0, 0))) {
                mov.0 = 0;
                coef = 1.0;
            }
            if level.is_wall(player.next_pos((0, mov.1))) {
                mov.1 = 0;
                coef = 1.0;
            }
            if player.move_by(mov, coef) {
                level.compute_fov(player.pos(), PLAYER_FOV_RADIUS);
            }
            level.update();
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut Ecs) {
        if self.loaded {
            self.clear_con();

            {
                let (mut level, player) = <(Write<Level>, Read<Player>)>::fetch_mut(&mut app.resources);
                level.render(self.map_con.buffer_mut(), player.pos());
            }

            self.render_entities(app);

            let fps = app.resources.get::<Fps>().unwrap().current();

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

        self.map_con.render(app);
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

    app.run(DoryenDemo::new());
}
