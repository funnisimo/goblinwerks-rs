use conapp::*;

struct MainScreen {
    con: Console,
}

impl MainScreen {
    pub fn new() -> Box<Self> {
        let con = Console::new(80, 50, "DEFAULT");
        Box::new(MainScreen { con })
    }
}

impl Screen for MainScreen {
    fn input(&mut self, _app: &mut AppContext, ev: &AppEvent) -> ScreenResult {
        match ev {
            AppEvent::KeyDown(_) => ScreenResult::Quit,
            AppEvent::MouseDown(_) => ScreenResult::Quit,
            _ => ScreenResult::Continue,
        }
    }

    fn render(&mut self, app: &mut AppContext) {
        let buffer = self.con.buffer_mut();

        buffer.fill(Some('.' as u32), Some(WHITE), Some(BLACK));

        draw::plain(buffer)
            .fg(RGBA::rgb(255, 0, 255))
            .print(10, 10, "Hello World");

        self.con.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768).title("Minimal Example").build();
    app.run_screen(MainScreen::new());
}
