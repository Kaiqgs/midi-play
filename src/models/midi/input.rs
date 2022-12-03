pub trait VirtualPianoKeyHandler {
    type Comparator;
    fn note_on(&self, comparator: Self::Comparator);
    fn note_off(&self, comparator: Self::Comparator);
}