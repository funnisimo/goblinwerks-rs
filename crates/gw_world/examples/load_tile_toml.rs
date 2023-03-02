use gw_app::color::init_colors;
use gw_util::toml::parse_reader;
use gw_world::tile::{load_tile_data, Tiles};
use std::{fs::File, io::BufReader};

const FILE: &str = "resources/tiles.toml";

fn main() {
    init_colors();

    let file = match File::open(FILE) {
        Err(e) => panic!("{:?}", e),
        Ok(f) => f,
    };
    let mut reader = BufReader::new(file);
    let toml = parse_reader(&mut reader).expect("Failed to parse TOML file.");

    let mut tiles = Tiles::default();
    let count = load_tile_data(&mut tiles, &toml).expect("Failed to load toml file.");

    println!("Loaded {} tiles.", count);
}
