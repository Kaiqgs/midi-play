use crate::models::midi::to_sheet::MidiSheetTransformer;
use crate::models::sheet::{from::SheetTransformer, SheetTrack};

impl MidiSheetTransformer {
    fn get_something() {}
}
impl SheetTransformer for MidiSheetTransformer {
    fn convert(&self, input: String) -> SheetTrack {
        unimplemented!()
    }
}
