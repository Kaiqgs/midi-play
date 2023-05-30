use ggez::{Context, mint::Point2};

use super::{window_context::WindowContext, sheet::track_window_ctx::TrackWindowContext};

#[derive(Clone)]
pub struct BuildContext<'a> {
    pub ctx: Option<&'a Context>,
    pub winctx: WindowContext,
}

impl<'a> BuildContext<'a> {
    pub fn new(ctx: Option<&'a Context>, winctx: WindowContext) -> Self {
        BuildContext { ctx, winctx }
    }
}


impl Default for BuildContext<'_> {
    fn default() -> Self {
        BuildContext {
            ctx: None,
            winctx: WindowContext::new(
                Point2 { x: 0, y: 0 },
                None::<TrackWindowContext>,
                None,
                None,
                None,
                None,
            ),
        }
    }
}
