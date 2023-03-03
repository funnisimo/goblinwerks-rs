pub struct Fps {
    counter: u32,
    start: f64,
    last: f64,
    total_frames: u64,
    fps: u32,
    average: u32,
    goal: u32,
}

impl Fps {
    pub fn new(goal: u32) -> Fps {
        let now = crate::app::now();
        Fps {
            counter: 0,
            total_frames: 0,
            start: now,
            last: now,
            fps: 0,
            average: 0,
            goal,
        }
    }

    pub fn goal(&self) -> u32 {
        self.goal
    }

    pub fn current(&self) -> u32 {
        self.fps
    }

    pub fn step(&mut self) {
        self.counter += 1;
        self.total_frames += 1;
        let curr = crate::app::now();
        if curr - self.last > 1.0 {
            self.last = curr;
            self.fps = self.counter;
            self.counter = 0;
            self.average = (self.total_frames as f64 / (self.last - self.start)) as u32;
        }
    }
    pub fn average(&self) -> u32 {
        self.average
    }
}
