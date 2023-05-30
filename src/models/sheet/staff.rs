use crate::{components::sheet::staff::StaffComponentData, models::note::Note};
use crate::models::build_context::BuildContext;

use super::{clef::Clef, sheet_const};

pub struct Staff {
    pub clef: Clef,
    pub notes: Vec<Note>,
    pub component_data: StaffComponentData,
}

impl Staff {
    pub fn new(clef: Clef, data: Option<StaffComponentData>, build: BuildContext) -> Self {
        let note = clef.note.id;
        let notes = Note::from_range(note, note + sheet_const::STAFF_SIZE);
        let notetoref = Note::from_range(note, note + sheet_const::STAFF_SIZE);
        let notesref = notetoref.iter().map(|n| n).collect();
        Staff {
            clef,
            notes,
            component_data: data.unwrap_or(StaffComponentData::new(notesref, build)),
        }
    }
}
