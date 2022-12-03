use crate::models::{midi::input::VirtualPianoKeyHandler, playmeter::QualityMeterObject};



pub struct VirtualPiano {}

impl VirtualPiano {
    pub fn new() -> Self {
        VirtualPiano {}
    }
}

impl VirtualPianoKeyHandler for VirtualPiano {
    type Comparator = QualityMeterObject;
    fn note_on(&self, comparator: QualityMeterObject) {
        unimplemented!()
    }

    fn note_off(&self, comparator: QualityMeterObject) {
        unimplemented!()
    }
}
