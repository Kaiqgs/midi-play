use super::{playmeter::PlayMeter, record::Recording, sheet::track_window_ctx::TrackWindowContext};

pub struct QualityManager {
    pub meter: PlayMeter,
    pub recording: Recording,
    pub track: Option<TrackWindowContext>,
}

impl QualityManager {
    pub fn new() -> Self {
        QualityManager {
            meter: PlayMeter::new(),
            recording: Recording::new(),
            track: None,
        }
    }
}
