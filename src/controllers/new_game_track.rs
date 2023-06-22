use std::time::Duration;

use crate::models::{
    game_track::{GameTrackInformation, SheetTrackInformation},
    new_game_track::NewGameTrack,
};

pub const EMPTY: Option<String> = None;

impl NewGameTrack {
    pub fn set_key(&mut self, key: &str, value: String) -> bool {
        self.game_track = Some(GameTrackInformation::default());
        self.sheet_track = Some(SheetTrackInformation::default());
        match key {
            // Sheet track info
            "sheet_track.name" => match &mut self.sheet_track {
                Some(sheet) => sheet.name = value,
                None => (),
            },
            "sheet_track.created_at" => match &mut self.sheet_track {
                Some(sheet) => sheet.created_at = value,
                None => (),
            },
            "sheet_track.hash" => match &mut self.sheet_track {
                Some(sheet) => sheet.hash = value,
                None => (),
            },
            "sheet_track.duration" => match &mut self.sheet_track {
                Some(sheet) => {
                    sheet.duration = match value.parse::<u64>() {
                        Ok(parsed) => Duration::from_secs(parsed),
                        Err(_) => return false,
                    }
                }
                None => (),
            },
            "sheet_track.difficulty" => match &mut self.sheet_track {
                Some(sheet) => {
                    sheet.difficulty = match value.parse::<u8>() {
                        Ok(parsed) => parsed,
                        Err(_) => return false,
                    }
                }
                None => (),
            },
            // Game track info
            "game_track.track_filename" => self.track_filename = Some(value),
            "game_track.cover_filename" => self.cover_filename = Some(value),
            "game_track.name" => match &mut self.game_track {
                Some(game) => game.name = value,
                None => (),
            },
            "game_track.author" => match &mut self.game_track {
                Some(game) => game.author = value,
                None => (),
            },
            "game_track.created_at" => match &mut self.game_track {
                Some(game) => game.created_at = value,
                None => (),
            },
            "game_track.hash" => match &mut self.game_track {
                Some(game) => game.hash = value,
                None => (),
            },
            _ => return false,
        }
        true
    }

    pub fn merge(&mut self, other: &Self) {
        self.cover_filename = if self.cover_filename.is_none() {
            other.cover_filename.clone()
        } else {
            self.cover_filename.clone()
        };
        self.track_filename = if self.track_filename.is_none() {
            other.track_filename.clone()
        } else {
            self.track_filename.clone()
        };
        self.track_filepath = if self.track_filepath.is_none() {
            other.track_filepath.clone()
        } else {
            self.track_filepath.clone()
        };
        self.cover_filepath = if self.cover_filepath.is_none() {
            other.cover_filepath.clone()
        } else {
            self.cover_filepath.clone()
        };
        // self.cover = match &self.cover{
        //     Some(y) => Some(y.clone()),
        //     None => other.cover,
        // };
        // self.track = if self.track.is_none() {
        //     other.track
        // } else {
        //     self.track
        // };
        self.track_type = if self.track_type.is_none() {
            other.track_type.clone()
        } else {
            self.track_type.clone()
        };

        // TODO: implement deep-merge for game_track & sheet_track
        // TODO: rename game_track to game_track_info
        // TODO: rename sheet_track to sheet_track_info

    }
}
