use crate::models::{
    midi::to_sheet::MidiSheetTransformer, track_manager::TrackManager, trackeable::Trackeable,
};

fn _setup(filepath: String) -> TrackManager {
    let boxtransf = Box::new(MidiSheetTransformer::new());
    TrackManager::new(filepath, boxtransf)
}

fn setup() -> TrackManager {
    _setup("existing_path".into())
    //TODO: set existing path;
}

fn bad_setup() -> TrackManager {
    _setup("unexisting_path".into())
}

#[test]
#[should_panic]
fn fail_from_file_unexistent() {
    let _fromf = bad_setup();
}
#[tokio::test]
async fn go_to_beggining() {
    let mut sheettrack = setup();
    let beggining = sheettrack.go_to(0).await;
    assert_eq!(beggining, 0);
}
