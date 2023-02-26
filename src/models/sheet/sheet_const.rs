use std::{
    fmt::{Display, Formatter},
    ops::Range,
};

const LEDGER: u32 = 2;

pub(crate) const A: u32 = 0;
pub(crate) const B: u32 = 1;
pub(crate) const C: u32 = 2;
pub(crate) const D: u32 = 3;
pub(crate) const E: u32 = 4;
pub(crate) const F: u32 = 5;
pub(crate) const G: u32 = 6;

pub(crate) const LINES: u32 = 5;
pub(crate) const SPACES: u32 = 4;
pub(crate) const LEDGERS: u32 = LEDGER * 2;
pub(crate) const STAFF_SIZE: u32 = LINES + SPACES;
pub(crate) const STAFF_LEDGERS_SIZE: u32 = STAFF_SIZE + LEDGERS;

pub(crate) const NATURAL_NOTES: u32 = 7;
pub(crate) const ACCIDENTAL_NOTES: u32 = 5;
pub(crate) const NOTES: u32 = NATURAL_NOTES + ACCIDENTAL_NOTES;

pub const fn get_id(note: u32, octave: u32) -> u32 {
    note + NATURAL_NOTES * octave
}

pub(crate) const MIDI_OFFSET: u32 = 21;
pub(crate) const LIMITING_INDEX: u32 = 8; //16;
pub(crate) const MIDDLE_C: u32 = get_id(C, 4);
pub(crate) const LAST_NOTE: u32 = get_id(G, LIMITING_INDEX);
pub(crate) const FIRST_NOTE: u32 = get_id(A, 0);

pub const fn compute_range(start_note: u32, end_note: u32) -> Range<u32> {
    let end = LAST_NOTE - end_note;
    let start = LAST_NOTE - start_note;
    start..end
}

pub enum Accidentals {
    Flat,
    Natural,
    Sharp,
}

impl Clone for Accidentals {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for Accidentals {}

impl Eq for Accidentals {}

impl PartialEq for Accidentals {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Display for Accidentals {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Accidentals::Flat => write!(f, "b"),
            Accidentals::Natural => write!(f, "n"),
            Accidentals::Sharp => write!(f, "#"),
        }
    }
}
