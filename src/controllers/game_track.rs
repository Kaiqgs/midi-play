use sha2::{Digest, Sha256};

use crate::models::game_track::GameTrack;
use crate::models::sheet::SheetTrack;
impl GameTrack {
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
}
