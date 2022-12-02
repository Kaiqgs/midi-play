use std::ops::Range;

pub trait Trackeable {
    fn go_to(&mut self, time: u32) -> bool {
        unimplemented!()
    }

    fn set_loop(&mut self, range: Range<u32>) -> bool {
        unimplemented!()
    }
}
