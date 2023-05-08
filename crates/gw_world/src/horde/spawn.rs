use super::{Horde, HordeRef};
use crate::{being::spawn_being, horde::HordeFlags};
use gw_app::log;
use gw_ecs::{Entity, World};
use gw_util::point::Point;
use gw_util::value::{Key, Value};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct HordeSpawn {
    pub id: String,
    pub next_time: u64,
    pub check_delay: u64,
    pub max_alive: u32,
    pub required_tags: Vec<String>,
    pub forbidden_tags: Vec<String>,
}

impl HordeSpawn {
    pub fn new() -> Self {
        HordeSpawn {
            id: "DEFAULT".to_string(),
            next_time: 0,
            check_delay: 1000,
            max_alive: 5,
            required_tags: Vec::new(),
            forbidden_tags: Vec::new(),
        }
    }
}

impl Default for HordeSpawn {
    fn default() -> Self {
        HordeSpawn::new()
    }
}

#[derive(Debug)]
pub enum HordeSpawnParseError {
    InvalidValueType(Value),
    InvalidField(Key, Value),
    UnknownField(Key),
}

pub fn parse_spawn(value: &Value) -> Result<Vec<HordeSpawn>, HordeSpawnParseError> {
    let mut out = Vec::new();

    if value.is_bool() {
        if value.as_bool().unwrap() {
            out.push(HordeSpawn::default());
        }
        return Ok(out);
    } else if value.is_map() {
        // Single spawn instance
        match parse_spawn_map(value.as_map().unwrap()) {
            Err(e) => return Err(e),
            Ok(v) => {
                out.push(v);
            }
        }
    } else if value.is_list() {
        // Multiple...
        for val in value.as_list().unwrap().iter() {
            match parse_spawn_map(val.as_map().unwrap()) {
                Err(e) => return Err(e),
                Ok(v) => {
                    out.push(v);
                }
            }
        }
    } else {
        return Err(HordeSpawnParseError::InvalidValueType(value.clone()));
    };

    Ok(out)
}

fn parse_spawn_map(value: &HashMap<Key, Value>) -> Result<HordeSpawn, HordeSpawnParseError> {
    let mut out = HordeSpawn::new();

    for (k, v) in value.iter() {
        match k.as_str().unwrap() {
            "id" => out.id = v.to_string(),
            "check_delay" => match v.as_int() {
                None => return Err(HordeSpawnParseError::InvalidField(k.clone(), v.clone())),
                Some(v) => out.check_delay = v as u64,
            },
            "max_alive" | "max" => match v.as_int() {
                None => return Err(HordeSpawnParseError::InvalidField(k.clone(), v.clone())),
                Some(v) => out.max_alive = v as u32,
            },
            "tags" => {
                let tags: Vec<String> = if v.is_list() {
                    let l = v.as_list().unwrap();
                    l.iter()
                        .map(|v| v.to_string())
                        .filter_map(|v| match v.trim() {
                            "" => None,
                            x => Some(x.to_string()),
                        })
                        .collect()
                } else if v.is_string() {
                    v.to_string()
                        .split(&[',', '|', ' ', '+'])
                        .filter_map(|v| match v.trim() {
                            "" => None,
                            x => Some(x.to_string()),
                        })
                        .collect()
                } else {
                    return Err(HordeSpawnParseError::InvalidField(k.clone(), v.clone()));
                };

                for t in tags.iter() {
                    if t.starts_with("!") {
                        let t = t.strip_prefix("!").unwrap().trim();
                        out.forbidden_tags.push(t.to_string());
                    } else {
                        out.required_tags.push(t.clone());
                    }
                }
            }
            _ => return Err(HordeSpawnParseError::UnknownField(k.clone())),
        }
    }

    Ok(out)
}

// There are 2 ways to spawn hordes:
// 1) Spawn all members
// 2) Leader as avatar
//
// This is a choice of the game which to do.
// Most roguelikes (Brogue, DCSS, ...) use option #1.  With this option, the members of the horde (minions if you will) spawn
// around the leader position using the horde flags as modifiers.
// Some classic games use #2 - like Ultima 3, Wizardry, Bards Tale, ...  In this method, only the leader
// is spawned.  The horde is attached to the leader and will come into play if/when there is combat.
//
// The difference is which spawn type you configure in the horde.
// The default is to spawn all (#1), but there is a flag on the horde to do the avatar version.
//
// If you do the avatar version, there is a combat map that will open for combat.
// This is configured two ways -
// A) If there is a combat_map field on the horde, that map is used
// B) Otherwise, the map is automatically created from the tiles around the horde and the attacker (the hero)
//

pub fn spawn_horde(horde: &Arc<Horde>, world: &mut World, point: Point) -> Entity {
    // 	if (horde.machine) {
    // 		// Build the accompanying machine (e.g. a goblin encampment)
    // 		RUT.Map.Blueprint.build(horde.machine, map, x, y);
    // 	}

    let leader_entity = spawn_being(&horde.leader, world, point);

    if horde.flags.intersects(HordeFlags::SPAWN_AS_AVATAR) {
        world
            .write_component()
            .insert(leader_entity, HordeRef::new(Arc::clone(horde)));
        log(format!(
            "Spawn Horde Avatar - {} @ {:?}",
            horde.leader.id, point
        ));
        return leader_entity;
    }

    // if horde.flags.intersects(HordeFlags.HORDE_LEADER_CAPTIVE) {

    // 	leader.state |= BeingState.BS_CAPTIVE;
    // 	leader.state |= BeingState.BS_WANDERING;
    // 	leader.stats.set('health', Math.round(leader.stats.max.health / 4) + 1);  // captives are injured

    // 	// Draw the manacles unless the horde spawns in weird terrain (e.g. cages).
    // 	if (!horde.spawnTile) {
    // 		RUT.Map.Decorators.manacles(map, x, y);
    // 	}
    // } else if (horde.flags & HordeFlags.HORDE_ALLIED_WITH_PLAYER) {
    // 	RUT.Monster.becomeAllyWith(leader);
    // }

    // 	if (RUT.Monster.canSubmergeNow(leader)) {
    // 		leader.state |= BeingState.BS_SUBMERGED;
    // 	}

    // 	spawn_minions(horde, leader_entity, point, false);

    panic!("Only support spawn as avatar right now");
}

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

// pub fn spawn_minions(horde, leader, leader_point, summoned=false) {
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
