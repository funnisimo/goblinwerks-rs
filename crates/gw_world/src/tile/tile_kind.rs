use crate::fl;
use bitflags::bitflags;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Default)]
    pub struct TileKind: u32 {
    const FLOOR = fl!(0);
    const HALL = fl!(1);
    const FIXTURE = fl!(2);
    const DOOR = fl!(3);
    const WALL = fl!(4);
    const IMPREGNABLE = fl!(5);
    const LAKE = fl!(6);
    const RIVER = fl!(7);
    const CHASM = fl!(8);
    const BRIDGE = fl!(9);
    const STREAMER = fl!(10);
    const TRAP = fl!(11);
    const STAIRS = fl!(12); // do we need up and down?
    const PORTAL = fl!(13); // how to do this?
    }
}

impl TileKind {
    pub fn apply(&mut self, flags: &str) {
        for val in flags.split("|") {
            if val.trim().starts_with("!") {
                match Self::from_str(&val[1..]) {
                    Ok(flag) => self.remove(flag),
                    Err(_) => {}
                }
            } else {
                match Self::from_str(val) {
                    Ok(flag) => self.insert(flag),
                    Err(_) => {}
                }
            }
        }
    }
}

impl fmt::Display for TileKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TileKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = TileKind::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                "FLOOR" => result |= TileKind::FLOOR,
                "HALL" => result |= TileKind::HALL,
                "FIXTURE" => result |= TileKind::FIXTURE,
                "DOOR" => result |= TileKind::DOOR,
                "WALL" => result |= TileKind::WALL,
                "IMPREGNABLE" => result |= TileKind::IMPREGNABLE,
                "LAKE" => result |= TileKind::LAKE,
                "RIVER" => result |= TileKind::RIVER,
                "CHASM" => result |= TileKind::CHASM,
                "BRIDGE" => result |= TileKind::BRIDGE,
                "STREAMER" => result |= TileKind::STREAMER,
                "TRAP" => result |= TileKind::TRAP,
                "STAIRS" => result |= TileKind::STAIRS,
                "PORTAL" => result |= TileKind::PORTAL,
                "" => {}
                _ => return Err(format!("Unknown TileFlag1: {}", s)),
            }
        }
        Ok(result)
    }
}

impl From<&str> for TileKind {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(err) => panic!("{}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::prelude::*;

    #[test]
    fn from_str() {
        assert_eq!(TileKind::FLOOR, "FLOOR".into());
        assert_eq!(TileKind::WALL, "wall".into());
        assert_eq!(TileKind::RIVER, "River".parse().unwrap());
    }

    #[test]
    fn to_string() {
        assert_eq!(format!("{:?}", TileKind::LAKE), "LAKE");
    }
}
