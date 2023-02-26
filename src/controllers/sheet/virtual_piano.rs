use crate::models::sheet::virtual_piano::VirtualPianoKeyHandler;
use crate::models::{note::Note, playmeter::QualityMeter, sheet::virtual_piano::VirtualPiano};

impl Default for VirtualPiano {
    fn default() -> Self {
        Self::new()
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

impl VirtualPiano {
    pub fn on_note(&mut self, note: &Note) -> Result<(), ()> {
        let found_note = self.notes.iter().position(|n| n.id == note.id);
        match found_note {
            Some(note_idx) => {
                self.notes[note_idx].on = note.on;
                self.notes[note_idx].naturality = note.naturality;
                Ok(())
            }
            None => Err(()),
        }
    }
}
