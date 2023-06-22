use std::path::Path;

use log::warn;

use crate::components::track_library::TrackLibraryComponentData;
use crate::models::build_context::BuildContext;

use super::game_track::GameTrack;
use super::new_game_track::{GameTrackType, NewGameTrack};

pub const DEFAULT_TRACKS: [&str; 3] = [
    "/do_re_.mid",
    "/mc_sweden.mid",
    "/pachelbel_canon_d_major.mid",
];
pub struct TrackLibrary {
    pub tracks: Vec<GameTrack>,
    pub hover_track: usize,
    pub selected_track: Option<usize>,
    pub playing_track: Option<usize>,
    pub component_data: TrackLibraryComponentData,
}

impl TrackLibrary {
    pub fn new(bctx: BuildContext) -> TrackLibrary {
        let mut tracks = DEFAULT_TRACKS
            .map(|s| {
                let mut track = GameTrack::new(
                    NewGameTrack::new(s.into(), GameTrackType::Simple),
                    bctx.clone(),
                );
                track.load_cover(&bctx);
                track
            })
            .to_vec();
        tracks.extend(Self::read_resources(bctx.clone()));
        let component_data = TrackLibraryComponentData::new();
        TrackLibrary {
            hover_track: tracks.len() / 2,
            selected_track: None,
            playing_track: None,
            tracks,
            component_data,
        }
    }

    fn read_resources(bctx: BuildContext) -> Vec<GameTrack> {
        let ctx = bctx.ctx.expect("Context not set");
        // let box_parse: Box<dyn SheetFromFile> = Box::new(midi_parse);
        // let resources_path = bctx.winctx.resources_folder.clone();
        let files = ctx.fs.read_dir("/").expect("Failed to read dir");
        let mut tracks: Vec<GameTrack> = vec![];
        let resources_folder = "/".to_string();
        let resources_folder = Path::new(&resources_folder);

        warn!("Files:  ");
        for file in files {
            let filebuf = resources_folder.join(file.as_path());
            let file_str = filebuf.to_str();
            if file_str.is_none() {
                continue;
            }
            let file_str = file_str.unwrap();
            let track_type = GameTrack::compute_track_type(file_str.into());
            warn!("File:  {:?}", file_str);
            if track_type.is_err() {
                continue;
            }
            warn!("FilePassed:  {:?}", file_str);
            let track_type = track_type.unwrap();
            let track =
                GameTrack::from_new(NewGameTrack::new(file_str.into(), track_type), bctx.clone());
            if track.is_err() {
                continue;
            }
            let mut track = track.unwrap();
            track.load_cover(&bctx);
            tracks.push(track);
        }
        tracks
    }
}
