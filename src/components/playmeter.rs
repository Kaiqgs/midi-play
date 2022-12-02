use super::Component;
use super::PlayMeterModel;

/// Draws player performance
pub struct PlayMeter {
    meter: PlayMeterModel,
}

impl PlayMeter {
    pub fn new() -> Self {
        return PlayMeter {
            meter: PlayMeterModel::new(),
        };
    }

    pub fn load_file() -> Result<bool, String> {
        unimplemented!()
    }
}

impl Component for PlayMeter {
    fn draw() {
        unimplemented!()
    }
}
