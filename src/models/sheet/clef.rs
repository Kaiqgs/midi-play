use crate::{
    components::{component::BuildContext, sheet::clef::ClefComponentData},
    models::note::Note,
};

use super::sheet_const;

pub struct Clef {
    pub note: Note,
    pub alias: Option<String>,
    pub component_data: ClefComponentData,
}

impl Clef {
    pub fn new(note: Note, alias: Option<String>, data: Option<ClefComponentData>) -> Self {
        let component_data = match data {
            Some(component_data) => component_data,
            None => ClefComponentData::new(&note, None, BuildContext::default()),
        };

        Clef {
            note,
            alias,
            component_data,
        }
    }

    pub fn new_bass(component_data: Option<ClefComponentData>) -> Self {
        Clef::new(
            Note::new(sheet_const::G, 2),
            Some("Bass / F".into()),
            component_data,
        )
    }

    pub fn new_treble(component_data: Option<ClefComponentData>) -> Self {
        Clef::new(
            Note::new(sheet_const::E, 4),
            Some("Treble / G".into()),
            component_data,
        )
    }
}
