use ggez::graphics::{Mesh, MeshBuilder};

use crate::{
    components::component::{Component, Drawing},
    models::draw_util::DrawObject,
};

/* Draws current playing track */
pub struct Track {
    name: Option<String>,
    filepath: Option<String>,
    loaded: bool,
}

impl Track {
    pub fn new() -> Self {
        Track {
            name: None,
            filepath: None,
            loaded: false,
        }
    }

    pub fn load_file() -> Result<bool, String> {
        unimplemented!()
    }

    // pub fn go_to(&mut self, time: u32) -> bool{
    //     unimplemented!()
    // }

    // pub fn set_loop(&mut self, range: Range<u32>) -> bool{
    //     unimplemented!()
    // }
}

impl Component<Mesh> for Track {
    fn draw(&self, canvas: DrawObject) -> Drawing<Mesh> {
        
        return Drawing::new(MeshBuilder::new().build());
    }
}
