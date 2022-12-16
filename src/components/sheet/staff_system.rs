use ggez::{graphics::DrawParam, Context};
use std::{borrow::Borrow, sync::Arc};

use ggez::{
    context::Has,
    graphics::{Color, MeshBuilder},
    mint::Point2,
};

use crate::{
    components::drawing::{DrawResult, Drawing},
    components::{
        component::{BuildContext, Component, ComponentObject, RenderUtilObject},
        draw_util::DrawUtilGG,
        drawing, pallete,
    },
    models::{
        draw_util::DrawUtil,
        note::Note,
        sheet::{staff, staff_system::StaffSystem},
    },
};

use super::definition;

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
        DrawUtilGG::staff_block(&mut drawing, build, notesref, pallete::LIGHT);

        StaffSystemComponentData {
            position: Point2::from([0, 0]),
            size: Point2::from([0, 0]),
            drawing,
        }
    }
}

impl Component for StaffSystem {
    fn draw(&self, canvas: RenderUtilObject) -> DrawResult {
        let params = DrawParam::new();
        DrawResult {
            params,
            drawing: &self.component_data.drawing,
        }
    }

    fn next(&self) -> Vec<ComponentObject> {
        let mut result: Vec<ComponentObject> = Vec::new();
        for staff in &self.staffs {
            result.push(Arc::new(staff));
        }
        result
    }
}
