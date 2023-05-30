use log::{debug, info};

use super::{
    midi::midi_const,
    sheet::sheet_const::{self, Accidentals},
};
use crate::models::clock::Clock;
use core::hash::Hash;
use std::{hash::Hasher, time::Duration};

#[derive(Debug)]
pub struct Note {
    pub id: u32,   // index + octave * 12
    pub line: u32, // position in height;
    pub midi: u32, // midi note
    pub time: Clock,
    pub naturality: Accidentals,
    pub velocity: u32,
    pub on: Option<bool>,
    pub channel: u8,
    pub trigger: Option<Duration>,
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
            channel: 0,
            trigger: None,
        }
    }

    pub fn eq_to_input(&self, input: &Note) -> bool {
        self.id == input.id
            && self.on == input.on
            && self.trigger.is_some() == input.trigger.is_some()
    }

    pub fn channel(&self, channel: u8) -> Self {
        let mut this = self.clone();
        this.channel = channel;
        this
    }

    pub fn trigger(&self, time: Duration) -> Self {
        let mut this = self.clone();
        this.trigger = Some(time);
        this
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
    }
}

impl Clone for Note {
    fn clone(&self) -> Self {
        Note {
            id: self.id,
            line: self.line,
            midi: self.midi,
            time: self.time.clone(),
            naturality: self.naturality.clone(),
            velocity: self.velocity,
            on: self.on,
            channel: self.channel,
            trigger: self.trigger.clone(),
        }
    }
}

impl Eq for Note {}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.channel == other.channel
    }
}

impl Hash for Note {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.channel.hash(state);
    }
}
