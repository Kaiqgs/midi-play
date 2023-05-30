use std::time::Duration;

use ggez::mint::Point2;

use crate::components::sheet::sheet_component_const;

use super::{draw_state::DrawState, sheet::track_window_ctx::TrackWindowContext};

#[derive(Clone)]
pub struct WindowContext {
    pub size: Point2<u32>,
    pub trackwinctx: TrackWindowContext,
    pub delta: Duration,
    pub since_start: Duration,
    pub scale: f32,
    pub yoffset: f32,
    pub state: DrawState,
}

impl WindowContext {
    pub fn new(
        size: Point2<u32>,
        trackwinctx: Option<TrackWindowContext>,
        delta: Option<Duration>,
        since_start: Option<Duration>,
        scale: Option<f32>,
        state: Option<DrawState>,
    ) -> Self {
        WindowContext {
            size,
            trackwinctx: trackwinctx.unwrap_or(TrackWindowContext::default()),
            delta: delta.unwrap_or(Duration::from_secs(0)),
            since_start: since_start.unwrap_or(Duration::from_secs(0)),
            scale: scale.unwrap_or(sheet_component_const::SCALE as f32),
            yoffset: sheet_component_const::YOFFSET as f32,
            state: state.unwrap_or(DrawState::Ok),
        }
    }

    pub fn get_scaled_size(&self) -> Point2<f32> {
        self.to_scale(Point2::from([self.size.x as f32, self.size.y as f32]))
    }

    pub fn get_scaled(&self, p: Point2<f32>, convert_into: bool) -> Point2<f32> {
        Point2::from([
            if convert_into {
                p.x / self.scale
            } else {
                p.x * self.scale
            },
            if convert_into {
                p.y / self.scale
            } else {
                p.y * self.scale
            },
        ])
    }

    pub fn to_scale(&self, p: Point2<f32>) -> Point2<f32> {
        self.get_scaled(p, true)
    }

    pub fn from_scale(&self, p: Point2<f32>) -> Point2<f32> {
        self.get_scaled(p, false)
    }
}
