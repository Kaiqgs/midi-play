use log::debug;

use crate::models::{render_util::RenderUtil, track_manager::TrackManager};

use super::component::Component;

impl Component for TrackManager {
    fn get_name(&self) -> String {
        "[Track Manager]".to_string()
    }
    fn update(&mut self, canvas: RenderUtil) {
        self.tick_time += 1.0;
        debug!("Updating TrackManager");
        if self.filepath.is_some() {
            debug!("Updating SheetTrack");
            // self.sheet_track.tick_time = self.tick_time;
            self.sheet_track.update(canvas.clone());
        }
    }

    fn next(&self) -> Vec<super::component::ComponentObject> {
        vec![&self.system, &self.sheet_track]
    }
}
