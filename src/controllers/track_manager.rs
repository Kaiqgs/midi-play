use async_trait::async_trait;

use crate::models::{
    sheet::from::SheetTransformer, track_manager::TrackManager, trackeable::Trackeable,
};

#[async_trait]
impl<T> Trackeable for TrackManager<T>
where
    T: SheetTransformer,
{
    async fn go_to(&mut self, time: u32) -> u32 {
        unimplemented!()
    }

    fn set_loop(&mut self, range: std::ops::Range<u32>) -> bool {
        unimplemented!()
    }
}
