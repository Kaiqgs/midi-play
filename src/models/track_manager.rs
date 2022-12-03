use async_trait::async_trait;

use super::sheet::from::SheetTransformer;
use super::sheet::SheetTrack;
use super::midi::MidiTrack;
use super::trackeable::Trackeable;

pub type TransformObject = Box<dyn SheetTransformer>;

pub struct TrackManager {
    midi: MidiTrack,
    sheet: SheetTrack,
    loaded: bool,
    transform: TransformObject,
}

impl TrackManager {
    pub fn new(filepath: String, transform: TransformObject) -> Self {
        TrackManager {
            transform,
            sheet: SheetTrack::new(None),
            loaded: false,
            midi: MidiTrack::new(filepath),
        }
    }
}

#[async_trait]
impl Trackeable for TrackManager {
    async fn go_to(&mut self, time: u32) -> u32 {
        unimplemented!()
    }

    fn set_loop(&mut self, range: std::ops::Range<u32>) -> bool {
        unimplemented!()
    }
}
