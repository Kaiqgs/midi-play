use super::{Track};
pub trait SheetTransformer: Send {
    fn convert(&self) -> Track;
}


pub struct SheetTransform<T: SheetTransformer> {
    transformer: T,
}

impl<T: SheetTransformer> SheetTransform<T> {
    pub fn new(transformer: T) -> Self {
        SheetTransform { transformer }
    }
}
