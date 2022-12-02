pub struct Recording {
    samples: Vec<String>,
}
impl Recording {
    pub fn new() -> Recording {
        Recording { samples: vec![] }
    }

    pub fn push(&mut self, sample: String) -> bool {
        unimplemented!()
    }
    pub fn iter(&self) {
        unimplemented!()
    }
    pub fn write(&self, filepath: String) -> Result<bool, bool> {
        unimplemented!()
    }
}
