use crate::models::config;

pub struct File {
    pub filepath: String,
    pub name: String,
    pub loaded: bool,
    pub bytes: Vec<u8>,
}

impl File {
    pub fn new(filepath: String) -> Self {
        let path = config::RELATIVE_RESOURCES.to_owned() + &filepath;
        let name = String::from("todo");
        File {
            filepath: path,
            name,
            loaded: false,
            bytes: vec![],
        }
    }
}
