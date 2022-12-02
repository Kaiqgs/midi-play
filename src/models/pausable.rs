pub trait Pausable {
    fn pause(&mut self) -> bool;
    fn resume(&mut self) -> bool;
}