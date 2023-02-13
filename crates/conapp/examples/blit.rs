use conapp::*;

const FONT: &str = "resources/terminal_8x8.png";

struct MyRoguelike {
    con: Console,
    c1_pos: (i32, i32),
    c1_spd: (i32, i32),
    c2_pos: (i32, i32),
    c2_spd: (i32, i32),
    c1: Buffer,
    c2: Buffer,
    alpha: f32,
    step: usize,
}

fn move_con(pos: &mut (i32, i32), spd: &mut (i32, i32), size: (i32, i32)) {
    pos.0 += spd.0;
    if pos.0 == size.0 - 20 || pos.0 == 0 {
        spd.0 = -spd.0;
    }
    pos.1 += spd.1;
    if pos.1 == size.1 - 20 || pos.1 == 0 {
        spd.1 = -spd.1;
    }
}

impl MyRoguelike {
    fn new() -> Box<Self> {
        let con = Console::new(80, 50, FONT);

        let mut c1 = Buffer::new(20, 20);
        let mut c2 = Buffer::new(20, 20);
        for y in 0..20 {
            for x in 0..20 {
                c1.back(x, y, (((x + y * 10) % 255) as u8, 0, 0, 255).into());
                c2.back(
                    x,
                    y,
                    if (x - 10) * (x - 10) + (y - 10) * (y - 10) < 100 {
                        (255, 192, 32, 255 - x as u8 * 10).into()
                    } else {
                        (0, 0, 0, 255).into()
                    },
                );
            }
        }
        draw::plain(&mut c1)
            .align(TextAlign::Center)
            .print(10, 10, "Hello");
        draw::plain(&mut c2)
            .align(TextAlign::Center)
            .print(10, 10, "Circle");

        Box::new(MyRoguelike {
            con,
            c1_pos: (5, 5),
            c2_pos: (15, 20),
            c1_spd: (1, 1),
            c2_spd: (-1, 1),
            c1,
            c2,
            alpha: 1.0,
            step: 0,
        })
    }
}

impl Screen for MyRoguelike {
    fn update(&mut self, _app: &mut AppContext, _ms: f64) -> ScreenResult {
        if self.step == 0 {
            let size = (self.con.width() as i32, self.con.height() as i32);
            move_con(&mut self.c1_pos, &mut self.c1_spd, size);
            move_con(&mut self.c2_pos, &mut self.c2_spd, size);
        }
        self.alpha = (self.alpha + 0.01) % 1.0;
        self.step = (self.step + 1) % 10;
        ScreenResult::Continue
    }

    fn render(&mut self, app: &mut AppContext) {
        let buffer = self.con.buffer_mut();
        let buf_size = buffer.size();

        buffer.fill(Some(' ' as u32), Some((0, 0, 0, 255).into()), None);

        for x in 0..buffer.width() as i32 {
            for y in 0..buffer.height() as i32 {
                buffer.back(
                    x,
                    y,
                    if (x + y) & 1 == 1 {
                        (96, 64, 32, 255).into()
                    } else {
                        (32, 64, 96, 255).into()
                    },
                );
            }
        }
        draw::plain(buffer)
            .align(TextAlign::Center)
            .fg((255, 255, 255).into())
            .print_lines(
                (buf_size.0 / 2) as i32,
                (buf_size.1 / 2) as i32,
                "You can create offscreen consoles\nand blit them on other consoles",
            );

        self.c1.blit(
            self.c1_pos.0,
            self.c1_pos.1,
            buffer,
            self.alpha,
            self.alpha,
            None,
        );
        self.c2.blit(
            self.c2_pos.0,
            self.c2_pos.1,
            buffer,
            1.0 - self.alpha,
            1.0 - self.alpha,
            Some((0, 0, 0, 255).into()),
        );
        self.con.render(app);
    }
}

fn main() {
    let app = AppBuilder::new(1024, 768)
        .title("Blitting Example")
        .font(FONT)
        .build();
    app.run_screen(MyRoguelike::new());
}
