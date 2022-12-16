use std::path::Path;
use crate::models::trackeable::Trackeable;

pub struct Track {
    pub filepath: String,
    pub name: String,
}

impl Track {
    pub fn new(filepath: String) -> Self {
        let path = Path::new(&filepath);
        let name = String::from("");

        Track { filepath, name }
    }
}
