use crate::models::new_game_track::NewGameTrack;
use ini::Ini;
use log::warn;

#[derive(Default)]
pub struct GameTrackMetadata {
    pub new: NewGameTrack,
}

pub const KEYS: [[&str; 7]; 2] = [
    [
        "sheet_track", // key;
        "name",
        "created_at",
        "hash",
        "duration",
        "difficulty",
        "_",
    ],
    [
        "game_track", // key;
        "name",
        "artist",
        "created_at",
        "hash",
        // Alternatively, create another group with key = None
        // and put these in there
        //  this would be good to do if we start using global properties
        //  and this would isolate GameTrackInformation logic
        "cover_filename",
        "track_filename",
    ],
];

impl GameTrackMetadata {
    // new with ini
    pub fn new(new: NewGameTrack) -> Self {
        GameTrackMetadata { new }
    }

    pub fn from_ini(ini: Ini) -> Self {
        let mut game_track_metadata = Self::default();
        warn!("from_ini: {:?}", ini);
        for (sec, prop) in ini.iter() {
            //TODO: undo this if we start using global properties
            if sec.is_none() {
                continue;
            }
            let sec = sec.unwrap();
            for key in KEYS {
                if sec != key[0] {
                    continue;
                }
                for ini_prop in key {
                    if !prop.contains_key(ini_prop) {
                        continue;
                    }
                    let full_prop = format!("{}.{}", sec, ini_prop);
                    // let full_prop
                    let prop_value = prop.get(ini_prop);
                    if prop_value.is_none() {
                        continue;
                    }
                    let prop_value = prop_value.unwrap();
                    game_track_metadata
                        .new
                        .set_key(full_prop.as_str(), prop_value.to_string());
                }
            }
        }
        Self::new(game_track_metadata.new)
    }
}
