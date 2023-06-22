use ggez::graphics::Image;

use crate::models::{bit_mode::BitMask, game_mode::GameMode, game_track::GameTrack};

use super::{
    component::Component,
    drawing::{DrawResult, Drawing, DrawingReference, RetrieveDrawing},
    util::load_cover,
};
use crate::models::build_context::BuildContext;

#[derive(Default, Clone)]
pub struct GameTrackComponentData {
    pub drawing: Drawing,
}

pub enum CoverComponent<'a> {
    Reference(BuildContext<'a>, String),
    Object(Image),
    Empty,
}

impl GameTrackComponentData {
    pub fn new(cover: CoverComponent) -> Self {
        match cover {
            CoverComponent::Reference(bctx, cover_filepath) => {
                Self::from_image(load_cover(bctx, cover_filepath))
            }
            CoverComponent::Object(im) => Self::from_image(im),
            CoverComponent::Empty => Self::default(),
        }
    }

    pub fn from_image(image: Image) -> Self {
        Self {
            drawing: Drawing::new_image(image),
        }
    }
}

impl Component for GameTrack {
    fn get_name(&self) -> String {
        String::from("[Game Track]")
    }

    fn draw(&self, _canvas: crate::models::render_util::RenderUtil) -> super::drawing::DrawResult {
        DrawResult::Draw(self.component_data.drawing.params)
    }

    // fn get_new_drawing(&self) -> super::drawing::Drawing {
    //     super::drawing::Drawing::default()
    // }

    fn get_drawing(&self) -> RetrieveDrawing {
        RetrieveDrawing::Ok(DrawingReference::new(self.component_data.drawing.clone()))
    }

    // fn next(&self) -> Vec<super::component::ComponentObject> {
    //     Vec::new()
    // }

    fn get_mask(&self) -> crate::models::bit_mode::BitMask {
        BitMask::default().allow(GameMode::Library)
    }
}
