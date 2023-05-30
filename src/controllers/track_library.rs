use rfd::FileDialog;
use crate::models::{game_track::GameTrack, track_library::TrackLibrary};

impl TrackLibrary {
    pub fn push(&mut self, track: GameTrack) {
        self.tracks.push(track);
    }

    pub fn remove(&mut self, index: usize) {
        self.tracks.remove(index);
    }

    pub fn import(&mut self){
        let files = FileDialog::new()
            .add_filter("Track", &["midtrk", "trk"])
            .add_filter("Mid", &["mid"])
            .pick_files();

        if let Some(files) = files {
            for _file in files {
                // let track = GameTrack::new(file);
                // self.push(track);
            }
        }

    }
}
