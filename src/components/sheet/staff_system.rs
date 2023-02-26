use std::cell::RefCell;

use ggez::{graphics::MeshBuilder, mint::Point2};

use crate::{
    components::drawing::{DrawResult, Drawing},
    components::{
        component::{BuildContext, Component, ComponentObject},
        draw_util::DrawUtil,
        drawing::RetrieveDrawing,
        pallete,
    },
    models::{note::Note, render_util::RenderUtil, sheet::staff_system::StaffSystem},
};

/// Draw systems of Staffs;
pub struct StaffSystemComponentData {
    position: Point2<u32>,
    size: Point2<u32>,
    pub drawing: Drawing,
}

impl Default for StaffSystemComponentData {
    fn default() -> Self {
        StaffSystemComponentData::new(None, BuildContext::default())
    }
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

        StaffSystemComponentData {
            position: Point2::from([0, 0]),
            size: Point2::from([0, 0]),
            drawing,
        }
    }
}

impl Component for StaffSystem {
    fn get_name(&self) -> String {
        "[Staff System]".to_string()
    }
    fn get_drawing(&self) -> RetrieveDrawing {
        RetrieveDrawing::Ok(RefCell::new(self.component_data.drawing.clone()))
    }
    fn draw(&self, _canvas: RenderUtil) -> DrawResult {
        DrawResult::Draw(
            // DrawParam::new()
            //     .dest([0.0, 0.0])
            //     .scale([sheet_component_const::SCALEF, sheet_component_const::SCALEF])
            //     .z(0),
            self.component_data.drawing.params,
        )
    }

    fn next(&self) -> Vec<ComponentObject> {
        let mut result: Vec<ComponentObject> = Vec::new();
        for staff in &self.staffs {
            result.push(staff);
        }
        result
    }
}
