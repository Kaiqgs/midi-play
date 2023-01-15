use crate::{components::sheet::track::SheetTrackComponentData, models::note::Note};

use super::staff_system::StaffSystem;

pub struct Track<'a> {
    pub system: Option<&'a StaffSystem>,
    pub track: Vec<Vec<Note>>,
    pub component_data: Option<SheetTrackComponentData>,
}

impl<'a> Track<'a> {
    pub fn new(
        system: Option<&'a StaffSystem>,
        data: Option<SheetTrackComponentData>,
        track: Vec<Vec<Note>>,
    ) -> Self {
        Track {
            system,
            component_data: data,
            track,
        }
    }

    pub fn load_file() -> Result<bool, String> {
        unimplemented!()
    }
}
