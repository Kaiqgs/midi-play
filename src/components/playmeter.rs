use std::{cell::RefCell, time::Duration};

use ggez::graphics::{Color, DrawParam, PxScale, Text};
use itertools::{iproduct, Itertools};
use log::{trace, warn};

use crate::models::{
    bit_mode::BitMask, game_mode::NOTES_MASK, note::Note, playmeter::PlayMeter,
    render_util::RenderUtil,
};

use super::{
    component::{Component, ComponentObject},
    drawing::{DrawResult, Drawing, RetrieveDrawing},
    sheet::sheet_component_const::{self, Zindex},
};

fn remove_outbounds_pool(pool: &mut Vec<Note>, width: Duration, now: Duration) -> Vec<Note> {
    let mut removed: Vec<Note> = vec![];

    let mut i: i32 = 0;

    while i < pool.len() as i32 {
        let hwidth = width / 2;
        let input_note = &pool[i as usize];
        let note_trigger = input_note.trigger.expect("Note should've been triggered");
        let trigger_start = if note_trigger > hwidth {
            note_trigger - hwidth
        } else {
            Duration::from_secs(0)
        };
        let trigger_end = note_trigger + hwidth;
        let in_bounds = now >= trigger_start && now < trigger_end;
        if !in_bounds {
            removed.push(pool.remove(i as usize));
            i -= 1;
        }
        i += 1;
    }
    removed
}

pub fn remove_indexes(list: &mut Vec<Note>, indexes: &mut Vec<usize>) {
    indexes.sort();
    indexes.reverse();
    for i in indexes {
        let i = i.clone();
        if list.len() == 0 {
            break;
        }
        if i >= list.len() {
            continue;
        }
        list.swap_remove(i);
    }
}

pub struct PlayMeterComponentData {
    drawing: Drawing,
}

impl PlayMeterComponentData {
    pub fn new() -> Self {
        Self {
            drawing: Drawing::default(),
        }
    }
}
impl Component for PlayMeter {
    fn get_name(&self) -> String {
        String::from("[Play Meter]")
    }

    fn get_drawing(&self) -> RetrieveDrawing {
        RetrieveDrawing::Ok(RefCell::new(self.component_data.drawing.clone()))
    }

    fn update(&mut self, reutil: RenderUtil) {
        let removed_input = remove_outbounds_pool(
            &mut self.unpaired_input_pool,
            self.acceptance_range,
            reutil.winctx.since_start,
        );
        let removed_track = remove_outbounds_pool(
            &mut self.unpaired_track_pool,
            self.acceptance_range,
            reutil.winctx.since_start,
        );

        let subtracted = removed_input.len() + removed_track.len();
        self.average_quality -= subtracted as f64;
        //print pools length
        trace!(
            "input: {}, track: {}, quality: {}",
            self.unpaired_input_pool.len(),
            self.unpaired_track_pool.len(),
            self.average_quality
        );

        let mut remove_input: Vec<usize> = vec![];
        let mut remove_track: Vec<usize> = vec![];
        //find matches
        for comb in iproduct!(
            0..self.unpaired_input_pool.len(),
            0..self.unpaired_track_pool.len()
        )
        .unique()
        {
            let input = &self.unpaired_input_pool[comb.0];
            let track = &self.unpaired_track_pool[comb.1];
            if track.eq_to_input(input) {
                warn!("found match");
                self.average_quality += 1.0;
                remove_input.push(comb.0);
                remove_track.push(comb.1);
            }
        }

        remove_track.sort();
        remove_track.reverse();
        for i in remove_track {
            if self.unpaired_track_pool.len() == 0 {
                break;
            }
            if i >= self.unpaired_track_pool.len() {
                continue;
            }
            self.unpaired_track_pool.swap_remove(i);
        }

        let mut txt = Text::new(format!("\n\n\n\n\nQuality={}", self.average_quality));
        txt.set_scale(PxScale::from(sheet_component_const::SCALEF * 6.0).round());
        self.component_data.drawing.text = Some(txt);
    }

    fn draw(&self, reutil: RenderUtil) -> DrawResult {
        DrawResult::Draw(
            DrawParam::new()
                .dest([100.0, 100.0])
                .scale([reutil.winctx.scale, reutil.winctx.scale])
                .color(Color::BLACK)
                .z(Zindex::Debug.get()),
        )
    }

    fn next(&self) -> Vec<ComponentObject> {
        Vec::new()
    }

    fn get_mask(&self) -> BitMask {
        NOTES_MASK
    }
}
