use crate::models::sheet::SheetTrack;

pub trait SheetTransformer: Send + Sync {
    fn convert(&self) -> SheetTrack;
}

pub trait SheetFromFile: Send + Sync {
    fn parse(&mut self, filepath: String) -> SheetTrack;
}
