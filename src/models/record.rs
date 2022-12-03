pub struct Recording {
    samples: Vec<String>,
}

impl Recording {
    pub fn new() -> Recording {
        Recording { samples: vec![] }
    }
}
