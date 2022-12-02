use std::path::Path;

use super::Trackeable;
pub struct Track {
    filepath: String,
    name: String,
}

impl Track {
    pub fn new(filepath: String) -> Self {
        let path = Path::new(&filepath);
        let name = String::from("");

        return Track { filepath, name };
    }
}

impl Trackeable for Track {
    
}
