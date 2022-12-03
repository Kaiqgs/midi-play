use ggez::graphics::Image;

use super::component::{Component, Drawing};
use crate::models::{playmeter::PlayMeter as PlayMeterModel, draw_util::DrawObject};

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

impl Component<Image> for PlayMeter {
    fn draw(&self, canvas: DrawObject) -> Drawing<Image> {
        unimplemented!()
    }
}
