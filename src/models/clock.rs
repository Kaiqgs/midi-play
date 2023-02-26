pub struct Clock {
    pub sec: f64,
    pub tick: u32,
}

impl Clone for Clock {
    fn clone(&self) -> Self {
        Clock {
            sec: self.sec,
            tick: self.tick,
        }
    }
}

pub struct ClockFloat {
    pub sec: f64,
    pub tick: f64,
}
