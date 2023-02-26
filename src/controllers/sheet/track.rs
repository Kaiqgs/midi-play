use crate::components::component::Component;
use crate::models::render_util::RenderUtil;
use crate::models::sheet::SheetTrack;
use crate::models::trackeable::Trackeable;
use async_trait::async_trait;
use std::ops::Range;

impl Trackeable for SheetTrack {
    fn go_to(&mut self, time: u32) -> u32 {
        unimplemented!()
    }

    fn set_loop(&mut self, range: Range<u32>) -> bool {
        unimplemented!()
    }
}

impl SheetTrack {
    pub fn compute_render_range(&mut self, canvas: RenderUtil) -> (f64, f64) {
        self.update(canvas);
        self.component_data
            .range
            .expect("Range not set in update!?")
    }
}
