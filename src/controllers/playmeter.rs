use crate::models::{note::Note, playmeter::PlayMeter};

impl PlayMeter {
    pub fn measure(&self, input: &Note, expected: &Note) -> f64 {
        (input.time.sec - expected.time.sec) / 100.0
            + (self.average_quality - self.average_quality / 100.0)
    }

    pub fn capture(&mut self, input: Vec<Note>, track: Vec<Note>) {
        self.unpaired_track_pool.extend(input);
        self.unpaired_input_pool.extend(track);
    }
}
