use std::sync::mpsc::Sender;

use midir::MidiOutputConnection;

use crate::models::note::Note;

pub(crate) const NOTE_ON: u8 = 0x90;
pub(crate) const NOTE_OFF: u8 = 0x80;
pub struct MidiPlayback {
    // channels: Vec<u8>,
    pub conn_out: Result<MidiOutputConnection, ()>,
    pub note_tx: Option<Sender<Option<Note>>>,
    // notes_on: Arc<Mutex<HashSet<Note>>>,
    tick_played: u32,
}

impl MidiPlayback {
    pub fn new(opt_name: Option<String>) -> MidiPlayback {
        let mut playback = MidiPlayback {
            conn_out: Err(()),
            note_tx: None,
            // notes_on: Arc::new(Mutex::new(HashSet::new())),
            tick_played: 0,
        };
        playback.open(opt_name.unwrap_or("MidiPlayback".into()));
        playback
    }
}
