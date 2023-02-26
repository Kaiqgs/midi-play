use crate::models::playmeter::{PlayMeter, QualityMeter};

impl QualityMeter for PlayMeter {
    fn compare(&mut self, _expected: u32, _received: u32) -> f64 {
        unimplemented!()
    }

    fn reset(&mut self) -> bool {
        unimplemented!()
    }
}
