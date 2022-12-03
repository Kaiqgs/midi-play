use std::path::Path;

use async_trait::async_trait;

use crate::models::trackeable::Trackeable;

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

#[async_trait]
impl Trackeable for Track {
    async fn go_to(&mut self,time:u32) -> u32 {
        unimplemented!()
    }
        
    

    fn set_loop(&mut self,range:std::ops::Range<u32>) -> bool {
        unimplemented!()
    }
}
