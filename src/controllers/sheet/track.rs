use std::ops::Range;
use async_trait::async_trait;
use crate::models::sheet::SheetTrack;
use crate::models::trackeable::Trackeable;

#[async_trait]
impl Trackeable for SheetTrack {
    async fn go_to(&mut self, time: u32) -> u32 {
        unimplemented!()
    }

    fn set_loop(&mut self, range: Range<u32>) -> bool {
        unimplemented!()
    }
}
