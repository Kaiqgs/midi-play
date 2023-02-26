use std::time::Duration;

use ggez::event::EventHandler;
use ggez::graphics::{DrawParam, Mesh, MeshBuilder};
use ggez::graphics::Canvas;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::winit::event::VirtualKeyCode;

use crate::components::component::{MutComponentObject, WindowContext};
use crate::components::component_render;
use crate::components::drawing::{self, DrawResult, DrawingReference, RetrieveDrawing};
use crate::components::sheet::sheet_component_const::Zindex;
use crate::components::{
    component::{BuildContext, Component, ComponentObject},
    draw_util::DrawUtil,
    drawing::Drawing,
    pallete,
    sheet::{sheet_component_const, staff, staff_system::StaffSystemComponentData, track},
};
use crate::models::input::input::MidiPlayInput;
use crate::models::input::input_manager::InputManager;
use crate::models::midi::peripheral::MidiPeripheral;
use crate::models::midi::playback::MidiPlayback;
use crate::models::midi::to_sheet::MidiSheetFromFile;
use crate::models::render_util::RenderUtil;
use crate::models::sheet::from::SheetFromFile;
use crate::models::{
    self,
    midi::to_sheet::MidiSheetTransformer,
    note::Note,
    pausable::Pausable,
    sheet::{staff::Staff, staff_system::StaffSystem},
    track_manager::TrackManager,
};
use ggez::mint::Point2;
use ggez::{Context, GameError, GameResult};
use log::{debug, info, trace, warn};

pub struct MidiPlay {
    winctx: WindowContext,
    track: TrackManager,
    input: InputManager,
    playback: MidiPlayback,
    pause: bool,
}

impl MidiPlay {
    pub fn new(mut buildctx: BuildContext, filepath: Option<String>) -> Self {
        let midi_parse = MidiSheetFromFile::new();
        let box_parse: Box<dyn SheetFromFile> = Box::new(midi_parse);
        let mut track = TrackManager::new(filepath, box_parse, buildctx.clone());

        let playback = MidiPlayback::new(Some("<Main Playback>".into()));
        let input_peripheral = MidiPeripheral::from(&playback).name("<Input>".into());
        let play_peripheral = MidiPeripheral::from(&playback).name("<Music>".into());
        let input = InputManager::new(Some(input_peripheral));

        let successful = track.set_track(
            // Some(String::from("mc_sweden.mid")),
            // Some(String::from("mc_sweden.mid")),
            Some(String::from("do_re_.mid")),
            RenderUtil::new(&buildctx.winctx, Duration::from_millis(0)),
            play_peripheral,
        );

        match successful {
            Ok(track_window_context) => {
                buildctx.winctx.track = track_window_context;
                info!("Track Context Loaded");
            }
            Err(e) => {
                warn!("Error loading track");
            }
        }

        let mut newm = MidiPlay {
            track,
            input,
            winctx: buildctx.winctx,
            playback,
            pause: true,
        };

        newm.resume();

        newm
    }

    pub async fn pick_track(&mut self, filepath: &str) -> bool {
        unimplemented!();
    }

    pub async fn quit(&mut self) -> bool {
        unimplemented!();
    }

    pub fn draw_component(
        &self,
        component: &ComponentObject,
        render_util: RenderUtil,
        gfx: &Context,
        screen: &mut Canvas,
    ) {
        match component.draw(render_util) {
            DrawResult::Draw(params) => match component.get_drawing() {
                RetrieveDrawing::Ok(drawing) => {
                    trace!("Drawing: {}", component.get_name());
                    component_render::render_drawing(drawing, params, screen, gfx, &self.winctx)
                }
                RetrieveDrawing::Err(_) => (),
            },
            DrawResult::Skip => (),
        }
    }

    pub fn handle_input(&mut self, input: MidiPlayInput) {
        info!("Input: {:?}", input);
        match input {
            MidiPlayInput::PauseStart(state_input) => {
                let pause = match state_input {
                    Some(state) => state,
                    None => !self.pause,
                };
                if pause {
                    self.pause();
                } else {
                    self.resume();
                }
            }
            MidiPlayInput::NoteKey(note) => {
                self.input.on_note(&note);
            }
        }
    }

    pub fn grab_context(&mut self, ctx: &mut Context) -> WindowContext {
        let width = ctx.gfx.window().inner_size().width;
        let height = ctx.gfx.window().inner_size().height;
        trace!("Grabbing Canvas width={} height={}", width, height);
        WindowContext::new(
            Point2 {
                x: width,
                y: height,
            },
            Some(self.winctx.track.clone()),
        )
    }
}

impl Pausable for MidiPlay {
    fn pause(&mut self) -> bool {
        let success = !self.pause;
        self.pause = true;
        success
    }

    fn resume(&mut self) -> bool {
        let success = self.pause;
        self.pause = false;
        success
    }
}

impl EventHandler for MidiPlay {
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) -> Result<(), GameError> {
        self.winctx.track.zoom = (self.winctx.track.zoom + _y as f64 * 1e-2).clamp(0.0, 1.0);
        Ok(())
    }

    fn on_error(
        &mut self,
        _ctx: &mut Context,
        _origin: ggez::event::ErrorOrigin,
        _e: GameError,
    ) -> bool {
        self.playback.close();
        debug!("Error! and handle playback close");
        false
    }
    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GameError> {
        self.playback.close();
        debug!("Quitting and handle playback close");
        Ok(false)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        if _repeated {
            return Ok(());
        }

        match self.input.keyboard.handle_keyboard_down(input) {
            Some(input) => self.handle_input(input),
            None => (),
        }

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        _input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), GameError> {
        info!("KeyUp: {:?}", _input);

        match self.input.keyboard.handle_keyboard_up(_input) {
            Some(input) => self.handle_input(input),
            None => (),
        }

        match _input.keycode {
            Some(keycode) => match keycode {
                VirtualKeyCode::Space => self.handle_input(MidiPlayInput::PauseStart(None)),
                _ => (),
            },
            None => (),
        }
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let winctx = self.grab_context(ctx);
        let delta = ctx.time.delta();
        debug!("Delta: {:?}", delta);
        if !self.pause {
            self.track.update(RenderUtil::new(&winctx, delta));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let winctx = self.grab_context(ctx);
        let delta = ctx.time.delta();
        debug!("Delta: {:?}", delta);
        let mut canvas = graphics::Canvas::from_frame(ctx, pallete::LIGHTER_LIGHT);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        let mut mb = MeshBuilder::new();
        for i in (0..winctx.size.y).rev() {
            if i % 2 == 0 {
                continue;
            }
            mb.line(
                &[
                    Point2::from([0.0, i as f32]),
                    Point2::from([winctx.size.x as f32 / 10.0, i as f32]),
                ],
                1.0,
                pallete::DARK,
            )?;
        }
        let meshdata = Mesh::from_data(ctx, mb.build());
        canvas.draw(&meshdata, DrawParam::new().z(Zindex::Debug.get()));

        let mut track_manager_obj: ComponentObject = &self.track;
        let mut input_manager_obj: ComponentObject = &self.input;
        let mut stack: Vec<ComponentObject> = vec![track_manager_obj, input_manager_obj];
        let mut counter = 0;
        while !stack.is_empty() {
            let mut comp: ComponentObject = stack.pop().expect("err negative index");
            let next: Vec<ComponentObject> = comp.next();
            if next.len() > 0 {
                stack.extend(next);
            }
            self.draw_component(&comp, RenderUtil::new(&winctx, delta), ctx, &mut canvas);
            counter += 1;
        }
        debug!("Updated[{}]", counter);
        canvas.finish(ctx)
    }
}
