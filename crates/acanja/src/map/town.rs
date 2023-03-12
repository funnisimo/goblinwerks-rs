use super::prefab::*;
use gw_app::log;
use gw_util::point::Point;
use gw_util::rng::SliceRandom;
use gw_world::map::{Builder, Map, PortalFlags, PortalInfo};
use gw_world::tile::Tiles;

pub fn build_town_map<'t>(
    tiles: &'t Tiles,
    prefabs: &Prefabs,
    width: u32,
    height: u32,
    id: &str,
) -> Map {
    let mut builder = Builder::new(tiles, width, height);

    loop {
        builder.fill("LAKE");

        log("Adding landing...");
        let mut x = match add_landing(&mut builder) {
            None => break,
            Some(x) => x,
        };

        // Add way out...
        let y = height as i32 / 2 - 1;
        builder.place_tile(0, y, "EXIT_TOWN_LEFT");
        builder.place_tile(0, y + 1, "EXIT_TOWN_LEFT");
        builder.place_tile(0, y + 2, "EXIT_TOWN_LEFT");

        builder.set_location("START", Point::new(1, y + 1));
        let mut portal = PortalInfo::new("WORLD", id);
        portal.set_flavor("a way back to the world");
        portal.set_flags(PortalFlags::ON_CLIMB);
        builder
            .set_portal(Point::new(0, y), portal.clone())
            .set_portal(Point::new(0, y + 1), portal.clone())
            .set_portal(Point::new(0, y + 2), portal);

        log("Adding bridge...");
        x = match add_bridge(&mut builder, x) {
            None => break,
            Some(x) => x,
        };

        log("Adding island...");
        match add_island(&mut builder, x) {
            None => break,
            Some(_) => {}
        }

        log("Adding stores...");
        match add_stores(&mut builder, prefabs) {
            None => break,
            Some(_) => {}
        }

        break;
    }

    builder.build()
}

fn add_island(builder: &mut Builder, x: i32) -> Option<()> {
    let size = builder.size();

    let factor = (0.8, 0.8);

    let build_size = (
        ((size.0 as i32 - x) as f32 * factor.0).trunc() as u32,
        (size.1 as f32 * factor.1).trunc() as u32,
    );

    let x0 = x;
    let y0 = (size.1 - build_size.1) as i32 / 2;

    for dy in 0..build_size.1 as i32 {
        for dx in 0..build_size.0 as i32 {
            builder.set_tile(x0 + dx, y0 + dy, "ISLAND");
        }
    }

    // TODO - Round corners?

    Some(())
}

fn add_landing(builder: &mut Builder) -> Option<i32> {
    // fawn style
    let size = builder.size();

    let pct = builder.rng_mut().frange(0.5, 2.0);
    let mut height = ((size.1 as f32) * pct) as i32; // first line is full height?

    let mut x = 0;
    while height >= 7 {
        log(format!("landing size = {}", height));

        let y0 = (size.1 as i32 - height) / 2;

        for dy in 0..height {
            builder.set_tile(x, y0 + dy, "LANDING");
        }

        x += 1;

        let pct = builder.rng_mut().frange(0.5, 0.8);
        height = (height as f32 * pct) as i32;
    }

    if x == 0 {
        return None;
    }
    Some(x)
}

fn add_bridge(builder: &mut Builder, x: i32) -> Option<i32> {
    let y0 = builder.size().1 as i32 / 2 - 1;

    for dx in 0..9 {
        builder.set_tile(x + dx, y0, "BRIDGE");
        builder.set_tile(x + dx, y0 + 1, "BRIDGE");
        builder.set_tile(x + dx, y0 + 2, "BRIDGE");
    }

    Some(x + 9)
}

fn add_stores(builder: &mut Builder, prefabs: &Prefabs) -> Option<u32> {
    let mut count = 0;

    let store_prefabs: Vec<(&String, &Prefab)> = prefabs
        .iter()
        .filter(|(_, prefab)| prefab.has_tag("STORE"))
        .collect();

    log(format!("- Store count - {}", store_prefabs.len()));

    loop {
        if let Some((key, prefab)) = store_prefabs.choose(builder.rng_mut()) {
            match add_store(builder, key, prefab) {
                Ok(_) => {}
                Err(_) => return None,
            }
        } else {
            return None;
        }

        if count == 4 {
            break;
        }
        count += 1;
    }

    if count == 0 {
        return None;
    }
    Some(count)
}

fn add_store(builder: &mut Builder, key: &str, prefab: &Prefab) -> Result<(), ()> {
    let mut tries_left = 100;

    let (width, height) = builder.size();

    while tries_left > 0 {
        let x = builder.rng_mut().rand(width);
        let y = builder.rng_mut().rand(height);

        log(format!("- Trying prefab {} @ {},{}", key, x, y));
        if prefab.fits_at(x as i32, y as i32, builder) {
            log(format!("- FITS"));
            prefab.build_at(x as i32, y as i32, builder);
            return Ok(());
        }

        tries_left -= 1;
    }

    Err(())
}
