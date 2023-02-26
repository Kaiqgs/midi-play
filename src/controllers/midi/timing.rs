use midly::num::{u15, u24};

use crate::models::midi::timing::TimingInformation;

impl Clone for TimingInformation {
    fn clone(&self) -> Self {
        TimingInformation {
            tempo: self.tempo,
            ticks_per_beat: self.ticks_per_beat,
            us_per_tick: self.us_per_tick,
            sec_per_tick: self.sec_per_tick,
        }
    }
}

impl Default for TimingInformation {
    fn default() -> Self {
        TimingInformation {
            tempo: u24::new(120),
            ticks_per_beat: u15::new(480),
            us_per_tick: 0.0,
            sec_per_tick: 0.0,
        }
    }
}
