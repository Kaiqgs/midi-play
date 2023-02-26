use crate::models::midi::{peripheral::MidiPeripheral, playback::MidiPlayback};
use std::{
    sync::mpsc::Sender,
    thread::{self, sleep},
    time::Duration,
};

use log::debug;

use crate::models::note::Note;

impl MidiPeripheral {
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn open(&mut self, playback: Sender<Option<Note>>) {
        self.tick_played = 0;
        self.note_tx = Some(playback);
    }

    pub fn close(&mut self) {
        self.note_tx = None;
    }

    pub fn set_time(&mut self, tick: u32) {
        self.tick_played = (tick + 1).max(self.tick_played);
    }

    pub fn note(&mut self, note_start: &Note, note_end: &Note) -> Result<(), ()> {
        if let None = self.note_tx {
            debug!("No playback set");
            return Err(());
        }
        if note_start.time.tick < self.tick_played {
            return Ok(());
        }

        let tx = self.note_tx.as_ref().unwrap().clone();
        let note_start = note_start.clone();
        let note_end = note_end.clone();
        thread::Builder::new()
            .name("note".to_string())
            .spawn(move || {
                let duration_sec = note_end.time.sec - note_start.time.sec;
                tx.send(Some(note_start.clone()))
                    .expect("Failed to send note on");
                sleep(Duration::from_micros((duration_sec * 1e6) as u64));
                tx.send(Some(note_end.clone()))
                    .expect("Failed to send note off");
            });
        Ok(())
    }

    pub fn note_change(&mut self, note: &Note) -> Result<(), ()> {
        if let None = self.note_tx {
            debug!("No playback set");
            return Err(());
        }
        if note.time.tick < self.tick_played {
            return Ok(());
        }

        let tx = self.note_tx.as_ref().unwrap();
        debug!("MidiPlayBack: Note on: {}", note.midi);
        match tx.send(Some(note.clone())) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
