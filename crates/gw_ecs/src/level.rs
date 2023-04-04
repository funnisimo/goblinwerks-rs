#[derive(Default, Debug)]
pub struct Level {
    pub(crate) index: usize,
}

impl Level {
    pub fn new() -> Self {
        Level { index: 0 }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
