

use ggez::graphics::MeshBuilder;

use crate::models::build_context::BuildContext;
use crate::{
    components::{
        component::{Component, ComponentObject},
        draw_util::DrawUtil,
        drawing::{DrawResult, Drawing, DrawingReference, RetrieveDrawing},
        pallete,
    },
    models::{note::Note, render_util::RenderUtil, sheet::staff::Staff, draw_state::DrawState, game_mode::NOTES_MASK},
};


pub struct StaffComponentData {
    drawing: DrawingReference,
}

impl StaffComponentData {
    pub fn new(notes: Vec<&Note>, build: BuildContext) -> Self {
        let mut drawing = Drawing::default();
        drawing.meshbuilder = Some(MeshBuilder::new());
        DrawUtil::staff_block(&mut drawing, build, notes, pallete::DARKER_LIGHT);
        StaffComponentData {
            drawing: DrawingReference::new(drawing),
        }
    }
}

/// Draws Staff w/ lines & spaces;
impl Component for Staff {
    fn get_name(&self) -> String {
        "[Staff]".to_string()
    }
    fn get_drawing(&self) -> RetrieveDrawing {
        RetrieveDrawing::Ok(self.component_data.drawing.clone())
    }
    fn draw(&self, reutil: RenderUtil) -> DrawResult {
        if reutil.winctx.state == DrawState::ScaleChange {
            let mut drawing = self.component_data.drawing.borrow_mut();
            DrawUtil::staff_block(
                &mut drawing,
                BuildContext::new(None, reutil.winctx.clone()),
                self.notes.iter().collect(),
                pallete::DARKER_LIGHT,
            );
        }

        let drawing = self.component_data.drawing.borrow();
        DrawResult::Draw(drawing.params)
    }

    fn next(&self) -> Vec<ComponentObject> {
        vec![&self.clef]
    }

    fn get_mask(&self) -> crate::models::bit_mode::BitMask {
        NOTES_MASK
    }
}
