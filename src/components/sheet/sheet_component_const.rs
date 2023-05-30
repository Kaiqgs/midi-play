pub(crate) const SCALE: f32 = 2.69;
// pub(crate) const SCALEFF: f64 = SCALE as f64;
pub const SCALEF: f32 = 4.0;
pub(crate) const YOFFSET: u32 = 0;
pub(crate) const NOTE_HEIGHT: u32 = 3;
pub(crate) const TRIGGER_OFFSET_RATIO: f64 = 0.2;
pub(crate) const NOTE_OFFSET_RATIO: f64 = TRIGGER_OFFSET_RATIO + 0.1;
pub(crate) const TRIGER_NOTE_DIFF_RATIO: f64 = NOTE_OFFSET_RATIO - TRIGGER_OFFSET_RATIO;
pub(crate) const _TRIGGER_WIDTH: u32 = 1;
pub(crate) const TRIGGER_RANGE: f32 = 0.5;


pub enum ThreeWideKey {
    UpDown,
    Up,
    Down,
}
pub(crate) const THREE_WIDE_PROGRESSION: [ThreeWideKey; 7] = [
    // ThreeWideKey::Down, // B note
    // ThreeWideKey::UpDown,
    // ThreeWideKey::UpDown,
    // ThreeWideKey::Up,
    // ThreeWideKey::Down,
    // ThreeWideKey::UpDown,
    // ThreeWideKey::Up, // C note
    // ThreeWideKey::Up, // C note
    // ThreeWideKey::UpDown,
    // ThreeWideKey::Down,
    // ThreeWideKey::Up,
    // ThreeWideKey::UpDown,
    // ThreeWideKey::UpDown,
    // ThreeWideKey::Down, // B note
    ThreeWideKey::UpDown,
    ThreeWideKey::Down, // B note
    ThreeWideKey::Up,   // C note
    ThreeWideKey::UpDown,
    ThreeWideKey::Down,
    ThreeWideKey::Up,
    ThreeWideKey::UpDown,
];

pub enum Zindex {
    Debug,
    Background,
    Track,
    VirtualPiano,
    Note,
    Trigger,
    GameTrack,
    TrackLibrary,
}

impl Zindex {
    pub fn get(&self) -> i32 {
        self.clone() as i32
    }
}
impl Clone for Zindex {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for Zindex {}
