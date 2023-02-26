use crate::models::{
    midi::{peripheral::MidiPeripheral, playback::MidiPlayback},
    sheet::virtual_piano::VirtualPiano,
};

use super::keyboard_input::KeyboardInputSource;

pub struct InputManager {
    pub keyboard: KeyboardInputSource,
    // midi: MidiInputSource,
    pub virtual_piano: VirtualPiano,
    pub playback: MidiPeripheral,
}

impl InputManager {
    pub fn new(playback: Option<MidiPeripheral>) -> Self {
        let mut default = MidiPeripheral::new("<Input Peripheral>".into());
        InputManager {
            playback: playback.unwrap_or(default),
            keyboard: KeyboardInputSource::new(),
            virtual_piano: VirtualPiano::new(),
        }
    }
}
