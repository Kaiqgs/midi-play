use crate::components::sheet::track::SheetTrackComponentData;

use super::staff_system::StaffSystem;

pub struct Track<'a> {
    pub system: Option<&'a StaffSystem>,
    pub component_data: Option<SheetTrackComponentData>,
}

impl<'a> Track<'a> {
    pub fn new(system: Option<&'a StaffSystem>, data: Option<SheetTrackComponentData> ) -> Self {
        Track {
            system,
            component_data: data,
        }
    }


    pub fn load_file() -> Result<bool, String> {
        unimplemented!()
    }
}
