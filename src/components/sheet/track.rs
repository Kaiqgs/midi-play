use ggez::{graphics::{Mesh, MeshBuilder, GraphicsContext}, context::Has};
use mockall::{automock, mock};

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

#[automock]
pub trait MyGraphicsCtx {
    fn retrieve(&self) -> &GraphicsContext;
}


mock!{
    MyGraphicsContext {}
    impl Has<GraphicsContext> for MyGraphicsContext{
        fn retrieve(&self) -> &GraphicsContext;
    }
}

impl Component<Mesh> for Track {
    fn draw(&self, canvas: DrawObject, gfx: &impl Has<GraphicsContext>) -> Drawing<Mesh> {
        let mdata = MeshBuilder::new().build();
        let mesh = Mesh::from_data(gfx, mdata);
        Drawing::new(mesh)
    }
}
