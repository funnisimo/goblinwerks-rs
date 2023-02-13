use conapp::*;

const CONSOLE_WIDTH: u32 = 40;
const CONSOLE_HEIGHT: u32 = 25;

const FONTS: [&str; 19] = [
    "resources/terminal_8x8.png",
    "resources/terminal_colored_8x8.png",
    "resources/terminal_8x12.png",
    "resources/terminal_10x16.png",
    "resources/terminal_12x12.png",
    "resources/SmoothWalls_9x9.png",
    "resources/Aesomatica_16x16.png",
    "resources/Bisasam_20x20.png",
    "resources/Buddy--graphical_10x10.png",
    "resources/Cheepicus_8x8.png",
    "resources/Cheepicus_15x15.png",
    "resources/Cheepicus_16x16.png",
    "resources/Herrbdog_12x12.png",
    "resources/Kein_5x5.png",
    "resources/Mkv_curses_6x6.png",
    "resources/Runeset_24x24.png",
    "resources/Teeto_K_18x18.png",
    "resources/Terbert_7x7.png",
    "resources/Yayo_tunur_13x13.png",
];

struct MyRoguelike {
    con: Console,
    cur_font: usize,
    cur_font_name: String,
}

impl MyRoguelike {
    fn new() -> Box<Self> {
        let con = Console::new(CONSOLE_WIDTH, CONSOLE_HEIGHT, FONTS[0]);

        Box::new(MyRoguelike {
            con,
            cur_font: 0,
            cur_font_name: FONTS[0].to_owned(),
        })
    }
}

impl Screen for MyRoguelike {
    fn input(&mut self, app: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        let mut font_path = None;
        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Down => {
                    self.cur_font = (self.cur_font + 1) % FONTS.len();
                    font_path = Some(FONTS[self.cur_font]);
                }
                VirtualKeyCode::Up => {
                    self.cur_font = (self.cur_font + FONTS.len() - 1) % FONTS.len();
                    font_path = Some(FONTS[self.cur_font]);
                }
                _ => {}
            },
            _ => {}
        }

        if let Some(font_path) = font_path {
            self.cur_font_name = font_path.to_owned();

            match app.get_font(font_path) {
                None => {}
                Some(font) => self.con.set_font(font),
            }
        }

        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut AppContext) {
        let buffer = self.con.buffer_mut();
        draw::rect(buffer)
            .glyph('.' as u32)
            .fg((128, 128, 128, 255).into())
            .draw(0, 0, CONSOLE_WIDTH, CONSOLE_HEIGHT);

        buffer.area(
            10,
            10,
            5,
            5,
            Some('&' as u32),
            Some((255, 64, 64, 255).into()),
            Some((128, 32, 32, 255).into()),
        );
        buffer.glyph(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2 - 10) as i32,
            '@' as u32,
        );
        buffer.fore(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT / 2 - 10) as i32,
            (255, 255, 255, 255).into(),
        );

        draw::rect(buffer)
            .bg((255, 255, 255, 255).into())
            .fg((0, 0, 0, 255).into())
            .glyph(' ' as u32)
            .draw(
                (CONSOLE_WIDTH / 2 - 20) as i32,
                (CONSOLE_HEIGHT / 2 - 2) as i32,
                40,
                7,
            );

        draw::plain(buffer)
            .align(TextAlign::Center)
            .fg((64, 64, 200).into())
            .print(
                (CONSOLE_WIDTH / 2) as i32,
                (CONSOLE_HEIGHT / 2) as i32,
                &self.cur_font_name,
            );

        draw::plain(buffer)
            .align(TextAlign::Center)
            .fg((255, 78, 32, 255).into())
            .print(
                (CONSOLE_WIDTH / 2) as i32,
                (CONSOLE_HEIGHT / 2) as i32 + 2,
                "Up/Down to change font",
            );

        self.con.render(app)
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Input Example")
        .fonts(&FONTS)
        .build();

    app.run_screen(MyRoguelike::new());
}
