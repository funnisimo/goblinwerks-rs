use crate::fl;
use bitflags::bitflags;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Default)]
    pub struct MapFlags: u32 {

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

        const ALL_VISIBLE = fl!(0);
        const ALL_REVEALED = fl!(1);

        // TODO
        // ENTITY_CHANGED
        // TILE_CHANGED

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

    }
}

impl MapFlags {
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

impl fmt::Display for MapFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for MapFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = MapFlags::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                "ALL_VISIBLE" => result |= MapFlags::ALL_VISIBLE,
                "ALL_REVEALED" => result |= MapFlags::ALL_REVEALED,

                "" => {}
                _ => return Err(format!("Unknown TileFlag1: {}", s)),
            }
        }
        Ok(result)
    }
}

impl From<&str> for MapFlags {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(err) => panic!("{}", err),
        }
    }
}
