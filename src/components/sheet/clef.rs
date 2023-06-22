use std::cell::RefCell;

use log::warn;

use crate::components::util::image_from_optional;
use crate::models::build_context::BuildContext;
use crate::models::game_mode::NOTES_MASK;
use crate::{
    components::{
        component::Component,
        draw_util::DrawUtil,
        drawing::{DrawResult, Drawing, DrawingReference, RetrieveDrawing},
    },
    models::{draw_state::DrawState, note::Note, render_util::RenderUtil, sheet::clef::Clef},
};

/// Draws clef symbol;
pub struct ClefComponentData {
    drawing: DrawingReference,
}

impl ClefComponentData {
    pub fn new(note: &Note, filepath: Option<String>, build: BuildContext) -> Self {
        let mut drawing = Drawing::default();

        match build.ctx {
            Some(ctx) => {
                drawing.image = Some(image_from_optional(&ctx, filepath.clone()));
                DrawUtil::left_image(&mut drawing, &build.winctx, note);
            }
            None => {
                warn!("No build context found, cannot draw clef symbol")
            }
        };
        ClefComponentData {
            drawing: RefCell::new(drawing),
        }
    }
}

impl Component for Clef {
    fn get_name(&self) -> String {
        "[Clef]".to_string()
    }
    fn get_drawing(&self) -> RetrieveDrawing {
        RetrieveDrawing::Ok(self.component_data.drawing.clone())
    }
    fn draw(&self, reutil: RenderUtil) -> DrawResult {
        if reutil.winctx.state == DrawState::ScaleChange {
            let mut drawing = self.component_data.drawing.borrow_mut();
            let params = DrawUtil::left_image(&mut drawing, reutil.winctx, &self.note);
            return DrawResult::Draw(params);
        }
        let drawing = self.component_data.drawing.borrow();
        DrawResult::Draw(drawing.params)
    }
    fn get_mask(&self) -> crate::models::bit_mode::BitMask {
        NOTES_MASK
    }
}
