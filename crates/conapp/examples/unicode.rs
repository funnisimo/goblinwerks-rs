use conapp::*;

const FONT: &str = "resources/unicode_16x16.png";

const CONSOLE_WIDTH: u32 = 40;
const CONSOLE_HEIGHT: u32 = 25;

struct MyRoguelike {
    con: Console,
}

impl MyRoguelike {
    fn new() -> Box<Self> {
        let con = Console::new(CONSOLE_WIDTH, CONSOLE_HEIGHT, FONT);
        Box::new(MyRoguelike { con })
    }
}
impl Screen for MyRoguelike {
    fn render(&mut self, app: &mut AppContext) {
        let buffer = self.con.buffer_mut();
        buffer.fill(
            None,
            Some((32, 16, 0, 255).into()),
            Some((255, 240, 224, 255).into()),
        );
        buffer.area(
            5,
            5,
            30,
            15,
            Some(' ' as u32),
            Some((255, 255, 255, 255).into()),
            Some((0, 0, 0, 255).into()),
        );

        let mut printer = draw::plain(buffer).align(TextAlign::Center);

        printer.print(20, 8, "こんにちは!");
        printer.print(20, 10, "真棒!");
        printer.print(20, 12, "классно");
        printer.print(20, 14, "Φοβερός!");
        printer.print(20, 16, "Ça, c'est énorme!");

        self.con.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Unicode Example")
        .font(FONT)
        .build();
    app.run_screen(MyRoguelike::new());
}
