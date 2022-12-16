use std::{char, collections::HashMap, ops::Range};

const LEDGER: u32 = 2;
pub type Note = u32;

pub(crate) const A: u32 = 0;
pub(crate) const A_: u32 = 1;
pub(crate) const B: u32 = 1;
pub(crate) const C: u32 = 2;
pub(crate) const C_: u32 = 3;
pub(crate) const D: u32 = 3;
pub(crate) const D_: u32 = 4;
pub(crate) const E: u32 = 4;
pub(crate) const F: u32 = 5;
pub(crate) const F_: u32 = 6;
pub(crate) const G: u32 = 6;
pub(crate) const G_: u32 = 0;

pub(crate) const LINES: u32 = 5;
pub(crate) const SPACES: u32 = 4;
pub(crate) const LEDGERS: u32 = LEDGER * 2;
pub(crate) const STAFF_SIZE: u32 = LINES + SPACES;
pub(crate) const STAFF_LEDGERS_SIZE: u32 = STAFF_SIZE + LEDGERS;

pub(crate) const NATURAL_NOTES: u32 = 7;
pub(crate) const ACCIDENTAL_NOTES: u32 = 5;
pub(crate) const NOTES: u32 = NATURAL_NOTES + ACCIDENTAL_NOTES;

pub const fn get_id(note: u32, index: u32, offset: u32) -> u32 {
    note + NATURAL_NOTES * index + offset
}

pub(crate) const MIDI_OFFSET: u32 = 21;
pub(crate) const LIMITING_INDEX: u32 = 8; //16;
pub(crate) const MIDDLE_C: u32 = get_id(C, 4, 0);
pub(crate) const LAST_NOTE: u32 = get_id(G, LIMITING_INDEX, 0);
pub(crate) const FIRST_NOTE: u32 = get_id(A, 0, 0);

pub const fn compute_range(start_note: u32, end_note: u32) -> Range<u32> {
    let end = LAST_NOTE - end_note;
    let start = LAST_NOTE - start_note;
    start..end
}
