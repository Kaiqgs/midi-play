use std::path::Path;

use crate::models::{game_track::{GameTrack, EXTENSIONS}, render_util::RenderUtil, track_library::TrackLibrary};
use log::warn;
use rfd::FileDialog;

impl TrackLibrary {
    pub fn push(&mut self, track: GameTrack) {
        self.tracks.push(track);
    }

    pub fn remove(&mut self, index: usize) {
        self.tracks.remove(index);
    }


    pub fn import(&mut self, reutil: RenderUtil) {
        let files = FileDialog::new()
            .add_filter("Mid", &EXTENSIONS[0..0]) // TODO: move filter into second position after track;
            .add_filter("Track", &EXTENSIONS[1..])
            .pick_files();

        if let Some(files) = files {
            for file in files {
                if !file.exists() {
                    continue;
                }
                let extension = file.extension().expect("File has no extension");
                if extension == EXTENSIONS[0] {
                    let mut stem = file
                        .file_stem()
                        .expect("File has no stem")
                        .to_str()
                        .expect("Failed to convert stem to string")
                        .to_string();
                    stem.push_str(&(".".to_string() + EXTENSIONS[0]));

                    let dir = reutil
                        .winctx
                        .resources_folder
                        .clone();

                    let dir = Path::new(&dir);
                    let result = dir.join(Path::new(&stem));
                    warn!("Stem: {:?}", stem);
                    warn!("Dir: {:?}", dir);
                    warn!("Result: {:?}", result);
                    warn!("Filepath: {:?}", file.as_path());
                    // result.as_path();
                    // dir.push_str(stem);
                    // dir.push_str(".mid");
                    let _copy = std::fs::copy(file.as_path(), result).expect("Failed to copy file");

                    warn!("Imported file: {:?} bytes {:?}", file, _copy);
                    warn!("To file: {:?}", dir);
                    // mid uses the track and default cover
                }
                // midtrk uses the track and track cover from zip

                warn!("Importing file: {:?}", file);
                warn!("Extension: {:?}", file.extension())

                // let track = GameTrack::new(file);
                // self.push(track);
            }
        }
    }
}
