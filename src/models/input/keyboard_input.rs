use ggez::input::keyboard::KeyCode;

pub(crate) const NUMERIC_KEY_CODE: [KeyCode; 10] = [
    KeyCode::Key0,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
];

pub(crate) const NUMERIC_PAD_KEY_CODE: [KeyCode; 10] = [
    KeyCode::Numpad0,
    KeyCode::Numpad1,
    KeyCode::Numpad2,
    KeyCode::Numpad3,
    KeyCode::Numpad4,
    KeyCode::Numpad5,
    KeyCode::Numpad6,
    KeyCode::Numpad7,
    KeyCode::Numpad8,
    KeyCode::Numpad9,
];

pub(crate) const NATURAL_KEY_CODE: [KeyCode; 7] = [
    KeyCode::A,
    KeyCode::B,
    KeyCode::C,
    KeyCode::D,
    KeyCode::E,
    KeyCode::F,
    KeyCode::G,
];

pub struct KeyboardInputSource {
    pub shift_down: bool,
    pub ctrl_down: bool,
    pub numeric_shift: u8,
}

impl KeyboardInputSource {
    pub fn new() -> Self {
        KeyboardInputSource {
            shift_down: false,
            ctrl_down: false,
            numeric_shift: 4,
        }
    }
}
