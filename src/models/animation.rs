use std::time::Duration;

use lerp::Lerp;

pub struct Animation {
    pub start_t: Duration,
    pub duration: Duration,
    pub start: f32,
    pub end: f32,
    pub function: fn(f32) -> f32,
}

impl Animation {
    pub fn new(start_t: Duration, duration: Duration, start: f32, end: f32) -> Animation {
        Animation {
            start_t,
            duration,
            start,
            end,
            function: ezing::back_out,
        }
    }

    pub fn elapsed(&self, t: Duration) -> f32 {
        let elapsed = t - self.start_t;
        let elapsed = elapsed.as_secs_f32();
        let duration = self.duration.as_secs_f32();
        (elapsed / duration).clamp(0.0, 1.0)
    }

    pub fn animated(&self, t: Duration) -> f32 {
        let elapsed = (self.function)(self.elapsed(t));
        self.start.lerp(self.end, elapsed)
    }

    pub fn finished(&self, t: Duration) -> bool {
        self.elapsed(t) >= 1.0
    }
}

impl Default for Animation {
    fn default() -> Self {
        Animation {
            start_t: Duration::default(),
            duration: Duration::default(),
            start: 0.0,
            end: 0.0,
            function: ezing::back_out,
        }
    }
}
