pub struct Recording {
    samples: Vec<String>,
}

impl Default for Recording {
    fn default() -> Self {
        Self::new()
    }
}

impl Recording {
    pub fn new() -> Recording {
        Recording { samples: vec![] }
    }
}
