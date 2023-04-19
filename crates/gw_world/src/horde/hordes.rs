use gw_app::log;

use super::Horde;
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
}
