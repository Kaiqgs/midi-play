pub struct MidiSheetTransformer {
    pub bytes: Vec<u8>,
}

pub struct MidiSheetFromFile {
    pub bytes: Vec<u8>,
}

impl MidiSheetTransformer {
    pub fn new() -> Self {
        MidiSheetTransformer { bytes: vec![] }
    }
}

impl MidiSheetFromFile {
    pub fn new() -> Self {
        MidiSheetFromFile { bytes: vec![] }
    }
}
