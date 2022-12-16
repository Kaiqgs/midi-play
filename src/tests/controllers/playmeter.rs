use crate::models::playmeter::{PlayMeter, QualityMeter};

fn setup() -> PlayMeter {
    PlayMeter::new()
}

#[test]
fn compare() {
    let mut pmeter = setup();
    let acceptable = pmeter.compare(10, 10);
    let under_acceptance = pmeter.compare(10, 7);
    let over_acceptance = pmeter.compare(10, 13);
    assert!(acceptable >= under_acceptance);
    assert!(acceptable >= over_acceptance);
}

#[test]
fn reset() {
    let mut pmeter = setup();
    assert!(!pmeter.reset())
}
