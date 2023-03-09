use gw_util::toml::parse_file;

fn main() {
    let doc = parse_file("./assets/tiles.toml").expect("Failed to parse json");

    println!("{:?}", doc);
}
