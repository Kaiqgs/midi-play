use ggez::{context::Has, graphics::MeshBuilder};

use crate::{
    components::{
        component::{Component, RenderUtilObject, ComponentObject},
        drawing::{Drawing, DrawResult},
    },
    models::{draw_util::DrawUtil, sheet::SheetTrack},
};

/* Draws current playing track */
pub struct SheetTrackComponentData {
    name: Option<String>,
    filepath: Option<String>,
    loaded: bool,
}

impl Has<SheetTrackComponentData> for SheetTrack<'_> {
    fn retrieve(&self) -> &SheetTrackComponentData {
        self.component_data.as_ref().unwrap()
    }
}

impl Component for SheetTrack<'_> {
    fn draw(&self, canvas: RenderUtilObject) -> DrawResult {
        // let params = DrawParam::new();
        // match self.component_data {
        //     Some(c) => DrawResult {params, drawing: &c.drawing},
        //     None => DrawResult { params, drawing: &Drawing::default() },
        // }
        unimplemented!()
    }

    fn next(&self) -> Vec<ComponentObject> {
        todo!()
    }
}
