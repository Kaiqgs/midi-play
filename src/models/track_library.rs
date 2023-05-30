use crate::components::track_library::TrackLibraryComponentData;
use crate::models::build_context::BuildContext;

use super::game_track::GameTrack;

pub const DEFAULT_TRACKS: [&str; 3] =
    ["do_re_.mid", "mc_sweden.mid", "pachelbel_canon_d_major.mid"];
pub struct TrackLibrary {
    pub tracks: Vec<GameTrack>,
    pub hover_track: usize,
    pub selected_track: Option<usize>,
    pub playing_track: Option<usize>,
    pub component_data: TrackLibraryComponentData,
}

impl TrackLibrary {
    pub fn new(bctx: BuildContext) -> TrackLibrary {
        let tracks = DEFAULT_TRACKS
            .map(|s| {
                GameTrack::new(
                    s.to_string(),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    bctx.clone(),
                )
            })
            .to_vec();
        let component_data = TrackLibraryComponentData::new();
        TrackLibrary {
            hover_track: tracks.len()/2,
            selected_track: None,
            playing_track: None,
            tracks,
            component_data,
        }
    }
}
