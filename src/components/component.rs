use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use ggez::{
    context::Has,
    graphics::{Color, DrawParam, Image, MeshBuilder, Text},
    mint::Point2,
    Context,
};
use log::debug;

use super::drawing::{DrawError, DrawResult, Drawing, DrawingReference, RetrieveDrawing};
use crate::models::{render_util::RenderUtil, sheet::track_window_ctx::TrackWindowContext};

pub trait Component {
    fn get_name(&self) -> String {
        String::from("[Component]")
    }

    fn update(&mut self, canvas: RenderUtil) {
        ()
    }
    fn draw(&self, canvas: RenderUtil) -> DrawResult {
        DrawResult::Skip
    }
    fn get_new_drawing(&self) -> Drawing {
        Drawing::default()
    }
    fn get_drawing(&self) -> RetrieveDrawing {
        RetrieveDrawing::Ok(DrawingReference::new(self.get_new_drawing()))
    }
    fn next(&self) -> Vec<ComponentObject> {
        Vec::new()
    }
}

pub struct WindowContext {
    pub size: Point2<u32>,
    pub track: TrackWindowContext,
}

impl WindowContext {
    pub fn new(size: Point2<u32>, trackwinctx: Option<TrackWindowContext>) -> Self {
        WindowContext {
            size,
            track: trackwinctx.unwrap_or(TrackWindowContext::default()),
        }
    }
}

impl Clone for WindowContext {
    fn clone(&self) -> Self {
        WindowContext::new(self.size, Some(self.track.clone()))
    }
}

pub struct BuildContext<'a> {
    pub ctx: Option<&'a Context>,
    pub winctx: WindowContext,
}

impl<'a> BuildContext<'a> {
    pub fn new(
        ctx: Option<&'a Context>,
        canvas_size: Point2<u32>,
        trackwinctx: Option<TrackWindowContext>,
    ) -> Self {
        BuildContext {
            ctx,
            winctx: WindowContext::new(canvas_size, trackwinctx),
        }
    }
}

impl Clone for BuildContext<'_> {
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            winctx: self.winctx.clone(),
        }
    }
}

impl Default for BuildContext<'_> {
    fn default() -> Self {
        BuildContext {
            ctx: None,
            winctx: WindowContext::new(Point2 { x: 0, y: 0 }, None::<TrackWindowContext>),
        }
    }
}

pub type ComponentObject<'a> = &'a dyn Component;
pub type MutComponentObject<'a> = &'a mut dyn Component;
