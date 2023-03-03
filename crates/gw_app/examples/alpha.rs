use gw_app::color::RGBA;
use gw_app::*;

/*
* This example show how to use root console transparency to display the current frame on top of the previous one.
* Each frame, we only draw a circle of blue dots. We fill the rest of the console with a transparent black color.
* When rendering the console, we still see the previous frame but it slowly fades to black as layers of transparent black are added every new frame.
* Note that currently, there's no way to clear the framebuffer. If you don't want to see the previous frame, all the console cells must be opaque (alpha = 255).
* You can still use transparent colors on offscreen console that you blit on the opaque root console. Simply fill the root console with opaque black (0,0,0,255).
*/

const FONT: &str = "resources/terminal_8x8.png";

struct AlphaTest {
    con: Panel,
    cx: f32,
    cy: f32,
    radius: f32,
    angle: f32,
}

impl AlphaTest {
    fn new() -> Box<Self> {
        let con = Panel::new(80, 50, FONT);

        Box::new(AlphaTest {
            con,
            cx: 0.0,
            cy: 0.0,
            radius: 10.0,
            angle: 0.0,
        })
    }
}

impl Screen for AlphaTest {
    fn update(&mut self, _app: &mut Ecs) -> ScreenResult {
        // update the circle radius and center position
        self.angle += 0.6;
        self.radius = 10.0 + 3.0 * (self.angle / 10.0).sin();
        let cs = (self.angle / 20.0).cos();
        let sn = (self.angle / 15.0).sin();
        self.cx = (self.con.width() / 2) as f32 + cs * 15.0;
        self.cy = (self.con.height() / 2) as f32 + sn * 15.0;
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut Ecs) {
        let buffer = self.con.buffer_mut();

        // reduce the alpha of each cell until they are transparent.
        buffer.update(|_, _, _, _, bg| {
            bg.3 = bg.3.saturating_sub(10);
        });

        // here we render current frame (only a circle of blue dots)
        for r in 0..10 {
            let angle = self.angle + r as f32 * std::f32::consts::PI * 2.0 / 10.0;
            let cs = angle.cos();
            let sn = angle.sin();
            let x = self.cx + self.radius * cs;
            let y = self.cy + self.radius * sn;
            buffer.back(x as i32, y as i32, RGBA::rgba(0, 0, 255, 255));
        }
        self.con.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Alpha Test")
        .font_with_transform(FONT, &codepage437::to_glyph, &codepage437::from_glyph)
        .build();
    app.run(AlphaTest::new());
}
