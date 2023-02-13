use conapp::color::RGBA;
use conapp::*;

const FONT: &str = "resources/terminal_8x8.png";

struct MainScreen {
    left: Console,
    right: Console,
    pos: (i32, i32),
}

impl MainScreen {
    fn new() -> Box<Self> {
        let left = Console::new(60, 50, FONT).with_extents(0.0, 0.0, 0.75, 1.0);
        let right = Console::new(20, 30, FONT).with_extents(0.75, 0.0, 1.0, 1.0);
        let pos = (30, 25);
        Box::new(MainScreen { left, right, pos })
    }
}

impl MainScreen {
    fn render_left(&mut self, app: &mut AppContext) {
        let buffer = self.left.buffer_mut();
        buffer.fill(
            Some('.' as u32),
            Some(RGBA::rgb(32, 32, 32)),
            Some(RGBA::rgb(0, 0, 0)),
        );
        buffer.draw(
            self.pos.0,
            self.pos.1,
            '@' as Glyph,
            RGBA::rgb(255, 255, 0),
            RGBA::rgb(0, 0, 0),
        );

        draw::plain(buffer)
            .fg(RGBA::rgb(255, 0, 255))
            .print(5, 10, "Hello World");

        draw::colored(buffer).fg(RGBA::rgb(255, 255, 255)).print(
            5,
            12,
            "Use the #[#F00]arrow keys#[] to move the '#[#FF0]@#[]' symbol around.",
        );

        draw::colored(buffer).fg(RGBA::rgb(255, 255, 255)).print(
            5,
            14,
            "Click the #[#F00]left mouse button#[] to #[#0FF]Quit#[] the app.",
        );

        self.left.render(app);
    }

    fn render_right(&mut self, app: &mut AppContext) {
        let buffer = self.right.buffer_mut();

        buffer.fill(Some(0), None, Some(RGBA::rgb(32, 64, 32)));

        draw::plain(buffer)
            .fg(RGBA::rgb(192, 0, 0))
            .print(2, 2, "Hello World");

        draw::plain(buffer)
            .fg(RGBA::rgb(0, 192, 192))
            .width(18)
            .print_lines(2, 5, "This is a\ndifferent\nconsole.");

        self.right.render(app);
    }
}

impl Screen for MainScreen {
    fn input(&mut self, _app: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::KeyDown(key_down) => match key_down.key_code {
                VirtualKeyCode::Left => self.pos.0 = (self.pos.0 - 1).max(0),
                VirtualKeyCode::Right => {
                    self.pos.0 = (self.pos.0 + 1).min(self.left.width() as i32 - 1)
                }
                VirtualKeyCode::Up => self.pos.1 = (self.pos.1 - 1).max(0),
                VirtualKeyCode::Down => {
                    self.pos.1 = (self.pos.1 + 1).min(self.left.height() as i32 - 1)
                }
                _ => return ScreenResult::Quit,
            },
            AppEvent::MouseDown(_) => return ScreenResult::Quit,
            _ => {}
        }
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut AppContext) {
        self.render_left(app);
        self.render_right(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Basic Example")
        .font(FONT)
        .build();
    app.run_screen(MainScreen::new());
}
