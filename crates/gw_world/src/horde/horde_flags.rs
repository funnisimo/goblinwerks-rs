use bitflags::bitflags;
use gw_util::fl;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;
use std::str::FromStr;

////////////////////////////////////////////
// AI Flags

bitflags! {
    #[derive(Default, Deserialize, Serialize)]
    pub struct HordeFlags: u32 {

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

        // const AIMLESS_MOVE = fl!(0);    // Will move around, but not too much

        const DIES_ON_LEADER_DEATH = fl!(0);	  // if the leader dies, the horde will die instead of electing new leader
        const IS_SUMMONED = fl!(1);	          // minions summoned when any creature is the same species as the leader and casts summon
        const SUMMONED_AT_DISTANCE = fl!(2);   // summons will appear across the level, and will naturally path back to the leader
        const LEADER_CAPTIVE = fl!(3);	        // the leader is in chains and the followers are guards
        const NO_PERIODIC_SPAWN = fl!(4);	    // can spawn only when the level begins -- not afterwards
        const ALLIED_WITH_PLAYER = fl!(5);
        const NEVER_OOD = fl!(6);              // Horde cannot be generated out of depth
        const SPAWN_AS_AVATAR = fl!(7);

        const MEMBERS_IN_FRONT = fl!(10);       // Spawn the members between the leader and the heros
        const MEMBERS_BEHIND = fl!(11);         // Spawn the members behind the leader (away from hero)
        const MEMBERS_LOOSE = fl!(12);          // Spawn members with space between them

        // These need to be converted to tags
        const MACHINE_BOSS = fl!(20);	          // used in machines for a boss challenge
        const MACHINE_WATER_MONSTER = fl!(21);	// used in machines where the room floods with shallow water
        const MACHINE_CAPTIVE = fl!(22);	      // powerful captive monsters without any captors
        const MACHINE_STATUE = fl!(23);	        // the kinds of monsters that make sense in a statue
        const MACHINE_TURRET = fl!(24);	        // turrets, for hiding in walls
        const MACHINE_MUD = fl!(25);	          // bog monsters, for hiding in mud
        const MACHINE_KENNEL = fl!(26);	        // monsters that can appear in cages in kennels
        const VAMPIRE_FODDER = fl!(27);	        // monsters that are prone to capture and farming by vampires
        const MACHINE_LEGENDARY_ALLY = fl!(28);	// legendary allies
        const MACHINE_THIEF = fl!(29);          // monsters that can be generated in the key thief area machines
        const MACHINE_GOBLIN_WARREN = fl!(30);  // can spawn in goblin warrens

        const MACHINE_ONLY = Self::MACHINE_BOSS.bits() | Self::MACHINE_WATER_MONSTER.bits() |
            Self::MACHINE_CAPTIVE.bits() |  Self::MACHINE_STATUE.bits() |
            Self::MACHINE_TURRET.bits() |   Self::MACHINE_MUD.bits() |
            Self::MACHINE_KENNEL.bits() |   Self::VAMPIRE_FODDER.bits() |
            Self::MACHINE_LEGENDARY_ALLY.bits() | Self::MACHINE_THIEF.bits() |
            Self::MACHINE_GOBLIN_WARREN.bits();

        const AVOID_SPAWN = Self::MACHINE_ONLY.bits() | Self::NO_PERIODIC_SPAWN.bits();
    }
}

impl HordeFlags {
    pub fn apply(&mut self, flags: &str) {
        for val in flags.split("|") {
            if val.trim().starts_with("!") {
                match HordeFlags::from_str(&val[1..]) {
                    Ok(flag) => self.remove(flag),
                    Err(_) => {}
                }
            } else {
                match HordeFlags::from_str(val) {
                    Ok(flag) => self.insert(flag),
                    Err(_) => {}
                }
            }
        }
    }
}

impl FromStr for HordeFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = HordeFlags::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                // "AIMLESS_MOVE" => result |= HordeFlags::AIMLESS_MOVE,
                "DIES_ON_LEADER_DEATH" => result |= HordeFlags::DIES_ON_LEADER_DEATH,
                "IS_SUMMONED" => result |= HordeFlags::IS_SUMMONED,
                "SUMMONED_AT_DISTANCE" => result |= HordeFlags::SUMMONED_AT_DISTANCE,
                "LEADER_CAPTIVE" => result |= HordeFlags::LEADER_CAPTIVE,
                "NO_PERIODIC_SPAWN" => result |= HordeFlags::NO_PERIODIC_SPAWN,
                "ALLIED_WITH_PLAYER" => result |= HordeFlags::ALLIED_WITH_PLAYER,
                "NEVER_OOD" => result |= HordeFlags::NEVER_OOD,
                "SPAWN_AS_AVATAR" | "AVATAR" => result |= HordeFlags::SPAWN_AS_AVATAR,

                "MEMBERS_IN_FRONT" | "FRONT" => result |= HordeFlags::MEMBERS_IN_FRONT,
                "MEMBERS_BEHIND" | "BEHIND" => result |= HordeFlags::MEMBERS_BEHIND,
                "MEMBERS_LOOSE" | "LOOSE" => result |= HordeFlags::MEMBERS_LOOSE,

                "MACHINE_BOSS" => result |= HordeFlags::MACHINE_BOSS,
                "MACHINE_WATER_MONSTER" => result |= HordeFlags::MACHINE_WATER_MONSTER,
                "MACHINE_CAPTIVE" => result |= HordeFlags::MACHINE_CAPTIVE,
                "MACHINE_STATUE" => result |= HordeFlags::MACHINE_STATUE,
                "MACHINE_TURRET" => result |= HordeFlags::MACHINE_TURRET,
                "MACHINE_MUD" => result |= HordeFlags::MACHINE_MUD,
                "MACHINE_KENNEL" => result |= HordeFlags::MACHINE_KENNEL,
                "VAMPIRE_FODDER" => result |= HordeFlags::VAMPIRE_FODDER,
                "MACHINE_LEGENDARY_ALLY" => result |= HordeFlags::MACHINE_LEGENDARY_ALLY,
                "MACHINE_THIEF" => result |= HordeFlags::MACHINE_THIEF,
                "MACHINE_GOBLIN_WARREN" => result |= HordeFlags::MACHINE_GOBLIN_WARREN,

                "MACHINE_ONLY" => result |= HordeFlags::MACHINE_ONLY,

                "AVOID_SPAWN" => result |= HordeFlags::AVOID_SPAWN,

                "" => {}
                _ => return Err(format!("Unknown ActorFlag1: {}", s)),
            }
        }
        Ok(result)
    }
}

impl From<&str> for HordeFlags {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(err) => panic!("{}", err),
        }
    }
}

impl fmt::Display for HordeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
