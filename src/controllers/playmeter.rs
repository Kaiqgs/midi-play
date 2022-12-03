use crate::models::playmeter::{PlayMeter, QualityMeter};

impl QualityMeter for PlayMeter {
    type Input = u32;
    type Quality = f64;

    fn compare(&mut self, expected: Self::Input, received: Self::Input) -> Self::Quality {
        unimplemented!()
    }

    fn reset(&mut self) -> bool {
        unimplemented!()
    }
}
