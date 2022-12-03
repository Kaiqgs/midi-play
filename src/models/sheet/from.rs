use super::track::Track;

pub trait SheetTransformer: Send {
    //TODO: proper input type;
    fn convert(&self, input: String) -> Track;
}
