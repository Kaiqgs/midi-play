use log::{debug, info};

use super::{
    midi::midi_const,
    sheet::sheet_const::{self, Accidentals},
};
use crate::{components::note, models::clock::Clock};
use core::hash::Hash;
use std::hash::Hasher;

pub struct Note {
    pub id: u32,   // index + octave * 12
    pub line: u32, // position in height;
    pub midi: u32, // midi note
    pub time: Clock,
    pub naturality: Accidentals,
    pub velocity: u32,
    pub on: Option<bool>,
}

impl Note {
    pub fn new(note: u32, index: u32) -> Self {
        Note::from_note(
            sheet_const::get_id(note, index),
            None,
            None,
            None,
            None,
            None,
        )
    }
    pub fn from_note(
        id: u32,
        opt_on: Option<bool>,
        opt_nat: Option<Accidentals>,
        opt_clock: Option<Clock>,
        opt_midi: Option<u32>,
        opt_velocity: Option<u32>,
    ) -> Self {
        Note {
            id,
            //TODO: opt_midi _or is unsafe // midi_offset is not linear with sheet_offset;
            midi: opt_midi.unwrap_or(id + sheet_const::MIDI_OFFSET),
            line: sheet_const::LAST_NOTE - id,
            time: opt_clock.unwrap_or(Clock { sec: 0.0, tick: 0 }),
            naturality: opt_nat.unwrap_or(Accidentals::Natural),
            velocity: opt_velocity.unwrap_or(if opt_on.unwrap_or(false) { 100 } else { 0 }),
            on: opt_on,
        }
    }

    pub fn from_range(note_start: u32, note_end: u32) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        for nid in note_start..note_end {
            result.push(Note::from_note(nid, None, None, None, None, None));
        }
        result
    }

    pub fn from_midi(key: midly::num::u7, vel: midly::num::u7, time: Clock, on: bool) -> Note {
        let intkey: u32 = key.as_int().into();

        let midi_shifted = intkey - sheet_const::MIDI_OFFSET;
        let size = midi_const::NOTES.len() as u32;
        let mut midi_natural = midi_shifted % size;
        info!("midi_natural: {}", midi_natural);
        let midi_octave = midi_shifted as f32 / size as f32;
        let naturality = if midi_const::ACCIDENTS.contains(&midi_natural) {
            //TODO: explicit preference for sharp over flat;
            debug!("yes, accidental");
            midi_natural -= 1;
            Accidentals::Sharp
        } else {
            Accidentals::Natural
        };

        midi_natural = midi_const::NATURALS
            .iter()
            .position(|&r| r == midi_natural)
            .unwrap() as u32;
        let id = sheet_const::get_id(midi_natural, midi_octave.ceil() as u32);
        let vel: u32 = vel.as_int().into();
        info!(
            "midi_natural: {} midi_octave: {}, naturality: {}, id: {}, vel: {}",
            midi_natural,
            midi_octave,
            naturality == Accidentals::Natural,
            id,
            vel
        );
        return Note::from_note(
            id,
            Some(on),
            Some(naturality),
            Some(time),
            Some(intkey),
            Some(vel),
        );
        //
        // let id = intkey - sheet_const::MIDI_OFFSET;
        // let is_on = on && vel > 0;
        // let is_off = !on && vel == 0;
        // Note {
        //     id,
        //     midi: intkey,
        //     line: sheet_const::LAST_NOTE - id,
        //     time,
        //     naturality,
        //     velocity: vel,
        //     on: Some(on),
        // }
    }
}

impl Clone for Note {
    fn clone(&self) -> Self {
        Note {
            id: self.id,
            line: self.line,
            midi: self.midi,
            time: self.time.clone(),
            naturality: self.naturality,
            velocity: self.velocity,
            on: self.on,
        }
    }
}

impl Eq for Note {}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Note {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
