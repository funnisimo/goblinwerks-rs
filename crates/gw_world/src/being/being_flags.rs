/*
// L_DYNAMIC = Fl(0), // for movable things like actors or items
*/

use bitflags::bitflags;
use gw_util::fl;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct BeingFlags: u32 {

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!
        // const HERO = fl!(0);

        const DESTROYED = fl!(1); // has been destroyed
        const SPAWNED = fl!(2);

        // const SECRETLY_PASSABLE = fl!(2); // will become passable if discovered/activated/etc...

        const ALLOWS_MOVE = fl!(3); // can be walked through
        const BLOCKS_VISION = fl!(4); // blocks line of sight
        // const BLOCKS_SURFACE = fl!(6); // grass, blood, etc. cannot exist on this tile
        // const BLOCKS_LIQUID = fl!(8);
        // const BLOCKS_GAS = fl!(7); // blocks the permeation of gas
        // const BLOCKS_ITEMS = fl!(5); // items can't be on this tile
        // const BLOCKS_ACTORS = fl!(11); // actors can't be on this tile
        // const BLOCKS_EFFECTS = fl!(9);
        // const BLOCKS_DIAGONAL = fl!(10); // can't step diagonally around this tile

        const INTERRUPT_WHEN_SEEN = fl!(12);
        const NO_SIDEBAR = fl!(13); // terrain will be listed in the sidebar with a description of the terrain type
        const VISUALLY_DISTINCT = fl!(14); // terrain will be color-adjusted if necessary so the character stands out from the background
        const BRIGHT_MEMORY = fl!(15); // no blue fade when this tile is out of sight
        const INVERT_WHEN_HIGHLIGHTED = fl!(16); // will flip fore and back colors when highlighted with pathing

        const ON_MAP = fl!(17); // entity is currently on a map
        const IN_SIDEBAR = fl!(18); // SHOWN IN SIDEBAR

        const FORMAL_NAME = fl!(20); // "Henry" instead of "the Goblin"
        const ALWAYS_PLURAL = fl!(21); // So that nouns and verbs are tensed correctly (mostly for player)

        const DEFAULT_ACTOR = 0;
        const DEFAULT_ITEM = 0;

        // const BLOCKED_BY_STAIRS = Self::BLOCKS_ITEMS.bits() |
        //     Self::BLOCKS_SURFACE.bits() |
        //     Self::BLOCKS_GAS.bits() |
        //     Self::BLOCKS_LIQUID.bits() |
        //     Self::BLOCKS_EFFECTS.bits() |
        //     Self::BLOCKS_ACTORS.bits();

        // const BLOCKS_SCENT = Self::BLOCKS_MOVE.bits() | Self::BLOCKS_VISION.bits();
        // const DIVIDES_LEVEL = Self::BLOCKS_MOVE.bits();
        // const WAYPOINT_BLOCKER = Self::BLOCKS_MOVE.bits();

        // const WALL_FLAGS = Self::BLOCKS_MOVE.bits() |
        //     Self::BLOCKS_VISION.bits() |
        //     Self::BLOCKS_LIQUID.bits() |
        //     Self::BLOCKS_GAS.bits() |
        //     Self::BLOCKS_EFFECTS.bits() |
        //     Self::BLOCKS_DIAGONAL.bits();

        // const BLOCKS_EVERYTHING = Self::WALL_FLAGS.bits() |
        //     Self::BLOCKS_ITEMS.bits() |
        //     Self::BLOCKS_ACTORS.bits() |
        //     Self::BLOCKS_SURFACE.bits();


        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

    }
}

impl FromStr for BeingFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = BeingFlags::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                // "EXAMPLE" => result |= ActorFlags::EXAMPLE,
                "DESTROYED" => result |= BeingFlags::DESTROYED,
                "SPAWNED" => result |= BeingFlags::SPAWNED,

                // "SECRETLY_PASSABLE" => result |= ActorFlags::SECRETLY_PASSABLE,
                "ALLOWS_MOVE" => result |= BeingFlags::ALLOWS_MOVE,
                "BLOCKS_VISION" => result |= BeingFlags::BLOCKS_VISION,
                // "BLOCKS_SURFACE" => result |= ActorFlags::BLOCKS_SURFACE,
                // "BLOCKS_LIQUID" => result |= ActorFlags::BLOCKS_LIQUID,
                // "BLOCKS_GAS" => result |= ActorFlags::BLOCKS_GAS,
                // "BLOCKS_ITEMS" => result |= ActorFlags::BLOCKS_ITEMS,
                // "BLOCKS_ACTORS" => result |= ActorFlags::BLOCKS_ACTORS,
                // "BLOCKS_EFFECTS" => result |= ActorFlags::BLOCKS_EFFECTS,
                // "BLOCKS_DIAGONAL" => result |= ActorFlags::BLOCKS_DIAGONAL,
                "INTERRUPT_WHEN_SEEN" => result |= BeingFlags::INTERRUPT_WHEN_SEEN,
                "NO_SIDEBAR" => result |= BeingFlags::NO_SIDEBAR,
                "VISUALLY_DISTINCT" => result |= BeingFlags::VISUALLY_DISTINCT,
                "BRIGHT_MEMORY" => result |= BeingFlags::BRIGHT_MEMORY,
                "INVERT_WHEN_HIGHLIGHTED" => result |= BeingFlags::INVERT_WHEN_HIGHLIGHTED,

                "ON_MAP" => result |= BeingFlags::ON_MAP,
                "IN_SIDEBAR" => result |= BeingFlags::IN_SIDEBAR,

                "FORMAL_NAME" => result |= BeingFlags::FORMAL_NAME,
                "ALWAYS_PLURAL" => result |= BeingFlags::ALWAYS_PLURAL,

                "DEFAULT_ACTOR" => result |= BeingFlags::DEFAULT_ACTOR,
                "DEFAULT_ITEM" => result |= BeingFlags::DEFAULT_ITEM,

                // "BLOCKED_BY_STAIRS" => result |= ActorFlags::BLOCKED_BY_STAIRS,

                // "BLOCKS_SCENT" => result |= ActorFlags::BLOCKS_SCENT,
                // "DIVIDES_LEVEL" => result |= ActorFlags::DIVIDES_LEVEL,
                // "WAYPOINT_BLOCKER" => result |= ActorFlags::WAYPOINT_BLOCKER,

                // "WALL_FLAGS" => result |= ActorFlags::WALL_FLAGS,

                // "BLOCKS_EVERYTHING" => result |= ActorFlags::BLOCKS_EVERYTHING,
                "" => {}
                _ => return Err(format!("Unknown ActorFlags: {}", s)),
            }
        }
        Ok(result)
    }
}

impl BeingFlags {
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

impl From<&str> for BeingFlags {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(err) => panic!("{}", err),
        }
    }
}

impl fmt::Display for BeingFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
