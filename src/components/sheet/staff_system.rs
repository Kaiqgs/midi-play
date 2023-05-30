use ggez::graphics::MeshBuilder;

use crate::models::build_context::BuildContext;
use crate::{
    components::drawing::{DrawResult, Drawing},
    components::{
        component::{Component, ComponentObject},
        draw_util::DrawUtil,
        drawing::{RetrieveDrawing, DrawingReference},
        pallete,
    },
    models::{note::Note, render_util::RenderUtil, sheet::staff_system::StaffSystem, draw_state::DrawState, game_mode::NOTES_MASK},
};

/// Draw systems of Staffs;
pub struct StaffSystemComponentData {
    pub drawing: DrawingReference,
}

impl StaffSystemComponentData {
    pub fn new(noteopt: Option<Vec<Note>>, build: BuildContext) -> Self {
        let notes = match noteopt {
            Some(n) => n,
            None => StaffSystem::notes(),
        };
        let mut drawing = Drawing::default();
        let notesref = notes.iter().map(|n| n).collect();

        drawing.meshbuilder = Some(MeshBuilder::new());
        DrawUtil::staff_block(&mut drawing, build, notesref, pallete::LIGHT);

        StaffSystemComponentData { drawing: DrawingReference::new(drawing) }
    }
}

impl Component for StaffSystem {
    fn get_name(&self) -> String {
        "[Staff System]".to_string()
    }
    fn get_drawing(&self) -> RetrieveDrawing {
        RetrieveDrawing::Ok(self.component_data.drawing.clone())
    }
    fn draw(&self, reutil: RenderUtil) -> DrawResult {
        if reutil.winctx.state == DrawState::ScaleChange{
            let mut drawing = self.component_data.drawing.borrow_mut();
            DrawUtil::staff_block(
                &mut drawing,
                BuildContext::new(None, reutil.winctx.clone()),
                self.notes.iter().collect(),
                pallete::DARKER_LIGHT,
            );
            // drawing.meshbuilder = Some(MeshBuilder::new());
            // DrawUtil::staff_block(&mut drawing, canvas.build, self.notes.iter().map(|n| n).collect(), pallete::LIGHT);
        }

        let drawing = self.component_data.drawing.borrow();
        DrawResult::Draw(
            drawing.params,
        )
    }

    fn next(&self) -> Vec<ComponentObject> {
        let mut result: Vec<ComponentObject> = Vec::new();
        for staff in &self.staffs {
            result.push(staff);
        }
        result
    }

    fn get_mask(&self) -> crate::models::bit_mode::BitMask {
        NOTES_MASK
    }
}
