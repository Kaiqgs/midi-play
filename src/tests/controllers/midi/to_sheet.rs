use crate::models::{
    midi::{to_sheet::MidiSheetTransformer, MidiTrack},
    sheet::{SheetTrack, from::SheetTransformer},
    track_manager::TrackManager,
};

fn setup() -> MidiSheetTransformer {
    MidiSheetTransformer::new()
    //TODO: set existing path;
}

#[test]
fn converts() {
    let midtrans = setup();
    let track = midtrans.convert("acceptable_input".into());
    assert!(track.system.is_some());
    let system = track.system.unwrap();
    assert_ne!(system.staffs.len(), 0);
}

#[test]
fn converts_faulty() {
    let midtrans = setup();
    let track = midtrans.convert("unnaceptable_input".into());
    assert!(track.system.is_none());
}
