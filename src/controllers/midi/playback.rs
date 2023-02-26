use std::{
    collections::HashSet,
    error::Error,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use log::{debug, info};
use midir::{MidiOutput, MidiOutputConnection};

use crate::models::midi::playback::MidiPlayback;
use crate::models::{
    midi::{
        playback::{NOTE_OFF, NOTE_ON},
        timing::TimingInformation,
    },
    note::Note,
};

impl MidiPlayback {
    pub fn open(&mut self, playback_name: String) -> Result<(), Box<dyn Error>> {
        let (tx, rx) = std::sync::mpsc::channel::<Option<Note>>();
        self.note_tx = Some(tx.clone());
        let conn_name = playback_name.clone();
        let output_name = playback_name.clone();
        let reader_thread = thread::Builder::new()
            .name(playback_name.clone())
            .spawn(move || {
                let midi_output =
                    MidiOutput::new(&output_name).expect("Failed to create MidiOutput");
                let out_ports = midi_output.ports();
                // debug show all ports
                debug!("Available output ports:");
                for (i, p) in out_ports.iter().enumerate() {
                    debug!("{}: {}", i, midi_output.port_name(p).unwrap());
                }
                let out_port = &out_ports[0];
                debug!(
                    "Attempting to connect to output port: {}",
                    midi_output.port_name(out_port).unwrap()
                );

                let mut conn_out = midi_output
                    .connect(out_port, &conn_name)
                    .expect("Failed to connect to MidiOutput");

                let mut on_notes = HashSet::new();

                loop {
                    let note_recv = rx.recv().expect("Failed to receive");
                    let note = match note_recv {
                        Some(note) => note,
                        None => break,
                    };
                    match note.on {
                        Some(note_state) => {
                            if note_state && !on_notes.contains(&note) {
                                conn_out
                                    .send(&[NOTE_ON, note.midi as u8, note.velocity as u8])
                                    .expect("Failed to send note on");
                                let insert_success = on_notes.insert(note.clone());
                                info!("Note on: {}, success: {}", note.midi, insert_success);
                            } else if !note_state && on_notes.contains(&note) {
                                conn_out
                                    .send(&[NOTE_OFF, note.midi as u8, note.velocity as u8])
                                    .expect("Failed to send note off");
                                let remove_success = on_notes.remove(&note);
                                info!("Note off: {}, success: {}", note.midi, remove_success);
                            }
                        }
                        None => (),
                    }
                }
            });

        // Initialize delay
        debug!("Initializing reader...");

        debug!(
            "Opened: {}, sucess = {}",
            playback_name,
            self.note_tx.is_some()
        );
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Box<dyn Error>> {
        match self.note_tx.as_ref() {
            Some(tx) => {
                tx.send(None).expect("Failed to send note off");
                self.note_tx = None;
                Ok(())
            }
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No connection to close",
            ))),
        }
    }
}
