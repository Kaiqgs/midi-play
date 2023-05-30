use crate::models::{bit_mode::BitMask, game_mode::GameMode, game_track::GameTrack};

use super::{
    component::Component,
    drawing::{DrawResult, Drawing, RetrieveDrawing},
    util::load_cover,
};
use crate::models::build_context::BuildContext;

#[derive(Default, Clone)]
pub struct GameTrackComponentData {
    pub drawing: Drawing,
}

impl GameTrackComponentData {
    pub fn new(bctx: BuildContext, cover_filepath: String) -> Self {
        let cover = load_cover(bctx, cover_filepath); // drawing: Drawing::new(ctx, cover_filepath
        Self {
            drawing: Drawing::new_image(Some(cover)),
        }
    }
}

impl Component for GameTrack {
    fn get_name(&self) -> String {
        String::from("[Game Track]")
    }

    // fn update(&mut self, _canvas: crate::models::render_util::RenderUtil) {
    //     ()
    // }

    fn draw(&self, _canvas: crate::models::render_util::RenderUtil) -> super::drawing::DrawResult {
        match &self.component_data {
            Some(component) => DrawResult::Draw(component.drawing.params),
            None => DrawResult::Skip,
        }
    }

    // fn get_new_drawing(&self) -> super::drawing::Drawing {
    //     super::drawing::Drawing::default()
    // }

    fn get_drawing(&self) -> RetrieveDrawing {
        match &self.component_data {
            Some(component) => RetrieveDrawing::Ok(super::drawing::DrawingReference::new(
                component.drawing.clone(),
            )),
            None => RetrieveDrawing::Ok(super::drawing::DrawingReference::new(
                super::drawing::Drawing::default(),
            )),
        }
    }

    // fn next(&self) -> Vec<super::component::ComponentObject> {
    //     Vec::new()
    // }

    fn get_mask(&self) -> crate::models::bit_mode::BitMask {
        BitMask::default().allow(GameMode::Library)
    }
}
