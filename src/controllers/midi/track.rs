use std::fs::{File, self};
use midly::Smf;

use async_trait::async_trait;

use crate::models::{midi::MidiTrack, trackeable::Trackeable};

impl MidiTrack {
    pub fn from_file(path: String ) -> Self{
        let track = MidiTrack::new(path);
        track.read_filepath();
        track
    }

    fn read_filepath(&self){
        let bytes = fs::read("./resources/").unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        for (i, track) in smf.tracks.iter().enumerate() {
            println!("track {} has {} events", i, track.len());
        }
        let mut midi_file = File::open(self.filepath.clone()).unwrap();
    }

/*


use midir::{MidiReader, Ignored};

// Open the MIDI file and create a MidiReader.
let mut midi_file = std::fs::File::open("my_midi_file.mid").unwrap();
let mut midi_reader = MidiReader::new(&mut midi_file).unwrap();

// Create a vector to store the MIDI events.
let mut events = Vec::new();

// Read all the MIDI events from the file and store them in the vector.
for event in midi_reader.track_iter().flat_map(Result::unwrap) {
    // Handle the event depending on its type.
    match event {
        // For a note on event, print the note and velocity.
        midir::event::Event::NoteOn {
            channel: _,
            note,
            velocity,
        } => {
            println!("Note on: {} ({})", note, velocity);
        }
        // For a note off event, print the note and velocity.
        midir::event::Event::NoteOff {
            channel: _,
            note,
            velocity,
        } => {
            println!("Note off: {} ({})", note, velocity);
        }
        // For other events, just ignore them.
        _ => {}
    }

    // Add the event to the vector.
    events.push(event);
}


*/

}

#[async_trait]
impl Trackeable for MidiTrack {
    async fn go_to(&mut self,time:u32) -> u32 {
        unimplemented!()
    }
        
    

    fn set_loop(&mut self,range:std::ops::Range<u32>) -> bool {
        unimplemented!()
    }
}

