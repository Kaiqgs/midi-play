use crate::components::component::MutComponentObject;
use crate::components::component_render;
use crate::components::drawing::{DrawResult, RetrieveDrawing};
use crate::components::{component::ComponentObject, pallete};
use crate::models::draw_state::DrawState;
use crate::models::game_mode::GameMode;
use crate::models::input::input::MidiPlayInput;
use crate::models::midiplay::MidiPlay;
use crate::models::pausable::Pausable;
use crate::models::render_util::RenderUtil;
use crate::models::restartable::Restartable;
use crate::models::window_context::WindowContext;
use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas};
use ggez::input::keyboard;
use ggez::mint::Point2;
use ggez::{Context, GameError, GameResult};
use log::{debug, info, trace};

impl Restartable for MidiPlay {
    fn restart(&mut self) -> Result<(), ()> {
        self.quality.restart()?;
        self.track.restart()?;
        Ok(())
    }
}
impl MidiPlay {
    pub fn pick_track(&mut self, reutil: RenderUtil, filepath: String) -> bool {
        // let winctx = self.grab_context(ctx)
        // let peripheral = MidiPeripheral::from(&mut self.playback).name("<Music>".into());
        let peripheral = self.track.sheet_track.component_data.playback.reuse();
        // self.track.sheet_track.component_data.playback.close();
        // peripheral.restart().expect("Failed to restart peripheral");
        let successful = self
            .track
            .set_track(Some(filepath), reutil, peripheral);
        self.quality.set_track(successful.as_ref().unwrap().clone());
        true
    }

    pub async fn quit(&mut self) -> bool {
        unimplemented!();
    }

    pub fn draw_component(
        &self,
        component: &ComponentObject,
        reutil: RenderUtil,
        gfx: &Context,
        canvas: &mut Canvas,
    ) -> bool {
        match component.draw(reutil.clone()) {
            DrawResult::Draw(params) => match component.get_drawing() {
                RetrieveDrawing::Ok(drawing) => {
                    let _name = component.get_name();
                    let game_mode = self.game_mode.clone();
                    if component.get_mask().check(&game_mode) {
                        trace!("Drawing: {}", component.get_name());
                        component_render::render_drawing(
                            drawing,
                            params,
                            canvas,
                            gfx,
                            &self.winctx,
                        );
                        true
                    } else {
                        false
                    }
                }
                RetrieveDrawing::Err(_) => false,
            },
            DrawResult::Skip => false,
        }
    }

    pub fn handle_inputs(&mut self, reutil: RenderUtil, ctx: &mut Context) {
        while !self.inputs.is_empty() {
            let input = self.inputs.pop();
            if input.is_some() {
                self.handle_input(input.unwrap(), reutil.clone(), ctx);
            }
        }
    }

    pub fn handle_input(&mut self, input: MidiPlayInput, reutil: RenderUtil, ctx: &mut Context) {
        info!("Input: {:?}", input);
        match input.clone() {
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
                self.input.on_note(&note, reutil.clone());
            }
            MidiPlayInput::Restart => self.restart().expect("Restart failed"),
            MidiPlayInput::ModeChange(mode) => {
                match &mode {
                    GameMode::Play(track) => {
                        self.pick_track(reutil, track.filepath.clone());
                    }
                    _ => (),
                };
                self.game_mode = mode;
            }
            MidiPlayInput::Quit => ctx.request_quit(),
            _ => (),
        };

        let stack: Vec<MutComponentObject> = vec![
            &mut self.track,
            &mut self.input,
            &mut self.quality,
            &mut self.user_interface,
        ];

        MidiPlay::iterate_components(self.game_mode.clone(), stack, |component| {
            component.handle_input(input.clone());
            // self.handle_input(input.clone(), reutil)
        });
    }

    pub fn grab_context(&mut self, ctx: &mut Context) -> WindowContext {
        let width = ctx.gfx.window().inner_size().width;
        let height = ctx.gfx.window().inner_size().height;
        let delta = ctx.time.delta();
        let since_start = ctx.time.time_since_start();
        trace!(
            "Grabbing Canvas width={} height={} Delta={:?}",
            width,
            height,
            delta
        );
        self.winctx.size = Point2::from([width, height]);
        self.winctx.delta = delta;
        self.winctx.since_start = since_start;

        let newctx = self.winctx.clone();
        //TODO: pick on rounding scale or nah
        // newctx.scale = self.winctx.scale.round();

        newctx
    }

    pub fn iterate_components(
        mode: GameMode,
        initial: Vec<MutComponentObject>,
        mut func: impl FnMut(MutComponentObject),
    ) {
        let mut stack = Vec::new();
        stack.extend(initial);
        while !stack.is_empty() {
            let component: MutComponentObject = stack.pop().unwrap();
            if component.get_mask().check(&mode) {
                func(component);
            }
            let next = component.next_mut();
            stack.extend(next);
        }
    }

    pub fn iterate_components_s(
        &mut self,
        mode: GameMode,
        initial: Vec<MutComponentObject>,
        func: impl Fn(&mut MidiPlay, MutComponentObject),
    ) {
        let mut stack = Vec::new();
        stack.extend(initial);
        while !stack.is_empty() {
            let component: MutComponentObject = stack.pop().unwrap();
            if component.get_mask().check(&mode) {
                func(self, component);
            }
            let next = component.next_mut();
            stack.extend(next);
        }
    }
}

impl Pausable for MidiPlay {
    fn pause(&mut self) -> bool {
        let success = !self.pause;
        self.winctx.state = DrawState::Pause;
        self.pause = true;
        success
    }

    fn resume(&mut self) -> bool {
        let success = self.pause;
        self.pause = false;
        self.winctx.state = DrawState::Ok;
        success
    }
}

impl EventHandler for MidiPlay {
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) -> Result<(), GameError> {
        self.winctx.scale = (self.winctx.scale + _y * 1e-1).clamp(1.0, 100.0);
        self.winctx.state = DrawState::ScaleChange;
        // self.winctx.trackwinctx.zoom = (self.winctx.trackwinctx.zoom + _y as f64 * 1e-2).clamp(0.0, 1.0);
        Ok(())
    }

    fn on_error(
        &mut self,
        _ctx: &mut Context,
        _origin: ggez::event::ErrorOrigin,
        _e: GameError,
    ) -> bool {
        self.playback.close().expect("Failed to close playback");
        debug!("Error! and handle playback close");
        false
    }
    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GameError> {
        self.playback.close().expect("Failed to close playback");
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

        let winctx = self.grab_context(ctx);
        let reutil = RenderUtil::new(&winctx);
        match self.input.keyboard.handle_keyboard_down(input) {
            Some(input) => self.handle_input(input, reutil, ctx),
            None => (),
        }

        Ok(())
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        _input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), GameError> {
        info!("KeyUp: {:?}", _input);
        let winctx = self.grab_context(ctx);
        let reutil = RenderUtil::new(&winctx);
        match self.input.keyboard.handle_keyboard_up(_input) {
            Some(input) => self.handle_input(input, reutil, ctx),
            None => (),
        }
        Ok(())
    }

    // fn mouse_enter_or_leave(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _entered: bool,
    // ) -> Result<(), GameError> {
    //     if _entered {
    //         info!("Mouse entered");
    //     } else {
    //         info!("Mouse left");
    //     }
    //     Ok(())
    // }

    fn resize_event(
        &mut self,
        _ctx: &mut Context,
        _width: f32,
        _height: f32,
    ) -> Result<(), GameError> {
        let scale_px = self.winctx.size.y as f32 / self.winctx.scale;
        self.winctx.scale = _height / scale_px;
        self.winctx.state = DrawState::ScaleChange;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let winctx = self.grab_context(ctx);
        let reutil = RenderUtil::new(&winctx);

        self.quality.capture(&mut self.input, &mut self.track);
        let stack: Vec<MutComponentObject> = vec![
            &mut self.track,
            &mut self.input,
            &mut self.quality,
            &mut self.user_interface,
        ];

        MidiPlay::iterate_components(self.game_mode.clone(), stack, |component| {
            let name = component.get_name();
            component.update(reutil.clone());
            trace!("Updating: {}", name);
        });

        let mut stack: Vec<MutComponentObject> = vec![
            &mut self.track,
            &mut self.input,
            &mut self.quality,
            &mut self.user_interface,
        ];
        let mode = self.game_mode.clone();
        while !stack.is_empty() {
            let component = stack.pop().unwrap();
            if component.get_mask().check(&mode) {
                let name = component.get_name();
                let input_request = component.request_input();
                match input_request {
                    Some(input) => {
                        self.inputs.push(input);
                        trace!("Input Requested: {}", name);
                    }
                    None => (),
                };
            }
            let next = component.next_mut();
            stack.extend(next);
        }
        self.handle_inputs(reutil, ctx);

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let winctx = self.grab_context(ctx);
        let mut canvas = graphics::Canvas::from_frame(ctx, pallete::LIGHTER_LIGHT);
        let mut stack: Vec<ComponentObject> =
            vec![&self.track, &self.input, &self.quality, &self.user_interface];
        let mut counter = 0;

        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        while !stack.is_empty() {
            let comp: ComponentObject = stack.pop().expect("err negative index");
            let next: Vec<ComponentObject> = comp.next();
            stack.extend(next);
            if self.draw_component(&comp, RenderUtil::new(&winctx), ctx, &mut canvas) {
                counter += 1;
            }
        }
        debug!("Drawn {}x components", counter);
        if self.winctx.state != DrawState::Pause {
            self.winctx.state = DrawState::Ok;
        }
        canvas.finish(ctx).expect("err canvas finish");
        Ok(())
    }
}
