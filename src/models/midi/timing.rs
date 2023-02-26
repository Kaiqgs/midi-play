use midly::num::{u15, u24};

pub struct TimingInformation {
    pub tempo: u24,          // microseconds per quarter note
    pub ticks_per_beat: u15, // division, ppq, ticks per quarter note
    pub us_per_tick: f64,
    pub sec_per_tick: f64,
}
