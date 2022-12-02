use super::{StaffSystem, Trackeable};
use std::ops::Range;

pub struct Track {
    staves: Option<StaffSystem>,
}

impl Track {
    pub fn new(staves: Option<StaffSystem>) -> Self {
        Track { staves }
    }
}

impl Trackeable for Track {
    fn go_to(&mut self, time: u32) -> bool {
        unimplemented!()
    }

    fn set_loop(&mut self, range: Range<u32>) -> bool {
        unimplemented!()
    }
}
