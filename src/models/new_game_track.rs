use ggez::graphics::Image;

use crate::models::sheet::SheetTrack;

use super::game_track::{SheetTrackInformation, GameTrackInformation};

#[derive(Default)]
pub struct NewGameTrack {
    pub filepath: String,

    pub track_filepath: Option<String>,
    pub cover_filepath: Option<String>,

    pub track_filename: Option<String>,
    pub cover_filename: Option<String>,

    pub cover: Option<Image>,
    pub track: Option<SheetTrack>,

    pub track_type: Option<GameTrackType>,
    
    //items
    pub sheet_track: Option<SheetTrackInformation>,
    pub game_track: Option<GameTrackInformation>,
}

impl NewGameTrack {
    pub fn new(filepath: String, track_type: GameTrackType) -> Self {
        let mut new = NewGameTrack::default();
        new.track_type = Some(track_type.clone());
        match track_type {
            GameTrackType::Simple => {
                new.track_filepath = Some(filepath.clone());
                new.filepath = filepath;
            }
            GameTrackType::Complex => {
                new.filepath = filepath;
            }
        };
        new
    }
}

#[derive(Clone, Default)]
pub enum GameTrackType {
    #[default]
    Simple,
    Complex,
}

impl GameTrackType {
    pub fn is_mid(&self) -> bool {
        match self {
            GameTrackType::Simple => true,
            GameTrackType::Complex => false,
        }
    }

    pub fn is_track(&self) -> bool {
        match self {
            GameTrackType::Simple => false,
            GameTrackType::Complex => true,
        }
    }
}
