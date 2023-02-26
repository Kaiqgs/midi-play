use std::fmt::Error;

use crate::models::record::Recording;

impl Recording {
    pub fn push(&mut self, _sample: String) -> bool {
        unimplemented!()
    }
    pub fn iter(&self) {
        unimplemented!()
    }
    pub fn write(&self, _filepath: String) -> Result<bool, Error> {
        unimplemented!()
    }
}
