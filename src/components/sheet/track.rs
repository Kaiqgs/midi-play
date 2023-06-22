use std::collections::HashSet;

use ggez::{
    graphics::{Color, DrawMode, DrawParam, FillOptions, MeshBuilder, PxScale, Rect, Text},
    mint::Point2,
};
use log::debug;

use crate::models::game_mode::NOTES_MASK;
use crate::{
    components::{
        component::Component,
        drawing::{DrawResult, Drawing, DrawingReference, RetrieveDrawing},
        sheet::sheet_component_const,
    },
    models::{
        clock::ClockFloat, draw_state::DrawState, midi::peripheral::MidiPeripheral, note::Note,
        render_util::RenderUtil, sheet::SheetTrack,
    },
};

use super::sheet_component_const::Zindex;

/* Draws current playing track */
pub struct SheetTrackComponentData {
    pub drawing: DrawingReference,
    pub update: ClockFloat,
    pub render_all: bool,
    pub playback: MidiPeripheral,
    pub range: Option<(f64, f64)>,
    pub closest_key: u32,
    pub notes_on: HashSet<Note>,
}

impl SheetTrackComponentData {
    pub fn new(drawing: Drawing) -> Self {
        let playback = MidiPeripheral::new(String::from("<SheetTrack Playback>"));

        SheetTrackComponentData {
            drawing: DrawingReference::new(drawing),
            update: ClockFloat::new(),
            closest_key: 0,
            render_all: true,
            playback,
            range: None,
            notes_on: HashSet::new(),
        }
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

    fn update(&mut self, reutil: RenderUtil) {
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

        let width_px = reutil.winctx.size.x as f64 / reutil.winctx.scale as f64;
        let height_px = reutil.winctx.size.y as f64 / reutil.winctx.scale as f64;
        let last_tick = self.last_track_time.tick as f64;
        let last_sec = self.last_track_time.sec as f64;

        let tick_to_px = last_tick / width_px;
        let _px_to_tick = width_px / last_tick;
        let _sec_to_px = last_sec / width_px;
        let _px_to_sec = width_px / last_sec;

        let global_note_offset_px = width_px * sheet_component_const::NOTE_OFFSET_RATIO;
        let trigger_pos_px = width_px * sheet_component_const::TRIGGER_OFFSET_RATIO;
        // let trigger_ofst_tick = last_tick * sheet_component_const::TRIGGER_OFFSET_RATIO;

        let trigger_note_diff_tick = last_tick * sheet_component_const::TRIGER_NOTE_DIFF_RATIO;
        let trigger_note_diff_sec = last_sec * sheet_component_const::TRIGER_NOTE_DIFF_RATIO;
        // let time_now_percent = (self.time.tick - trigger_ofst_tick) / last_tick; //TODO: stop when percentage >= 1

        let delta_sec = reutil.winctx.delta.as_millis() as f64 / 1000.0;
        let delta_tick = delta_sec / timing.sec_per_tick;
        let delta_px = delta_tick / tick_to_px;

        debug!("zoom: {}", reutil.winctx.trackwinctx.get_zoom());
        debug!(
            "delta_sec={} delta_tick={} delta_px={} zoom={}",
            delta_sec,
            delta_tick,
            delta_px,
            reutil.winctx.trackwinctx.get_zoom()
        );
        self.component_data.update = ClockFloat::new().set(delta_tick, delta_sec);
        if reutil.winctx.state != DrawState::Pause {
            self.time.sec += delta_sec;
            self.time.tick += delta_tick;
        }
        let trigerless_time_tick = self.time.tick - trigger_note_diff_tick;
        let trigerless_time_sec = self.time.sec - trigger_note_diff_sec;

        text.add(&format!("time_tick={:.0}\n", self.time.tick));
        text.add(&format!("ttime_tick={:.0}\n", trigerless_time_tick));
        text.add(&format!("time_sec={:.2}\n", self.time.sec));
        text.add(&format!("ttime_sec={:.2}\n", trigerless_time_sec));
        text.set_scale(PxScale::from(sheet_component_const::SCALEF * 6.0).round());
        // text.set_font(String::from("LiberationMono-Regular"));

        self.component_data.range = Some((1.0, 3.0));
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
        )
        .expect("Failed to draw line");
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
                    let note_diff_px = target_diff_ticks / tick_to_px; // * reutil.winctx.track.get_zoom();
                    let local_note_ofst_px = note_diff_px;

                    let note_pos_x_px = local_note_ofst_px + global_note_offset_px;
                    let target_diff = (trigger_pos_px - note_pos_x_px).abs();
                    let note_pos_x_px = note_pos_x_px as f32;
                    let is_target_hit = target_diff < sheet_component_const::TRIGGER_RANGE.into();
                    let note_pos_y = note.line * sheet_component_const::NOTE_HEIGHT;
                    let note_pos_y_px = note_pos_y as f32 + 0.5; // + 0.5 to center the note;

                    //TODO: capture all hit_notes first:
                    //TODO: so we can handle on&off trigger in same frame;
                    if is_target_hit {
                        if note.on.is_some() {
                            let note = note.trigger(reutil.winctx.since_start);
                            let note_state = note.on.unwrap();

                            if note_state && !self.component_data.notes_on.contains(&note) {
                                let pairs = &self.track_pairs[track_index];
                                let note_end_index = pairs[&note_index];
                                let note_end = &track[note_end_index];
                                //TODO change closest_key
                                debug!("Note on: sent!");
                                self.component_data
                                    .playback
                                    .note(&note, note_end)
                                    .expect("Note on failed");
                                self.component_data.notes_on.insert(note.clone());
                                time_now = Some(note.time.tick);
                                self.reported.push(note.clone());
                            } else if !note_state && self.component_data.notes_on.contains(&note) {
                                self.component_data.notes_on.remove(&note);
                                time_now = Some(note.time.tick);
                                self.reported.push(note);
                            }
                        }
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
                    )
                    .expect("Failed to draw center left");
                    mb.rectangle(
                        DrawMode::Fill(FillOptions::default()),
                        Rect::new(note_pos_x_px + 1.0, note_pos_y_px - 1.0, 1.0, 1.0),
                        color,
                    )
                    .expect("Failed to draw center right");
                    mb.rectangle(
                        DrawMode::Fill(FillOptions::default()),
                        Rect::new(note_pos_x_px, note_pos_y_px, 1.0, 1.0),
                        color,
                    )
                    .expect("Failed to draw center up");
                    mb.rectangle(
                        DrawMode::Fill(FillOptions::default()),
                        Rect::new(note_pos_x_px, note_pos_y_px - 2.0, 1.0, 1.0),
                        color,
                    )
                    .expect("Failed to draw center down");
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

    fn draw(&self, reutil: RenderUtil) -> DrawResult {
        DrawResult::Draw(
            DrawParam::new()
                .dest([0.0, 0.0])
                .scale([reutil.winctx.scale, reutil.winctx.scale])
                .z(Zindex::Note.get()),
        )
    }

    fn get_mask(&self) -> crate::models::bit_mode::BitMask {
        NOTES_MASK
    }
}
