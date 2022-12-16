use crate::models::{
    midi::input::{VirtualPiano, VirtualPianoKeyHandler},
    playmeter::MockQualityMeter,
};

fn setup() -> VirtualPiano {
    //TODO: set existing path;
    VirtualPiano::new()
}

#[test]
fn good_note_on() {
    let vpiano = setup();
    let mut meter = MockQualityMeter::new();
    meter.expect_compare().return_const(1.0).times(1);
    vpiano.note_on(meter);
}

#[test]
fn good_note_off() {
    let vpiano = setup();
    let mut meter = MockQualityMeter::new();
    meter.expect_compare().return_const(1.0).times(1);
    vpiano.note_off(meter);
}

#[test]
fn bad_note_on() {
    let vpiano = setup();
    let mut meter = MockQualityMeter::new();
    meter.expect_compare().return_const(0.0).times(1);
    vpiano.note_on(meter);
}

#[test]
fn bad_note_off() {
    let vpiano = setup();
    let mut meter = MockQualityMeter::new();
    meter.expect_compare().return_const(0.0).times(1);
    vpiano.note_off(meter);
}
