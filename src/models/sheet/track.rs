use super::system::StaffSystem;

pub struct Track {
    pub system: Option<StaffSystem>,
}

impl Track {
    pub fn new(system: Option<StaffSystem>) -> Self {
        Track { system }
    }
}
