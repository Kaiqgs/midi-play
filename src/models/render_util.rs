use std::time::Duration;

use crate::components::component::WindowContext;

pub struct RenderUtil<'a> {
    pub winctx: &'a WindowContext,
    pub delta: Duration,
}

impl<'a> RenderUtil<'a> {
    pub fn new(winctx: &'a WindowContext, delta: Duration) -> Self {
        RenderUtil { winctx, delta }
    }
}

impl<'a> Clone for RenderUtil<'a> {
    fn clone(&self) -> Self {
        RenderUtil::new(self.winctx, self.delta)
    }
}
