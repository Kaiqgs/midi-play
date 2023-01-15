use ggez::mint::Point2;

use crate::components::component::BuildContext;
use crate::models::pausable::Pausable;
use crate::MidiPlay;

fn setup<'a>() -> MidiPlay<'a> {
    MidiPlay::new(BuildContext::new(None, Point2 { x: 192, y: 192 }), None)
}

#[test]
fn force_resume() {
    let mut midi = setup();
    assert!(!midi.resume());
    assert!(!midi.resume());
}

#[test]
fn attempt_resume() {
    let mut midi = setup();
    midi.pause();
    assert!(midi.resume());
}

#[test]
fn force_pause() {
    let mut midi = setup();
    assert!(midi.pause());
    assert!(!midi.pause());
    assert!(!midi.pause());
}

#[test]
fn attempt_pause() {
    let mut midi = setup();
    assert!(midi.pause());
    assert!(!midi.pause());
    assert!(!midi.pause());
}

#[tokio::test]
async fn quit() {
    let mut midi = setup();
    assert!(midi.quit().await);
}
