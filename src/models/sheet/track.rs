use std::collections::HashMap;

use ggez::graphics::MeshBuilder;

use crate::{
    components::{
        component::BuildContext, drawing::Drawing, sheet::track::SheetTrackComponentData,
    },
    models::{
        clock::{Clock, ClockFloat},
        midi::timing::TimingInformation,
        note::Note,
    },
};

use super::staff_system::StaffSystem;

pub struct Track {
    pub system: StaffSystem,
    pub track: Vec<Vec<Note>>,
    pub last_track_time: Clock,
    pub track_pairs: Vec<HashMap<usize, usize>>,
    pub track_timing: HashMap<u32, TimingInformation>,
    pub component_data: SheetTrackComponentData,
    pub time: ClockFloat,
}

impl Track {
    pub fn new(
        system: StaffSystem,
        data: Option<SheetTrackComponentData>,
        track: Vec<Vec<Note>>,
        track_pairs: Vec<HashMap<usize, usize>>,
        track_timing: HashMap<u32, TimingInformation>,
        last_tick: Option<Clock>,
    ) -> Self {
        let component_data = match data {
            Some(c) => c,
            None => {
                let mut drawing = Drawing::default();
                drawing.meshbuilder = Some(MeshBuilder::new());
                SheetTrackComponentData::new(drawing, BuildContext::default())
            }
        };

        Track {
            system,
            component_data,
            track,
            track_pairs,
            track_timing,
            last_track_time: last_tick.unwrap_or(Clock { sec: 0.0, tick: 0 }),
            time: ClockFloat {
                sec: 0.0,
                tick: 0.0,
            },
        }
    }
}
