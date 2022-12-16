use std::{rc::Rc, sync::Arc};

use ggez::{
    context::Has,
    graphics::{Color, DrawParam, Image, MeshBuilder, Text},
    mint::Point2,
    Context,
};

use crate::models::{draw_util::DrawUtil, render_util::RenderUtil};

use super::drawing::{DrawResult, Drawing};

pub trait Component {
    fn draw(&self, canvas: RenderUtilObject) -> DrawResult;
    fn next(&self) -> Vec<ComponentObject> {
        Vec::new()
    }
}

pub struct BuildContext<'a> {
    pub ctx: Option<&'a Context>,
    pub canvas_size: Point2<u32>,
}

impl<'a> BuildContext<'a> {
    pub fn new(ctx: Option<&'a Context>, canvas_size: Point2<u32>) -> Self {
        BuildContext { ctx, canvas_size }
    }
}

impl Clone for BuildContext<'_> {
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            canvas_size: self.canvas_size.clone(),
        }
    }
}

impl Default for BuildContext<'_> {
    fn default() -> Self {
        BuildContext {
            ctx: None,
            canvas_size: Point2 { x: 0, y: 0 },
        }
    }
}

pub type RenderUtilObject<'a> = Rc<&'a dyn RenderUtil>;
pub type ComponentObject<'a> = Arc<&'a dyn Component>;
