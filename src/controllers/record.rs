use crate::models::{record::Recording, restartable::Restartable};
impl Recording {
    pub fn push(&mut self, _sample: String) -> bool {
        true
    }

}
impl Restartable for Recording{
    fn restart(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
