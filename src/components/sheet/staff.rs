use std::cell::RefCell;

use ggez::graphics::MeshBuilder;

use crate::{
    components::{
        component::{BuildContext, Component, ComponentObject},
        draw_util::DrawUtil,
        drawing::{DrawResult, Drawing, RetrieveDrawing},
        pallete,
    },
    models::{note::Note, render_util::RenderUtil, sheet::staff::Staff},
};

const LINES: i32 = 5;
const SPACES: i32 = 4;

pub struct StaffComponentData {
    drawing: Drawing,
}

impl StaffComponentData {
    pub fn new(notes: Vec<&Note>, build: BuildContext) -> Self {
        let mut drawing = Drawing::default();
        drawing.meshbuilder = Some(MeshBuilder::new());
        DrawUtil::staff_block(&mut drawing, build, notes, pallete::DARKER_LIGHT);
        StaffComponentData { drawing }
    }
}

impl Default for StaffComponentData {
    fn default() -> Self {
        StaffComponentData::new(vec![], BuildContext::default())
    }
}

/// Draws Staff w/ lines & spaces;
impl Component for Staff {
    fn get_name(&self) -> String {
        "[Staff]".to_string()
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
        vec![&self.clef]
    }
}
