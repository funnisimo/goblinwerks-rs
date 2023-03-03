use gw_app::color::RGBA;
use gw_app::messages::Messages;
use gw_app::*;

const _WHITE: RGBA = RGBA::rgb(255, 255, 255);
const BLACK: RGBA = RGBA::rgb(0, 0, 0);
const GRAY: RGBA = RGBA::rgb(128, 128, 128);
const RED: RGBA = RGBA::rgb(255, 0, 0);
const BLUE: RGBA = RGBA::rgb(0, 0, 255);

const FONT: &str = "assets/terminal_8x8.png";

/*
* This example show how you can intercept the user trying to close the game window.
* All you have to do is to add the `intercept_close_request: true` option when creating the application
* and calling the `InputApi.close_requested()` to detect the event.
* This only works on native target right now.
*/

struct MainScreen {
    con: Panel,
}

impl MainScreen {
    fn new() -> Box<Self> {
        let con = Panel::new(50, 30, FONT);
        Box::new(MainScreen { con })
    }
}

impl Screen for MainScreen {
    fn message(&mut self, _app: &mut Ecs, _id: String, value: Option<Value>) -> ScreenResult {
        match value {
            Some(Value::Boolean(true)) => {
                log("You chose to quit.");
                ScreenResult::Quit
            }
            _ => ScreenResult::Continue,
        }
    }

    fn update(&mut self, ecs: &mut Ecs) -> ScreenResult {
        let input = ecs.resources.get::<AppInput>().unwrap();

        if input.key(VirtualKeyCode::Escape) || input.close_requested() {
            ScreenResult::Push(Popup::new())
        } else {
            ScreenResult::Continue
        }
    }

    fn render(&mut self, app: &mut Ecs) {
        let buffer = self.con.buffer_mut();
        buffer.fill(Some('.' as u32), Some(GRAY), Some(BLACK));

        draw::frame(buffer)
            .border(BorderType::Double)
            .fg(RED)
            .bg(GRAY)
            .fill(Some(' ' as u32), None, Some(BLACK))
            .draw(10, 10, 30, 10);

        draw::plain(buffer).align(TextAlign::Center).print_lines(
            25,
            14,
            "Press Escape\nto quit the app",
        );

        self.con.render(app);
    }
}

struct Popup {
    con: Panel,
}

impl Popup {
    fn new() -> Box<Self> {
        let con = Panel::new(24, 20, FONT).with_extents(0.25, 0.25, 0.5, 0.75);
        Box::new(Popup { con })
    }
}

impl Screen for Popup {
    fn is_full_screen(&self) -> bool {
        false
    }

    fn input(&mut self, app: &mut Ecs, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Y => {
                    app.resources
                        .get_mut::<Messages>()
                        .unwrap()
                        .push("QUIT", Some(true.into()));
                    ScreenResult::Pop
                }
                VirtualKeyCode::N => {
                    app.resources
                        .get_mut::<Messages>()
                        .unwrap()
                        .push("QUIT", Some(false.into()));
                    ScreenResult::Pop
                }
                _ => ScreenResult::Continue,
            },
            AppEvent::CloseRequested => ScreenResult::Quit,
            _ => ScreenResult::Continue,
        }
    }

    fn render(&mut self, app: &mut Ecs) {
        let buf = self.con.buffer_mut();

        draw::frame(buf)
            .border(BorderType::Double)
            .fg(BLUE)
            .bg(GRAY)
            .fill(Some(' ' as u32), None, Some(BLACK))
            .draw(0, 0, 24, 20);

        draw::plain(buf)
            .width(18)
            .print_lines(5, 4, "Do you really\nwant to quit?");

        let mut p = draw::plain(buf).align(TextAlign::Left);
        p.print(5, 12, "[N]o");
        p.print(5, 14, "[Y]es");

        self.con.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Close App Example")
        .font_with_transform(FONT, &codepage437::to_glyph, &codepage437::from_glyph)
        .intercept_close_request(true)
        .build();

    app.run(MainScreen::new());
}
