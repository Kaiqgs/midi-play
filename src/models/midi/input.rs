use crate::models::playmeter::QualityMeter;

pub trait VirtualPianoKeyHandler<Q: QualityMeter> {
    fn note_on(&self, comparator: Q);
    fn note_off(&self, comparator: Q);
}

pub struct VirtualPiano {}