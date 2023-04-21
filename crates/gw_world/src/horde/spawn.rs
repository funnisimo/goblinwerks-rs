pub struct HordeSpawn {
    pub next_time: u64,
    pub check_delay: u64,
    pub max_alive: u32,
}

impl HordeSpawn {
    pub fn new() -> Self {
        HordeSpawn {
            next_time: 0,
            check_delay: 1000,
            max_alive: 5,
        }
    }
}
