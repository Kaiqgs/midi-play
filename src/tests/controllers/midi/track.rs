use crate::{
    models::midi::MidiTrack,
    tests::common::{log_setup, REFERENCE_MID, REFERENCE_UNE_MID},
};

fn _setup(filepath: String) -> MidiTrack {
    log_setup();
    MidiTrack::new_read(filepath)
}

fn setup() -> MidiTrack {
    _setup(REFERENCE_MID.into())
}

fn bad_setup() -> MidiTrack {
    _setup(REFERENCE_UNE_MID.into())
}

#[test]
#[should_panic]
fn fail_unexistent_track() {
    bad_setup();
}

#[test]
fn load_track() {
    let _mtrack = setup();
    assert!(_mtrack.loaded);
}
