use ggez::{graphics::{Color}};
use mockall::automock;

use crate::components::{drawing::Drawing, component::BuildContext};

use super::note::Note;

#[automock]
pub trait DrawUtil {
    // fn horizontal_line(&self, mesh: &mut ggez::graphics::MeshBuilder, level: u32, width: f32, color: Color) -> bool;
       
    fn staff_block<'a>(draw:&'a mut Drawing, build: BuildContext<'a>, notes: Vec<&'a Note>, color: Color);

    fn left_image<'a>(draw:&'a mut Drawing, build: BuildContext<'a>, note: &'a Note);

    fn rect<'a>(draw:&'a mut Drawing, build: BuildContext<'a>, color: Color);
}
