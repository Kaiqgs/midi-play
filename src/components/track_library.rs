use std::{f32::consts::PI, time::Duration, usize};

use ggez::{
    graphics::{
        Color,
        DrawMode::{self, Stroke},
        DrawParam, MeshBuilder, Rect, StrokeOptions,
    },
    mint::Point2,
};
use log::{trace, warn};

use crate::models::{
    animation::Animation, bit_mode::BitMask, config, game_mode::GameMode,
    input::input::MidiPlayInput, render_util::RenderUtil, track_library::TrackLibrary,
};

use super::{
    component::{Component, ComponentObject},
    drawing::{DrawResult, Drawing},
    sheet::sheet_component_const::Zindex,
};

pub struct TrackLibraryComponentData {
    pub drawing: Drawing,
    pub switch_track_animation: Animation,
    pub last_selection: Duration,
    pub hover_angle_pad: f32,
}

pub struct TrackCircle {
    pub hover_angle: f32,
    pub angle_per_track: f32,
    pub center: Point2<f32>,
    pub radius: f32,
    pub start_angle: f32,
    pub hovered_start_angle: f32,
    pub stop_angle: f32,
}

pub const TRACK_DURATION_MS: u64 = 250;

impl TrackLibraryComponentData {
    pub fn new() -> TrackLibraryComponentData {
        TrackLibraryComponentData {
            switch_track_animation: Animation::new(
                Duration::default(),
                Duration::default(),
                0.0,
                0.0,
            ),
            drawing: Drawing::default(),
            last_selection: Duration::default(),
            hover_angle_pad: PI / 8.0,
        }
    }

    fn compute_circle(
        &self,
        reutil: RenderUtil,
        track_count: usize,
        hover_track: usize,
    ) -> TrackCircle {
        let screen_size = reutil.winctx.get_scaled_size();
        let _center_y = screen_size.y as f32 / 2.0;
        let center_x = screen_size.x as f32 / 2.0;

        // let circle_multi = 1.75;
        // let circle_center = Point2::from([center_x, center_y]);
        let circle_center = Point2::from([center_x, screen_size.y as f32]);
        let circle_radius = screen_size.x as f32 / 2.0;

        // let start_angle = 3.0 * PI / 4.0;
        // let stop_angle = PI / 4.0;
        let up_hemisphere = true; //TODO: put in signature
        let upside_angle = if up_hemisphere { PI } else { 0.0 };
        let half_tracks = track_count / 2;
        let hover_length = hover_track as i32 - half_tracks as i32;
        let start_angle = PI / 6.0 + upside_angle;
        let stop_angle = 5.0 * PI / 6.0 + upside_angle;
        // let start_angle = 0.0;
        // let stop_angle = PI;
        //TODO: inteporlate the f*3#@ out of this
        let anglength = stop_angle - start_angle;
        let angle_per_track = anglength / track_count as f32;
        let hover_angle = hover_length as f32 * angle_per_track;

        let hovered_angle = self
            .switch_track_animation
            .animated(reutil.winctx.since_start);
        let hovered_start_angle = start_angle - hovered_angle + angle_per_track / 2.0;
        TrackCircle {
            hover_angle,
            center: circle_center,
            radius: circle_radius,
            angle_per_track,
            start_angle,
            stop_angle,
            hovered_start_angle,
        }
    }
}

impl Component for TrackLibrary {
    fn get_name(&self) -> String {
        String::from("[Track Library]")
    }

    fn update(&mut self, reutil: crate::models::render_util::RenderUtil) {
        let circle =
            self.component_data
                .compute_circle(reutil.clone(), self.tracks.len(), self.hover_track);
        let mut mb = MeshBuilder::new();
        let _circle = mb.circle(
            DrawMode::Stroke(StrokeOptions::default()),
            circle.center,
            circle.radius,
            0.1,
            Color::BLACK,
        );
        // let delta = reutil.winctx.delta.as_millis() as f64 / 1000.0;
        // stop_angle += hover_length as f64 * angle_per_track;
        // self.component_data.last_angle
        let mut angle = circle.hovered_start_angle - self.component_data.hover_angle_pad;
        for (i, track) in self.tracks.iter_mut().enumerate() {
            let is_hover = i == self.hover_track;
            if is_hover {
                angle += self.component_data.hover_angle_pad;
            }
            let x = circle.center.x + circle.radius * angle.cos() as f32
                - config::DEFAULT_COVER_SIZE / 2.0;
            let y = circle.center.y + circle.radius * angle.sin() as f32
                - config::DEFAULT_COVER_SIZE / 2.0;
            let topleft = Point2::from([x, y]);
            let rect = Rect {
                x,
                y,
                w: config::DEFAULT_COVER_SIZE,
                h: config::DEFAULT_COVER_SIZE,
            };
            let color = if is_hover {
                Color::from_rgba(32, 200, 32, 200)
            } else {
                Color::from_rgba(0, 0, 0, 128)
            };
            if is_hover {
                angle += self.component_data.hover_angle_pad;
            }
            mb.rectangle(Stroke(StrokeOptions::default()), rect, color)
                .expect("Failed to draw rectangle");

            angle += circle.angle_per_track;
            track.component_data.drawing.params = DrawParam::new()
                .dest(reutil.winctx.from_scale(topleft))
                .scale([reutil.winctx.scale, reutil.winctx.scale])
                .z(if is_hover {
                    Zindex::SelectedGameTrack.get()
                } else {
                    Zindex::GameTrack.get()
                });
            trace!(
                "Track at {:?} at scale {:?}",
                topleft,
                reutil.winctx.from_scale(topleft)
            );
        }
        let start_angle = 0.0;
        let stop_angle = 2.0 * PI;
        let sample_size: usize = 300;
        angle = stop_angle;
        let angle_per_track = (stop_angle - start_angle) / sample_size as f32;
        for i in 0..sample_size {
            let topleft = Point2::from([
                angle.cos() as f32 * circle.radius + circle.center.x,
                angle.sin() as f32 * circle.radius + circle.center.y,
            ]);
            angle += angle_per_track;
            // let rect = Rect::new(
            //     topleft.x,
            //     topleft.y,
            //     config::DEFAULT_COVER_SIZE,
            //     config::DEFAULT_COVER_SIZE,
            // );
            let color = if i % 2 == 0 {
                Color::from_rgba(0, 0, 0, 128)
            } else {
                Color::from_rgba(0, 0, 0, 64)
            };
            mb.circle(DrawMode::stroke(1.0), topleft, 1.0, 1.0, color)
                .expect("Failed to draw circle");
        }
        self.component_data.drawing.params = DrawParam::new()
            .scale([reutil.winctx.scale, reutil.winctx.scale])
            .z(Zindex::TrackLibrary.get());
        self.component_data.drawing.meshbuilder = Some(mb);
    }

    fn draw(&self, _reutil: crate::models::render_util::RenderUtil) -> DrawResult {
        DrawResult::Draw(self.component_data.drawing.params)
    }

    fn get_drawing(&self) -> super::drawing::RetrieveDrawing {
        super::drawing::RetrieveDrawing::Ok(super::drawing::DrawingReference::new(
            self.component_data.drawing.clone(),
        ))
    }

    fn next(&self) -> Vec<ComponentObject> {
        let mut result: Vec<ComponentObject> = Vec::new();
        for track in &self.tracks {
            result.push(track);
        }
        result
    }

    fn get_mask(&self) -> crate::models::bit_mode::BitMask {
        BitMask::default().allow(GameMode::Library)
    }

    fn handle_input(&mut self, input: MidiPlayInput, reutil: RenderUtil) {
        match input {
            MidiPlayInput::NextOption => {
                let last_delta = reutil.winctx.since_start - self.component_data.last_selection;
                if self
                    .component_data
                    .switch_track_animation
                    .finished(reutil.winctx.since_start)
                {
                    self.hover_track = (self.hover_track + 1) % self.tracks.len();
                    let new_angle = self
                        .component_data
                        .compute_circle(reutil.clone(), self.tracks.len(), self.hover_track)
                        .hover_angle;
                    self.component_data.switch_track_animation = Animation::new(
                        reutil.winctx.since_start,
                        Duration::from_millis(
                            last_delta.as_millis().min(TRACK_DURATION_MS as u128) as u64,
                        ),
                        self.component_data.switch_track_animation.end,
                        new_angle,
                    );
                }
                self.component_data.last_selection = reutil.winctx.since_start;
            }
            MidiPlayInput::PreviousOption => {
                let last_delta = reutil.winctx.since_start - self.component_data.last_selection;
                if self
                    .component_data
                    .switch_track_animation
                    .finished(reutil.winctx.since_start)
                {
                    self.hover_track =
                        (self.hover_track + self.tracks.len() - 1) % self.tracks.len();
                    let new_angle = self
                        .component_data
                        .compute_circle(reutil.clone(), self.tracks.len(), self.hover_track)
                        .hover_angle;
                    self.component_data.switch_track_animation = Animation::new(
                        reutil.winctx.since_start,
                        Duration::from_millis(
                            last_delta.as_millis().min(TRACK_DURATION_MS as u128) as u64,
                        ),
                        self.component_data.switch_track_animation.end,
                        new_angle,
                    );
                }
                self.component_data.last_selection = reutil.winctx.since_start;
            }
            MidiPlayInput::SelectOption => {
                self.selected_track = Some(self.hover_track);
                self.playing_track = None;
            }
            MidiPlayInput::ModeChange(_) => {
                self.selected_track = None;
                self.playing_track = None;
            }
            MidiPlayInput::BackOption => {
                self.import(reutil);
            }
            _ => {}
        }
    }

    fn request_input(&mut self) -> Option<MidiPlayInput> {
        match self.playing_track {
            Some(_) => return None,
            None => (),
        }
        self.playing_track = self.selected_track;
        match self.selected_track {
            Some(track_idx) => {
                warn!("Playing track {}", self.tracks[track_idx].filepath);
                let game_mode = GameMode::Play(self.tracks[track_idx].clone());
                Some(MidiPlayInput::ModeChange(game_mode))
            }
            None => None,
        }
    }
}
