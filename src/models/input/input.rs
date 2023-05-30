use crate::models::{bit_mode::UintGet, game_mode::GameMode, note::Note};
use std::fmt;

#[derive(Clone)]
pub enum MidiPlayInput {
    PauseStart(Option<bool>),
    NextPlay,
    PreviousPlay,
    Restart,
    NextOption,
    PreviousOption,
    SelectOption,
    BackOption,
    Quit,
    NoteKey(Note),
    ModeChange(GameMode),
}

impl fmt::Debug for MidiPlayInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MidiPlayInput::PauseStart(_) => f.write_str("#Pause/Start"),
            MidiPlayInput::NoteKey(note) => {
                f.write_str(format!("#{} @ {}#", note.midi, note.time.sec).as_str())
            }
            MidiPlayInput::NextPlay => f.write_str("#NextPlay"),
            MidiPlayInput::PreviousPlay => f.write_str("#PreviousPlay"),
            MidiPlayInput::NextOption => f.write_str("#NextOption"),
            MidiPlayInput::PreviousOption => f.write_str("#PreviousOption"),
            MidiPlayInput::Restart => f.write_str("#Restart"),
            MidiPlayInput::Quit => f.write_str("#Quit"),
            MidiPlayInput::ModeChange(mode) => {
                f.write_str(format!("#ModeChange: {}#", mode.get()).as_str())
            }
            MidiPlayInput::SelectOption => f.write_str("#SelectOption"),
            MidiPlayInput::BackOption => todo!(),
        }
    }
}
impl fmt::Display for MidiPlayInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
