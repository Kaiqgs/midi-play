
pub trait Restartable {
    fn restart(&mut self) -> Result<(), ()> {
        Err(())
    }
}
