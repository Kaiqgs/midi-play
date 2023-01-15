use crate::{
    components::component::BuildContext,
    models::{
        midi::to_sheet::MidiSheetTransformer, midi::MidiTrack, track_manager::TrackManager,
        trackeable::Trackeable,
    },
    tests::common::{REFERENCE_MID, REFERENCE_UNE_MID},
};

fn _setup<'a>(filepath: Option<String>) -> TrackManager<MidiSheetTransformer<'a>> {
    match filepath {
        Some(fpath) => {
            let mut track = MidiTrack::new(fpath.clone());
            let mtransf = MidiSheetTransformer::new(Some(track.open()));
            TrackManager::new(Some(fpath), mtransf, BuildContext::default())
        }
        None => TrackManager::new(
            None,
            MidiSheetTransformer::new(None),
            BuildContext::default(),
        ),
    }
}

fn setup<'a>() -> TrackManager<MidiSheetTransformer<'a>> {
    _setup(Some(REFERENCE_MID.into()))
    //TODO: set existing path;
}

fn bad_setup<'a>() -> TrackManager<MidiSheetTransformer<'a>> {
    _setup(Some(REFERENCE_UNE_MID.into()))
}

fn empty_setup<'a>() -> TrackManager<MidiSheetTransformer<'a>> {
    _setup(None)
}

#[tokio::test]
#[should_panic]
async fn fail_from_file_unexistent() {
    let mut bad_track = bad_setup();
    let beggining = bad_track.go_to(0).await;
    assert_eq!(beggining, 0);
}

#[tokio::test]
#[should_panic]
async fn empty_go_to_beggining() {
    let mut empty_track = empty_setup();
    let beggining = empty_track.go_to(0).await;
    assert_eq!(beggining, 0);
}

#[tokio::test]
async fn go_to_beggining() {
    let mut sheet_track = setup();
    let beggining = sheet_track.go_to(0).await;
    assert_eq!(beggining, 0);
}
