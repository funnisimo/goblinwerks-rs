use gw_app::color::RGBA;
use gw_app::*;

const FONT: &str = "resources/terminal_8x8.png";

const _WHITE: RGBA = RGBA::rgba(255, 255, 255, 255);

struct MyRoguelike {
    txt: String,
    cursor: usize,
    con: Console,
}

impl MyRoguelike {
    fn new() -> Box<dyn Screen> {
        let con = Console::new(80, 50, FONT);

        Box::new(MyRoguelike {
            con,
            txt: String::new(),
            cursor: 0,
        })
    }
}

impl Screen for MyRoguelike {
    fn input(&mut self, _app: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::CharEvent(ch) => {
                let glyph = codepage437::to_glyph(*ch);
                if glyph > 0 {
                    self.txt.push_str(&ch.to_string());
                } else {
                    println!("Unprintable character - {} ({})", ch, *ch as u32);
                }
            }
            AppEvent::KeyDown(KeyEvent {
                key_code: VirtualKeyCode::Back,
                ..
            }) => {
                self.txt.pop();
            }
            _ => {}
        }

        // // input.text returns the characters typed by the player since last update
        // for ch in input.text() {
        //     if ch.len() == 1 {
        //         self.txt.push_str(ch);
        //     } else if ch == "Backspace" {
        //         // convoluted way to remove the last character of the string
        //         // in a way that also works with utf-8 graphemes
        //         // where one character != one byte
        //         let mut graphemes = self.txt.graphemes(true).rev();
        //         graphemes.next();
        //         self.txt = graphemes.rev().collect();
        //     } else if ch == "Tab" {
        //         self.txt.push_str("   ");
        //     }
        // }
        self.cursor += 1;
        ScreenResult::Continue
    }

    fn update(&mut self, _app: &mut AppContext, _frame_time_ms: f64) -> ScreenResult {
        self.cursor += 1;
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut AppContext) {
        let buffer = self.con.buffer_mut();
        buffer.fill(Some(' ' as u32), None, None);

        draw::plain(buffer).print(
            5,
            5,
            &format!(
                "Type some text : {}{}",
                self.txt,
                // blinking cursor
                if self.cursor % 25 < 12 { '_' } else { ' ' }
            ),
        );

        self.con.render(app)
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Input Example")
        .font(FONT)
        .build();
    app.run_screen(MyRoguelike::new());
}
