use std::ops::Range;

use crate::{
    components::{component::BuildContext, sheet::staff::StaffComponentData},
    models::note::Note,
};

use super::{clef::Clef, definition};

pub struct Staff {
    pub clef: Clef,
    pub notes: Vec<Note>,
    pub component_data: StaffComponentData,
}

impl Staff {
    pub fn new(clef: Clef, data: Option<StaffComponentData>, build: BuildContext) -> Self {
        let note = clef.note.id;
        let notes = Note::from_range(note, note + definition::STAFF_SIZE);
        let notetoref = Note::from_range(note, note + definition::STAFF_SIZE);
        let notesref = notetoref.iter().map(|n|n).collect();
        Staff {
            clef,
            notes,
            component_data: data.unwrap_or(StaffComponentData::new(notesref, build)),
        }
    }
}

impl Default for Staff {
    fn default() -> Self {
        let treble = Clef::new_treble(None);
        Self::new(treble, None, BuildContext::default())
    }
}
