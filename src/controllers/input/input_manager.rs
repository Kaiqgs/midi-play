use crate::models::{
    input::input_manager::InputManager, note::Note, note_reporter::NoteReporter,
    render_util::RenderUtil,
};

impl NoteReporter for InputManager {
    fn report(&mut self) -> Vec<Note> {
        let notes = self.reported.to_owned();
        self.reported = Vec::new();
        notes
    }
}

impl InputManager {
    pub fn on_note(&mut self, note: &Note, canvas: RenderUtil) {
        self.virtual_piano
            .on_note(note)
            .expect("Failed to play note");
        self.playback
            .note_change(note)
            .expect("Failed to play note");
        self.reported.push(note.trigger(canvas.winctx.since_start));
        // self.virtual_piano.on_note(note).virtual_piano.on_note(&note);
    }
}
