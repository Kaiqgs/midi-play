use mockall::automock;

#[automock]
pub trait QualityMeter {
    fn compare(&mut self, expected: u32, received: u32) -> f64;
    fn reset(&mut self) -> bool;
}

pub struct PlayMeter {
    average_quality: f64,
}

impl Default for PlayMeter {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayMeter {
    pub fn new() -> Self {
        PlayMeter {
            average_quality: 1.0,
        }
    }
}
