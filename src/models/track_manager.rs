use ggez::Context;

use crate::components::sheet::clef::ClefComponentData;
use crate::models::sheet::SheetTrack;
use std::rc::Rc;
use std::sync::Arc;

use crate::components::component::{BuildContext, Component};
use crate::components::sheet::staff_system::StaffSystemComponentData;

use super::midi::MidiTrack;
use super::sheet::clef::Clef;
use super::sheet::from::SheetTransformer;

use super::sheet::staff::Staff;
use super::sheet::staff_system::StaffSystem;

pub struct TrackManager<T>
where
    T: SheetTransformer,
{
    pub midi: MidiTrack,
    pub sheet: StaffSystem,
    // TODO: create sheet track that display notes;
    //pub sheet_track: SheetTrack,
    // pub sheet: ESheetTrack,
    pub loaded: bool,
    pub transform: T,
}

impl<M> TrackManager<M>
where
    M: SheetTransformer,
{
    pub fn new(filepath: String, transform: M, build: BuildContext) -> Self {
       
        let staffsys = StaffSystem::new(None, None, build);

        TrackManager {
            transform,
            sheet: staffsys,
            loaded: false,
            midi: MidiTrack::from_file(filepath),
        }
    }
}
