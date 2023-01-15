use crate::models::sheet::SheetTrack;

pub trait SheetTransformer: Send + Sync {
    fn convert(&self) -> SheetTrack;
}
