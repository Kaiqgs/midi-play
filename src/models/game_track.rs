use crate::models::build_context::BuildContext;
use crate::{components::game_track::GameTrackComponentData, models::sheet::SheetTrack};
use std::{path::Path, time::Duration};

use sha2::{Digest, Sha512};

use super::config;

#[derive(Clone, Default)]
pub struct GameTrack {
    // Metadata:
    pub name: String,
    pub filepath: String,
    pub cover_filepath: String,
    pub artist: String,
    pub duration: Duration,
    pub difficulty: u8,
    pub sheet_hash: String,
    pub uuid: String,
    pub component_data: Option<GameTrackComponentData>,
}

impl GameTrack {
    pub fn get_uuid(&self) -> String {
        let mut hasher = Sha512::new();
        hasher.update(self.filepath.as_bytes());
        // hasher.update(self.cover_filepath.as_bytes());
        hasher.update(self.artist.as_bytes());
        hasher.update(self.duration.as_nanos().to_be_bytes());
        hasher.update(self.difficulty.to_be_bytes());
        hasher.update(self.sheet_hash.as_bytes());
        hasher
            .finalize()
            .to_vec()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect()
    }
    pub fn new(
        filepath: String,
        cover_filepath: Option<String>,
        artist: Option<String>,
        track: Option<SheetTrack>,
        duration: Option<Duration>,
        difficulty: Option<u8>,
        sheet_hash: Option<String>,
        bctx: BuildContext,
    ) -> Self {
        match track {
            Some(track) => Self::from_track(filepath, cover_filepath, artist, track, bctx),
            None => {
                let empty = "".to_string();
                let cover_filepath =
                    cover_filepath.unwrap_or(config::DEFAULT_COVER_FILEPATH.into());
                let artist = artist.unwrap_or("Unknown Artist".to_string());
                let duration = duration.unwrap_or(Duration::from_secs(0));
                let difficulty = difficulty.unwrap_or(0);
                let sheet_hash = sheet_hash.unwrap_or(empty.clone());
                let name = Path::new(&filepath)
                    .file_stem()
                    .expect("Invalid stem")
                    .to_str()
                    .expect("Invalid string")
                    .to_string();

                // let name = Path::file_stem(Path::from(&filepath))
                //     .unwrap()
                //     .to_str()
                //     .unwrap()
                //     .to_string();))

                let mut obj = GameTrack {
                    name,
                    filepath,
                    cover_filepath: cover_filepath.clone(),
                    artist,
                    duration,
                    difficulty,
                    sheet_hash,
                    uuid: empty,
                    component_data: Some(GameTrackComponentData::new(bctx, cover_filepath.clone())),
                };
                obj.uuid = obj.get_uuid();
                obj
            }
        }
    }

    pub fn from_track(
        filepath: String,
        cover_filepath: Option<String>,
        artist: Option<String>,
        track: SheetTrack,
        bctx: BuildContext,
    ) -> Self {
        let duration_sec = track.last_track_time.sec;
        let sheet_hash = Self::compute_sheet_hash(&track);
        let duration = Duration::from_millis((duration_sec * 1000.0) as u64);
        let difficulty = Self::compute_difficulty(&track);
        Self::new(
            filepath,
            cover_filepath,
            artist,
            None,
            Some(duration),
            Some(difficulty),
            Some(sheet_hash),
            bctx,
        )
    }
}

impl PartialEq for GameTrack {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for GameTrack {}

pub const EMPTY_TRACK: GameTrack = GameTrack {
    name: String::new(),
    filepath: String::new(),
    cover_filepath: String::new(),
    artist: String::new(),
    duration: Duration::new(0, 0),
    difficulty: 0,
    sheet_hash: String::new(),
    uuid: String::new(),
    component_data: None,
};
