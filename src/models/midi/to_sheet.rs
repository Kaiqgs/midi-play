pub struct MidiSheetTransformer<'a> {
    pub smf: Option<midly::Smf<'a>>,
}

impl Default for MidiSheetTransformer<'_> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl MidiSheetTransformer<'_> {
    pub fn new(smf: Option<midly::Smf<'_>>) -> Self {
        MidiSheetTransformer { smf }
    }
}
