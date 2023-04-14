use gw_ecs::{Comp, CompMut, Ecs, Global, GlobalMut};
use gw_util::rng::RandomNumberGenerator;

struct Time {
    t: u32,
}

impl Time {
    pub fn new() -> Self {
        Time { t: 0 }
    }

    pub fn tick(&mut self) -> u32 {
        self.t += 1;
        self.t
    }
}

struct Stock {
    name: String,
    price: f32,
}

impl Stock {
    pub fn new(name: &str, price: f32) -> Self {
        Stock {
            name: name.to_string(),
            price,
        }
    }
}

fn setup(ecs: &mut Ecs) {
    // need to register components
    ecs.register_component::<Stock>();

    ecs.insert_global(Time::new());
    ecs.insert_global(RandomNumberGenerator::new());

    // spawn some entities with these components
    ecs.spawn((Stock::new("IBM", 100.0),));
    ecs.spawn((Stock::new("APPLE", 100.0),));
    ecs.spawn((Stock::new("GM", 100.0),));
    ecs.spawn((Stock::new("DISNEY", 100.0),));
    ecs.spawn((Stock::new("NRG", 100.0),));
}

fn update(ecs: &mut Ecs) {
    let (mut rng, mut time, mut stocks) = ecs.fetch_mut::<(
        GlobalMut<RandomNumberGenerator>,
        GlobalMut<Time>,
        CompMut<Stock>,
    )>();

    time.tick();

    for stock in stocks.iter_mut() {
        stock.price += rng.frange(-2.0, 2.0);
    }
}

fn render(ecs: &Ecs) {
    let (time, stocks) = ecs.fetch::<(Global<Time>, Comp<Stock>)>();

    println!("=====================");
    println!("TIME = {}", time.t);
    for stock in stocks.iter() {
        println!("  {:10} {:6.2}", stock.name, stock.price);
    }
}

fn main() {
    let mut ecs = Ecs::new();
    setup(&mut ecs);

    for _ in 0..5 {
        update(&mut ecs);
        render(&ecs);
    }
}
