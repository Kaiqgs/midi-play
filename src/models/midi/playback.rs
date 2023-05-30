use std::sync::mpsc::Sender;

use midir::MidiOutputConnection;

use crate::models::note::Note;

pub(crate) const NOTE_ON: u8 = 0x90;
pub(crate) const NOTE_OFF: u8 = 0x80;
pub struct MidiPlayback {
    pub channels: [Option<()>; 16],
    pub conn_out: Result<MidiOutputConnection, ()>,
    pub note_tx: Option<Sender<Option<Note>>>,
    pub tick_played: u32,
}

impl MidiPlayback {
    pub fn new(opt_name: Option<String>) -> MidiPlayback {
        let channels: [Option<()>; 16] = [Option::None; 16];

        let mut playback = MidiPlayback {
            channels,
            conn_out: Err(()),
            note_tx: None,
            tick_played: 0,
        };
        playback
            .open(opt_name.unwrap_or("MidiPlayback".into()))
            .expect("Failed to open midi output");
        playback
    }
}
