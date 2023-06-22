use crate::components::game_track::CoverComponent;
use crate::components::game_track::GameTrackComponentData;
use crate::models::build_context::BuildContext;
use crate::models::game_track_metadata::GameTrackMetadata;
use std::{path::Path, time::Duration};

use ggez::GameError;
use log::warn;

use super::config;
use super::new_game_track::{GameTrackType, NewGameTrack};

pub const EXTENSIONS: [&str; 3] = ["mid", "mtrk", "midtrk"];

#[derive(Clone, Default)]
pub struct GameTrackInformation {
    pub name: String,
    pub author: String,
    pub created_at: String,
    pub hash: String,
}

#[derive(Clone, Default)]
pub struct SheetTrackInformation {
    pub name: String,
    pub artist: String,
    pub created_at: String,
    pub hash: String,
    pub duration: Duration,
    pub difficulty: u8,
}

#[derive(Clone, Default)]
pub struct GameTrack {
    // Metadata:
    pub uuid: String,
    pub filepath: String,
    pub track_reference: String,
    pub cover_reference: String,
    pub track_type: GameTrackType,
    pub information: Option<GameTrackInformation>,
    pub track_information: Option<SheetTrackInformation>,
    // pub metadata: GameTrackMetadata,
    // pub sheet_track: Option<SheetTrack>,
    pub component_data: GameTrackComponentData,
}

impl GameTrack {
    pub fn new(
        track: NewGameTrack,
        bctx: BuildContext,
        // filepath: String,
        // cover_filepath: Option<String>,
        // artist: Option<String>,
        // track: Option<SheetTrack>,
        // duration: Option<Duration>,
        // difficulty: Option<u8>,
        // sheet_hash: Option<String>,
        // bctx: Option<BuildContext>,
    ) -> Self {
        match Self::from_new(track, bctx) {
            Ok(track) => track,
            Err(err) => {
                warn!("Error creating game track: {}", err);
                Self::default()
            }
        }
    }

    pub fn from_new(mut complex: NewGameTrack, bctx: BuildContext) -> Result<GameTrack, GameError> {
        if bctx.ctx.is_none() {
            return Err(GameError::CustomError("No context provided".into()));
        }
        let ctx = bctx.ctx.unwrap();
        let filepath = complex.filepath.clone();
        let _fp = filepath.clone();
        let filepath_path = Path::new(&_fp);
        let track_type = complex.track_type.clone();
        let track_type = track_type.unwrap_or(Self::compute_track_type(filepath.clone())?);

        if !ctx.fs.exists(filepath_path) {
            return Err(GameError::FilesystemError(format!(
                "File {} does not exist",
                filepath_path.display()
            )));
        }
        let stem = filepath_path.file_stem();
        if stem.is_none() {
            return Err(GameError::FilesystemError(format!(
                "File {} has no stem",
                filepath_path.display()
            )));
        }
        let stem = stem.unwrap().to_str();
        if stem.is_none() {
            return Err(GameError::FilesystemError(format!(
                "File {} has no stem",
                filepath_path.display()
            )));
        }
        let stem = stem.unwrap().to_string();
        let mut sheet_track_artist: String = config::DEFAULT_ARTIST_ALIAS.into();
        let _game_track_name = stem.clone();

        if track_type.is_mid() {
            complex.track_filepath = Some(filepath.clone());
        } else if track_type.is_track() {
            //load metadata;
            let mut file = ctx.fs.open(filepath_path)?;
            let metadata_filesearch =
                Self::find_in_zip(&mut file, config::DEFAULT_METADATA_FILENAME)?;
            if metadata_filesearch.is_some() {
                let metadata_file = metadata_filesearch.unwrap();
                let ini = ini::Ini::load_from_str_noescape(metadata_file.as_str());
                if ini.is_err() {
                    return Err(GameError::FilesystemError(
                        "Could not read INI metadata".into(),
                    ));
                }
                let ini = ini.unwrap();
                let metadata = GameTrackMetadata::from_ini(ini);
                if metadata.new.sheet_track.is_some() {
                    let sheet_artist_name = &metadata.new.sheet_track.as_ref().unwrap().artist;
                    sheet_track_artist.clear();
                    sheet_track_artist.push_str(&sheet_artist_name);
                }
                let cover_filename = metadata.new.cover_filename.clone();
                let cover_filename =
                    cover_filename.unwrap_or(config::DEFAULT_COVER_FILENAME.into());

                // track_information = Some(metadata.new.sheet_track);
                // information = Some(metadata.new.game_track);
                complex.merge(&metadata.new);
                // information = Some(metadata.new.game_track);
                match Self::load_cover_zip(&mut file, &cover_filename, &bctx) {
                    Ok(cover) => complex.cover = Some(cover),
                    Err(err) => warn!("Could not load cover due to error: {}", err),
                };
            }
        }

        let cover_reference = complex.cover_filename.clone().unwrap_or(
            complex
                .cover_filename
                .unwrap_or(config::DEFAULT_COVER_FILEPATH.into()),
        );
        let track_reference = complex.track_filepath.clone().unwrap_or(
            complex
                .track_filename
                .unwrap_or(config::DEFAULT_TRACK_FILENAME.into()),
        );
        let duration = match &complex.sheet_track {
            Some(track) => track.duration,
            None => Duration::from_secs(0),
        };

        let cover_component = match complex.cover {
            Some(cover) => CoverComponent::Object(cover),
            //todo: use bctx.default_cover
            None => CoverComponent::Reference(bctx, cover_reference.clone()),
        };
        let component_data = GameTrackComponentData::new(cover_component);

        let uuid = Self::compute_uuid(
            filepath.clone(),
            sheet_track_artist.clone(),
            duration,
            String::new(),
        );


        Ok(GameTrack {
            uuid,
            filepath,
            information: complex.game_track,
            track_information: complex.sheet_track,
            track_reference,
            cover_reference,
            track_type,
            component_data,
        })
    }
}

impl PartialEq for GameTrack {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for GameTrack {}
