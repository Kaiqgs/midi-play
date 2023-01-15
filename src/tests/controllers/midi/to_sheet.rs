use crate::models::midi::MidiTrack;
use crate::models::{midi::to_sheet::MidiSheetTransformer, sheet::from::SheetTransformer};
use crate::tests::common::{REFERENCE_MID, REFERENCE_UNE_MID};

fn _setup(smf: Option<midly::Smf>) -> MidiSheetTransformer {
    MidiSheetTransformer::new(smf)
}

fn setup<'a>() -> MidiSheetTransformer<'a> {
    _setup(Some(MidiTrack::new(REFERENCE_MID.into()).open()))
}

fn bad_setup<'a>() -> MidiSheetTransformer<'a> {
    _setup(Some(MidiTrack::new(REFERENCE_UNE_MID.into()).open()))
}

#[test]
fn converts() {
    let midtrans = setup();
    let track = midtrans.convert();
    assert!(track.system.is_some());
    let system = track.system.unwrap();
    assert_ne!(system.staffs.len(), 0);
}

#[test]
fn converts_faulty() {
    let midtrans = bad_setup();
    let track = midtrans.convert();
    assert!(track.system.is_none());
}
