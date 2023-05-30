use crate::models::build_context::BuildContext;
use crate::models::render_util::RenderUtil;
use crate::models::sheet::SheetTrack;
use std::collections::HashMap;

use super::clock::Clock;
use super::midi::peripheral::MidiPeripheral;
use super::note::Note;
use super::sheet::from::SheetFromFile;
use super::sheet::sheet_const;

use super::sheet::staff_system::StaffSystem;
use super::sheet::track_window_ctx::TrackWindowContext;

pub struct TrackManager {
    pub filepath: Option<String>,
    pub system: StaffSystem,
    pub sheet_track: SheetTrack,
    pub parser: Box<dyn SheetFromFile>,
    pub tick_time: f64,
}

impl TrackManager {
    pub fn new(
        filepath: Option<String>,
        parser: Box<dyn SheetFromFile>,
        build: BuildContext,
    ) -> Self {
        let staves_for_sheet = StaffSystem::default_staff(build.clone());
        let staff_for_sheet = StaffSystem::new(staves_for_sheet, None, build.clone());

        let staves = StaffSystem::default_staff(build.clone());
        let staff = StaffSystem::new(staves, None, build);
        let sheet_track =
            SheetTrack::new(staff_for_sheet, None, vec![], vec![], HashMap::new(), None);
        TrackManager {
            filepath,
            parser,
            system: staff,
            sheet_track,
            tick_time: 0.0,
        }
    }

    pub fn set_track(
        &mut self,
        optional_path: Option<String>,
        reutil: RenderUtil,
        peripheral: MidiPeripheral,
    ) -> Result<TrackWindowContext, ()> {
        self.filepath = optional_path.clone();
        //TODO update render_range here
        match optional_path {
            Some(path) => {
                let mut sheet_track = self.parser.parse(path);
                let render_range = sheet_track.compute_render_range(reutil);
                let ctx = TrackWindowContext::new(
                    sheet_track.track_pairs.len(),
                    None,
                    Some(render_range),
                );
                self.sheet_track = sheet_track;
                self.sheet_track.component_data.playback = peripheral;
                //.resume
                // self.sheet_track.component_data.playback.open(playback.note_tx.unwrap().clone());
                let mut note_start = Note::new(sheet_const::E, 4);
                note_start.time = Clock { tick: 0, sec: 0.0 };
                note_start.on = Some(true);
                let mut note_end = Note::new(sheet_const::E, 4);
                note_end.time = Clock {
                    tick: 1000,
                    sec: 1.0,
                };
                note_end.on = Some(false);
                self.sheet_track
                    .component_data
                    .playback
                    .note(&note_start, &note_end)
                    .expect("Failed to send note");
                return Ok(ctx);
            }
            None => {
                self.sheet_track.component_data.playback = peripheral;
                Ok(TrackWindowContext::new(0, None, None))
            },
        }
    }
}
