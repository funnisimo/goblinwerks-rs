use gw_app::color::RGBA;
use gw_app::*;

const FONT: &str = "resources/terminal_8x8.png";

struct TextScreen {
    con: Panel,
}

impl TextScreen {
    fn new() -> Box<Self> {
        let con = Panel::new(80, 50, FONT);
        Box::new(TextScreen { con })
    }
}

impl Screen for TextScreen {
    fn setup(&mut self, _app: &mut Ecs) {
        let buffer = self.con.buffer_mut();
        buffer.clear(true, false, false);

        // PLAIN (NO BG, NO WIDTH)
        let y = 2;

        let mut draw = draw::plain(buffer).fg(RGBA::rgb(192, 32, 32));
        draw.print(2, y, "No bg, no width");
        draw.print_lines(
            2,
            y + 2,
            "print_lines can\nhandle newlines, but\nwill not word wrap.",
        );
        draw.wrap(26, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");

        let y = 10;

        let mut draw = draw::plain(buffer)
            .fg(RGBA::rgb(255, 0, 255))
            .bg(RGBA::rgb(0, 64, 255));
        draw.print(2, y, "bg, no width");
        draw.print_lines(
            2,
            y + 2,
            "print_lines can\nhandle newlines, but\nwill not word wrap.",
        );
        draw.wrap(26, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");

        let y = 18;

        draw::plain(buffer).print(26, y, "Align::Left");
        draw::plain(buffer)
            .align(TextAlign::Center)
            .print(52, y, "Align::Center");
        draw::plain(buffer)
            .align(TextAlign::Right)
            .print(78, y, "Align::Right");

        // width, no bg
        let y = 20;

        let mut draw = draw::plain(buffer).fg(RGBA::rgb(64, 128, 32)).width(15);
        draw.print(2, y, "no bg, width=15");
        draw.print_lines(
            2,
            y + 2,
            "print_lines can\nhandle newlines, but\nwill not word wrap.",
        );
        draw.wrap(26, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");
        draw = draw.align(TextAlign::Center);
        draw.wrap(52, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");
        draw = draw.align(TextAlign::Right);
        draw.wrap(78, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");

        // width, no bg
        let y = 33;

        let mut draw = draw::plain(buffer)
            .fg(RGBA::rgb(255, 255, 255))
            .bg(RGBA::rgb(0, 64, 255))
            .width(15);
        draw.print(2, y, "width=15 and bg=blueish");
        draw.print_lines(
            2,
            y + 2,
            "print_lines can\nhandle newlines, but\nwill not word wrap.",
        );
        draw.wrap(26, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");
        draw = draw.align(TextAlign::Center);
        draw.wrap(52, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");
        draw = draw.align(TextAlign::Right);
        draw.wrap(78, y,  "Inside a call to wrap, you can place a long text and it will automatically be wrapped at the width you specify.  Or at the end of the buffer.");
    }

    fn render(&mut self, app: &mut Ecs) {
        self.con.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Basic Example")
        .font_with_transform(FONT, &codepage437::to_glyph, &codepage437::from_glyph)
        .build();
    app.run(TextScreen::new());
}
