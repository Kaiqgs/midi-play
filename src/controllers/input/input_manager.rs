use crate::models::{input::input_manager::InputManager, note::Note};

impl InputManager {
    pub fn on_note(&mut self, note: &Note) {
        self.virtual_piano.on_note(note);

        self.playback.note_change(note);
        // self.virtual_piano.on_note(note).virtual_piano.on_note(&note);
    }
}
