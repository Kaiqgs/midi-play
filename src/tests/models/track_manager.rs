#[cfg(test)]
mod tests {

    use crate::models::{
        midi::{to_sheet::MidiSheetTransformer, track::Track as MidiTrack},
        sheet::{SheetTransform, Track as SheetTrack},
        track_manager::TrackManager, Trackeable,
    };

    pub fn setup() -> TrackManager<MidiSheetTransformer> {
        return TrackManager::new(String::from("existing_path"), MidiSheetTransformer::new()); //TODO: set existing path;
    }

    pub fn bad_setup() -> TrackManager<MidiSheetTransformer> {
        return TrackManager::new(String::from("unexistent_random_path"), MidiSheetTransformer::new());
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
}
