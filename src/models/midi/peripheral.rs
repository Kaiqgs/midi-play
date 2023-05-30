use std::sync::mpsc::Sender;

use crate::models::note::Note;

use super::playback::MidiPlayback;

#[derive(Clone)]
pub struct MidiPeripheral {
    pub tick_played: u32,
    pub name: String,
    pub note_tx: Option<Sender<Option<Note>>>,
    pub channel: Option<u8>,
}

impl MidiPeripheral {
    pub fn new(name: String) -> Self {
        MidiPeripheral {
            tick_played: 0,
            note_tx: None,
            name,
            channel: None,
        }
    }
}

impl From<&mut MidiPlayback> for MidiPeripheral {
    fn from(playback: &mut MidiPlayback) -> Self {
        let free_idx = playback
            .channels
            .iter()
            .position(|c| c.is_none())
            .expect("No free channels");
        playback.channels[free_idx] = Some(());
        MidiPeripheral {
            tick_played: 0,
            note_tx: playback.note_tx.clone(),
            name: "peripheral".into(),
            channel: Some(free_idx as u8),
        }
    }
}
