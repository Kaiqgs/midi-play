use crate::{
    components::{
        component::BuildContext,
        sheet::{clef::ClefComponentData, staff_system::StaffSystemComponentData},
    },
    models::note::Note,
};

use super::{clef::Clef, sheet_const, staff::Staff};

pub struct StaffSystem {
    pub staffs: Vec<Staff>,
    pub notes: Vec<Note>,
    pub component_data: StaffSystemComponentData,
}

impl StaffSystem {
    pub fn new(
        staffs: Vec<Staff>,
        _data: Option<StaffSystemComponentData>,
        build: BuildContext,
    ) -> Self {
        let staffdata = StaffSystemComponentData::new(None, build);

        StaffSystem {
            staffs,
            notes: Note::from_range(sheet_const::FIRST_NOTE, sheet_const::LAST_NOTE),
            component_data: staffdata,
        }
    }

    pub fn notes() -> Vec<Note> {
        Note::from_range(sheet_const::FIRST_NOTE, sheet_const::LAST_NOTE)
    }

    pub fn default_staff(build: BuildContext) -> Vec<Staff> {
        let bass_note = Note::new(sheet_const::G, 2);
        let treble_note = Note::new(sheet_const::E, 4);
        let tstaff = Staff::new(
            Clef::new_treble(Some(ClefComponentData::new(
                &treble_note,
                Some("/treble-clef.png".into()),
                build.clone(),
            ))),
            None,
            build.clone(),
        );
        let bstaff = Staff::new(
            Clef::new_bass(Some(ClefComponentData::new(
                &bass_note,
                Some("/bass-clef.png".into()),
                build.clone(),
            ))),
            None,
            build.clone(),
        );
        vec![tstaff, bstaff]
    }
}

impl Default for StaffSystem {
    fn default() -> Self {
        let build = BuildContext::default();
        let staves = Self::default_staff(build.clone());
        Self::new(staves, None, build)
    }
}
