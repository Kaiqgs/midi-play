use super::game_track::GameTrack;

pub(crate) const _RECENT_MAX: usize = 10;

pub struct UserSettings {
    _recent: Vec<GameTrack>,
    mute_track: bool,
    mute_input: bool,
    volume: f32,
    input_volume: f32,
}

impl UserSettings {
    pub fn new() -> Self {
        Self {
            _recent: Vec::new(),
            mute_track: false,
            mute_input: false,
            volume: 1.0,
            input_volume: 1.0,
        }
    }
}
