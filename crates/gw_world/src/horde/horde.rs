use std::sync::Arc;

use crate::being::BeingKind;

use super::horde_flags::HordeFlags;
use gw_util::frequency::Frequency;

#[derive(Debug)]
pub struct Horde {
    pub(super) leader: Arc<BeingKind>,
    pub(super) frequency: Frequency, // TODO - Need Frequency type
    pub(super) members: Vec<(Arc<BeingKind>, u32)>,
    pub(super) spawn_tile: Option<String>,
    pub(super) machine_id: u32,
    pub(super) flags: HordeFlags,
    pub(super) tags: Vec<String>,
}

impl Horde {
    pub(super) fn new(leader: Arc<BeingKind>) -> Self {
        Horde {
            leader,
            frequency: Frequency::new(),
            members: Vec::new(),
            spawn_tile: None,
            machine_id: 0,
            flags: HordeFlags::empty(),
            tags: Vec::new(),
        }
    }

    pub fn frequency(&self) -> &Frequency {
        // if (typeof horde.frequency == 'number') return horde.frequency;
        // if (Array.isArray(horde.frequency)) {
        //   const delta = Math.max(0, depth - horde.levelRange[0]);
        //   return Math.max(0, horde.frequency[0] + delta * horde.frequency[1]);
        // }
        // return 1;

        &self.frequency
    }
}

// pub fn pick_horde(depth: u32, forbiddenFlags: HordeFlags, requiredFlags: HordeFlags, summonerKind: String) -> Option<Horde> {
// 	let possCount = 0;

//   if (typeof summonerKind == 'string') {
//     summonerKind = RUT.Monsters.get(summonerKind);
//   }

// 	for (let i=0; i<RUT.Hordes.all.length; i++) {
//     const horde = RUT.Hordes.all[i];
// 		if (horde.flags & forbiddenFlags) continue;
// 		if (~(horde.flags) & requiredFlags) continue;
//     const levelOk = (!summonerKind && horde.levelRange[0] <= depth && horde.levelRange[1] >= depth);
//     if (levelOk) {
//       possCount += RUT.Horde.frequency(horde, depth);
//     }
//     else if (summonerKind && horde.leaderKind) {
//       const leaderKind = RUT.Monsters.get(horde.leaderKind);
//       if (leaderKind === summonerKind) {
//         possCount += horde.frequency;
//       }
//     }
// 	}

// 	if (possCount == 0) {
// 		return undefined;
// 	}

// 	let index = RUT.RNG.inRange(1, possCount);

// 	for (let i=0; i<RUT.Hordes.all.length; i++) {
//     const horde = RUT.Hordes.all[i];
// 		if (horde.flags & forbiddenFlags) continue;
// 		if (~(horde.flags) & requiredFlags) continue;

//     const frequency = RUT.Horde.frequency(horde, depth);
//     const levelOk = (!summonerKind && horde.levelRange[0] <= depth && horde.levelRange[1] >= depth);
//     if (levelOk) {
//       if (index <= frequency) {
//         return horde;
//       }
//       index -= frequency;
//     }
//     else if (summonerKind && horde.leaderKind) {
//       const leaderKind = RUT.Monsters.get(horde.leaderKind);
//       if (leaderKind === summonerKind) {
//         if (index <= frequency) {
// 					return horde;
// 				}
// 				index -= frequency;
//       }
//     }
// 	}
// 	return undefined; // should never happen
// }

// // If hordeID is 0, it's randomly assigned based on the depth, with a 10% chance of an out-of-depth spawn from 1-5 levels deeper.
// // If x is negative, location is random.
// // Returns a pointer to the leader.
// pub fn spawn_random(map, blockedFov, forbiddenFlags=0, requiredFlags=0)
// {
//   let failsafe;
//   let horde;
//   let depth = map.level || 0;

// 	if ((depth > 1) && (RUT.RNG.rollDie(100) < 10)) {
// 		depth = map.level + RUT.RNG.inRange(1, Math.min(5, Math.round(map.level / 2)));
// 		if (depth > DEEPEST_LEVEL) {
// 			depth = DEEPEST_LEVEL; // Math.max(map.level, AMULET_LEVEL);
// 		}
//     forbiddenFlags |= HordeFlags.HORDE_NEVER_OOD;
// 	}

// 	horde = RUT.Horde.pick(depth, forbiddenFlags, requiredFlags);
// 	if (!horde) {
//     console.log('No qualifying hordes.', depth, forbiddenFlags, requiredFlags);
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

// pub fn spawn(horde, map, x, y) {

// 	if (horde.machine) {
// 		// Build the accompanying machine (e.g. a goblin encampment)
// 		RUT.Map.Blueprint.build(horde.machine, map, x, y);
// 	}

//   // console.log('HORDE SPAWN', horde);
// 	let leader = RUT.Monster.create(horde.leader); // TODO - generateMonster(horde.leaderType, true, true);

// 	if (horde.flags & HordeFlags.HORDE_LEADER_CAPTIVE) {
// 		leader.state |= BeingState.BS_CAPTIVE;
// 		leader.state |= BeingState.BS_WANDERING;
// 		leader.stats.set('health', Math.round(leader.stats.max.health / 4) + 1);  // captives are injured

// 		// Draw the manacles unless the horde spawns in weird terrain (e.g. cages).
// 		if (!horde.spawnTile) {
// 			RUT.Map.Decorators.manacles(map, x, y);
// 		}
// 	} else if (horde.flags & HordeFlags.HORDE_ALLIED_WITH_PLAYER) {
// 		RUT.Monster.becomeAllyWith(leader);
// 	}

//   RUT.Map.removeBeingsAt(map, x, y);
//   RUT.Map.addBeing(map, leader, x, y);

// 	if (RUT.Monster.canSubmergeNow(leader)) {
// 		leader.state |= BeingState.BS_SUBMERGED;
// 	}

// 	RUT.Horde.spawnMinions(horde, leader, false);

// 	return leader;
// }

// pub fn populate_generic_spawn_map(map, spawnMap, originX, originY, maxDist, blockingTileFlags, blockingCellFlags)
// {
//   RUT.Grid.fill(spawnMap, 30000);

//   maxDist = maxDist || spawnMap.width * spawnMap.height;

//   function travelToCell(i, j, cost) {
//     const xy = { x: i, y: j };

//     if (cost > maxDist) return;

//     if (!RUT.Map.makeValidXy(map, xy)) return;
//     const cell = RUT.Map.getCell(map, xy.x, xy.y);
//     if (!cell) return;
//     if (spawnMap[xy.x][xy.y] <= cost) return;

//     // console.log('travel to cell', i, j, cost);

//     if (xy.x == originX && xy.y == originY) {
//       spawnMap[xy.x][xy.y] = 0;
//     }
//     else if (RUT.Cell.hasTileFlag(cell, blockingTileFlags) || RUT.Cell.hasFlag(cell, blockingCellFlags)) {
//       spawnMap[xy.x][xy.y] = PDS_FORBIDDEN;
//       return;
//     }

//     spawnMap[xy.x][xy.y] = cost;
//     travelToCell(xy.x + 1, xy.y, cost + 1);
//     travelToCell(xy.x - 1, xy.y, cost + 1);
//     travelToCell(xy.x, xy.y + 1, cost + 1);
//     travelToCell(xy.x, xy.y - 1, cost + 1);
//   }

//   travelToCell(originX, originY, 0);
//   RUT.Grid.findReplace(spawnMap, 30000, 30000, PDS_FORBIDDEN);
// }

// pub fn spawn_minions(horde, leader, summoned=false) {
// 	let atLeastOneMinion = false;

//   const map = leader.map;
// 	const x = leader.xy.x;
// 	const y = leader.xy.y;

//   const spawnMap = RUT.Grid.alloc(map.width, map.height);

//   // console.log('spawnMinions', horde.members, horde.counts);

// 	for (let iSpecies = 0; iSpecies < horde.members.length; iSpecies++) {
// 		let count = RUT.Calc.calc(horde.counts[iSpecies]);
//     const memberType = RUT.Monsters.get(horde.members[iSpecies]);

// 		let forbiddenTileFlags = RUT.Monster.forbiddenTileFlags(memberType);
// 		if (horde.spawnTile) {
//       const tile = RUT.Tiles.get(horde.spawnTile);
// 			forbiddenTileFlags &= ~(tile.flags);
// 		}

//     const spawnTile = RUT.Tiles.get(horde.spawnTile);
//     RUT.Horde.populateGenericSpawnMap(map, spawnMap, x, y, 20, TileFlags.T_DIVIDES_LEVEL & forbiddenTileFlags, CellFlags.HAS_PLAYER | CellFlags.HAS_STAIRS);

// 		for (let iMember = 0; iMember < count; iMember++) {
// 			let failsafe = 0;
//       let xy;
// 			do {
//         xy = RUT.Map.getQualifyingLocNear(map, x, y,
//                 summoned, // hallwaysAllowed
//                 spawnMap, // okMap
//                 forbiddenTileFlags,   // forbiddenTileFlags
//                 CellFlags.HAS_MONSTER,   // forbiddenCellFlags
//                 false);  // deterministic
// 			} while (spawnTile && (!RUT.Map.cellHasTile(map, xy.x, xy.y, spawnTile)) && failsafe++ < 20);
// 			if (failsafe >= 20) {
// 				// abort
// 				break;
// 			}

//       const monst = RUT.Monster.create(memberType); // , true, !summoned);
// 			if (RUT.Monster.canSubmergeNow(monst)) {
// 				monst.state |= BeingState.BS_SUBMERGED;
// 			}
//       RUT.Map.addBeing(map, monst, xy.x, xy.y);

// 			monst.state |= (BeingState.BS_FOLLOWER | BeingState.BS_JUST_SUMMONED);
// 			monst.leader = leader;
// 			monst.state |= (leader.state & BeingState.BS_AI_STATES);
// 			// monst->mapToMe = NULL;
// 			if (horde.flags & HordeFlags.HORDE_DIES_ON_LEADER_DEATH) {
// 				monst.state |= BeingState.BS_BOUND_TO_LEADER;
// 			}
// 			if (horde.flags & HordeFlags.HORDE_ALLIED_WITH_PLAYER) {
// 				RUT.Monster.becomeAllyWith(monst);
// 			}
// 			atLeastOneMinion = true;
// 		}
// 	}

// 	if (atLeastOneMinion && !(horde.flags & HordeFlags.HORDE_DIES_ON_LEADER_DEATH)) {
// 		leader.state |= BeingState.BS_LEADER;
// 	}

//   RUT.Grid.free(spawnMap);
// 	return atLeastOneMinion;
// }
