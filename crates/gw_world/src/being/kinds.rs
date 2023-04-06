use super::BeingKind;
use gw_app::log;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Default)]
pub struct BeingKinds {
    kinds: HashMap<String, Arc<BeingKind>>,
}

impl BeingKinds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty() -> Self {
        BeingKinds {
            kinds: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    pub fn get(&self, name: &str) -> Option<Arc<BeingKind>> {
        match self.kinds.get(name) {
            None => None,
            Some(tile) => Some(tile.clone()),
        }
    }

    pub fn insert(&mut self, kind: Arc<BeingKind>) {
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
