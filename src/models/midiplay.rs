use crate::models::build_context::BuildContext;
use crate::models::input::input_manager::InputManager;
use crate::models::midi::peripheral::MidiPeripheral;
use crate::models::midi::playback::MidiPlayback;
use crate::models::midi::to_sheet::MidiSheetFromFile;
use crate::models::pausable::Pausable;
use crate::models::quality_manager::QualityManager;
use crate::models::sheet::from::SheetFromFile;
use crate::models::track_manager::TrackManager;
use crate::models::user_settings::UserSettings;
use crate::models::window_context::WindowContext;

use super::game_mode::{GameMode, self};
use super::input::input::MidiPlayInput;
use super::render_util::RenderUtil;
use super::user_interface::UserInterface;
// use crate::models::user_interface::UserInterface;

pub struct MidiPlay {
    pub game_mode: GameMode,
    pub user_settings: UserSettings,
    pub winctx: WindowContext,
    pub track: TrackManager,
    pub input: InputManager,
    pub quality: QualityManager,
    pub playback: MidiPlayback,
    pub user_interface: UserInterface,
    pub pause: bool,
    pub inputs: Vec<MidiPlayInput>,
}

impl MidiPlay {
    pub fn new(bctx: BuildContext) -> Self {
        let midi_parse = MidiSheetFromFile::new();
        let box_parse: Box<dyn SheetFromFile> = Box::new(midi_parse);
        let mut track = TrackManager::new(None, box_parse, bctx.clone());
        let mut playback = MidiPlayback::new(Some("<Main Playback>".into()));
        let input_peripheral = MidiPeripheral::from(&mut playback).name("<Input>".into());
        let play_peripheral = MidiPeripheral::from(&mut playback).name("<Music>".into());
        let input = InputManager::new(Some(input_peripheral));
        let quality = QualityManager::new();

        track
            .set_track(None, RenderUtil::new(&bctx.winctx), play_peripheral)
            .expect("Failed to set track");

        // let default_track = String::from("mc_sweden.mid");
        // let default_track = String::from("do_re_.mid");
        // let game_mode = GameMode::default();
        // let game_mode = GameMode::Play("mc_sweden.mid".into());
        let game_mode = GameMode::Menu;
        let game_mode = GameMode::Library;
        //// DISABLING FOR TEST WITH TRACK LIBRARY
        // let default_track = &track_library.tracks[0].filepath;
        // let successful = track.set_track(
        //     Some(default_track.clone()),
        //     RenderUtil::new(&buildctx.winctx),
        //     play_peripheral,
        // );
        // quality.set_track(successful.as_ref().unwrap().clone());
        // match successful {
        //     Ok(track_window_context) => {
        //         buildctx.winctx.trackwinctx = track_window_context;
        //         info!("Track Context Loaded");
        //     }
        //     Err(_e) => {
        //         warn!("Error loading track");
        //     }
        // }
        let user_settings = UserSettings::new();
        let user_interface = UserInterface::new(bctx.clone());
        let inputs = Vec::new();

        let mut newm = MidiPlay {
            game_mode,
            track,
            input,
            quality,
            winctx: bctx.winctx,
            playback,
            pause: true,
            user_settings,
            inputs,
            user_interface,
        };

        newm.resume();
        newm
    }
}
