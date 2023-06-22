use crate::models::{
    input::{
        input::MidiPlayInput,
        keyboard_input::{
            KeyboardInputSource, NATURAL_KEY_CODE, NUMERIC_KEY_CODE, NUMERIC_PAD_KEY_CODE,
        },
    },
    midi::midi_const,
    note::Note,
    sheet::sheet_const::{self, Accidentals}, game_mode::GameMode, game_track::GameTrack, 
};
use ggez::input::keyboard::KeyCode;
use log::info;

impl KeyboardInputSource {
    fn get_accidental(&self) -> Accidentals {
        if self.shift_down {
            Accidentals::Sharp
        } else if self.ctrl_down {
            Accidentals::Flat
        } else {
            Accidentals::Natural
        }
    }
    pub fn handle_keyboard_up(
        &mut self,
        input: ggez::input::keyboard::KeyInput,
    ) -> Option<MidiPlayInput> {
        if input.keycode.is_none() {
            return None;
        }
        let key_code = input.keycode.expect("Keycode is none");
        if self.handle_default_numeric(key_code, &NUMERIC_KEY_CODE) {
            return None;
        }
        if self.handle_default_numeric(key_code, &NUMERIC_PAD_KEY_CODE) {
            return None;
        }
        match key_code {
            KeyCode::LShift => {
                self.shift_down = false;
            }
            KeyCode::RShift => {
                self.shift_down = false;
            }
            KeyCode::LControl => {
                self.ctrl_down = false;
            }
            KeyCode::RControl => {
                self.ctrl_down = false;
            }
            KeyCode::Space => {
                return Some(MidiPlayInput::PauseStart(None));
            },
            KeyCode::Escape => {
                return Some(MidiPlayInput::Restart);
            },
            KeyCode::Home => {
                return Some(MidiPlayInput::ModeChange(GameMode::Menu));
            },
            KeyCode::Insert => {
                return Some(MidiPlayInput::ModeChange(GameMode::Library));
            },
            KeyCode::F5 => {
                return Some(MidiPlayInput::ModeChange(GameMode::Play(GameTrack::default())));
            },
            KeyCode::Right => {
                return Some(MidiPlayInput::NextOption);
            },
            KeyCode::Left => {
                return Some(MidiPlayInput::PreviousOption);
            },
            KeyCode::Down => {
                return Some(MidiPlayInput::NextOption);
            },
            KeyCode::Up => {
                return Some(MidiPlayInput::PreviousOption);
            },
            KeyCode::Return => {
                return Some(MidiPlayInput::SelectOption);
            },
            KeyCode::Back => {
                return Some(MidiPlayInput::BackOption);
            },
            _other => {
                return self.handle_key_change(key_code, false);
            }
        };
        info!("Shift down: {}", self.shift_down);
        None
    }
    fn handle_default_numeric(&mut self, key_code: KeyCode, numeric: &[KeyCode]) -> bool {
        if numeric.contains(&key_code) {
            if self.shift_down {
                self.numeric_shift = NUMERIC_KEY_CODE
                    .iter()
                    .position(|&r| r == key_code)
                    .unwrap() as u8;
                return true;
            }
        }
        false
    }

    fn handle_key_change(&self, key_code: KeyCode, on: bool) -> Option<MidiPlayInput> {
        if !NATURAL_KEY_CODE.contains(&key_code) {
            return None;
        }
        let natural_id = NATURAL_KEY_CODE
            .iter()
            .position(|&r| r == key_code)
            .unwrap();

        let accidental = self.get_accidental();
        let natural = natural_id as u32;
        let octave = self.numeric_shift.into();

        let note_id = sheet_const::get_id(natural, octave);
        let is_accidental = accidental != Accidentals::Natural;
        let midi_id = midi_const::get_id(natural, octave, is_accidental);
        match midi_id {
            Ok(midi_id) => {
                //TODO: add Clock
                let note = Note::from_note(
                    note_id,
                    Some(on),
                    Some(accidental.clone()),
                    None,
                    Some(midi_id),
                    None,
                );
                info!(
            "Key On{}: natural: {} octave: {} accidental_is_natural: {} note_id: {} midi_id: {}",
            on,
            natural,
            octave,
            accidental != Accidentals::Natural,
            note_id,
            midi_id
        );
                return Some(MidiPlayInput::NoteKey(note));
            }
            Err(_) => return None,
        }
    }

    pub fn handle_keyboard_down(
        &mut self,
        input: ggez::input::keyboard::KeyInput,
    ) -> Option<MidiPlayInput> {
        if input.keycode.is_none() {
            return None;
        }
        let key_code = input.keycode.expect("Keycode is none");
        match key_code {
            KeyCode::LShift => {
                self.shift_down = true;
            }
            KeyCode::RShift => {
                self.shift_down = true;
            }
            KeyCode::LControl => {
                self.ctrl_down = true;
            }
            KeyCode::RControl => {
                self.ctrl_down = true;
            }
            _other => {
                return self.handle_key_change(key_code, true);
            }
        };
        info!("Shift down: {}", self.shift_down);
        None
    }
}
