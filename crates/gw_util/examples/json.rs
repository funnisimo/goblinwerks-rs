use gw_util::json::parse_file;

fn main() {
    let doc = parse_file("./assets/world.jsonc").expect("Failed to parse json");

    let maps_all: Vec<String> = doc
        .get_path("maps.all")
        .unwrap()
        .as_list()
        .unwrap()
        .iter()
        .map(|v| v.to_string())
        .collect();
    println!("maps.all = {:?}", maps_all);

    let world = doc.get_path("maps.WORLD").unwrap();
    let width: u32 = world.get_value("width").unwrap().as_int().unwrap() as u32;
    let height: u32 = world.get_value("height").unwrap().as_int().unwrap() as u32;
    let steps: Vec<String> = world
        .get_value("steps")
        .unwrap()
        .as_list()
        .unwrap()
        .iter()
        .map(|v| v.to_string())
        .collect();
    println!(
        "maps.WORLD = {{ height = {}, width = {}, steps = {:?} }}",
        height, width, steps
    );
}
