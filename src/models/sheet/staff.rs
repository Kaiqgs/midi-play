use super::{clef::Clef, definition};



pub struct Staff {
    clef: Clef,
    range: definition::Notes,
}

impl Staff {
    pub fn new(clef: Clef) -> Self {
        let start = (definition::MIDDLE_C - clef.offset) as u32;
        let end = (definition::MIDDLE_C + clef.offset) as u32;
        assert!(start > 0, "todo...");
        Staff {
            clef,
            range: start..end,
        }
    }
}
