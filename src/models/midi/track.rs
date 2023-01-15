use crate::models::{note::Note, trackeable::Trackeable};
use std::path::Path;

pub struct Track {
    pub filepath: String,
    pub name: String,
    pub loaded: bool,
    pub bytes: Vec<u8>,
}

impl Track {
    pub fn new(filepath: String) -> Self {
        //TODO: make ./resources not hardcoded;
        let path = "./resources/".to_owned() + &filepath;
        let name = String::from("");
        Track {
            filepath: path,
            name,
            loaded: false,
            bytes: vec![],
        }
    }
}
