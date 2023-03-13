use bitflags::bitflags;
use gw_util::fl;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Default)]
    pub struct CellFlags: u32 {

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

        const IS_CURSOR = fl!(0);
        const IS_HIGHLIGHTED = fl!(1);

        const NEEDS_DRAW = fl!(2);
        const DRAWN_THIS_FRAME = fl!(3);
        const NEEDS_SNAPSHOT = fl!(4);
        const TILE_CHANGED = fl!(5);   // NO STABLE SNAPSHOT
        const ENTITY_CHANGED = fl!(6);    // NEEDS REDRAW

        const IS_PORTAL = fl!(7);

        // TODO
        // BLOCKED_MOVE
        // BLOCKED_VISION

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

    }
}

impl CellFlags {
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

impl fmt::Display for CellFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for CellFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = CellFlags::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                "IS_CURSOR" => result |= CellFlags::IS_CURSOR,
                "IS_HIGHLIGHTED" => result |= CellFlags::IS_HIGHLIGHTED,

                "NEEDS_DRAW" => result |= CellFlags::NEEDS_DRAW,
                "DRAWN_THIS_FRAME" => result |= CellFlags::DRAWN_THIS_FRAME,
                "NEEDS_SNAPSHOT" => result |= CellFlags::NEEDS_SNAPSHOT,
                "TILE_CHANGED" => result |= CellFlags::TILE_CHANGED,
                "ENTITY_CHANGED" => result |= CellFlags::ENTITY_CHANGED,

                "IS_PORTAL" => result |= CellFlags::IS_PORTAL,

                "" => {}
                _ => return Err(format!("Unknown TileFlag1: {}", s)),
            }
        }
        Ok(result)
    }
}

impl From<&str> for CellFlags {
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
        assert_eq!(CellFlags::IS_CURSOR, "IS_CURSOR".into());
        assert_eq!(CellFlags::IS_HIGHLIGHTED, "is_highlighted".into());
        assert_eq!(CellFlags::TILE_CHANGED, "Tile_Changed".parse().unwrap());

        assert_eq!(
            CellFlags::TILE_CHANGED | CellFlags::NEEDS_DRAW,
            "tile_changed | needs_draw".parse().unwrap()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(format!("{:?}", CellFlags::IS_CURSOR), "IS_CURSOR");

        let flags = CellFlags::NEEDS_DRAW | CellFlags::NEEDS_SNAPSHOT;
        assert_eq!(format!("{:?}", flags), "NEEDS_DRAW | NEEDS_SNAPSHOT");
    }
}
