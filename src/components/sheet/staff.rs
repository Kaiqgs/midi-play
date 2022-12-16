use std::{rc::Rc, sync::Arc};

use ggez::{
    context::Has,
    graphics::{DrawParam, MeshBuilder},
};

use crate::{
    components::{
        component::{Component, ComponentObject, RenderUtilObject, BuildContext},
        drawing::{DrawResult, Drawing},
        pallete, draw_util::DrawUtilGG,
    },
    models::{draw_util::DrawUtil, sheet::staff::Staff, note::Note},
};

use super::definition;

const LINES: i32 = 5;
const SPACES: i32 = 4;

pub struct StaffComponentData {
    drawing: Drawing,
}

impl StaffComponentData {
    pub fn new(notes:Vec<&Note>, build: BuildContext) -> Self {
        let mut drawing = Drawing::default();
        drawing.meshbuilder = Some(MeshBuilder::new());
        DrawUtilGG::staff_block(&mut drawing, build, notes, pallete::DARKER_LIGHT);
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
    fn draw(&self, canvas: RenderUtilObject) -> DrawResult {        
        let params = DrawParam::new();
        DrawResult {
            params,
            drawing: &self.component_data.drawing,
        }
    }

    fn next(&self) -> Vec<ComponentObject> {
        vec![Arc::new(&self.clef)]
    }
}
