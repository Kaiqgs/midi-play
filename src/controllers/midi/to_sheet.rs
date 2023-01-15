use crate::models::midi::to_sheet::MidiSheetTransformer;
use crate::models::note::Note;
use crate::models::sheet::{from::SheetTransformer, SheetTrack};
use log::debug;
use midly::{
    num::{u15, u24, u7},
    Format, MetaMessage, MidiMessage, Smf, Timing, TrackEventKind,
};

impl MidiSheetTransformer<'_> {}

impl SheetTransformer for MidiSheetTransformer<'_> {
    fn convert(&self) -> SheetTrack {
        if self.smf.is_none() {
            unimplemented!()
        }

        let smf = self.smf.unwrap();
        let head = smf.header;

        //tempo will escribe a (length, beat, quarter note) / seconds;
        let default_tempo = u24::new(120);
        let mut tick_per_beat = u15::new(32);
        let tick_per_second = u24::new(32);
        let fps = 0;
        match head.timing {
            Timing::Metrical(tpb) => tick_per_beat = tpb,
            Timing::Timecode(nfps, subf) => (),
        }

        match head.format {
            Format::SingleTrack => {
                debug!("singling");
            }
            Format::Parallel => {
                debug!("all at once this bitch")
            }
            Format::Sequential => {
                debug!("one after die oder")
            }
        };
        match head.timing {
            Timing::Metrical(ntpb) => {
                debug!("Tick per beat: {}", ntpb);
                tick_per_beat = ntpb;
            }
            Timing::Timecode(_, _) => {
                unimplemented!();
            }
        };
        let mut track_notes: Vec<Vec<Note>> = vec![];
        let mut note_count = 0;
        for (i, track) in smf.tracks.iter().enumerate() {
            debug!("track {} has {} events", i, track.len());
            // Tempo tracks beat duration in secods (beat/sec);
            // A beat is a 1/4 step;
            let mut tempo = u24::new(default_tempo.as_int() as u32);
            let mut bpm = u24::new(120);
            let (mut dd, mut nn, mut cc, mut bb) = (0, 0, 0, 0);
            let mut key_sign_val: i8 = 0;
            let mut key_sign_bool = false;
            let mut midi_port = u7::new(0);
            let mut notes: Vec<Note> = Vec::new();

            let mut time_tick: u32 = 0;

            for (event) in track {
                // debug!("{}", event.delta);
                // debug!("Tempo: {}", tempo);
                match event.kind {
                    TrackEventKind::Midi { channel, message } => match message {
                        MidiMessage::NoteOff { key, vel } => {
                            debug!("NoteOff:{} {} {}", channel, key, vel);
                        }
                        MidiMessage::NoteOn { key, vel } => {
                            debug!("NoteOn:{} {} {}", channel, key, vel);
                            let note = Note::from_midi(key, vel);
                            notes.push(note);
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
                            debug!("Tempo changed: {}", newtempo);
                            tempo = newtempo;
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
            note_count += notes.len();
            track_notes.push(notes);
        }
        debug!(
            "Loaded {}x notes for {}x tracks.",
            note_count,
            track_notes.len()
        );

        SheetTrack::new(None, None, track_notes)
    }
}
