use super::sheet::definition;

pub struct Note {
    pub id: u32,
    pub line: u32,
    pub midi: u32,
}

impl Note {
    pub fn new(note: u32, index: u32) -> Self {
        Note::from_note(definition::get_id(note, index, 0))
    }

    fn from_note(id: definition::Note) -> Self {
        Note {
            id,
            midi: id + definition::MIDI_OFFSET,
            line: definition::LAST_NOTE - id,
        }
    }

    pub fn from_range(note_start: definition::Note, note_end: definition::Note) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        for nid in note_start..note_end {
            result.push(Note::from_note(nid));
        }
        result
    }

    pub fn from_midi(key: midly::num::u7, vel: midly::num::u7) -> Note {
        let intkey: u32 = key.as_int().into();
        let id = intkey - definition::MIDI_OFFSET;
        Note {
            id,
            midi: intkey,
            line: definition::LAST_NOTE - id,
        }
    }
}
