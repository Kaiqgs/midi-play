use crate::models::{
    bit_mode::{BitMask, BitmaskSetup},
    input::input::MidiPlayInput,
};

use super::drawing::{DrawResult, Drawing, DrawingReference, RetrieveDrawing};
use crate::models::render_util::RenderUtil;

#[allow(unused_variables)]
pub trait Component {
    fn get_name(&self) -> String {
        String::from("[Component]")
    }
    fn update(&mut self, reutil: RenderUtil) {// -> Option<MidiPlayInput> {
        ()
    }
    fn draw(&self, reutil: RenderUtil) -> DrawResult {
        DrawResult::Skip
    }
    fn handle_input(&mut self, input: MidiPlayInput, reutil: RenderUtil) {
        ()
    }
    fn request_input(&mut self) -> Option<MidiPlayInput> {
        None
    }
    fn get_new_drawing(&self) -> Drawing {
        Drawing::default()
    }
    fn get_drawing(&self) -> RetrieveDrawing {
        RetrieveDrawing::Ok(DrawingReference::new(self.get_new_drawing()))
    }
    fn get_mask(&self) -> BitMask {
        BitMask::new(BitmaskSetup::All)
    }
    fn next(&self) -> Vec<ComponentObject> {
        Vec::new()
    }
    fn next_mut(&mut self) -> Vec<MutComponentObject> {
        Vec::new()
    }
}

pub type ComponentObject<'a> = &'a dyn Component;
pub type MutComponentObject<'a> = &'a mut dyn Component;
