use mockall::automock;

#[automock(
    type Input = u32;
    type Quality= f64;
)]
pub trait QualityMeter {
    type Input;
    type Quality;
    fn compare(&mut self, expected: Self::Input, received: Self::Input) -> Self::Quality;
    fn reset(&mut self) -> bool;
}

pub type QualityMeterObject = Box<dyn QualityMeter<Input = u32, Quality = f64>>;

pub struct PlayMeter {
    average_quality: f64,
}

impl PlayMeter {
    pub fn new() -> Self {
        return PlayMeter {
            average_quality: 1.0,
        };
    }
}
