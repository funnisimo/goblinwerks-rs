use crate::fl;
use bitflags::bitflags;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Default)]
    pub struct Treasure: u32 {
        const HAS_GOLD = fl!(0);
        const HAS_ITEM = fl!(1);

        const DROP_30       = fl!(2);     /* Drop an item/gold (30%) */
        const DROP_60       = fl!(3);     /* Drop an item/gold (60%) */
        const DROP_90     = fl!(4);       /* Drop an item/gold (90%) */
        const DROP_1D2 = fl!(5);    // Drop 1-2 items.
        const DROP_1D3 = fl!(6);     // Drop 1-3 items (cumulative with DROP_1D2).
        const DROP_1D4     = fl!(7);      /* Drop 1d4 items/gold */

        const DROP_GOOD = fl!(8);    // Force good drops.
        const DROP_GREAT = fl!(9);   // Force great drops.
        const DROP_USEFUL  = fl!(10);      /* Drop "useful" items */
        const DROP_CHOSEN  = fl!(11);      /* Drop "chosen" items */

        const DROP_CHEST     = fl!(12);      /* Chests ('&') */
        const DROP_MISSILE   = fl!(13);      /* Slings/Bows/Xbows/Ammo */
        const DROP_TOOL      = fl!(14);      /* Shovels/Picks/Spikes */
        const DROP_WEAPON    = fl!(15);      /* Weapons */
        const DROP_MUSIC     = fl!(16);      /* Musical instruments/Song books */
        const DROP_CLOTHES   = fl!(17);      /* Boots/Gloves/Cloaks/Soft Armor */
        const DROP_ARMOR     = fl!(18);      /* Hard Armor/Helms/Shields/Dragon Armor */
        const DROP_LITE      = fl!(19);      /* Lites/Flasks */
        const DROP_JEWELRY   = fl!(20);      /* Rings/Amulets/Crowns */
        const DROP_RSW       = fl!(21);      /* Rod/staff/wand */
        const DROP_WRITING   = fl!(22);      /* Books/scrolls */
        const DROP_POTION    = fl!(23);
        const DROP_FOOD      = fl!(24);
        const DROP_JUNK      = fl!(25);      /* Sticks, Pottery, etc ('~') */

        const DROP_ESSENCE = fl!(26);      /* Drop essences */
        const DROP_MUSHROOM = fl!(27);      /* Drop specific mushrooms */
        const DROP_MINERAL = fl!(28);      /* Drop specific gold */

    }
}

impl fmt::Display for Treasure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Treasure {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Treasure::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                "HAS_GOLD" => result |= Treasure::HAS_GOLD,
                "HAS_ITEM" => result |= Treasure::HAS_ITEM,
                "DROP_30" => result |= Treasure::DROP_30,
                "DROP_60" => result |= Treasure::DROP_60,
                "DROP_90" => result |= Treasure::DROP_90,
                "DROP_1D2" => result |= Treasure::DROP_1D2,
                "DROP_1D3" => result |= Treasure::DROP_1D3,
                "DROP_1D4" => result |= Treasure::DROP_1D4,
                "DROP_GOOD" => result |= Treasure::DROP_GOOD,
                "DROP_GREAT" => result |= Treasure::DROP_GREAT,
                "DROP_USEFUL" => result |= Treasure::DROP_USEFUL,
                "DROP_CHOSEN" => result |= Treasure::DROP_CHOSEN,

                "DROP_CHEST" => result |= Treasure::DROP_CHEST,
                "DROP_MISSILE" => result |= Treasure::DROP_MISSILE,
                "DROP_TOOL" => result |= Treasure::DROP_TOOL,
                "DROP_WEAPON" => result |= Treasure::DROP_WEAPON,
                "DROP_MUSIC" => result |= Treasure::DROP_MUSIC,
                "DROP_CLOTHES" => result |= Treasure::DROP_CLOTHES,
                "DROP_ARMOR" => result |= Treasure::DROP_ARMOR,
                "DROP_LITE" => result |= Treasure::DROP_LITE,
                "DROP_JEWELRY" => result |= Treasure::DROP_JEWELRY,
                "DROP_RSW" => result |= Treasure::DROP_RSW,
                "DROP_WRITING" => result |= Treasure::DROP_WRITING,
                "DROP_POTION" => result |= Treasure::DROP_POTION,
                "DROP_FOOD" => result |= Treasure::DROP_FOOD,
                "DROP_JUNK" => result |= Treasure::DROP_JUNK,
                "DROP_ESSENCE" => result |= Treasure::DROP_ESSENCE,
                "DROP_MUSHROOM" => result |= Treasure::DROP_MUSHROOM,
                "DROP_MINERAL" => result |= Treasure::DROP_MINERAL,

                "" => {}
                _ => return Err(format!("Unknown TileFlag1: {}", s)),
            }
        }
        Ok(result)
    }
}

impl Treasure {
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

impl From<&str> for Treasure {
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
        assert_eq!(Treasure::HAS_GOLD, "HAS_GOLD".into());
        assert_eq!(Treasure::HAS_ITEM, "has_item".into());
        assert_eq!(Treasure::DROP_1D2, "Drop_1d2".parse().unwrap());

        assert_eq!(
            Treasure::HAS_GOLD | Treasure::DROP_GREAT,
            "has_gold | DROP_GREAT".parse().unwrap()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(format!("{:?}", Treasure::DROP_GOOD), "DROP_GOOD");

        let flags = Treasure::HAS_ITEM | Treasure::DROP_GREAT;
        assert_eq!(format!("{:?}", flags), "HAS_ITEM | DROP_GREAT");
    }
}
