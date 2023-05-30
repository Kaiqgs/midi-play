use crate::components::component::Component;
use crate::models::clock::ClockFloat;
use crate::models::note::Note;
use crate::models::note_reporter::NoteReporter;
use crate::models::render_util::RenderUtil;
use crate::models::restartable::Restartable;
use crate::models::sheet::SheetTrack;

impl NoteReporter for SheetTrack {
    fn report(&mut self) -> Vec<Note> {
        let reported = self.reported.clone();
        self.reported = Vec::new();
        reported
    }
}

impl SheetTrack {
    pub fn compute_render_range(&mut self, reutil: RenderUtil) -> (f64, f64) {
        self.update(reutil);
        self.component_data
            .range
            .expect("Range not set in update!?")
    }
}

impl Restartable for SheetTrack {
    fn restart(&mut self) -> Result<(), ()> {
        self.time = ClockFloat::new();
        self.reported = Vec::new();
        self.component_data.playback.restart()?;
        Ok(())
    }
}
