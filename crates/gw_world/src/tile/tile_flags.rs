use crate::fl;
use bitflags::bitflags;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Default)]
    pub struct TileFlags: u32 {

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!
        const SECRET = fl!(0);
        const NOTICE = fl!(1);
        const REMEMBER = fl!(2);    // bright memory?
        const GLYPH = fl!(3);   // is there a different way to do this - using BLOCKS_MONS + ENTER?

        const SPAWN_MONS = fl!(10);
        const SPAWN_ITEM = fl!(11);

        const CLIMB_PORTAL = fl!(20);
        const DESCEND_PORTAL = fl!(21);

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

    }
}

impl TileFlags {
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

impl fmt::Display for TileFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TileFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = TileFlags::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                "SECRET" => result |= TileFlags::SECRET,
                "NOTICE" => result |= TileFlags::NOTICE,
                "REMEMBER" => result |= TileFlags::REMEMBER, // bright memory?
                "GLYPH" => result |= TileFlags::GLYPH, // is there a different way to do this - using BLOCKS_MONS + ENTER?

                "SPAWN_MONS" => result |= TileFlags::SPAWN_MONS,
                "SPAWN_ITEM" => result |= TileFlags::SPAWN_ITEM,

                "CLIMB_PORTAL" => result |= TileFlags::CLIMB_PORTAL,
                "DESCEND_PORTAL" => result |= TileFlags::DESCEND_PORTAL,

                "" => {}
                _ => return Err(format!("Unknown TileFlag: {}", s)),
            }
        }
        Ok(result)
    }
}

impl From<&str> for TileFlags {
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
        assert_eq!(TileFlags::CLIMB_PORTAL, "CLIMB_PORTAL".into());
        assert_eq!(TileFlags::REMEMBER, "remember".into());
        assert_eq!(TileFlags::SECRET, "Secret".parse().unwrap());

        assert_eq!(
            TileFlags::NOTICE | TileFlags::REMEMBER,
            "notice | remember".parse().unwrap()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(format!("{:?}", TileFlags::DESCEND_PORTAL), "DESCEND_PORTAL");

        let flags = TileFlags::REMEMBER | TileFlags::SECRET;
        assert_eq!(format!("{:?}", flags), "SECRET | REMEMBER");
    }
}
