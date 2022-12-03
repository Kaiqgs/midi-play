use async_trait::async_trait;

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

#[async_trait]
impl Trackeable for Track {
    async fn go_to(&mut self, time: u32) -> u32 {
        unimplemented!()
    }

    fn set_loop(&mut self, range: Range<u32>) -> bool {
        unimplemented!()
    }
}
