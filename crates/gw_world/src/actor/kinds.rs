use super::ActorKind;
use gw_app::log;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Default)]
pub struct ActorKinds {
    kinds: HashMap<String, Arc<ActorKind>>,
}

impl ActorKinds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty() -> Self {
        ActorKinds {
            kinds: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Arc<ActorKind>> {
        match self.kinds.get(name) {
            None => None,
            Some(tile) => Some(tile.clone()),
        }
    }

    pub fn insert(&mut self, kind: Arc<ActorKind>) {
        self.kinds.insert(kind.id.clone(), kind);
    }

    // pub fn load(&mut self, toml: &StringTable) -> Result<(), String> {
    //     match load_tile_data(self, toml) {
    //         Err(e) => Err(e),
    //         Ok(count) => {
    //             log(format!("Loaded tiles :: count={}", count));
    //             Ok(())
    //         }
    //     }
    // }

    pub fn dump(&self) {
        log("ActorKinds");
        for (id, tile) in self.kinds.iter() {
            log(format!("{} : {:?}", id, tile));
        }
    }
}
