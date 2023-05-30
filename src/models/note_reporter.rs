use super::note::Note;

pub trait NoteReporter {
    fn report(&mut self) -> Vec<Note> {
        Vec::new()
    }
}
