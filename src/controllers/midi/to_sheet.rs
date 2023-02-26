use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::components::component::BuildContext;
use crate::models::clock::Clock;
use crate::models::midi::timing::TimingInformation;
use crate::models::midi::to_sheet::{MidiSheetFromFile, MidiSheetTransformer};
use crate::models::note::Note;
use crate::models::sheet::from::SheetFromFile;
use crate::models::sheet::staff_system::StaffSystem;
use crate::models::sheet::{from::SheetTransformer, SheetTrack};
use log::{debug, trace, warn};
use midly::{
    num::{u15, u24, u7},
    Format, MetaMessage, MidiMessage, Smf, Timing, TrackEventKind,
};

impl MidiSheetTransformer {}

fn _iter_tracks() {}

fn _note_change(state: bool, notes: &mut Vec<Note>, notes_on: &mut HashSet<Note>, note: &Note) {
    // let mut final_note: Note;
    // if state { // State is NOTE_ON
    //    // if notes_on.contains(note)
    //    //  {
    //    //      final_note = note.clone();
    //    //      final_note.on = Some(false);
    //    //  }
    //    //  else{
    //    //      final_note = note.clone();
    //    //      notes_on.insert(note.clone());
    //    //  }
    //
    //     final_note = note.clone();
    // }
    // else{ // State is NOTE_OFF
    //     final_note = note.clone();
    // }
    notes.push(note.clone());
}

fn _parse_smf<'a>(smf: midly::Smf<'a>) -> SheetTrack {
    let head = smf.header;
    //tempo will escribe a (length, beat, quarter note) / seconds;
    let default_tempo = u24::new(120);
    let tick_per_second = u24::new(32);
    let mut tick_per_beat = u15::new(32);
    let mut timing_info = TimingInformation {
        tempo: default_tempo,
        ticks_per_beat: tick_per_beat,
        us_per_tick: 0.0,
        sec_per_tick: 0.0,
    };
    let mut timing_changes: HashMap<u32, TimingInformation> = HashMap::new();
    let fps = 0;
    let mut farthest_tick: Clock = Clock { tick: 0, sec: 0.0 };

    let mut track_notes: Vec<Vec<Note>> = vec![];
    let mut track_pairs: Vec<HashMap<usize, usize>> = vec![];
    let mut note_count = 0;
    let mut note_off_count = 0;
    let mut note_on_count = 0;

    match head.timing {
        Timing::Metrical(tpb) => tick_per_beat = tpb,
        Timing::Timecode(nfps, subf) => (),
    }
    match head.format {
        Format::SingleTrack => {
            debug!("singling: format 0");
        }
        Format::Parallel => {
            debug!("all at once this bitch: format 1")
        }
        Format::Sequential => {
            debug!("one after die oder: format 2")
        }
    };
    match head.timing {
        Timing::Metrical(ntpb) => {
            debug!("Tick per beat: {}", ntpb);
            let mut t_info = timing_info.clone();
            t_info.ticks_per_beat = ntpb;
            timing_info = t_info;
        }
        Timing::Timecode(fps, subframe) => {
            warn!("not tested");
            let ntpb = u15::new((1.0 / fps.as_f32() / (subframe as f32)) as u16);
            let mut t_info = timing_info.clone();
            t_info.ticks_per_beat = ntpb;
            timing_info = t_info;
        }
    };

    for (i, track) in smf.tracks.iter().enumerate() {
        debug!("track {} has {} events", i, track.len());
        // Tempo tracks beat duration in secods (beat/sec);
        // A beat is a 1/4 step;
        let mut bpm = u24::new(120);
        let (mut dd, mut nn, mut cc, mut bb) = (0, 0, 0, 0);
        let mut key_sign_val: i8 = 0;
        let mut key_sign_bool = false;
        let mut midi_port = u7::new(0);
        let mut notes: Vec<Note> = Vec::new();
        let mut notes_off: Vec<Note> = Vec::new();
        let mut time_tick: u32 = 0;
        let mut time_sec: f64 = 0.0;
        let mut note_pairer: HashMap<Note, usize> = HashMap::new();
        let mut note_pairs: HashMap<usize, usize> = HashMap::new();
        // let mut time_sig_changes: Vec<(u32, (u8, u8))> = Vec::new();

        for (event) in track {
            // ticks_per_quarter = <PPQ from the header>
            // µs_per_quarter = <Tempo in latest Set Tempo event>
            // µs_per_tick = µs_per_quarter / ticks_per_quarter
            // seconds_per_tick = µs_per_tick / 1.000.000
            // seconds = ticks * seconds_per_tick
            trace!("Current Event Delta: {}", event.delta);
            time_tick += event.delta.as_int();
            time_sec += timing_info.sec_per_tick * event.delta.as_int() as f64;
            match event.kind {
                TrackEventKind::Midi { channel, message } => match message {
                    MidiMessage::NoteOff { key, vel } => {
                        trace!(
                            "NoteOff: channel[{}] key[{}] vel[{}] @ tick={} sec={}",
                            channel,
                            key,
                            vel,
                            time_tick,
                            time_sec
                        );
                        let note = Note::from_midi(
                            key,
                            vel,
                            Clock {
                                tick: time_tick,
                                sec: time_sec,
                            },
                            false,
                        );
                        if (note_pairer.contains_key(&note)) {
                            match note_pairer.get(&note) {
                                Some(note_on) => {
                                    note_pairs.insert(note_on.clone(), notes.len());
                                    note_pairer.remove(&note);
                                }
                                None => todo!(),
                            }
                        }
                        notes.push(note);
                        note_off_count += 1;
                        // _note_change(false, &mut notes, &mut notes_on, &note);
                    }
                    MidiMessage::NoteOn { key, vel } => {
                        trace!(
                            "NoteOn: channel[{}] key[{}] vel[{}] @ tick={} sec={}",
                            channel,
                            key,
                            vel,
                            time_tick,
                            time_sec
                        );
                        let note = Note::from_midi(
                            key,
                            vel,
                            Clock {
                                tick: time_tick,
                                sec: time_sec,
                            },
                            true,
                        );
                        note_pairer.insert(note.clone(), notes.len());
                        notes.push(note);
                        note_on_count += 1;
                        // _note_change(true, &mut notes, &mut notes_on, &note);
                    }
                    MidiMessage::Aftertouch { key, vel } => (),
                    MidiMessage::Controller { controller, value } => (),
                    MidiMessage::ProgramChange { program } => (),
                    MidiMessage::ChannelAftertouch { vel } => (),
                    MidiMessage::PitchBend { bend } => (),
                },
                TrackEventKind::SysEx(d) => (),
                TrackEventKind::Escape(d) => (),
                TrackEventKind::Meta(meta) => match meta {
                    MetaMessage::EndOfTrack => break,
                    MetaMessage::TrackNumber(_) => (),
                    MetaMessage::Text(_) => (),
                    MetaMessage::Copyright(_) => (),
                    MetaMessage::TrackName(_) => (),
                    MetaMessage::InstrumentName(_) => (),
                    MetaMessage::Lyric(_) => (),
                    MetaMessage::Marker(_) => (),
                    MetaMessage::CuePoint(_) => (),
                    MetaMessage::ProgramName(_) => (),
                    MetaMessage::DeviceName(_) => (),
                    MetaMessage::MidiChannel(_) => (),
                    MetaMessage::MidiPort(port) => midi_port = port,
                    MetaMessage::Tempo(newtempo) => {
                        //Tempo is Quarter notes in microseconds
                        let us_per_tick = newtempo.as_int() as f64 / tick_per_beat.as_int() as f64;
                        let sec_per_tick = us_per_tick as f64 / 1_000_000.0;
                        debug!("Tempo changed: {} at {}", newtempo, time_tick);

                        let t_info = TimingInformation {
                            tempo: newtempo,
                            ticks_per_beat: tick_per_beat,
                            us_per_tick,
                            sec_per_tick,
                        };
                        timing_changes.insert(time_tick, t_info.clone());

                        timing_info = t_info;
                    }
                    MetaMessage::SmpteOffset(_) => (),
                    MetaMessage::TimeSignature(ndd, nnn, ncc, nbb) => {
                        debug!("TimeSignature changed: {} {} {} {}", ndd, nnn, ncc, nbb);
                        (dd, nn, cc, bb) = (ndd, nnn, ncc, nbb);
                    }
                    MetaMessage::KeySignature(sign, signb) => {
                        key_sign_val = sign;
                        key_sign_bool = signb;
                    }
                    MetaMessage::SequencerSpecific(_) => (),
                    MetaMessage::Unknown(_, _) => (),
                },
            }
        }
        farthest_tick = if time_tick > farthest_tick.tick {
            Clock {
                tick: time_tick,
                sec: time_sec,
            }
        } else {
            farthest_tick
        };
        note_count += notes.len();
        track_notes.push(notes);
        track_pairs.push(note_pairs);
        debug!(
            "Notes On[{}] Off[{}] Total[{}]",
            note_on_count, note_off_count, note_count
        );
    }
    debug!(
        "Loaded {}x notes for {}x tracks.",
        note_count,
        track_notes.len()
    );
    SheetTrack::new(
        StaffSystem::default(),
        None,
        track_notes,
        track_pairs,
        timing_changes,
        Some(farthest_tick),
    )
}

impl SheetTransformer for MidiSheetTransformer {
    fn convert(&self) -> SheetTrack {
        let parse_smf = midly::Smf::parse(&self.bytes);
        match parse_smf {
            Ok(smf) => _parse_smf(smf),
            Err(_) => unimplemented!(),
        }
    }
}

impl SheetFromFile for MidiSheetFromFile {
    fn parse(&mut self, filepath: String) -> SheetTrack {
        //TODO: make ./resources not hardcoded;
        let resource_path = "./resources/".to_owned() + &filepath;
        debug!("MidiSheetFrom: {}", resource_path);
        let path = Path::new(&resource_path);
        let bytes = fs::read(path).unwrap();
        self.bytes = bytes.to_vec();
        let smf = Smf::parse(&self.bytes).unwrap();
        _parse_smf(smf)
    }
}
