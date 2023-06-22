use crate::models::note::Note;
use crate::models::note_reporter::NoteReporter;
use crate::models::restartable::Restartable;
use crate::models::{track_manager::TrackManager, trackeable::Trackeable};

impl Trackeable for TrackManager {
    fn go_to(&mut self, _time: u32) -> u32 {
        unimplemented!()
    }

    fn set_loop(&mut self, _range: std::ops::Range<u32>) -> bool {
        unimplemented!()
    }
}

impl NoteReporter for TrackManager {
    fn report(&mut self) -> Vec<Note> {
        self.sheet_track.report()
    }
}

impl Restartable for TrackManager {
    fn restart(&mut self) -> Result<(), ()>{
        self.sheet_track.restart()?;
        Ok(())
    }
}
