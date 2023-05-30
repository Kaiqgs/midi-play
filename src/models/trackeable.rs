use std::ops::Range;

// use async_trait::async_trait;

// #[async_trait]
pub trait Trackeable {
    fn go_to(&mut self, tick: u32) -> u32;
    fn set_loop(&mut self, range: Range<u32>) -> bool;
}
