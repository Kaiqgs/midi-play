use crate::models::{game_mode::NOTES_MASK, quality_manager::QualityManager};

use super::component::Component;

impl Component for QualityManager {
    fn get_name(&self) -> String {
        String::from("[Quality Manager]")
    }

    fn update(&mut self, reutil: crate::models::render_util::RenderUtil) {
        self.meter.update(reutil);
        // self.recording.update();
    }

    fn next(&self) -> Vec<super::component::ComponentObject> {
        vec![&self.meter]
    }

    fn get_mask(&self) -> crate::models::bit_mode::BitMask {
        NOTES_MASK
    }
}
