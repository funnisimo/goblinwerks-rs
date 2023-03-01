use gw_app::*;

// This is similar to doryen-rs/examples/basic

/*
Apart from the basic real-time walking, this example shows how screenshots can be captured in-game.
Because it uses UpdateEvent, any combination of keys can be specified to activate it.
*/

const FONT: &str = "resources/terminal_8x8.png";

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

const WHITE: RGBA = RGBA::rgb(255, 255, 255);
const _RED: RGBA = RGBA::rgb(255, 0, 0);
const _GREEN: RGBA = RGBA::rgb(0, 255, 0);
const _BLUE: RGBA = RGBA::rgb(0, 0, 255);
const BLACK: RGBA = RGBA::rgb(0, 0, 0);
const GRAY: RGBA = RGBA::rgb(128, 128, 128);

struct MyRoguelike {
    con: Console,
    player_pos: (i32, i32),
    mouse_pos: (f32, f32),
    screenshot_idx: usize,
}

impl MyRoguelike {
    fn new() -> Box<Self> {
        let con = Console::new(CONSOLE_WIDTH, CONSOLE_HEIGHT, FONT);

        Box::new(MyRoguelike {
            con,
            player_pos: ((CONSOLE_WIDTH / 2) as i32, (CONSOLE_HEIGHT / 2) as i32),
            mouse_pos: (0.0, 0.0),
            screenshot_idx: 0,
        })
    }
}

impl Screen for MyRoguelike {
    fn input(&mut self, api: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::KeyDown(ev) => match ev.key_code {
                VirtualKeyCode::Left => {
                    self.player_pos.0 = (self.player_pos.0 - 1).max(1);
                }
                VirtualKeyCode::Right => {
                    self.player_pos.0 = (self.player_pos.0 + 1).min(CONSOLE_WIDTH as i32 - 2);
                }
                VirtualKeyCode::Up => {
                    self.player_pos.1 = (self.player_pos.1 - 1).max(1);
                }
                VirtualKeyCode::Down => {
                    self.player_pos.1 = (self.player_pos.1 + 1).min(CONSOLE_HEIGHT as i32 - 2);
                }
                _ => {}
            },
            AppEvent::MouseDown(info) => {
                log(&format!("click - {:?}", info.pos));
            }
            _ => {}
        }

        self.mouse_pos = api.input().mouse_pct();

        // capture the screen
        if api.input().key(VirtualKeyCode::LControl) && api.input().key_pressed(VirtualKeyCode::S) {
            self.screenshot_idx += 1;
            return ScreenResult::Capture(format!("screenshot_{:03}.png", self.screenshot_idx));
        }

        ScreenResult::Continue
    }

    fn render(&mut self, api: &mut AppContext) {
        let con = &mut self.con;
        let mouse_pos = con.mouse_pos(self.mouse_pos);

        let buffer = con.buffer_mut();

        draw::frame(buffer)
            .fg(GRAY)
            .bg(BLACK)
            .fill(Some('.' as u32), Some(GRAY), Some(BLACK))
            .draw(0, 0, CONSOLE_WIDTH, CONSOLE_HEIGHT);

        draw::rect(buffer)
            .glyph('&' as u32)
            .fg(RGBA::rgba(255, 64, 64, 255))
            .bg(RGBA::rgba(128, 32, 32, 255))
            .draw(10, 10, 5, 5);

        buffer.glyph(self.player_pos.0, self.player_pos.1, '@' as u32);
        buffer.fore(self.player_pos.0, self.player_pos.1, WHITE);

        draw::colored(buffer).align(TextAlign::Center).print(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT - 1) as i32,
            "#[red]arrows#[white] : move - #[red]CTRL-S#[white] : save screenshot",
        );

        if let Some(mouse_pos) = mouse_pos {
            draw::colored(buffer).align(TextAlign::Center).print(
                (CONSOLE_WIDTH / 2) as i32,
                (CONSOLE_HEIGHT - 3) as i32,
                &format!(
                    "#[white]Mouse coordinates: #[red]{}, {}",
                    mouse_pos.0, mouse_pos.1
                ),
            );
            buffer.back(mouse_pos.0 as i32, mouse_pos.1 as i32, WHITE);
        }
        draw::colored(buffer).align(TextAlign::Left).print(
            5,
            5,
            "#[blue]This blue text contains a #[red]red#[] word",
        );

        con.render(api);
    }
}

fn main() {
    let app = AppBuilder::new(CONSOLE_WIDTH * 8, CONSOLE_HEIGHT * 8)
        .title("My Roguelike")
        .font(FONT)
        .vsync(false)
        .build();

    app.run_screen(MyRoguelike::new());
}
