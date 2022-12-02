use super::MidiTrack;
use super::SheetTrack;
use super::Trackeable;
use {super::sheet::SheetTransform, super::sheet::SheetTransformer};
pub struct TrackManager<T: SheetTransformer> {
    midi: MidiTrack,
    sheet: SheetTrack,
    loaded: bool,
    transformer: SheetTransform<T>,
}

impl<T: SheetTransformer> TrackManager<T> {
    pub fn new(filepath: String, transform: T) -> Self {
        TrackManager {
            transformer: SheetTransform::new(transform),
            sheet: SheetTrack::new(None),
            loaded: false,
            midi: MidiTrack::new(filepath),
        }
    }
}

impl<T: SheetTransformer> Trackeable for TrackManager<T> {
    fn go_to(&mut self, time: u32) -> bool {
        unimplemented!()
    }

    fn set_loop(&mut self, range: std::ops::Range<u32>) -> bool {
        unimplemented!()
    }
}