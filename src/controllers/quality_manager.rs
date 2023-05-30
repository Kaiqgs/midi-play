use crate::models::{
    note_reporter::NoteReporter, quality_manager::QualityManager,
    sheet::track_window_ctx::TrackWindowContext, playmeter::PlayMeter, restartable::Restartable,
};

impl QualityManager {
    pub fn capture(&mut self, input: &mut impl NoteReporter, track: &mut impl NoteReporter) {
        let input_notes = input.report();
        let track_notes = track.report();
        self.meter.capture(input_notes, track_notes);
    }

    pub fn set_track(&mut self, trackwinctx: TrackWindowContext) {
        self.track = Some(trackwinctx);
    }

}

impl Restartable for QualityManager {
    
    fn restart(&mut self) -> Result<(), ()> {
        self.meter = PlayMeter::new();
        self.recording.restart()?;
        Ok(())
    }
}
