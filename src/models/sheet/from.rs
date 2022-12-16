use crate::models::sheet::SheetTrack;


pub trait SheetTransformer: Send + Sync {
    //TODO: proper input type;
    fn convert(&self, input: String) -> SheetTrack;
}
