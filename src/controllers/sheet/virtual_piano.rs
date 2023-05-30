use crate::models::{note::Note, sheet::virtual_piano::VirtualPiano};

impl Default for VirtualPiano {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualPiano {
    pub fn on_note(&mut self, note: &Note) -> Result<(), ()> {
        let found_note = self.notes.iter().position(|n| n.id == note.id);
        match found_note {
            Some(note_idx) => {
                self.notes[note_idx].on = note.on;
                self.notes[note_idx].naturality = note.naturality.clone();
                Ok(())
            }
            None => Err(()),
        }
    }
}
