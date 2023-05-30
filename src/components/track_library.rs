use std::f64::consts::PI;

use ggez::{
    conf,
    graphics::{
        Color,
        DrawMode::{self, Stroke},
        DrawParam, MeshBuilder, Rect, StrokeOptions, Text,
    },
    mint::Point2,
};
use log::{info, warn};

use crate::{
    components::util::mapf64,
    models::{
        bit_mode::BitMask, config, game_mode::GameMode, input::input::MidiPlayInput,
        track_library::TrackLibrary,
    },
};

use super::{
    component::{Component, ComponentObject},
    drawing::{DrawResult, Drawing},
    sheet::sheet_component_const::Zindex,
};

pub struct TrackLibraryComponentData {
    pub drawing: Drawing,
    // pub default_cover: Image,
    // pub covers: HashMap<String, Image>
}

impl TrackLibraryComponentData {
    pub fn new() -> TrackLibraryComponentData {
        TrackLibraryComponentData {
            drawing: Drawing::default(),
        }
    }
}

impl Component for TrackLibrary {
    fn get_name(&self) -> String {
        String::from("[Track Library]")
    }

    fn update(&mut self, reutil: crate::models::render_util::RenderUtil) {
        let screen_size = reutil.winctx.get_scaled_size();
        let center_y = screen_size.y as f32 / 2.0;
        let center_x = screen_size.x as f32 / 2.0;

        let track_count = self.tracks.len();

        // let circle_multi = 1.75;
        let circle_center = Point2::from([center_x, center_y]);
        let circle_radius = screen_size.x as f32 / 4.0;

        let mut mb = MeshBuilder::new();
        let _circle = mb.circle(
            DrawMode::Stroke(StrokeOptions::default()),
            circle_center,
            circle_radius,
            0.1,
            Color::BLACK,
        );

        // let start_angle = 3.0 * PI / 4.0;
        // let stop_angle = PI / 4.0;
        let up_hemisphere = true;
        let upside_angle = if up_hemisphere { PI } else { 0.0 };
        let half_tracks = self.tracks.len() / 2;
        let hover_length = self.hover_track as i32 - half_tracks as i32;
        let mut start_angle = PI / 6.0 + upside_angle;
        let stop_angle = 5.0 * PI / 6.0 + upside_angle;
        // let start_angle = 0.0;
        // let stop_angle = PI;
        //TODO: inteporlate the f*3#@ out of this
        let anglength = stop_angle - start_angle;
        let angle_per_track = anglength / track_count as f64;
        start_angle -= hover_length as f64 * angle_per_track;
        // stop_angle += hover_length as f64 * angle_per_track;
        let mut angle = start_angle + angle_per_track / 2.0;
        for (i, track) in self.tracks.iter_mut().enumerate() {
            let x = circle_center.x + circle_radius * angle.cos() as f32
                - config::DEFAULT_COVER_SIZE / 2.0;
            let y = circle_center.y + circle_radius * angle.sin() as f32
                - config::DEFAULT_COVER_SIZE / 2.0;
            let topleft = Point2::from([x, y]);
            let rect = Rect {
                x,
                y,
                w: config::DEFAULT_COVER_SIZE,
                h: config::DEFAULT_COVER_SIZE,
            };
            let color = if i == self.hover_track {
                Color::from_rgba(32, 200, 32, 200)
            } else {
                Color::from_rgba(0, 0, 0, 128)
            };
            mb.rectangle(Stroke(StrokeOptions::default()), rect, color)
                .expect("Failed to draw rectangle");
            match &mut track.component_data {
                Some(component) => {
                    angle += angle_per_track;
                    component.drawing.params = DrawParam::new()
                        .dest(reutil.winctx.from_scale(topleft))
                        .scale([reutil.winctx.scale, reutil.winctx.scale])
                        .z(Zindex::GameTrack.get());
                    info!(
                        "Track {} at {:?} at scale {:?}",
                        track.name,
                        topleft,
                        reutil.winctx.from_scale(topleft)
                    );
                }
                None => {
                    warn!("Failed to get component data for track: {}", track.name);
                }
            };
        }

        let start_angle = 0.0;
        let stop_angle = 2.0 * PI;
        let sample_size: usize = 300;
        angle = stop_angle;
        let angle_per_track = (stop_angle - start_angle) / sample_size as f64;
        for i in 0..sample_size {
            let topleft = Point2::from([
                angle.cos() as f32 * circle_radius + circle_center.x,
                angle.sin() as f32 * circle_radius + circle_center.y,
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

        //
        // let tracks_per_row = screen_size.x / config::DEFAULT_COVER_SIZE;
        // let mut horizontal_spacing = 0.0;
        //
        // if tracks_per_row >= self.tracks.len() as f32 {
        //     let width = screen_size.x as f32;
        //     let total_cover_width = config::DEFAULT_COVER_SIZE * self.tracks.len() as f32;
        //     // horizontal_spacing = screen_size.x / config::DEFAULT_COVER_SIZE;
        //     // screen is big enough to fit all the tracks
        //     horizontal_spacing = (width - total_cover_width) / (self.tracks.len() as f32 + 1.0);
        // }
        //
        // let texts: Vec<Text> = vec![];
        // let txtparams: Vec<DrawParam> = vec![];
        //
        // for (i, track) in self.tracks.iter_mut().enumerate() {
        //     let x =
        //         horizontal_spacing + (i as f32 * (config::DEFAULT_COVER_SIZE + horizontal_spacing));
        //     // let xrel = x / screen_size.x as f32;
        //     // info!("xrel: {}", xrel);
        //     // let xrad = (xrel * 2.0 - 1.0) * PI / 2.0;
        //     let xrad = mapf64(
        //         (x + config::DEFAULT_COVER_SIZE / 2.0) as f64,
        //         -screen_size.x as f64,
        //         (screen_size.x * 2.0) as f64,
        //         -PI / 2.0,
        //         3.0 * PI / 2.0,
        //         // 0.0,
        //         // PI,
        //         // -PI / 2.0,
        //     );
        //     info!("xrad: {}", xrad);
        //     info!("xsin: {}", xrad.sin());
        //
        //     let y = center_y - (config::DEFAULT_COVER_SIZE / 2.0)
        //         + reutil.winctx.size.y as f32 / 32.0 * -xrad.sin() as f32;
        //
        //     // let dx = x - circle_center.x;
        //     // let dy = y - circle_center.y;
        //     // let dist = (dx * dx + dy * dy).sqrt();
        //     // let angle = dy.atan2(dx);
        //
        //     let topleft = Point2::from([x, y]);
        //     let rect = Rect {
        //         x,
        //         y,
        //         w: config::DEFAULT_COVER_SIZE,
        //         h: config::DEFAULT_COVER_SIZE,
        //     };
        //     let color = if i == self.hover_track {
        //         Color::from_rgba(32, 200, 32, 200)
        //     } else {
        //         Color::from_rgba(0, 0, 0, 128)
        //     };
        //     mb.rectangle(Stroke(StrokeOptions::default()), rect, color)
        //         .expect("Failed to draw rectangle");
        //
        //     match &mut track.component_data {
        //         Some(component) => {
        //             component.drawing.params = DrawParam::new()
        //                 .dest(reutil.winctx.from_scale(topleft))
        //                 .scale([reutil.winctx.scale, reutil.winctx.scale])
        //                 .z(Zindex::GameTrack.get());
        //         }
        //
        //         None => {
        //             warn!("Failed to get component data for track: {}", track.name);
        //         }
        //     };
        // }

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

    fn handle_input(&mut self, input: MidiPlayInput) {
        match input {
            MidiPlayInput::NextOption => {
                self.hover_track = (self.hover_track + 1) % self.tracks.len();
            }
            MidiPlayInput::PreviousOption => {
                self.hover_track = (self.hover_track + self.tracks.len() - 1) % self.tracks.len();
            }
            MidiPlayInput::SelectOption => {
                self.selected_track = Some(self.hover_track);
                self.playing_track = None;
            }
            MidiPlayInput::ModeChange(_) => {
                self.selected_track = None;
                self.playing_track = None;
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
                let game_mode = GameMode::Play(self.tracks[track_idx].clone());
                Some(MidiPlayInput::ModeChange(game_mode))
            }
            None => None,
        }
    }
}
