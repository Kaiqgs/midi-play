use super::note::Note;

pub trait QualityMeter {
    fn compare(&mut self, expected: u32, received: u32) -> f64;
    fn reset(&mut self) -> bool;
}

pub struct PlayMeter {
    average_quality: f64,
    history: Vec<Note>,
}

impl Default for PlayMeter {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayMeter {
    pub fn new() -> Self {
        PlayMeter {
            history: Vec::new(),
            average_quality: 1.0,
        }
    }
}
