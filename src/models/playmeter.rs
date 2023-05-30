use std::time::Duration;

use crate::components::playmeter::PlayMeterComponentData;

use super::note::Note;

pub const ACCEPTANCE_MS: u64 = 500;
pub const ACCEPTANCE_DURATION: Duration = Duration::from_millis(ACCEPTANCE_MS);

pub enum QualityMode {
    //Strict Mode:
    // * min value of: -inf;
    // * max value of: note_count;
    // * counts missed notes & missed inputs;
    Strict,
    //Normal Mode:
    // * min value of: -note_count;
    // * max value of: note_count;
    // * counts missed notes & ignore missed inputs;
    Normal,
    //Loose Mode:
    // * min value of: -note_count;
    // * max value of: note_count;
    // * ignore missed notes & counts missed inputs regardless of time as long as mantain the same order;
    Loose,
}

pub struct PlayMeter {
    pub average_quality: f64,
    pub unpaired_track_pool: Vec<Note>,
    pub unpaired_input_pool: Vec<Note>,
    pub component_data: PlayMeterComponentData,
    //settings
    pub quality_mode: QualityMode,
    pub acceptance_range: Duration,
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
            quality_mode: QualityMode::Normal,
            unpaired_track_pool: Vec::new(),
            unpaired_input_pool: Vec::new(),
            acceptance_range: ACCEPTANCE_DURATION,
            component_data: PlayMeterComponentData::new(),
        }
    }
}
