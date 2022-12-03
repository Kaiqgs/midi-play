use crate::{
    controllers::midi::input::VirtualPiano,
    models::{midi::input::VirtualPianoKeyHandler, playmeter::MockQualityMeter},
};

fn setup() -> VirtualPiano {
    VirtualPiano::new()
    //TODO: set existing path;
}

#[test]
fn good_note_on() {
    let vpiano = setup();
    let mut meter = MockQualityMeter::new();
    meter.expect_compare().return_const(1.0).times(1);
    let bmeter = Box::new(meter);
    vpiano.note_on(bmeter);
}

#[test]
fn good_note_off() {
    let vpiano = setup();
    let mut meter = MockQualityMeter::new();
    meter.expect_compare().return_const(1.0).times(1);
    let bmeter = Box::new(meter);
    vpiano.note_off(bmeter);
}
