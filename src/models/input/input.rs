use crate::models::note::Note;
use std::fmt;
pub enum MidiPlayInput {
    PauseStart(Option<bool>),
    NoteKey(Note),
}

impl fmt::Debug for MidiPlayInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MidiPlayInput::PauseStart(_) => f.write_str("Pause/Start"),
            MidiPlayInput::NoteKey(note) => {
                f.write_str(format!("<{} @ {}>", note.midi, note.time.sec).as_str())
            }
        }
    }
}
impl fmt::Display for MidiPlayInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
