use std::ops::Range;

use super::clef::Clef;
use super::constant as def;

pub struct Staff {
    clef: Clef,
    range: Range<u32>,
}

impl Staff {
    pub fn new(clef: Clef) -> Self {
        let start = (def::MIDDLE_C * -clef.offset) as u32;
        let end = (def::MIDDLE_C * clef.offset) as u32;
        assert!(start > 0, "todo...");
        Staff {
            clef,
            range: start..end,
        }
    }
}
