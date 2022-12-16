use ggez::{
    context::Has,
};

use super::{component::{Component, RenderUtilObject, ComponentObject}, drawing::{Drawing, self, DrawResult}};
use crate::models::{playmeter::PlayMeter, draw_util::DrawUtil};

/// Draws player performance

pub struct PlayMeterComponentData {}

impl Component for PlayMeter {
    fn draw(&self, canvas: RenderUtilObject) -> DrawResult {
        unimplemented!()
    }

    fn next(&self) -> Vec<ComponentObject> {
        todo!()
    }
}
