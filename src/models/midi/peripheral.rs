use std::sync::mpsc::Sender;

use crate::models::note::Note;

use super::playback::MidiPlayback;

pub struct MidiPeripheral {
    pub tick_played: u32,
    pub name: String,
    pub note_tx: Option<Sender<Option<Note>>>,
}

impl MidiPeripheral {
    pub fn new(name: String) -> Self {
        MidiPeripheral {
            tick_played: 0,
            note_tx: None,
            name,
        }
    }
}

impl From<&MidiPlayback> for MidiPeripheral {
    fn from(playback: &MidiPlayback) -> Self {
        MidiPeripheral {
            tick_played: 0,
            note_tx: playback.note_tx.clone(),
            name: "peripheral".into(),
        }
    }
}
