use std::io::{self, Read};
use std::path::Path;
use std::time::Duration;

use ggez::filesystem::Filesystem;
use ggez::graphics::Image;
use ggez::GameError;
use log::warn;
use sha2::{Digest, Sha256, Sha512};

use crate::models::build_context::BuildContext;
use crate::models::game_track::{GameTrack, SheetTrackInformation, EXTENSIONS};
use crate::models::midi::to_sheet::MidiSheetFromFile;
use crate::models::new_game_track::GameTrackType;
use crate::models::sheet::from::SheetFromFile;
use crate::models::sheet::SheetTrack;
impl GameTrack {
    pub fn compute_track_type(filepath: String) -> Result<GameTrackType, GameError> {
        let filepath = Path::new(&filepath);
        let extension = filepath.extension();

        if extension.is_none() {
            return Err(GameError::FilesystemError(format!(
                "File {} has no extension",
                filepath.display()
            )));
        }
        let extension = extension.unwrap().to_str();
        if extension.is_none() {
            return Err(GameError::FilesystemError(format!(
                "File {} has no extension",
                filepath.display()
            )));
        }
        let extension = extension.unwrap();

        if extension == EXTENSIONS[0] {
            return Ok(GameTrackType::Simple);
        }
        if !EXTENSIONS[1..].contains(&extension) {
            return Err(GameError::CustomError(format!(
                "File {} has an invalid extension: {}",
                filepath.display(),
                extension
            )));
        }
        Ok(GameTrackType::Complex)
    }
    pub fn compute_uuid(
        filepath: String,
        artist: String,
        dur: Duration,
        sheet_hash: String,
    ) -> String {
        let mut hasher = Sha512::new();
        //TODO: remove filepath
        hasher.update(filepath.as_bytes());
        hasher.update(artist.as_bytes());
        hasher.update(dur.as_nanos().to_be_bytes());
        hasher.update(sheet_hash.as_bytes());
        hasher
            .finalize()
            .to_vec()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect()
    }
    pub fn compute_duration(track: &SheetTrack) -> Duration {
        let duration_sec = track.last_track_time.sec;
        Duration::from_millis((duration_sec * 1000.0) as u64)
    }
    pub fn compute_sheet_hash(track: &SheetTrack) -> String {
        let mut hasher = Sha256::new();
        for track in track.track.iter() {
            for note in track {
                hasher.update(note.id.to_be_bytes());
                hasher.update(note.channel.to_be_bytes());
                hasher.update(note.time.tick.to_be_bytes());
            }
        }
        track.track_timing.iter().for_each(|x| {
            hasher.update(x.1.us_per_tick.to_be_bytes());
            hasher.update(x.1.sec_per_tick.to_be_bytes());
            hasher.update(x.1.tempo.as_int().to_be_bytes());
            hasher.update(x.1.ticks_per_beat.as_int().to_be_bytes());
        });
        hasher
            .finalize()
            .to_vec()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect()
    }

    pub fn compute_difficulty(track: &SheetTrack) -> u8 {
        // let max_difficulty = sheet_const::LAST_NOTE - sheet_const::FIRST_NOTE;
        let mut difficulty = 0.0;
        for track in track.track.iter() {
            for note in track {
                if note.on.unwrap_or(false) {
                    difficulty += 1.0;
                }
            }
        }
        difficulty = difficulty / track.last_track_time.sec;

        //scale with sigmoid
        difficulty = 1.0 / (1.0 + (-difficulty).exp());
        (difficulty * 255.0) as u8
    }

    pub fn load_track(&mut self, fs: &Filesystem) -> Result<SheetTrack, GameError> {
        let mut parser = MidiSheetFromFile::new();
        let filepath = Path::new(&self.filepath);
        let mut track_information = SheetTrackInformation::default();
        if !fs.exists(filepath) {
            return Err(GameError::FilesystemError("File not found".into()));
        }
        if self.track_type.is_mid() {
            let sheet_track = parser.parse(&filepath, &fs)?;
            let track_information = SheetTrackInformation::default();
            self.track_information = Some(track_information);
            return Ok(sheet_track);
        }
        let mut file = fs.open(filepath)?;
        warn!("Loading track reference: {}", self.track_reference);
        let filesearch = Self::find_in_zip_bytes(&mut file, self.track_reference.as_str())?;
        if filesearch.is_none() {
            return Err(GameError::FilesystemError("Track not found".into()));
        }
        let file_searchresult = filesearch.unwrap();

        let sheet_track = parser.parse_bytes(file_searchresult.as_ref())?;

        let latest_sheet_hash = Self::compute_sheet_hash(&sheet_track);
        let latest_difficulty = Self::compute_difficulty(&sheet_track);
        let latest_duration = Self::compute_duration(&sheet_track);
        //TODO: compare
        track_information.hash = latest_sheet_hash;
        track_information.difficulty = latest_difficulty;
        track_information.duration = latest_duration;
        self.track_information = Some(track_information);
        Ok(sheet_track)
    }

    pub fn load_cover(&mut self, bctx: &BuildContext) -> Image {
        let ctx = bctx.ctx.unwrap();
        if self.track_type.is_mid() {
            return bctx.winctx.default_cover.clone().unwrap();
        }

        let filepath = Path::new(&self.filepath);
        if !ctx.fs.exists(filepath) {
            return bctx.winctx.default_cover.clone().unwrap();
        }
        let file = ctx.fs.open(filepath);
        if file.is_err() {
            return bctx.winctx.default_cover.clone().unwrap();
        }
        return bctx.winctx.default_cover.clone().unwrap();
        //Unreachable
        // let file = file.unwrap();
        // let filesearch = Self::find_in_zip(&mut file, self.cover_reference.as_str());
        // if filesearch.is_err() {
        //     warn!("Cover not found in zip file");
        //     return bctx.winctx.default_cover.clone().unwrap();
        // }
        // let filesearch = filesearch.unwrap();
        // if filesearch.is_none() {
        //     warn!("Cover not found in zip file");
        //     return bctx.winctx.default_cover.clone().unwrap();
        // }
        // let cover_file = filesearch.unwrap();
        // let mut cover_buffer: Vec<u8> = vec![];
        // let cover_bytes_count = cover_file.read_to_end(&mut cover_buffer);
        //
        //
        // let cover = Image::from_bytes(ctx, cover_buffer.as_ref());
        // if cover.is_err() {
        //     warn!("Cover not found in zip file");
        //     return bctx.winctx.default_cover.clone().unwrap();
        // }
        // let cover = cover.unwrap();
        // cover
    }

    pub fn load_cover_zip<R: Read + io::Seek>(
        file: &mut R,
        name: &str,
        bctx: &BuildContext,
    ) -> Result<Image, GameError> {
        if bctx.ctx.is_none() {
            return Err(GameError::CustomError("Context not found".to_string()));
        }
        let ctx = bctx.ctx.unwrap();
        warn!("Load Cover Zip, Name: {}", name);
        let content = Self::find_in_zip_bytes(file, name)?;
        if content.is_none() {
            return Err(GameError::CustomError("Cover not found".to_string()));
        }
        let content = content.unwrap();
        Ok(Image::from_bytes(ctx, content.as_ref())?)
    }

    pub fn find_in_zip<R: Read + io::Seek>(
        file: &mut R,
        name: &str,
    ) -> Result<Option<String>, GameError> {
        let mut zip = zip::ZipArchive::new(file)?;
        let mut content = String::new();
        let mut file = zip.by_name(name)?;
        let bytes = file.read_to_string(&mut content)?;
        warn!("Bytes read: {}, of file: {}", bytes, name);
        return Ok(Some(content));
    }

    pub fn find_in_zip_bytes<R: Read + io::Seek>(
        file: &mut R,
        name: &str,
    ) -> Result<Option<Box<[u8]>>, GameError> {
        let mut zip = zip::ZipArchive::new(file)?;
        let mut file = zip.by_name(name)?;
        let mut buffer: Vec<u8> = vec![];
        let bytes_count = file.read_to_end(&mut buffer)?;
        if bytes_count == 0 {
            return Err(GameError::CustomError("Could not read".to_string()));
        }
        let bytes = buffer.into_boxed_slice();
        warn!("File found: {}", file.name());
        // warn!("Bytes read: {}, of file: {}", bytes, name);
        return Ok(Some(bytes));
    }

    // pub fn from_track(
    //     filepath: String,
    //     cover_filepath: Option<String>,
    //     artist: Option<String>,
    //     track: SheetTrack,
    //     bctx: BuildContext,
    // ) -> Self {
    //     let duration_sec = track.last_track_time.sec;
    //     let sheet_hash = Self::compute_sheet_hash(&track);
    //     let duration = Duration::from_millis((duration_sec * 1000.0) as u64);
    //     let difficulty = Self::compute_difficulty(&track);
    //     Self::new(
    //         NewGameTrack::Complex(NewComplexGameTrack {
    //             filepath: Some(filepath.clone()),
    //             track_filepath: Some(filepath.clone()),
    //             cover_filename: cover_filepath.clone(),
    //             cover_filepath,
    //
    //             track_filename: Some(filepath),
    //
    //             uuid: None, //TODO: use Some(uuid) to compare and find changes in file version;
    //             artist,
    //
    //             track: Some(track),
    //             cover: None,
    //             sheet_hash: Some(sheet_hash),
    //             duration: Some(duration),
    //             difficulty: Some(difficulty),
    //         }),
    //         Some(bctx),
    //     )
    // }
}
