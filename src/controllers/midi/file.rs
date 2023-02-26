use midly::Smf;
use std::fs::{self};

use crate::models::midi::MidiFile;

use std::path::Path;

impl MidiFile {
    pub fn new_read(path: String) -> Self {
        let mut track = MidiFile::new(path);
        track.open();
        track
    }

    pub fn open(&mut self) -> Smf {
        let path = Path::new(&self.filepath);
        let bytes = fs::read(path).unwrap();
        self.bytes = bytes.to_vec();
        Smf::parse(&self.bytes).unwrap()
    }
}
