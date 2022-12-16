use crate::models::{
    midi::input::{VirtualPiano, VirtualPianoKeyHandler},
    playmeter::QualityMeter,
};

impl Default for VirtualPiano {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualPiano {
    pub fn new() -> Self {
        VirtualPiano {}
    }
}

impl<Q> VirtualPianoKeyHandler<Q> for VirtualPiano
where
    Q: QualityMeter,
{
    fn note_on(&self, comparator: Q) {
        unimplemented!()
    }

    fn note_off(&self, comparator: Q) {
        unimplemented!()
    }
}
