use std::ops::Range;

pub(crate) const LINES: i32 = 5;
pub(crate) const SPACES: i32 = 4;
pub(crate) const SIZE: i32 = LINES * SPACES;
pub(crate) const NOTES_IN_SCALE: i32 = 12;
pub(crate) const MIDDLE_C: i32 = NOTES_IN_SCALE * 4;
pub(crate) const DEF_KEY_COUNT: i32 = 88;
pub type Notes = Range<u32>;