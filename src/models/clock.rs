use std::{default::Default, ops::Div};

#[derive(Clone, Debug)]
pub struct ClockAny<T> {
    pub sec: T,
    pub tick: T,
    pub tick_per_sec: Option<T>,
}

impl<T: Default + Clone + Div<Output = T>> ClockAny<T> {
    pub fn new() -> Self {
        ClockAny {
            sec: T::default(),
            tick: T::default(),
            tick_per_sec: None,
        }
    }

    pub fn sec(mut self, sec: T) -> Self {
        self.sec = sec;
        self
    }

    pub fn tick(mut self, tick: T) -> Self {
        self.tick = tick;
        self
    }

    pub fn set(mut self, tick: T, sec: T) -> Self {
        self.tick = tick.clone();
        self.sec = sec.clone();
        self.tick_per_sec = Some(tick / sec);
        self
    }
}

pub type ClockFloat = ClockAny<f64>;
#[derive(Clone, Debug)]
pub struct Clock {
    pub sec: f64,
    pub tick: u32,
}
