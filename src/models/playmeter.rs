pub struct PlayMeter {
    average_quality: f64,
}

impl PlayMeter {
    pub fn new() -> Self {
        return PlayMeter { average_quality: 1.0 };
    }

    pub fn compare(&mut self, expected: u32, received: u32) -> f64 {
        unimplemented!()
    }

    pub fn reset(&mut self) -> bool { 
        unimplemented!()
    }
}