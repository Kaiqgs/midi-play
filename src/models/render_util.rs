use crate::models::window_context::WindowContext;

pub struct RenderUtil<'a> {
    pub winctx: &'a WindowContext,
}

impl<'a> RenderUtil<'a> {
    pub fn new(winctx: &'a WindowContext) -> Self {
        RenderUtil { winctx }
    }
}

impl<'a> Clone for RenderUtil<'a> {
    fn clone(&self) -> Self {
        RenderUtil::new(self.winctx)
    }
}
