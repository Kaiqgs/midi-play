use std::ops::Range;

use async_trait::async_trait;

#[async_trait]
pub trait Trackeable: Sync + Send {
    async fn go_to(&mut self, time: u32) -> u32;
    fn set_loop(&mut self, range: Range<u32>) -> bool;
}
