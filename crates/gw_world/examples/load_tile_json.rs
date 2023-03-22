use gw_app::color::init_colors;
use gw_util::json::parse_file;
use gw_world::tile::{load_tile_data, Tiles};

const FILE: &str = "assets/tiles.jsonc";

fn main() {
    init_colors();

    let json = parse_file(FILE).expect("Failed to parse JSON file.");

    let mut tiles = Tiles::default();
    let count = load_tile_data(&mut tiles, json).expect("Failed to load toml file.");

    println!("Loaded {} tiles.", count);
}
