use gw_app::log;
use gw_ecs::World;
use gw_util::rng::RandomNumberGenerator;

use crate::being::BeingKind;

use super::{Horde, HordeFlags};
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct Hordes {
    all: Vec<Arc<Horde>>,
}

impl Hordes {
    pub fn new() -> Self {
        Hordes { all: Vec::new() }
    }

    pub fn push(&mut self, horde: Horde) {
        self.all.push(Arc::new(horde));
    }

    pub fn dump(&self) {
        log("Hordes");
        for horde in self.all.iter() {
            log(format!("{:?}", horde));
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Arc<Horde>> {
        self.all.iter()
    }
}

/* , forbidden_flags: HordeFlags, required_flags: HordeFlags, summonerKind: String */
pub fn pick_random_horde(world: &World, depth: u32) -> Option<Arc<Horde>> {
    //   if (typeof summonerKind == 'string') {
    //     summonerKind = RUT.Monsters.get(summonerKind);
    //   }

    let summoner_kind: Option<Arc<BeingKind>> = None;
    let forbidden_flags = HordeFlags::empty();
    let required_flags = HordeFlags::empty();

    let hordes = match world.try_read_global::<Hordes>() {
        None => {
            log("No hordes configured.");
            return None;
        }
        Some(h) => h,
    };

    let mut poss_count = 0;
    for horde in hordes.iter() {
        if forbidden_flags.is_empty() == false && horde.flags.intersects(forbidden_flags) {
            continue;
        }
        if required_flags.is_empty() == false && !horde.flags.contains(required_flags) {
            continue;
        }

        let freq = horde.frequency(depth);
        if summoner_kind.is_some() {
            // Make sure summoner == horde.leader_kind
        } else {
            poss_count += freq;
        }
    }

    if poss_count == 0 {
        log(format!("No hordes found for depth={}", depth));
        return None;
    }

    let mut rng = world.write_resource::<RandomNumberGenerator>();
    let mut index = rng.range(1, poss_count as i32) as u32;

    for horde in hordes.iter() {
        if horde.flags.intersects(forbidden_flags) {
            continue;
        }
        if !horde.flags.contains(required_flags) {
            continue;
        }

        let freq = horde.frequency(depth);
        if summoner_kind.is_some() {
            // Make sure summoner == horde.leader_kind
        }

        if index <= freq {
            return Some(Arc::clone(horde));
        }
        index -= freq;
    }

    None
}

// // If hordeID is 0, it's randomly assigned based on the depth, with a 10% chance of an out-of-depth spawn from 1-5 levels deeper.
// // If x is negative, location is random.
// // Returns a pointer to the leader.
// pub fn spawn_random(map, blockedFov, forbidden_flags=0, required_flags=0)
// {
//   let failsafe;
//   let horde;
//   let depth = map.level || 0;

// 	if ((depth > 1) && (RUT.RNG.rollDie(100) < 10)) {
// 		depth = map.level + RUT.RNG.inRange(1, Math.min(5, Math.round(map.level / 2)));
// 		if (depth > DEEPEST_LEVEL) {
// 			depth = DEEPEST_LEVEL; // Math.max(map.level, AMULET_LEVEL);
// 		}
//     forbidden_flags |= HordeFlags.HORDE_NEVER_OOD;
// 	}

// 	horde = RUT.Horde.pick(depth, forbidden_flags, required_flags);
// 	if (!horde) {
//     console.log('No qualifying hordes.', depth, forbidden_flags, required_flags);
// 		return undefined;
// 	}

//   const xy = RUT.Map.randomXy(map, {
//     tile: horde.spawnTile,
//     test: (x, y) => {
//       const inHallway = (RUT.Map.passableArcCount(map, x, y) > 1);
//       if (inHallway) return false;
//       // This is supposed to keep monsters from being spawned in front of the player or near the stairs (at generation time)
//       const isBlocked = blockedFov && RUT.FOV.canSeeOrSense(blockedFov, x, y);
//       if (isBlocked) return false;

//       return true;
//     }
//   });

//   if (!xy) return undefined;
//   return RUT.Horde.spawn(horde, map, xy.x, xy.y);
// }
