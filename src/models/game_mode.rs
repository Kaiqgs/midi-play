use std::ops::Range;

use super::{bit_mode::{BitMask, UintGet}, game_track::{GameTrack, EMPTY_TRACK}};

pub const EMPTY_PLAY: GameMode = GameMode::Play(EMPTY_TRACK);
pub const EMPTY_PRACTICE: GameMode = GameMode::Practice(EMPTY_TRACK, None);
pub const EMPTY_REPLAY: GameMode = GameMode::Replay(EMPTY_TRACK, String::new());

pub const NOTES_MASK: BitMask = BitMask::default()
    .set_bit(0, true)
    .set_bit(1, true)
    .set_bit(2, true);

#[derive(PartialEq, Eq, Default, Clone)]
pub enum GameMode {
    Play(GameTrack),
    Practice(GameTrack, Option<Range<usize>>),
    Replay(GameTrack, String),
    Library,
    #[default]
    Menu,
}

impl UintGet for GameMode {
    fn get(&self) -> u64 {
        match self {
            GameMode::Play(_) => 0,
            GameMode::Practice(_, _) => 1,
            GameMode::Replay(_, _) => 2,
            GameMode::Library => 3,
            GameMode::Menu => 4,
        }
    }
}
