use std::fmt::Error;

use crate::models::record::Recording;

impl Recording {
    pub fn push(&mut self, sample: String) -> bool {
        unimplemented!()
    }
    pub fn iter(&self) {
        unimplemented!()
    }
    pub fn write(&self, filepath: String) -> Result<bool, Error> {
        unimplemented!()
    }
}
