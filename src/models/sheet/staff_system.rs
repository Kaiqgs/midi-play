use std::{
    ops::{Deref, DerefMut, Range},
    rc::Rc,
    sync::Arc,
};

use ggez::Context;

use crate::{
    components::{
        component::BuildContext,
        drawing::Drawing,
        sheet::{clef::ClefComponentData, staff, staff_system::StaffSystemComponentData},
    },
    models::note::Note,
};

use super::{clef::Clef, definition, staff::Staff};

pub struct StaffSystem {
    pub staffs: Vec<Staff>,
    pub notes: Vec<Note>,
    pub component_data: StaffSystemComponentData,
}

impl StaffSystem {
    pub fn new(
        staffsopt: Option<Vec<Staff>>,
        data: Option<StaffSystemComponentData>,
        build: BuildContext,
    ) -> Self {
        let drawing = Drawing::default();

        let bass_note = Note::new(definition::G, 2);
        let treble_note = Note::new(definition::E, 4);

        let staffs = match staffsopt {
            Some(staffes) => staffes,
            None => {
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
        };

        let staffdata = StaffSystemComponentData::new(None, build);

        StaffSystem {
            staffs,
            notes: Note::from_range(definition::FIRST_NOTE, definition::LAST_NOTE),
            component_data: staffdata,
        }
    }

    pub fn notes() -> Vec<Note> {
        Note::from_range(definition::FIRST_NOTE, definition::LAST_NOTE)
    }
}

impl Default for StaffSystem {
    fn default() -> Self {
        Self::new(None, None, BuildContext::default())
    }
}
