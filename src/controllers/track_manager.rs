use crate::models::clock::Clock;
use crate::models::sheet::track_window_ctx::TrackWindowContext;
use crate::models::{
    midi::timing::TimingInformation,
    note::Note,
    sheet::{from::SheetTransformer, sheet_const},
    track_manager::TrackManager,
    trackeable::Trackeable,
};

impl Trackeable for TrackManager {
    fn go_to(&mut self, time: u32) -> u32 {
        unimplemented!()
    }

    fn set_loop(&mut self, range: std::ops::Range<u32>) -> bool {
        unimplemented!()
    }
}

impl TrackManager {
    pub fn get_window_context(&self) -> TrackWindowContext {
        self.sheet_track.component_data.winctx.track.clone()
    }
}
