use crate::models::note::Note;
use log::debug;

use midly::{
    num::{u15, u24, u7},
    Format, MetaMessage, MidiMessage, Smf, Timing, TrackEventKind,
};
use std::fs::{self, File};

use crate::models::{midi::MidiTrack, trackeable::Trackeable};
use async_trait::async_trait;
use std::path::Path;

impl MidiTrack {
    pub fn new_read(path: String) -> Self {
        let mut track = MidiTrack::new(path);
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

#[async_trait]
impl Trackeable for MidiTrack {
    async fn go_to(&mut self, time: u32) -> u32 {
        unimplemented!()
    }

    fn set_loop(&mut self, range: std::ops::Range<u32>) -> bool {
        unimplemented!()
    }
}
