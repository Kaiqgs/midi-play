use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    sync::RwLock,
};

use ggez::{
    context::Has,
    graphics::{
        Color, DrawMode, DrawParam, FillOptions, MeshBuilder, PxScale, Rect, StrokeOptions, Text,
    },
    mint::Point2,
};
use log::{debug, info, trace, warn};

use crate::{
    components::{
        component::{BuildContext, Component, ComponentObject, WindowContext},
        drawing::{DrawResult, Drawing, DrawingReference, RetrieveDrawing},
        sheet::sheet_component_const::{self, SCALE},
    },
    models::{
        clock::{Clock, ClockFloat},
        midi::{peripheral::MidiPeripheral, playback::MidiPlayback},
        note::Note,
        render_util::RenderUtil,
        sheet::sheet_const,
        sheet::SheetTrack,
    },
};

use super::sheet_component_const::Zindex;

/* Draws current playing track */
pub struct SheetTrackComponentData {
    pub drawing: DrawingReference,
    pub update: ClockFloat,
    pub winctx: WindowContext,
    pub render_all: bool,
    pub playback: MidiPeripheral,
    pub range: Option<(f64, f64)>,
    pub closest_key: u32,
}

impl SheetTrackComponentData {
    pub fn new(drawing: Drawing, build: BuildContext) -> Self {
        let mut playback = MidiPeripheral::new(String::from("<SheetTrack Playback>"));

        SheetTrackComponentData {
            drawing: DrawingReference::new(drawing),
            update: ClockFloat {
                tick: 0.0,
                sec: 0.0,
            },
            closest_key: 0,
            render_all: true,
            winctx: build.winctx,
            playback,
            range: None,
        }
    }
}

fn sub_u32(a: u32, b: u32) -> u32 {
    if b >= a {
        u32::MIN
    } else {
        a - b
    }
}
/**
Epic for performance:
Chunkify whole track into a list of meshes from getgo;
**/

impl Component for SheetTrack {
    fn get_name(&self) -> String {
        "[Sheet Track]".to_string()
    }
    fn get_drawing(&self) -> RetrieveDrawing {
        RetrieveDrawing::Ok(self.component_data.drawing.clone())
    }

    fn update(&mut self, canvas: RenderUtil) {
        let mut mb = MeshBuilder::new();
        let mut text = Text::new("debug display\n");
        let drawing = Drawing::default();
        let drawing_reference = DrawingReference::new(drawing.clone());
        self.component_data.drawing.swap(&drawing_reference);
        let mut drawing = self.component_data.drawing.get_mut();

        let timing = self
            .track_timing
            .get(&self.component_data.closest_key)
            .expect("No timing found");

        let width_px = canvas.winctx.size.x as f64 / sheet_component_const::SCALEFF;
        let height_px = canvas.winctx.size.y as f64 / sheet_component_const::SCALEFF;
        let last_tick = self.last_track_time.tick as f64;
        let last_sec = self.last_track_time.sec as f64;

        let tick_to_px = last_tick / width_px;
        let px_to_tick = width_px / last_tick;
        let sec_to_px = last_sec / width_px;
        let px_to_sec = width_px / last_sec;

        let global_note_offset_px =
            width_px * sheet_component_const::NOTE_OFFSET_RATIO * canvas.winctx.track.get_zoom();
        let trigger_pos_px = width_px * sheet_component_const::TRIGGER_OFFSET_RATIO;
        // let trigger_ofst_tick = last_tick * sheet_component_const::TRIGGER_OFFSET_RATIO;

        let trigger_note_diff_tick = last_tick * sheet_component_const::TRIGER_NOTE_DIFF_RATIO;
        let trigger_note_diff_sec = last_sec * sheet_component_const::TRIGER_NOTE_DIFF_RATIO;
        // let time_now_percent = (self.time.tick - trigger_ofst_tick) / last_tick; //TODO: stop when percentage >= 1

        //ZOOM !?!
        // let tick_render_length = last_tick as f64 * canvas.winctx.track.get_zoom();
        // let tick_render_round_length = tick_render_length.round() as u32;
        // let zoomed = tick_render_length / tick_to_px * sheet_component_const::SCALEF;

        let delta_sec = canvas.delta.as_millis() as f64 / 1000.0;
        let delta_tick = delta_sec / timing.sec_per_tick;
        let delta_px = delta_tick / tick_to_px;

        debug!("zoom: {}", canvas.winctx.track.get_zoom());
        debug!(
            "delta_sec={} delta_tick={} delta_px={} zoom={}",
            delta_sec,
            delta_tick,
            delta_px,
            canvas.winctx.track.get_zoom()
        );
        self.component_data.update = ClockFloat {
            sec: delta_sec,
            tick: delta_tick,
        };
        self.time.sec += self.component_data.update.sec;
        self.time.tick += self.component_data.update.tick;
        let trigerless_time_tick = self.time.tick - trigger_note_diff_tick;
        let trigerless_time_sec = self.time.sec - trigger_note_diff_sec;

        text.add(&format!("time_tick={:.0}\n", self.time.tick));
        text.add(&format!("ttime_tick={:.0}\n", trigerless_time_tick));
        text.add(&format!("time_sec={:.2}\n", self.time.sec));
        text.add(&format!("ttime_sec={:.2}\n", trigerless_time_sec));
        text.set_scale(PxScale::from(sheet_component_const::SCALEF * 6.0).round());
        // text.set_font(String::from("LiberationMono-Regular"));

        self.component_data.range = Some((0.5, 1.0));
        mb.line(
            &[
                Point2 {
                    x: trigger_pos_px as f32,
                    y: 0.0,
                },
                Point2 {
                    x: trigger_pos_px as f32,
                    y: height_px as f32,
                },
            ],
            1.0,
            Color::YELLOW,
        );
        // mb.rectangle(
        //     DrawMode::Fill(FillOptions::default()),
        //     Rect::new(0.0, 0.0, 100.0, 100.0),
        //     Color::BLUE,
        // );
        let mut count_all = 0;
        let mut count_rendered = 0;
        let mut time_now: Option<u32> = None;
        for (track_index, track) in self.track.iter().enumerate() {
            for (note_index, note) in track.iter().enumerate() {
                //+1.0 to round half of note_height/size
                // let start_render = sub_u32(self.tick_time as u32, tick_render_round_length);
                // let start_render_tick =
                //     (self.time.tick - tick_render_length).clamp(0.0, self.time.tick);
                // let finish_render_tick = self.last_track_time.tick as f64 + tick_render_length;
                // let finish_render = self.time.tick + tick_render_length;

                if self.component_data.render_all
                // || note_time >= start_render_tick && note_time < finish_render
                {
                    let target_diff_ticks = note.time.tick as f64 - self.time.tick;
                    let note_diff_px =
                        target_diff_ticks / tick_to_px * canvas.winctx.track.get_zoom();
                    let local_note_ofst_px = note_diff_px;

                    let note_pos_x_px = local_note_ofst_px + global_note_offset_px;
                    let target_diff = (trigger_pos_px - note_pos_x_px).abs();
                    let note_pos_x_px = note_pos_x_px as f32;
                    let is_target_hit = target_diff < sheet_component_const::TRIGGER_RANGE.into();
                    let note_pos_y = note.line * sheet_component_const::NOTE_HEIGHT;
                    let note_pos_y_px = note_pos_y as f32 + 0.5; // + 0.5 to center the note;

                    if is_target_hit {
                        match note.on {
                            Some(note_state) => {
                                if note_state {
                                    let pairs = &self.track_pairs[track_index];
                                    let note_end_index = pairs[&note_index];
                                    let note_end = &track[note_end_index];
                                    //TODO change closest_key
                                    debug!("Note on: sent!");
                                    self.component_data
                                        .playback
                                        .note(note, note_end)
                                        .expect("Note on failed");
                                }
                            }
                            None => warn!("Note without information"),
                        }
                        time_now = Some(note.time.tick);
                    }
                    let mut color = Color::new(1.0, 0.0, 0.0, 0.5);
                    if !note.on.unwrap_or(true) {
                        color = Color::new(0.0, 0.0, 1.0, 0.5);
                    }
                    if is_target_hit {
                        color = Color::new(0.0, 1.0, 0.0, 0.5);
                    }
                    mb.rectangle(
                        DrawMode::Fill(FillOptions::default()),
                        Rect::new(note_pos_x_px - 1.0, note_pos_y_px - 1.0, 1.0, 1.0),
                        color,
                    );
                    mb.rectangle(
                        DrawMode::Fill(FillOptions::default()),
                        Rect::new(note_pos_x_px + 1.0, note_pos_y_px - 1.0, 1.0, 1.0),
                        color,
                    );
                    mb.rectangle(
                        DrawMode::Fill(FillOptions::default()),
                        Rect::new(note_pos_x_px, note_pos_y_px, 1.0, 1.0),
                        color,
                    );
                    mb.rectangle(
                        DrawMode::Fill(FillOptions::default()),
                        Rect::new(note_pos_x_px, note_pos_y_px - 2.0, 1.0, 1.0),
                        color,
                    );
                    count_rendered += 1;
                }
                count_all += 1;
            }
        }
        if let Some(time_now) = time_now {
            self.component_data.playback.set_time(time_now);
        }
        let mesh_data = mb.build();
        debug!(
            "rendering: {}x rendered {}x total [ver={}, idx={}]",
            count_rendered,
            count_all,
            mesh_data.vertices.len(),
            mesh_data.indices.len()
        );
        drawing.meshbuilder = Some(mb.to_owned());
        drawing.text = Some(text);
    }

    fn draw(&self, canvas: RenderUtil) -> DrawResult {
        DrawResult::Draw(
            DrawParam::new()
                .dest([0.0, 0.0])
                .scale([sheet_component_const::SCALEF, sheet_component_const::SCALEF])
                .z(Zindex::Note.get()),
        )
    }
}
