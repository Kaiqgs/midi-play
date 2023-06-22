// use crate::models::{midi::to_sheet::MidiSheetFromFile, sheet::from::SheetFromFile};
// use crate::tests::common::{setup_log, REFERENCE_MID};
// use crate::controllers::midi::to_sheet;

// fn setup() -> MidiSheetFromFile {
//     setup_log();
//     MidiSheetFromFile::new()
// }
//
// #[test]
// fn parse_default() {
//     let mut midi_sheet = setup();
//     let sheet_track = midi_sheet.parse(REFERENCE_MID.into());
//     assert!(sheet_track.track.len() > 0);
// }
